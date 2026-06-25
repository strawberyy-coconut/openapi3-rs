use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::media_type::MediaType;
use crate::reference::RefOr;

/// A [Request Body Object](https://spec.openapis.org/oas/latest.html#request-body-object)
/// as defined in §4.13 of the OpenAPI 3.2 specification.
///
/// Describes a single request body.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `description` | `string` | A brief description of the request body. Supports CommonMark. |
/// | `content` | `Map<string, MediaType>` | **REQUIRED.** The content of the request body. |
/// | `required` | `boolean` | Whether the request body is required. Default is `false`. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    /// A brief description of the request body. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// **REQUIRED.** The content of the request body. Maps media types to their
    /// descriptions. The map SHOULD have at least one entry.
    pub content: IndexMap<String, MediaType>,

    /// Whether the request body is required in the request. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl RequestBody {
    /// Create a new RequestBody with the given content.
    pub fn new(content: IndexMap<String, MediaType>) -> Self {
        Self {
            content,
            description: None,
            required: None,
        }
    }
}

impl Default for RequestBody {
    fn default() -> Self {
        Self {
            description: None,
            content: IndexMap::new(),
            required: None,
        }
    }
}

/// A type alias for request body references used in Operation objects.
pub type RequestBodyRef = RefOr<RequestBody>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_body_json() {
        let json = r#"{
            "content": {
                "application/json": {
                    "schema": {"type": "object"}
                }
            }
        }"#;
        let body: RequestBody = serde_json::from_str(json).unwrap();
        assert!(body.content.contains_key("application/json"));
    }

    #[test]
    fn test_request_body_ref() {
        let json = r###"{"$ref": "#/components/requestBodies/PetBody"}"###;
        let body: RequestBodyRef = serde_json::from_str(json).unwrap();
        assert!(body.is_ref());
    }
}
