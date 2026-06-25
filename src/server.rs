use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;

/// A [Server Object](https://spec.openapis.org/oas/latest.html#server-object)
/// as defined in §4.5 of the OpenAPI 3.2 specification.
///
/// An object representing a Server.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `url` | `string` | **REQUIRED.** A URL to the target host. Supports Server Variables. |
/// | `description` | `string` | An optional string describing the host. Supports CommonMark. |
/// | `name` | `string` | An optional unique string to refer to the host. |
/// | `variables` | `Map<string, ServerVariable>` | A map of Server Variables for URL template substitution. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    /// A URL to the target host. Supports variable substitution with `{braces}`.
    /// MAY be relative. Query and fragment MUST NOT be part of this URL.
    pub url: String,

    /// An optional string describing the host. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An optional unique string to refer to the host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A map between a variable name and its value for URL template substitution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<IndexMap<String, ServerVariable>>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            url: String::new(),
            description: None,
            name: None,
            variables: None,
            extensions: Extensions::default(),
        }
    }
}

impl Server {
    /// Create a new Server with the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
            name: None,
            variables: None,
            extensions: Extensions::default(),
        }
    }

    /// Create a default localhost server (`url: "/"`).
    pub fn default_local() -> Self {
        Self::new("/")
    }
}

/// A [Server Variable Object](https://spec.openapis.org/oas/latest.html#server-variable-object)
/// as defined in §4.6 of the OpenAPI 3.2 specification.
///
/// An object representing a Server Variable for server URL template substitution.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `enum` | `[string]` | An enumeration of string values to be used for substitution. |
/// | `default` | `string` | **REQUIRED.** The default value to use for substitution. |
/// | `description` | `string` | An optional description for the server variable. Supports CommonMark. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerVariable {
    /// An enumeration of string values for substitution. MUST NOT be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,

    /// The default value to use for substitution. If `enum` is defined,
    /// this value MUST exist in the enum's values.
    pub default: String,

    /// An optional description for the server variable. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl ServerVariable {
    /// Create a new ServerVariable with the given default value.
    pub fn new(default: impl Into<String>) -> Self {
        Self {
            default: default.into(),
            r#enum: None,
            description: None,
            extensions: Extensions::default(),
        }
    }
}

impl Default for ServerVariable {
    fn default() -> Self {
        Self {
            default: String::new(),
            r#enum: None,
            description: None,
            extensions: Extensions::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_minimal() {
        let json = r#"{"url": "https://api.example.com"}"#;
        let server: Server = serde_json::from_str(json).unwrap();
        assert_eq!(server.url, "https://api.example.com");
    }

    #[test]
    fn test_server_with_variables() {
        let json = r#"{
            "url": "https://{username}.example.com:{port}/{basePath}",
            "variables": {
                "username": {
                    "default": "demo"
                },
                "port": {
                    "enum": ["8443", "443"],
                    "default": "8443"
                }
            }
        }"#;
        let server: Server = serde_json::from_str(json).unwrap();
        let vars = server.variables.as_ref().unwrap();
        assert_eq!(
            vars.get("username").unwrap().default,
            "demo"
        );
    }

    #[test]
    fn test_server_roundtrip() {
        let json = r#"{"url": "/", "description": "Default server"}"#;
        let server: Server = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&server).unwrap();
        let _back: Server = serde_json::from_str(&output).unwrap();
    }
}
