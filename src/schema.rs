use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value};
use thiserror::Error;

use crate::discriminator::Discriminator;
use crate::external_docs::ExternalDocumentation;
use crate::xml::XML;

/// A [Schema Object](https://spec.openapis.org/oas/latest.html#schema-object)
/// as defined in §4.24 of the OpenAPI 3.2 specification.
///
/// The Schema Object allows the definition of input and output data types.
/// It is a superset of JSON Schema Specification Draft 2020-12.
///
/// This type stores the raw JSON Schema content as a `serde_json::Value`, while
/// providing typed access to the OAS-specific fields (`discriminator`, `xml`,
/// `external_docs`, and `example`).
///
/// # Boolean Schemas
///
/// JSON Schema 2020-12 allows boolean values (`true` or `false`) as valid schemas.
/// The `Bool` variant handles this case. A boolean schema has no OAS-specific fields.
///
/// # Optional validation
///
/// When the `validate` crate feature is enabled, the `Schema::new_validated` constructor
/// uses the `jsonschema` crate to validate the JSON Schema content before storing it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Schema {
    /// A full schema object with JSON Schema keywords and optional OAS-specific fields.
    Object(SchemaObject),
    /// A boolean schema (`true` allows everything, `false` allows nothing).
    Bool(bool),
}

/// The object form of a Schema, containing OAS-specific fields plus all JSON Schema
/// keywords collected in `schema_data`.
///
/// Any JSON key that is not one of the OAS-specific fields (`discriminator`, `xml`,
/// `external_docs`, `example`) is captured into `schema_data` via serde's flatten.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaObject {
    /// Adds support for polymorphism. The discriminator is a hint for which
    /// alternative schema is expected to validate the structure of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,

    /// Additional metadata to describe the XML representation of this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,

    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// A free-form field to include an example of an instance for this schema.
    /// **Deprecated** in favor of the JSON Schema `examples` keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,

    /// All JSON Schema keywords (`type`, `properties`, `items`, `oneOf`, `allOf`,
    /// `$ref`, `$schema`, `$id`, etc.) are captured here as a raw JSON object.
    /// Keys recognized as OAS fields above are excluded from this map.
    #[serde(flatten)]
    pub schema_data: JsonMap<String, Value>,
}

impl Default for SchemaObject {
    fn default() -> Self {
        Self {
            discriminator: None,
            xml: None,
            external_docs: None,
            example: None,
            schema_data: JsonMap::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NewValidatedError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] jsonschema::ValidationError<'static>),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error)
}

impl Schema {
    /// Returns `true` if this is a boolean schema.
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns `true` if this is an object schema.
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns the object schema if this is an `Object` variant.
    pub fn as_object(&self) -> Option<&SchemaObject> {
        match self {
            Self::Object(obj) => Some(obj),
            Self::Bool(_) => None,
        }
    }

    /// Returns the boolean value if this is a `Bool` variant.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            Self::Object(_) => None,
        }
    }

    pub fn new_validated(value: Value) -> Result<Self, NewValidatedError> {
        // First validate the schema itself is valid JSON Schema
        let _ = jsonschema::validator_for(&value)?;
        // Then deserialize
        let schema: Self = serde_json::from_value(value)?;
        Ok(schema)
    }

    /// Create a Schema from a raw JSON value (object or bool).
    ///
    /// Any OAS-specific fields present in the JSON will be extracted; everything
    /// else goes into `schema_data`.
    pub fn from_value(value: Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_schema() {
        let json = "true";
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert!(schema.is_bool());
        assert_eq!(schema.as_bool(), Some(true));

        let json = "false";
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert!(schema.is_bool());
    }

    #[test]
    fn test_empty_schema() {
        let json = "{}";
        let schema: Schema = serde_json::from_str(json).unwrap();
        assert!(schema.is_object());
        let obj = schema.as_object().unwrap();
        assert!(obj.schema_data.is_empty());
    }

    #[test]
    fn test_schema_with_type() {
        let json = r#"{"type": "string"}"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        let obj = schema.as_object().unwrap();
        assert_eq!(obj.schema_data.get("type").unwrap(), &Value::String("string".into()));
    }

    #[test]
    fn test_schema_with_oas_fields() {
        let json = r#"{
            "type": "object",
            "discriminator": {"propertyName": "petType"},
            "externalDocs": {"url": "https://example.com/docs"}
        }"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        let obj = schema.as_object().unwrap();
        assert!(obj.discriminator.is_some());
        assert_eq!(obj.discriminator.as_ref().unwrap().property_name, "petType");
        assert!(obj.external_docs.is_some());
        assert_eq!(obj.schema_data.get("type").unwrap(), &Value::String("object".into()));
    }

    #[test]
    fn test_schema_roundtrip() {
        let json = r#"{"type": "array", "items": {"type": "string"}, "xml": {"name": "items"}}"#;
        let schema: Schema = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&schema).unwrap();
        let back: Schema = serde_json::from_str(&output).unwrap();
        let obj = back.as_object().unwrap();
        assert_eq!(obj.schema_data.get("type").unwrap(), &Value::String("array".into()));
        assert!(obj.xml.is_some());
    }
}
