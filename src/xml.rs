use serde::{Deserialize, Serialize};

use crate::extensions::Extensions;

/// Node types for the XML Object, as defined in §4.26.2 of the OpenAPI 3.2 specification.
///
/// Each Schema Object with XML metadata describes a particular type of DOM node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum XmlNodeType {
    /// The schema represents an XML element and describes its contents.
    Element,
    /// The schema represents an XML attribute and describes its value.
    Attribute,
    /// The schema represents a text node (parsed character data).
    Text,
    /// The schema represents a CDATA section.
    Cdata,
    /// The schema does not correspond to any XML node. Its subschemas are
    /// included directly under the parent's node.
    None,
}

impl Default for XmlNodeType {
    fn default() -> Self {
        Self::Element
    }
}

/// An [XML Object](https://spec.openapis.org/oas/latest.html#xml-object)
/// as defined in §4.26 of the OpenAPI 3.2 specification.
///
/// A metadata object that allows for more fine-tuned XML model definitions.
/// When using a Schema Object with XML, if no XML Object is present, the behavior
/// is determined by the XML Object's default field values.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `name` | `string` | Replaces the inferred XML element/attribute name. |
/// | `namespace` | `string` | The IRI of the namespace definition (non-relative). |
/// | `prefix` | `string` | The prefix to be used for the name. |
/// | `attribute` | `boolean` | **Deprecated.** Use `node_type: "attribute"` instead. |
/// | `wrapped` | `boolean` | **Deprecated.** Use `node_type: "element"` on array schemas instead. |
/// | `node_type` | `XmlNodeType` | The DOM node type (3.2). Overrides `attribute` and `wrapped`. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct XML {
    /// Replaces the inferred XML element/attribute name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The IRI of the namespace definition. MUST be a non-relative IRI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The prefix to be used for the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// **Deprecated.** Use `node_type: "attribute"` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,

    /// **Deprecated.** Signifies whether an array is wrapped in a container element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,

    /// The DOM node type. One of `element`, `attribute`, `text`, `cdata`, or `none`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<XmlNodeType>,

    /// Specification Extensions (`x-*` keys).
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Default for XML {
    fn default() -> Self {
        Self {
            name: None,
            namespace: None,
            prefix: None,
            attribute: None,
            wrapped: None,
            node_type: None,
            extensions: Extensions::default(),
        }
    }
}
