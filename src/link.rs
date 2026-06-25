use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::server::Server;

/// A [Link Object](https://spec.openapis.org/oas/latest.html#link-object)
/// as defined in §4.20 of the OpenAPI 3.2 specification.
///
/// Represents a possible design-time link for a response. The presence of a link
/// does not guarantee the caller's ability to successfully invoke it, rather it
/// provides a known relationship and traversal mechanism between responses and
/// other operations.
///
/// A linked operation MUST be identified using either an `operation_ref` or an
/// `operation_id`. These fields are mutually exclusive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// A URI reference to an OAS operation. Mutually exclusive with `operation_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,

    /// The name of an existing, resolvable OAS operation. Mutually exclusive with `operation_ref`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    /// A map representing parameters to pass to the linked operation.
    /// The key is the parameter name; the value can be a constant or a runtime
    /// expression (e.g., `$request.path.id`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IndexMap<String, serde_json::Value>>,

    /// A literal value or runtime expression to use as a request body when
    /// calling the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,

    /// A description of the link. Supports CommonMark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            operation_ref: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            description: None,
            server: None,
        }
    }
}

impl Link {
    /// Create a new Link identified by an `operation_id`.
    pub fn from_operation_id(id: impl Into<String>) -> Self {
        Self {
            operation_id: Some(id.into()),
            operation_ref: None,
            parameters: None,
            request_body: None,
            description: None,
            server: None,
        }
    }

    /// Create a new Link identified by an `operation_ref`.
    pub fn from_operation_ref(ref_path: impl Into<String>) -> Self {
        Self {
            operation_ref: Some(ref_path.into()),
            operation_id: None,
            parameters: None,
            request_body: None,
            description: None,
            server: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_operation_id() {
        let json = r#"{"operationId": "getUserAddress"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert_eq!(link.operation_id.as_deref(), Some("getUserAddress"));
    }

    #[test]
    fn test_link_operation_ref() {
        let json = r###"{"operationRef": "#/paths/~1users~1{id}/get"}"###;
        let link: Link = serde_json::from_str(json).unwrap();
        assert!(link.operation_ref.is_some());
    }

    #[test]
    fn test_link_with_params() {
        let json = r#"{
            "operationId": "getUserAddress",
            "parameters": {
                "userId": "$request.path.id"
            }
        }"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert!(link.parameters.is_some());
    }
}
