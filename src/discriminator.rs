use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A [Discriminator Object](https://spec.openapis.org/oas/latest.html#discriminator-object)
/// as defined in §4.25 of the OpenAPI 3.2 specification.
///
/// When request bodies or response payloads may be one of a number of different
/// schemas, a Discriminator Object provides a "hint" for which schema is expected
/// to validate the structure of the model. The discriminator MUST NOT change the
/// validation outcome.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `property_name` | `string` | **REQUIRED.** The name of the discriminating property in the payload. |
/// | `mapping` | `Map<string, string>` | Maps payload values to schema names or URI references. |
/// | `default_mapping` | `string` | Schema name/URI for when the discriminating property is absent or unmapped (3.2). |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    /// The name of the property in the payload that will hold the discriminating value.
    pub property_name: String,

    /// An object to hold mappings between payload values and schema names or URI references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<IndexMap<String, String>>,

    /// The schema name or URI reference expected when the discriminating property
    /// is not present or contains an unmapped value. Added in OpenAPI 3.2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<String>,
}

impl Discriminator {
    /// Create a new Discriminator with the given property name.
    pub fn new(property_name: impl Into<String>) -> Self {
        Self {
            property_name: property_name.into(),
            mapping: None,
            default_mapping: None,
        }
    }
}
