use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;
use crate::header::Header;
use crate::media_type::MediaType;
use crate::reference::RefOr;

/// A [Response Object](https://spec.openapis.org/oas/latest.html#response-object)
/// as defined in §4.17 of the OpenAPI 3.2 specification.
///
/// Describes a single response from an API operation, including design-time,
/// static `links` to operations based on the response.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `description` | `string` | **REQUIRED.** A description of the response. Supports CommonMark. |
/// | `summary` | `string` | A short summary of the meaning of the response. |
/// | `headers` | `Map<string, Header>` | Maps a header name to its definition. |
/// | `content` | `Map<string, MediaType>` | A map containing descriptions of potential response payloads. |
/// | `links` | `Map<string, Link>` | A map of operations links that can be followed from the response. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// **REQUIRED.** A description of the response. Supports CommonMark markdown.
    pub description: String,

    /// A short summary of the meaning of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Maps a header name to its definition. Header names are case-insensitive.
    /// If defined with the name `"Content-Type"`, it SHALL be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<IndexMap<String, RefOr<Header>>>,

    /// A map containing descriptions of potential response payloads.
    /// The key is a media type or media type range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IndexMap<String, RefOr<MediaType>>>,

    /// A map of operations links that can be followed from the response.
    /// The key is a short name for the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, RefOr<crate::link::Link>>>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            description: String::new(),
            summary: None,
            headers: None,
            content: None,
            links: None,
            extensions: Extensions::default(),
        }
    }
}

impl Response {
    /// Create a new Response with the given description.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            summary: None,
            headers: None,
            content: None,
            links: None,
            extensions: Extensions::default(),
        }
    }
}

/// A [Responses Object](https://spec.openapis.org/oas/latest.html#responses-object)
/// as defined in §4.16 of the OpenAPI 3.2 specification.
///
/// A container for the expected responses of an operation. Maps HTTP response
/// codes to the expected response. The `default` key MAY be used as a default
/// response for all undeclared HTTP codes.
///
/// Response codes can be exact (`200`) or wildcard ranges (`2XX`). The keys
/// are strings (not integers) for JSON/YAML compatibility.
///
/// # Serde flatten behavior
///
/// The `default` field is a named fixed field (§4.16.1), while `responses` holds
/// patterned fields (§4.16.2). The `#[serde(flatten)]` on `responses` works
/// correctly because serde extracts the known `"default"` key first and places
/// all other keys into the map. There is no collision risk — the spec's patterned
/// fields are HTTP status codes (e.g., `"200"`, `"2XX"`), none of which equal
/// the literal string `"default"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific
    /// HTTP response codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<RefOr<Response>>,

    /// HTTP status codes as keys, mapped to Response Objects or References.
    /// Keys are strings (e.g., `"200"`, `"2XX"`, `"4XX"`).
    #[serde(flatten)]
    pub responses: IndexMap<String, RefOr<Response>>,
}

impl Responses {
    /// Create a new empty Responses object.
    pub fn new() -> Self {
        Self {
            default: None,
            responses: IndexMap::new(),
        }
    }
}

impl Default for Responses {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_minimal() {
        let json = r#"{"description": "A simple response"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
        assert_eq!(resp.description, "A simple response");
    }

    #[test]
    fn test_response_with_content() {
        let json = r#"{
            "description": "A pet",
            "content": {
                "application/json": {
                    "schema": {"type": "object"}
                }
            }
        }"#;
        let resp: Response = serde_json::from_str(json).unwrap();
        assert!(resp.content.is_some());
    }

    #[test]
    fn test_responses() {
        let json = r#"{
            "200": {"description": "OK"},
            "404": {"description": "Not found"},
            "default": {"description": "Error"}
        }"#;
        let responses: Responses = serde_json::from_str(json).unwrap();
        assert_eq!(responses.responses.len(), 2);
        assert!(responses.default.is_some());
        // Verify `default` was NOT placed into the responses map
        assert!(!responses.responses.contains_key("default"));
    }

    #[test]
    fn test_responses_roundtrip() {
        // Verify that `default` + patterned fields round-trip correctly
        // without collision between the named field and the flattened map
        let json = r#"{
            "200": {"description": "OK"},
            "default": {"description": "Error"},
            "2XX": {"description": "Success range"}
        }"#;
        let responses: Responses = serde_json::from_str(json).unwrap();
        assert_eq!(responses.responses.len(), 2);
        assert!(responses.default.is_some());
        assert!(responses.responses.contains_key("200"));
        assert!(responses.responses.contains_key("2XX"));
        assert!(!responses.responses.contains_key("default"));

        let output = serde_json::to_string(&responses).unwrap();
        let roundtripped: Responses = serde_json::from_str(&output).unwrap();
        assert_eq!(roundtripped.responses.len(), 2);
        assert!(roundtripped.default.is_some());
    }
}
