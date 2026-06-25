use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;

/// The type of a [Security Scheme Object](https://spec.openapis.org/oas/latest.html#security-scheme-object)
/// as defined in §4.27.1 of the OpenAPI 3.2 specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    /// API key-based authentication.
    ApiKey,
    /// HTTP authentication (e.g., Basic, Bearer).
    Http,
    /// Mutual TLS certificate authentication.
    MutualTLS,
    /// OAuth 2.0 authentication.
    Oauth2,
    /// OpenID Connect authentication.
    OpenIdConnect,
}

/// The location of an API key, as defined in §4.27.1 of the OpenAPI 3.2 spec.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyLocation {
    /// API key in the query string.
    Query,
    /// API key in a request header.
    Header,
    /// API key in a cookie.
    Cookie,
}

/// A [Security Scheme Object](https://spec.openapis.org/oas/latest.html#security-scheme-object)
/// as defined in §4.27 of the OpenAPI 3.2 specification.
///
/// Defines a security scheme that can be used by the operations. Supported schemes
/// are HTTP authentication, API key, mutual TLS, OAuth2, and OpenID Connect.
///
/// The `type` field determines which other fields are required:
/// - `apiKey`: `name` and `in`
/// - `http`: `scheme` (and optionally `bearer_format` for `bearer`)
/// - `oauth2`: `flows` (and optionally `oauth2_metadata_url` in 3.2)
/// - `openIdConnect`: `open_id_connect_url`
/// - `mutualTLS`: no additional required fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    /// **REQUIRED.** The type of the security scheme.
    #[serde(rename = "type")]
    pub scheme_type: SecuritySchemeType,

    /// A description for the security scheme. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The name of the header, query, or cookie parameter (for `apiKey` type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The location of the API key (for `apiKey` type): `query`, `header`, or `cookie`.
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub location: Option<ApiKeyLocation>,

    /// The HTTP Authentication scheme name (for `http` type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// A hint for identifying the bearer token format (for `http` type with `bearer` scheme).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,

    /// Configuration for the OAuth2 flow types (for `oauth2` type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<OAuthFlows>,

    /// Well-known URL to discover OpenID Connect provider metadata (for `openIdConnect` type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_url: Option<String>,

    /// URL to the OAuth2 authorization server metadata (for `oauth2` type, added in OAS 3.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_metadata_url: Option<String>,

    /// Declares this security scheme to be deprecated (added in OAS 3.2).
    /// Consumers SHOULD refrain from usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for SecurityScheme {
    fn default() -> Self {
        Self {
            scheme_type: SecuritySchemeType::ApiKey,
            description: None,
            name: None,
            location: None,
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
            oauth2_metadata_url: None,
            deprecated: None,
            extensions: Extensions::default(),
        }
    }
}

/// An [OAuth Flows Object](https://spec.openapis.org/oas/latest.html#oauth-flows-object)
/// as defined in §4.28 of the OpenAPI 3.2 specification.
///
/// Allows configuration of the supported OAuth Flows.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `implicit` | `OAuthFlow` | Configuration for the OAuth Implicit flow. |
/// | `password` | `OAuthFlow` | Configuration for the OAuth Resource Owner Password flow. |
/// | `client_credentials` | `OAuthFlow` | Configuration for the OAuth Client Credentials flow. |
/// | `authorization_code` | `OAuthFlow` | Configuration for the OAuth Authorization Code flow. |
/// | `device_authorization` | `OAuthFlow` | Configuration for the OAuth Device Authorization flow (3.2). |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,

    /// Configuration for the OAuth Resource Owner Password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,

    /// Configuration for the OAuth Client Credentials flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,

    /// Configuration for the OAuth Authorization Code flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,

    /// Configuration for the OAuth Device Authorization flow (added in OAS 3.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_authorization: Option<OAuthFlow>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for OAuthFlows {
    fn default() -> Self {
        Self {
            implicit: None,
            password: None,
            client_credentials: None,
            authorization_code: None,
            device_authorization: None,
            extensions: Extensions::default(),
        }
    }
}

/// An [OAuth Flow Object](https://spec.openapis.org/oas/latest.html#oauth-flow-object)
/// as defined in §4.29 of the OpenAPI 3.2 specification.
///
/// Configuration details for a supported OAuth Flow.
///
/// | Field | Type | Applies to | Description |
/// |---|---|---|---|
/// | `authorization_url` | `string` | `implicit`, `authorizationCode` | **REQUIRED.** The authorization URL. |
/// | `device_authorization_url` | `string` | `deviceAuthorization` | **REQUIRED.** The device authorization URL (3.2). |
/// | `token_url` | `string` | `password`, `clientCredentials`, `authorizationCode`, `deviceAuthorization` | **REQUIRED.** The token URL. |
/// | `refresh_url` | `string` | All | The URL for obtaining refresh tokens. |
/// | `scopes` | `Map<string, string>` | All | **REQUIRED.** The available scopes. May be empty. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    /// The authorization URL for this flow. MUST be a valid URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,

    /// The device authorization URL for this flow. MUST be a valid URL (added in OAS 3.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_authorization_url: Option<String>,

    /// The token URL for this flow. MUST be a valid URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,

    /// The URL for obtaining refresh tokens. MUST be a valid URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,

    /// The available scopes for the OAuth2 security scheme.
    /// A map between the scope name and a short description. **REQUIRED.** May be empty.
    pub scopes: IndexMap<String, String>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for OAuthFlow {
    fn default() -> Self {
        Self {
            authorization_url: None,
            device_authorization_url: None,
            token_url: None,
            refresh_url: None,
            scopes: IndexMap::new(),
            extensions: Extensions::default(),
        }
    }
}

/// A [Security Requirement Object](https://spec.openapis.org/oas/latest.html#security-requirement-object)
/// as defined in §4.30 of the OpenAPI 3.2 specification.
///
/// Lists the required security schemes to execute this operation. Each name MUST
/// correspond to a security scheme declared in the Components Object. The value
/// is a list of scope names (for `oauth2`/`openIdConnect`) or role names (for
/// other types), and MAY be empty.
///
/// An empty Security Requirement Object (`{}`) indicates anonymous access.
pub type SecurityRequirement = IndexMap<String, Vec<String>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_scheme() {
        let json = r#"{"type": "apiKey", "name": "X-API-Key", "in": "header"}"#;
        let scheme: SecurityScheme = serde_json::from_str(json).unwrap();
        assert_eq!(scheme.scheme_type, SecuritySchemeType::ApiKey);
        assert_eq!(scheme.name.as_deref(), Some("X-API-Key"));
    }

    #[test]
    fn test_http_bearer_scheme() {
        let json = r#"{"type": "http", "scheme": "bearer", "bearerFormat": "JWT"}"#;
        let scheme: SecurityScheme = serde_json::from_str(json).unwrap();
        assert_eq!(scheme.scheme, Some("bearer".into()));
        assert_eq!(scheme.bearer_format, Some("JWT".into()));
    }

    #[test]
    fn test_oauth2_scheme() {
        let json = r#"{
            "type": "oauth2",
            "flows": {
                "implicit": {
                    "authorizationUrl": "https://example.com/oauth/dialog",
                    "scopes": {
                        "write:pets": "modify pets",
                        "read:pets": "read pets"
                    }
                }
            }
        }"#;
        let scheme: SecurityScheme = serde_json::from_str(json).unwrap();
        let flows = scheme.flows.as_ref().unwrap();
        let implicit = flows.implicit.as_ref().unwrap();
        assert_eq!(
            implicit.authorization_url.as_deref(),
            Some("https://example.com/oauth/dialog")
        );
    }

    #[test]
    fn test_security_requirement() {
        let json = r#"{"petstore_auth": ["write:pets", "read:pets"]}"#;
        let req: SecurityRequirement = serde_json::from_str(json).unwrap();
        assert_eq!(req.get("petstore_auth").unwrap().len(), 2);
    }

    #[test]
    fn test_security_requirement_empty() {
        let json = r#"{}"#;
        let req: SecurityRequirement = serde_json::from_str(json).unwrap();
        assert!(req.is_empty());
    }
}
