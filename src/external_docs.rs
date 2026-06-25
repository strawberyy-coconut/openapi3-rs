use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;

/// An [External Documentation Object](https://spec.openapis.org/oas/latest.html#external-documentation-object)
/// as defined in §4.11 of the OpenAPI 3.2 specification.
///
/// Allows referencing an external resource for extended documentation.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `description` | `string` | A description of the target documentation. Supports CommonMark. |
/// | `url` | `string` | **REQUIRED.** The URI for the target documentation. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalDocumentation {
    /// A description of the target documentation. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The URI for the target documentation. MUST be a valid URI.
    pub url: String,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for ExternalDocumentation {
    /// Creates a minimal ExternalDocumentation with empty URL.
    /// Prefer [`ExternalDocumentation::new`] for specification-compliant construction.
    fn default() -> Self {
        Self {
            url: String::new(),
            description: None,
            extensions: Extensions::default(),
        }
    }
}

impl ExternalDocumentation {
    /// Create a new ExternalDocumentation with the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: None,
            extensions: Extensions::default(),
        }
    }
}


