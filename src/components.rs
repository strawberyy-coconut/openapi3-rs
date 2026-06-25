use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::callback::Callback;
use crate::example::Example;
use crate::header::Header;
use crate::link::Link;
use crate::media_type::MediaType;
use crate::parameter::Parameter;
use crate::paths::PathItem;
use crate::reference::RefOr;
use crate::request_body::RequestBody;
use crate::response::Response;
use crate::schema::Schema;
use crate::security::SecurityScheme;

/// A [Components Object](https://spec.openapis.org/oas/latest.html#components-object)
/// as defined in §4.7 of the OpenAPI 3.2 specification.
///
/// Holds a set of reusable objects for different aspects of the OAS. All objects
/// defined within the Components Object have no effect on the API unless they are
/// explicitly referenced from outside the Components Object.
///
/// All map keys MUST match the regular expression: `^[a-zA-Z0-9\\.\\-_]+$`
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `schemas` | `Map<string, Schema>` | Reusable Schema Objects. |
/// | `responses` | `Map<string, Response \| Reference>` | Reusable Response Objects. |
/// | `parameters` | `Map<string, Parameter \| Reference>` | Reusable Parameter Objects. |
/// | `examples` | `Map<string, Example \| Reference>` | Reusable Example Objects. |
/// | `request_bodies` | `Map<string, RequestBody \| Reference>` | Reusable Request Body Objects. |
/// | `headers` | `Map<string, Header \| Reference>` | Reusable Header Objects. |
/// | `security_schemes` | `Map<string, SecurityScheme \| Reference>` | Reusable Security Scheme Objects. |
/// | `links` | `Map<string, Link \| Reference>` | Reusable Link Objects. |
/// | `callbacks` | `Map<string, Callback \| Reference>` | Reusable Callback Objects. |
/// | `path_items` | `Map<string, PathItem>` | Reusable Path Item Objects. |
/// | `media_types` | `Map<string, MediaType \| Reference>` | Reusable Media Type Objects (3.2). |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// Reusable Schema Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<IndexMap<String, Schema>>,

    /// Reusable Response Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<IndexMap<String, RefOr<Response>>>,

    /// Reusable Parameter Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<IndexMap<String, RefOr<Parameter>>>,

    /// Reusable Example Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IndexMap<String, RefOr<Example>>>,

    /// Reusable Request Body Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<IndexMap<String, RefOr<RequestBody>>>,

    /// Reusable Header Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<IndexMap<String, RefOr<Header>>>,

    /// Reusable Security Scheme Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<IndexMap<String, RefOr<SecurityScheme>>>,

    /// Reusable Link Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<IndexMap<String, RefOr<Link>>>,

    /// Reusable Callback Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<IndexMap<String, RefOr<Callback>>>,

    /// Reusable Path Item Objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_items: Option<IndexMap<String, PathItem>>,

    /// Reusable Media Type Objects (added in OAS 3.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_types: Option<IndexMap<String, RefOr<MediaType>>>,
}

impl Components {
    /// Create a new empty Components object.
    pub fn new() -> Self {
        Self {
            schemas: None,
            responses: None,
            parameters: None,
            examples: None,
            request_bodies: None,
            headers: None,
            security_schemes: None,
            links: None,
            callbacks: None,
            path_items: None,
            media_types: None,
        }
    }
}

impl Default for Components {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components_empty() {
        let json = r#"{}"#;
        let c: Components = serde_json::from_str(json).unwrap();
        assert!(c.schemas.is_none());
    }

    #[test]
    fn test_components_with_schemas() {
        let json = r#"{
            "schemas": {
                "Pet": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"}
                    }
                }
            }
        }"#;
        let c: Components = serde_json::from_str(json).unwrap();
        let schemas = c.schemas.as_ref().unwrap();
        assert!(schemas.contains_key("Pet"));
    }
}
