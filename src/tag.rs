use serde::{Deserialize, Serialize};

use crate::external_docs::ExternalDocumentation;

/// A [Tag Object](https://spec.openapis.org/oas/latest.html#tag-object)
/// as defined in §4.22 of the OpenAPI 3.2 specification.
///
/// Adds metadata to a single tag that is used by the Operation Object.
/// It is not mandatory to have a Tag Object per tag defined in the Operation
/// Object instances.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `name` | `string` | **REQUIRED.** The name of the tag. |
/// | `summary` | `string` | A short summary of the tag, used for display purposes. |
/// | `description` | `string` | A description for the tag. Supports CommonMark. |
/// | `external_docs` | `ExternalDocumentation` | Additional external documentation for this tag. |
/// | `parent` | `string` | The name of a parent tag (3.2). No circular references allowed. |
/// | `kind` | `string` | A machine-readable category (3.2). e.g., `nav`, `badge`, `audience`. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The name of the tag.
    pub name: String,

    /// A short summary of the tag, used for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description for the tag. Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// The name of a tag that this tag is nested under (added in OAS 3.2).
    /// The named tag MUST exist in the API description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// A machine-readable string to categorize what sort of tag it is (added in OAS 3.2).
    /// Common values: `nav`, `badge`, `audience`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            name: String::new(),
            summary: None,
            description: None,
            external_docs: None,
            parent: None,
            kind: None,
        }
    }
}

impl Tag {
    /// Create a new Tag with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            summary: None,
            description: None,
            external_docs: None,
            parent: None,
            kind: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_minimal() {
        let json = r#"{"name": "pets"}"#;
        let tag: Tag = serde_json::from_str(json).unwrap();
        assert_eq!(tag.name, "pets");
    }

    #[test]
    fn test_tag_with_parent() {
        let json = r#"{"name": "partner", "parent": "external", "kind": "audience"}"#;
        let tag: Tag = serde_json::from_str(json).unwrap();
        assert_eq!(tag.parent.as_deref(), Some("external"));
        assert_eq!(tag.kind.as_deref(), Some("audience"));
    }
}
