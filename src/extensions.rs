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
/// When used with `#[serde(flatten)]`, serde captures all unrecognized keys from
/// the parent object. Call [`validate_keys`](Self::validate_keys) to ensure only
/// `x-`-prefixed keys are present — this helps catch misspelled field names.
///
/// During serialization, all extensions are written back with their keys intact.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extensions {
    /// Extension key-value pairs. Per spec §5, keys MUST start with `x-`.
    /// The reserved prefixes `x-oai-` and `x-oas-` are for OpenAPI Initiative use.
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

    /// Return all keys that are NOT valid extension keys (don't start with `x-`).
    ///
    /// These are typically misspelled field names that serde couldn't match to
    /// any struct field. Call this after deserialization to detect them.
    ///
    /// # Example
    /// ```ignore
    /// let api: OpenAPI = serde_json::from_str(json_str)?;
    /// let bad_keys = api.extensions.validate_keys();
    /// if !bad_keys.is_empty() {
    ///     eprintln!("Warning: unrecognized non-extension keys: {:?}", bad_keys);
    /// }
    /// ```
    pub fn validate_keys(&self) -> Vec<&str> {
        self.extensions
            .keys()
            .filter(|k| !k.starts_with("x-"))
            .map(|k| k.as_str())
            .collect()
    }
}
