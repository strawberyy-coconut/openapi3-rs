use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A map of [Specification Extensions](https://spec.openapis.org/oas/latest.html#specification-extensions)
/// as defined in §5 of the OpenAPI 3.2 specification.
///
/// Extension properties are implemented as patterned fields prefixed by `x-`.
/// The value can be any valid JSON value.
///
/// # Serde behavior
///
/// When used with `#[serde(flatten)]`, this captures all `x-*` fields from the
/// parent object during deserialization and writes them back during serialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extensions {
    /// Extension key-value pairs. Keys must start with `x-`.
    #[serde(flatten)]
    pub extensions: IndexMap<String, Value>,
}

impl Extensions {
    /// Create a new empty Extensions map.
    pub fn new() -> Self {
        Self {
            extensions: IndexMap::new(),
        }
    }

    /// Insert an extension value.
    ///
    /// The key should include the `x-` prefix (e.g., `x-internal-id`).
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<Value>) -> Option<Value> {
        self.extensions.insert(key.into(), value.into())
    }

    /// Get an extension value by key.
    ///
    /// The key should include the `x-` prefix.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.extensions.get(key)
    }

    /// Returns true if there are no extensions.
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }
}
