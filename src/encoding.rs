use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;
use crate::reference::RefOr;

/// An [Encoding Object](https://spec.openapis.org/oas/latest.html#encoding-object)
/// as defined in §4.15 of the OpenAPI 3.2 specification.
///
/// A single encoding definition applied to a single value, with the mapping of
/// Encoding Objects to values determined by the Media Type Object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    /// The Content-Type for encoding a specific property. Can be a comma-separated
    /// list of media types (e.g., `image/png, image/jpeg`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// A map allowing additional information to be provided as headers.
    /// `Content-Type` is described separately and SHALL be ignored here.
    /// Ignored if the media type is not `multipart/*`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<IndexMap<String, RefOr<crate::header::Header>>>,

    /// Describes how a specific property value will be serialized.
    /// Ignored if the media type is not `application/x-www-form-urlencoded`
    /// or `multipart/form-data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::parameter::Style>,

    /// When `true`, array/object values generate separate parameters.
    /// For `style: "form"`, default is `true`. For all others, default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// When `true`, parameter values use reserved expansion (RFC6570 §3.2.3).
    /// Default is `false`. Ignored if media type is not url-encoded or multipart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,

    /// Nested encoding by name (3.2). Applied to `multipart/form-data` and
    /// `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<IndexMap<String, Encoding>>,

    /// Nested encoding by position (3.2). Applied to `multipart/*`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_encoding: Option<Vec<Encoding>>,

    /// Nested item encoding (3.2). Applied to `multipart/*` for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_encoding: Option<Box<Encoding>>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Encoding {
    /// Create a new empty Encoding.
    pub fn new() -> Self {
        Self {
            content_type: None,
            headers: None,
            style: None,
            explode: None,
            allow_reserved: None,
            encoding: None,
            prefix_encoding: None,
            item_encoding: None,
            extensions: Extensions::default(),
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_default() {
        let json = r#"{}"#;
        let enc: Encoding = serde_json::from_str(json).unwrap();
        assert!(enc.content_type.is_none());
    }

    #[test]
    fn test_encoding_content_type() {
        let json = r#"{"contentType": "image/png, image/jpeg"}"#;
        let enc: Encoding = serde_json::from_str(json).unwrap();
        assert_eq!(enc.content_type.as_deref(), Some("image/png, image/jpeg"));
    }
}
