use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value};
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
    ///
    /// **Deprecated** as of OAS 3.1 / JSON Schema 2020-12 in favor of the
    /// JSON Schema `examples` (plural) keyword. Use [`SchemaObject::examples`]
    /// to access the new keyword, or embed examples directly in `schema_data`.
    /// See [spec §4.24.2](https://spec.openapis.org/oas/latest.html#schema-object).
    #[deprecated(
        since = "0.1.0",
        note = "Use the JSON Schema `examples` (plural) keyword instead (OAS §4.24.2)"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,

    /// All JSON Schema keywords (`type`, `properties`, `items`, `oneOf`, `allOf`,
    /// `$ref`, `$schema`, `$id`, etc.) are captured here as a raw JSON object.
    /// Keys recognized as OAS fields above are excluded from this map.
    ///
    /// Note: the JSON Schema `$ref` keyword lives in `schema_data`. This is
    /// distinct from the OAS [Reference Object](https://spec.openapis.org/oas/latest.html#reference-object)
    /// (used by [`RefOr<T>`](crate::RefOr)), which *only* has `$ref`, `summary`,
    /// and `description` and cannot contain other JSON Schema keywords alongside
    /// `$ref`.  A Schema Object with `$ref` plus other keywords (like
    /// `description`) stores `$ref` here in `schema_data`.
    #[serde(flatten)]
    pub schema_data: JsonMap<String, Value>,
}

impl SchemaObject {
    /// Returns the JSON Schema `type` field, if present.
    pub fn schema_type(&self) -> Option<&str> {
        self.schema_data.get("type").and_then(|v| v.as_str())
    }

    /// Returns the JSON Schema `type` field as a list, if it's an array.
    pub fn schema_type_list(&self) -> Option<Vec<&str>> {
        self.schema_data.get("type")?.as_array()?
            .iter()
            .map(|v| v.as_str())
            .collect()
    }

    /// Returns true if this schema has a `$ref`.
    pub fn has_ref(&self) -> bool {
        self.schema_data.contains_key("$ref")
    }

    /// Get the `$ref` value, if present.
    pub fn ref_path(&self) -> Option<&str> {
        self.schema_data.get("$ref").and_then(|v| v.as_str())
    }

    /// Returns the `$schema` dialect URI, if present.
    pub fn schema_dialect(&self) -> Option<&str> {
        self.schema_data.get("$schema").and_then(|v| v.as_str())
    }

    /// Returns the `format` keyword value, if present.
    pub fn format(&self) -> Option<&str> {
        self.schema_data.get("format").and_then(|v| v.as_str())
    }

    /// Returns the `description` keyword value, if present.
    pub fn description(&self) -> Option<&str> {
        self.schema_data.get("description").and_then(|v| v.as_str())
    }

    // ── Compound keywords ──────────────────────────────────────────

    /// Returns the `properties` map, if present.
    pub fn properties(&self) -> Option<&JsonMap<String, Value>> {
        self.schema_data.get("properties")?.as_object()
    }

    /// Returns the `items` subschema, if present.
    pub fn items(&self) -> Option<&Value> {
        self.schema_data.get("items")
    }

    /// Returns the `additionalProperties` value, if present.
    /// Can be a boolean (`true`/`false`) or a Schema Object.
    pub fn additional_properties(&self) -> Option<&Value> {
        self.schema_data.get("additionalProperties")
    }

    /// Returns the `oneOf` array, if present.
    pub fn one_of(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("oneOf")?.as_array()
    }

    /// Returns the `allOf` array, if present.
    pub fn all_of(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("allOf")?.as_array()
    }

    /// Returns the `anyOf` array, if present.
    pub fn any_of(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("anyOf")?.as_array()
    }

    /// Returns the `prefixItems` array, if present (JSON Schema 2020-12).
    pub fn prefix_items(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("prefixItems")?.as_array()
    }

    /// Returns the `required` array as string slices, if present.
    pub fn required(&self) -> Option<Vec<&str>> {
        self.schema_data.get("required")?
            .as_array()?
            .iter()
            .map(|v| v.as_str())
            .collect()
    }

    /// Returns the `enum` array, if present.
    pub fn enum_values(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("enum")?.as_array()
    }

    /// Returns the `const` value, if present.
    pub fn const_value(&self) -> Option<&Value> {
        self.schema_data.get("const")
    }

    /// Returns the `examples` array (JSON Schema 2020-12 plural form), if present.
    pub fn examples(&self) -> Option<&Vec<Value>> {
        self.schema_data.get("examples")?.as_array()
    }

    /// Returns true if `nullable: true` is set (pre-2020-12 compatibility).
    pub fn is_nullable(&self) -> bool {
        self.schema_data
            .get("nullable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns `true` if `readOnly: true` is set.
    pub fn is_read_only(&self) -> bool {
        self.schema_data
            .get("readOnly")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns `true` if `writeOnly: true` is set.
    pub fn is_write_only(&self) -> bool {
        self.schema_data
            .get("writeOnly")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    // ── Numeric constraint keywords ─────────────────────────────────

    /// Returns the `minimum` value, if present.
    pub fn minimum(&self) -> Option<f64> {
        self.schema_data.get("minimum")?.as_f64()
    }

    /// Returns the `maximum` value, if present.
    pub fn maximum(&self) -> Option<f64> {
        self.schema_data.get("maximum")?.as_f64()
    }

    /// Returns the `exclusiveMinimum` value, if present.
    pub fn exclusive_minimum(&self) -> Option<f64> {
        self.schema_data.get("exclusiveMinimum")?.as_f64()
    }

    /// Returns the `exclusiveMaximum` value, if present.
    pub fn exclusive_maximum(&self) -> Option<f64> {
        self.schema_data.get("exclusiveMaximum")?.as_f64()
    }

    /// Returns the `multipleOf` value, if present.
    pub fn multiple_of(&self) -> Option<f64> {
        self.schema_data.get("multipleOf")?.as_f64()
    }

    // ── String constraint keywords ──────────────────────────────────

    /// Returns the `minLength` value, if present.
    pub fn min_length(&self) -> Option<u64> {
        self.schema_data.get("minLength")?.as_u64()
    }

    /// Returns the `maxLength` value, if present.
    pub fn max_length(&self) -> Option<u64> {
        self.schema_data.get("maxLength")?.as_u64()
    }

    /// Returns the `pattern` value, if present.
    pub fn pattern(&self) -> Option<&str> {
        self.schema_data.get("pattern")?.as_str()
    }

    /// Returns the `contentMediaType` value, if present.
    pub fn content_media_type(&self) -> Option<&str> {
        self.schema_data.get("contentMediaType")?.as_str()
    }

    /// Returns the `contentEncoding` value, if present.
    pub fn content_encoding(&self) -> Option<&str> {
        self.schema_data.get("contentEncoding")?.as_str()
    }

    // ── Array constraint keywords ───────────────────────────────────

    /// Returns the `minItems` value, if present.
    pub fn min_items(&self) -> Option<u64> {
        self.schema_data.get("minItems")?.as_u64()
    }

    /// Returns the `maxItems` value, if present.
    pub fn max_items(&self) -> Option<u64> {
        self.schema_data.get("maxItems")?.as_u64()
    }

    /// Returns `true` if `uniqueItems: true` is set.
    pub fn has_unique_items(&self) -> bool {
        self.schema_data
            .get("uniqueItems")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    // ── Object constraint keywords ──────────────────────────────────

    /// Returns the `minProperties` value, if present.
    pub fn min_properties(&self) -> Option<u64> {
        self.schema_data.get("minProperties")?.as_u64()
    }

    /// Returns the `maxProperties` value, if present.
    pub fn max_properties(&self) -> Option<u64> {
        self.schema_data.get("maxProperties")?.as_u64()
    }

    // ── Meta keywords ───────────────────────────────────────────────

    /// Returns the `title` keyword value, if present.
    pub fn title(&self) -> Option<&str> {
        self.schema_data.get("title")?.as_str()
    }

    /// Returns the `default` keyword value, if present.
    pub fn default(&self) -> Option<&Value> {
        self.schema_data.get("default")
    }

    /// Returns the `$id` value, if present.
    pub fn id(&self) -> Option<&str> {
        self.schema_data.get("$id")?.as_str()
    }

    /// Returns true if `deprecated: true` is set (JSON Schema 2020-12).
    pub fn is_deprecated(&self) -> bool {
        self.schema_data
            .get("deprecated")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    // ── Mutable access for programmatic construction ────────────────

    /// Insert a raw JSON Schema keyword into `schema_data`.
    ///
    /// The key should use the JSON Schema field name (e.g., `"minLength"`, `"oneOf"`).
    pub fn insert_keyword(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        self.schema_data.insert(key.into(), value)
    }

    /// Returns a mutable reference to `schema_data` for direct manipulation.
    pub fn schema_data_mut(&mut self) -> &mut JsonMap<String, Value> {
        &mut self.schema_data
    }
}

impl SchemaObject {
    /// Create a new empty SchemaObject (no type, no constraints).
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Create a SchemaObject with the given JSON Schema `type`.
    ///
    /// ```
    /// use openapi3_rs::SchemaObject;
    /// let obj = SchemaObject::with_type("string");
    /// assert_eq!(obj.schema_type(), Some("string"));
    /// ```
    pub fn with_type(typ: impl Into<String>) -> Self {
        let mut data = JsonMap::new();
        data.insert("type".into(), Value::String(typ.into()));
        Self {
            schema_data: data,
            ..<Self as Default>::default()
        }
    }

    /// Create a SchemaObject that references another schema via `$ref`.
    ///
    /// ```
    /// use openapi3_rs::SchemaObject;
    /// let obj = SchemaObject::with_ref("#/components/schemas/Pet");
    /// assert_eq!(obj.ref_path(), Some("#/components/schemas/Pet"));
    /// ```
    pub fn with_ref(ref_path: impl Into<String>) -> Self {
        let mut data = JsonMap::new();
        data.insert("$ref".into(), Value::String(ref_path.into()));
        Self {
            schema_data: data,
            ..<Self as Default>::default()
        }
    }
}

impl Default for SchemaObject {
    #[allow(deprecated)]
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

#[cfg(feature = "validate")]
#[derive(Debug, thiserror::Error)]
pub enum NewValidatedError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] jsonschema::ValidationError<'static>),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error)
}

impl From<SchemaObject> for Schema {
    fn from(obj: SchemaObject) -> Self {
        Self::Object(obj)
    }
}

impl From<bool> for Schema {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
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

    #[cfg(feature = "validate")]
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
