use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::encoding::Encoding;
use crate::extensions::Extensions;
use crate::reference::RefOr;
use crate::schema::Schema;

/// A [Media Type Object](https://spec.openapis.org/oas/latest.html#media-type-object)
/// as defined in §4.14 of the OpenAPI 3.2 specification.
///
/// Each Media Type Object describes content structured in accordance with the
/// media type identified by its key (e.g., `application/json`).
///
/// When `example` or `examples` are provided, the example SHOULD match the
/// specified schema. The `example` and `examples` fields are mutually exclusive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    /// A schema describing the complete content of the request/response/parameter/header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,

    /// A schema describing each item within a sequential media type (3.2).
    /// Supports streaming use cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_schema: Option<RefOr<Schema>>,

    /// Example of the media type. Mutually exclusive with `examples`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the media type. Mutually exclusive with `example`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, RefOr<crate::example::Example>>>,

    /// A map between a property name and its encoding information.
    /// Only applies to `multipart/*` and `application/x-www-form-urlencoded`.
    /// MUST NOT be present if `prefix_encoding` or `item_encoding` are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<IndexMap<String, Encoding>>,

    /// An array of positional encoding information (3.2).
    /// Only applies to `multipart/*`. MUST NOT be present if `encoding` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_encoding: Option<Vec<Encoding>>,

    /// A single Encoding Object for multiple array items (3.2).
    /// Only applies to `multipart/*`. MUST NOT be present if `encoding` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_encoding: Option<Encoding>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl MediaType {
    /// Create a new empty MediaType.
    pub fn new() -> Self {
        Self {
            schema: None,
            item_schema: None,
            example: None,
            examples: None,
            encoding: None,
            prefix_encoding: None,
            item_encoding: None,
            extensions: Extensions::default(),
        }
    }
}

impl Default for MediaType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_json() {
        let json = r#"{"schema": {"type": "object"}}"#;
        let mt: MediaType = serde_json::from_str(json).unwrap();
        assert!(mt.schema.is_some());
    }

    #[test]
    fn test_media_type_roundtrip() {
        let json = r#"{"schema": {"type": "array", "items": {"type": "string"}}}"#;
        let mt: MediaType = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&mt).unwrap();
        let _back: MediaType = serde_json::from_str(&output).unwrap();
    }
}
