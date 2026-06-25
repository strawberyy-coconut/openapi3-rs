use serde::{Deserialize, Serialize};

/// A value that can be either a direct object or a `$ref` reference to another object.
///
/// This is the standard pattern for OpenAPI fields that accept either an inline
/// definition or a reference to a component. Uses `#[serde(untagged)]` so that
/// the JSON representation is either a `{"$ref": "..."}` object or the inline
/// object directly.
///
/// # Examples
///
/// ```json
/// // Inline definition:
/// { "type": "string" }
///
/// // Reference:
/// { "$ref": "#/components/schemas/MyType" }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum RefOr<T> {
    /// A JSON `$ref` reference to another component.
    Ref(Reference),
    /// An inline definition of the object.
    Item(T),
}

/// A [Reference Object](https://spec.openapis.org/oas/latest.html#reference-object) as
/// defined in §4.23 of the OpenAPI 3.2 specification.
///
/// A simple object to allow referencing other components in the OpenAPI
/// Description, internally and externally.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `$ref` | `string` | **REQUIRED.** The reference identifier. MUST be a URI. |
/// | `summary` | `string` | Short summary overriding the referenced component's summary. |
/// | `description` | `string` | Description overriding the referenced component's description. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Reference {
    /// The URI reference to another component or document.
    #[serde(rename = "$ref")]
    pub ref_path: String,

    /// Optional summary that overrides the referenced component's summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Optional description that overrides the referenced component's description.
    /// Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Reference {
    /// Create a new Reference with the given URI.
    pub fn new(ref_path: impl Into<String>) -> Self {
        Self {
            ref_path: ref_path.into(),
            summary: None,
            description: None,
        }
    }
}

impl<T> RefOr<T> {
    /// Returns `true` if this is a reference.
    pub fn is_ref(&self) -> bool {
        matches!(self, Self::Ref(_))
    }

    /// Returns `true` if this is an inline item.
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }

    /// Returns the reference if this is a `Ref` variant.
    pub fn as_ref(&self) -> Option<&Reference> {
        match self {
            Self::Ref(r) => Some(r),
            Self::Item(_) => None,
        }
    }

    /// Returns the item if this is an `Item` variant.
    pub fn as_item(&self) -> Option<&T> {
        match self {
            Self::Item(item) => Some(item),
            Self::Ref(_) => None,
        }
    }

    /// Consumes self and returns the item, if this is an `Item` variant.
    pub fn into_item(self) -> Option<T> {
        match self {
            Self::Item(item) => Some(item),
            Self::Ref(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_or_item_ref() {
        let json = r###"{"$ref": "#/components/schemas/Foo"}"###;
        let v: RefOr<String> = serde_json::from_str(json).unwrap();
        assert!(v.is_ref());
        assert_eq!(
            v.as_ref().unwrap().ref_path,
            "#/components/schemas/Foo"
        );
    }

    #[test]
    fn test_ref_or_item_value() {
        let json = r#""hello world""#;
        let v: RefOr<String> = serde_json::from_str(json).unwrap();
        assert!(v.is_item());
        assert_eq!(v.as_item().unwrap(), "hello world");
    }

    #[test]
    fn test_ref_or_roundtrip() {
        let v: RefOr<String> = RefOr::Item("test".into());
        let json = serde_json::to_string(&v).unwrap();
        let back: RefOr<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.as_item().unwrap(), "test");
    }
}
