use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::MediaType;
use crate::extensions::Extensions;
use crate::reference::RefOr;
use crate::schema::Schema;

/// A [Header Object](https://spec.openapis.org/oas/latest.html#header-object)
/// as defined in §4.21 of the OpenAPI 3.2 specification.
///
/// Describes a single header for HTTP responses and for individual parts in
/// multipart representations. The Header Object follows the structure of the
/// Parameter Object, with the following changes:
///
/// 1. `name` MUST NOT be specified (it is given in the corresponding `headers` map)
/// 2. `in` MUST NOT be specified (it is implicitly `header`)
/// 3. `style` is limited to `"simple"`
/// 4. `allow_empty_value` MUST NOT be used
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// A brief description of the header. Supports CommonMark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this header is mandatory. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Specifies that the header is deprecated. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// Describes how the header value will be serialized.
    /// The default (and only legal value for headers) is `"simple"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// When `true`, header values of type array/object generate a single header
    /// with comma-separated values. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// The schema defining the type used for the header.
    /// Mutually exclusive with `content`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<Schema>>,

    /// Example of the header's potential value. Mutually exclusive with `examples`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the header's potential value. Mutually exclusive with `example`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, RefOr<crate::example::Example>>>,

    /// A map containing the representations for the header.
    /// The map MUST only contain one entry.
    /// Mutually exclusive with `schema`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IndexMap<String, RefOr<MediaType>>>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            description: None,
            required: None,
            deprecated: None,
            style: None,
            explode: None,
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
    fn test_header_simple() {
        let json = r#"{"description": "The number of allowed requests", "schema": {"type": "integer"}}"#;
        let header: Header = serde_json::from_str(json).unwrap();
        assert!(header.schema.is_some());
    }

    #[test]
    fn test_header_roundtrip() {
        let json = r#"{"required":true,"schema":{"type":"string"},"example":"xyzzy"}"#;
        let header: Header = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&header).unwrap();
        let _back: Header = serde_json::from_str(&output).unwrap();
    }
}
