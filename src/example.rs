use serde::{Deserialize, Serialize};

/// An [Example Object](https://spec.openapis.org/oas/latest.html#example-object)
/// as defined in §4.19 of the OpenAPI 3.2 specification.
///
/// An object grouping an internal or external example value with basic `summary`
/// and `description` metadata. The examples can show either data suitable for
/// schema validation, or serialized data as required by the containing Media Type,
/// Parameter, or Header Object.
///
/// The `value` and `external_value` fields are mutually exclusive (and `value`
/// is deprecated for non-JSON targets in 3.2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Long description for the example. Supports CommonMark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An example of the data structure that MUST be valid according to the
    /// relevant Schema Object (3.2). If present, `value` MUST be absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_value: Option<serde_json::Value>,

    /// An example of the serialized form of the value (3.2). SHOULD contain
    /// the serialization of `data_value` if both are present. MUST NOT include
    /// leading delimiters like `?` or `&` for form parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialized_value: Option<String>,

    /// A URI that identifies the serialized example in a separate document.
    /// Mutually exclusive with `serialized_value` and `value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,

    /// Embedded literal example. **Deprecated** for non-JSON targets.
    /// Mutually exclusive with `external_value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl Example {
    /// Create a new Example with the given value.
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            value: Some(value),
            summary: None,
            description: None,
            data_value: None,
            serialized_value: None,
            external_value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_minimal() {
        let json = r#"{"value": "hello"}"#;
        let ex: Example = serde_json::from_str(json).unwrap();
        assert_eq!(ex.value, Some(serde_json::Value::String("hello".into())));
    }

    #[test]
    fn test_example_data_value() {
        let json = r#"{"summary": "A cat", "dataValue": {"name": "Fluffy"}}"#;
        let ex: Example = serde_json::from_str(json).unwrap();
        assert!(ex.data_value.is_some());
    }
}
