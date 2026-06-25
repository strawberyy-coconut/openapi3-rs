use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;
use crate::reference::RefOr;
use crate::schema::Schema;

/// The location of a parameter, as defined in §4.12.1 of the OpenAPI 3.2 spec.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    /// Path parameter — part of the operation's URL.
    Path,
    /// Query parameter — appended to the URL.
    Query,
    /// Querystring parameter — treats the entire query string as a value (3.2).
    Querystring,
    /// Header parameter — custom headers expected as part of the request.
    Header,
    /// Cookie parameter — used to pass a specific cookie value.
    Cookie,
}

/// Parameter serialization styles as defined in §4.12.3 of the OAS 3.2 spec.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    /// Path-style parameters defined by RFC6570 §3.2.7 (path only).
    Matrix,
    /// Label-style parameters defined by RFC6570 §3.2.5 (path only).
    Label,
    /// Simple-style parameters defined by RFC6570 §3.2.2 (path, header).
    Simple,
    /// Form-style parameters defined by RFC6570 §3.2.8 (query, cookie).
    Form,
    /// Space-delimited array/object values (query only).
    SpaceDelimited,
    /// Pipe-delimited array/object values (query only).
    PipeDelimited,
    /// Deep object representation for nested query parameters (query only).
    DeepObject,
    /// Cookie-style: analogous to form but following RFC6265 syntax.
    Cookie,
}

/// A [Parameter Object](https://spec.openapis.org/oas/latest.html#parameter-object)
/// as defined in §4.12 of the OpenAPI 3.2 specification.
///
/// Describes a single operation parameter. A unique parameter is defined by a
/// combination of a `name` and `in` (location).
///
/// The parameter MUST include either a `schema` field or a `content` field, but
/// not both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// **REQUIRED.** The name of the parameter. Case-sensitive.
    pub name: String,

    /// **REQUIRED.** The location of the parameter.
    #[serde(rename = "in")]
    pub location: ParameterIn,

    /// A brief description of the parameter. Supports CommonMark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this parameter is mandatory. If `in` is `path`, this MUST be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Specifies that a parameter is deprecated. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// If `true`, clients MAY pass a zero-length string value.
    /// **Deprecated.** Valid only for `query` parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,

    /// Describes how the parameter value will be serialized.
    /// Default depends on `in`: `form` for query/cookie, `simple` for path/header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,

    /// When `true`, array/object values generate separate parameters.
    /// Default depends on `style`: `true` for form/cookie, `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// When `true`, parameter values are serialized using reserved expansion
    /// as defined by RFC6570 §3.2.3. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,

    /// The schema defining the type used for the parameter.
    /// Mutually exclusive with `content`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,

    /// Example of the parameter's potential value. Mutually exclusive with `examples`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the parameter's potential value. Mutually exclusive with `example`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, RefOr<crate::example::Example>>>,

    /// A map containing the representations for the parameter.
    /// The key is the media type. The map MUST only contain one entry.
    /// Mutually exclusive with `schema`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IndexMap<String, crate::media_type::MediaType>>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Parameter {
    /// Create a new Parameter with the required `name` and `location`.
    pub fn new(name: impl Into<String>, location: ParameterIn) -> Self {
        Self {
            name: name.into(),
            location,
            description: None,
            required: None,
            deprecated: None,
            allow_empty_value: None,
            style: None,
            explode: None,
            allow_reserved: None,
            schema: None,
            example: None,
            examples: None,
            content: None,
            extensions: Extensions::default(),
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self {
            name: String::new(),
            location: ParameterIn::Query,
            description: None,
            required: None,
            deprecated: None,
            allow_empty_value: None,
            style: None,
            explode: None,
            allow_reserved: None,
            schema: None,
            example: None,
            examples: None,
            content: None,
            extensions: Extensions::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_param() {
        let json = r#"{
            "name": "limit",
            "in": "query",
            "schema": {"type": "integer"}
        }"#;
        let param: Parameter = serde_json::from_str(json).unwrap();
        assert_eq!(param.name, "limit");
        assert_eq!(param.location, ParameterIn::Query);
        assert!(param.schema.is_some());
    }

    #[test]
    fn test_path_param() {
        let json = r#"{
            "name": "petId",
            "in": "path",
            "required": true,
            "schema": {"type": "string"}
        }"#;
        let param: Parameter = serde_json::from_str(json).unwrap();
        assert_eq!(param.required, Some(true));
    }

    #[test]
    fn test_param_roundtrip() {
        let json = r#"{"name":"filter","in":"query","style":"form","explode":true,"schema":true}"#;
        let param: Parameter = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&param).unwrap();
        let _back: Parameter = serde_json::from_str(&output).unwrap();
    }
}
