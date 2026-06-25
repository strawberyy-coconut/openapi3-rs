use serde::{Deserialize, Serialize};

/// An [Info Object](https://spec.openapis.org/oas/latest.html#info-object)
/// as defined in §4.2 of the OpenAPI 3.2 specification.
///
/// Provides metadata about the API. The metadata MAY be used by tooling as required.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `title` | `string` | **REQUIRED.** The title of the API. |
/// | `summary` | `string` | A short summary of the API. |
/// | `description` | `string` | A description of the API. Supports CommonMark. |
/// | `terms_of_service` | `string` | A URI for the Terms of Service. |
/// | `contact` | `Contact` | The contact information for the exposed API. |
/// | `license` | `License` | The license information for the exposed API. |
/// | `version` | `string` | **REQUIRED.** The version of the OpenAPI document. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the API.
    pub title: String,

    /// A short summary of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description of the API. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A URI for the Terms of Service for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,

    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// The version of the OpenAPI document (distinct from the spec version).
    pub version: String,
}

impl Info {
    /// Create a new Info with the required `title` and `version`.
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            summary: None,
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
        }
    }
}

/// A [Contact Object](https://spec.openapis.org/oas/latest.html#contact-object)
/// as defined in §4.3 of the OpenAPI 3.2 specification.
///
/// Contact information for the exposed API.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `name` | `string` | The identifying name of the contact person/organization. |
/// | `url` | `string` | The URI for the contact information. |
/// | `email` | `string` | The email address of the contact person/organization. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URI for the contact information. MUST be a valid URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The email address of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// A [License Object](https://spec.openapis.org/oas/latest.html#license-object)
/// as defined in §4.4 of the OpenAPI 3.2 specification.
///
/// License information for the exposed API. The `identifier` and `url` fields
/// are mutually exclusive.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `name` | `string` | **REQUIRED.** The license name used for the API. |
/// | `identifier` | `string` | An SPDX license expression. Mutually exclusive with `url`. |
/// | `url` | `string` | A URI for the license. Mutually exclusive with `identifier`. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct License {
    /// The license name used for the API.
    pub name: String,

    /// An SPDX license expression. Mutually exclusive with `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// A URI for the license used for the API. Mutually exclusive with `identifier`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl License {
    /// Create a new License with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            identifier: None,
            url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_minimal() {
        let json = r#"{"title": "My API", "version": "1.0.0"}"#;
        let info: Info = serde_json::from_str(json).unwrap();
        assert_eq!(info.title, "My API");
        assert_eq!(info.version, "1.0.0");
    }

    #[test]
    fn test_info_full_roundtrip() {
        let json = r#"{
            "title": "Example API",
            "summary": "An example",
            "description": "A longer description",
            "termsOfService": "https://example.com/terms",
            "contact": {
                "name": "API Support",
                "email": "support@example.com"
            },
            "license": {
                "name": "Apache 2.0",
                "identifier": "Apache-2.0"
            },
            "version": "1.0.0"
        }"#;
        let info: Info = serde_json::from_str(json).unwrap();
        assert_eq!(info.contact.as_ref().unwrap().name.as_deref(), Some("API Support"));
        assert_eq!(info.license.as_ref().unwrap().name, "Apache 2.0");
        let output = serde_json::to_string(&info).unwrap();
        let _back: Info = serde_json::from_str(&output).unwrap();
    }
}
