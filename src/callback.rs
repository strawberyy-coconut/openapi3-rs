use indexmap::IndexMap;

use crate::paths::PathItem;
use crate::reference::RefOr;

/// A [Callback Object](https://spec.openapis.org/oas/latest.html#callback-object)
/// as defined in §4.18 of the OpenAPI 3.2 specification.
///
/// A map of possible out-of-band callbacks related to the parent operation. Each
/// value in the map is a Path Item Object that describes a set of requests that
/// may be initiated by the API provider and the expected responses.
///
/// The key is a runtime expression that identifies the URL to use for the
/// callback operation (e.g., `{$request.query.queryUrl}`).
pub type Callback = IndexMap<String, RefOr<PathItem>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback() {
        let json = r#"{
            "{$request.query.callbackUrl}": {
                "post": {
                    "responses": {"200": {"description": "OK"}}
                }
            }
        }"#;
        let cb: Callback = serde_json::from_str(json).unwrap();
        assert!(cb.contains_key("{$request.query.callbackUrl}"));
    }
}
