use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::components::Components;
use crate::external_docs::ExternalDocumentation;
use crate::info::Info;
use crate::paths::{PathItem, Paths};
use crate::security::SecurityRequirement;
use crate::server::Server;
use crate::tag::Tag;

/// An [OpenAPI Object](https://spec.openapis.org/oas/latest.html#openapi-object)
/// as defined in §4.1 of the OpenAPI 3.2 specification.
///
/// This is the root object of the OpenAPI Description.
///
/// In addition to the required fields, at least one of `components`, `paths`,
/// or `webhooks` MUST be present.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `openapi` | `string` | **REQUIRED.** The version number of the OAS (e.g., `"3.2.0"`). |
/// | `info` | `Info` | **REQUIRED.** Provides metadata about the API. |
/// | `$self` | `string` | The self-assigned URI of this document (3.2). |
/// | `json_schema_dialect` | `string` | Default `$schema` for Schema Objects within this OAS document. |
/// | `servers` | `[Server]` | An array of Server Objects. Default is `[{"url": "/"}]`. |
/// | `paths` | `Paths` | The available paths and operations for the API. |
/// | `webhooks` | `Map<string, PathItem>` | Incoming webhooks that MAY be received (3.1+). |
/// | `components` | `Components` | An element to hold various Objects for the OAD. |
/// | `security` | `[SecurityRequirement]` | A declaration of which security mechanisms can be used. |
/// | `tags` | `[Tag]` | A list of tags used by the OAD with additional metadata. |
/// | `external_docs` | `ExternalDocumentation` | Additional external documentation. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    /// **REQUIRED.** The version number of the OpenAPI Specification being used.
    /// Example: `"3.2.0"`.
    pub openapi: String,

    /// **REQUIRED.** Provides metadata about the API.
    pub info: Info,

    /// The self-assigned URI of this document, which also serves as its base URI
    /// in accordance with RFC3986 §5.1.1 (added in OAS 3.2).
    #[serde(rename = "$self", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,

    /// The default value for the `$schema` keyword within Schema Objects contained
    /// within this OAS document. MUST be a URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<String>,

    /// An array of Server Objects providing connectivity information to a target
    /// server. If absent or empty, defaults to `[{"url": "/"}]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// The available paths and operations for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Paths>,

    /// Incoming webhooks that MAY be received as part of this API (added in OAS 3.1).
    /// Closely related to the callbacks feature. The key name is a unique string
    /// to refer to each webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<IndexMap<String, PathItem>>,

    /// An element to hold various Objects for the OpenAPI Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    /// A declaration of which security mechanisms can be used across the API.
    /// Only one Security Requirement Object needs to be satisfied to authorize.
    /// An empty security requirement `{}` makes security optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,

    /// A list of tags used by the OAD with additional metadata. Each tag name
    /// in the list MUST be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl Default for OpenAPI {
    fn default() -> Self {
        Self {
            openapi: String::new(),
            info: Info {
                title: String::new(),
                version: String::new(),
                summary: None,
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
            },
            self_uri: None,
            json_schema_dialect: None,
            servers: None,
            paths: None,
            webhooks: None,
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        }
    }
}

impl OpenAPI {
    /// Create a new OpenAPI object with the required `openapi` version and `info`.
    pub fn new(openapi_version: impl Into<String>, info: Info) -> Self {
        Self {
            openapi: openapi_version.into(),
            info,
            self_uri: None,
            json_schema_dialect: None,
            servers: None,
            paths: None,
            webhooks: None,
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_minimal() {
        let json = r#"{
            "openapi": "3.2.0",
            "info": {
                "title": "Minimal API",
                "version": "1.0"
            }
        }"#;
        let api: OpenAPI = serde_json::from_str(json).unwrap();
        assert_eq!(api.openapi, "3.2.0");
        assert_eq!(api.info.title, "Minimal API");
    }

    #[test]
    fn test_openapi_with_paths() {
        let json = r#"{
            "openapi": "3.2.0",
            "info": {"title": "Pet Store", "version": "1.0"},
            "paths": {
                "/pets": {
                    "get": {
                        "summary": "List all pets",
                        "responses": {
                            "200": {"description": "A list of pets"}
                        }
                    }
                }
            }
        }"#;
        let api: OpenAPI = serde_json::from_str(json).unwrap();
        let paths = api.paths.as_ref().unwrap();
        assert!(paths.contains_key("/pets"));
    }

    #[test]
    fn test_openapi_roundtrip() {
        let json = r#"{
            "openapi": "3.2.0",
            "info": {"title": "Test", "version": "0.1"},
            "servers": [{"url": "https://api.example.com"}]
        }"#;
        let api: OpenAPI = serde_json::from_str(json).unwrap();
        let output = serde_json::to_string(&api).unwrap();
        let _back: OpenAPI = serde_json::from_str(&output).unwrap();
    }
}
