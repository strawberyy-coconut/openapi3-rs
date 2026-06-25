use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::external_docs::ExternalDocumentation;
use crate::parameter::Parameter;
use crate::reference::RefOr;
use crate::request_body::RequestBodyRef;
use crate::response::Responses;
use crate::security::SecurityRequirement;
use crate::server::Server;

/// A [Paths Object](https://spec.openapis.org/oas/latest.html#paths-object)
/// as defined in §4.8 of the OpenAPI 3.2 specification.
///
/// Holds the relative paths to the individual endpoints and their operations.
/// The path is appended to the URL from the Server Object to construct the full
/// URL. The Paths Object MAY be empty, due to ACL constraints.
///
/// Each key is a relative path (MUST begin with `/`) mapping to a Path Item Object.
/// Path templating is supported with `{variable}` expressions.
pub type Paths = IndexMap<String, RefOr<PathItem>>;

/// A [Path Item Object](https://spec.openapis.org/oas/latest.html#path-item-object)
/// as defined in §4.9 of the OpenAPI 3.2 specification.
///
/// Describes the operations available on a single path. A Path Item MAY be empty,
/// due to ACL constraints. The path itself is still exposed to the documentation
/// viewer.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `$ref` | `string` | Allows a referenced definition of this path item. |
/// | `summary` | `string` | An optional summary, intended to apply to all operations on this path. |
/// | `description` | `string` | An optional description, intended to apply to all operations. Supports CommonMark. |
/// | `get` | `Operation` | A GET operation on this path. |
/// | `put` | `Operation` | A PUT operation on this path. |
/// | `post` | `Operation` | A POST operation on this path. |
/// | `delete` | `Operation` | A DELETE operation on this path. |
/// | `options` | `Operation` | An OPTIONS operation on this path. |
/// | `head` | `Operation` | A HEAD operation on this path. |
/// | `patch` | `Operation` | A PATCH operation on this path. |
/// | `trace` | `Operation` | A TRACE operation on this path. |
/// | `query` | `Operation` | A QUERY operation on this path (3.2). |
/// | `additional_operations` | `Map<string, Operation>` | Additional custom HTTP methods (3.2). |
/// | `servers` | `[Server]` | An alternative servers array to service all operations on this path. |
/// | `parameters` | `[Parameter \| Reference]` | Parameters applicable for all operations on this path. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    /// Allows for a referenced definition of this path item.
    /// The value MUST be a URI, and the referenced structure MUST be a Path Item Object.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub ref_path: Option<String>,

    /// An optional string summary, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// An optional string description, intended to apply to all operations.
    /// Supports CommonMark markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,

    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,

    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,

    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,

    /// A definition of an OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,

    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,

    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,

    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,

    /// A definition of a QUERY operation on this path (added in OAS 3.2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Operation>,

    /// A map of additional operations on this path (added in OAS 3.2).
    /// Keys are HTTP method names. MUST NOT contain entries for methods defined
    /// by other fixed fields (e.g., no `POST` entry as `post` is used).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_operations: Option<IndexMap<String, Operation>>,

    /// An alternative servers array to service all operations on this path.
    /// If specified at the OpenAPI Object level, it will be overridden by this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,

    /// A list of parameters that are applicable for all the operations described
    /// under this path. These can be overridden at the operation level but cannot
    /// be removed there.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RefOr<Parameter>>>,
}

impl PathItem {
    /// Create a new empty PathItem.
    pub fn new() -> Self {
        Self {
            ref_path: None,
            summary: None,
            description: None,
            get: None,
            put: None,
            post: None,
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            query: None,
            additional_operations: None,
            servers: None,
            parameters: None,
        }
    }

    /// Returns all operations defined on this path item.
    pub fn operations(&self) -> Vec<(&str, &Operation)> {
        let mut ops = Vec::new();
        if let Some(op) = &self.get { ops.push(("get", op)); }
        if let Some(op) = &self.put { ops.push(("put", op)); }
        if let Some(op) = &self.post { ops.push(("post", op)); }
        if let Some(op) = &self.delete { ops.push(("delete", op)); }
        if let Some(op) = &self.options { ops.push(("options", op)); }
        if let Some(op) = &self.head { ops.push(("head", op)); }
        if let Some(op) = &self.patch { ops.push(("patch", op)); }
        if let Some(op) = &self.trace { ops.push(("trace", op)); }
        if let Some(op) = &self.query { ops.push(("query", op)); }
        if let Some(additional) = &self.additional_operations {
            for (method, op) in additional {
                ops.push((method.as_str(), op));
            }
        }
        ops
    }
}

impl Default for PathItem {
    fn default() -> Self {
        Self::new()
    }
}

/// An [Operation Object](https://spec.openapis.org/oas/latest.html#operation-object)
/// as defined in §4.10 of the OpenAPI 3.2 specification.
///
/// Describes a single API operation on a path.
///
/// # Fields
///
/// | Field | Type | Description |
/// |---|---|---|
/// | `tags` | `[string]` | A list of tags for API documentation control. |
/// | `summary` | `string` | A short summary of what the operation does. |
/// | `description` | `string` | A verbose explanation of the operation behavior. Supports CommonMark. |
/// | `external_docs` | `ExternalDocumentation` | Additional external documentation for this operation. |
/// | `operation_id` | `string` | Unique string used to identify the operation. |
/// | `parameters` | `[Parameter \| Reference]` | A list of parameters applicable for this operation. |
/// | `request_body` | `RequestBody \| Reference` | The request body applicable for this operation. |
/// | `responses` | `Responses` | The list of possible responses from executing this operation. |
/// | `callbacks` | `Map<string, Callback \| Reference>` | A map of possible out-of-band callbacks. |
/// | `deprecated` | `boolean` | Declares this operation to be deprecated. Default is `false`. |
/// | `security` | `[SecurityRequirement]` | A declaration of which security mechanisms can be used. |
/// | `servers` | `[Server]` | An alternative servers array to service this operation. |
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A verbose explanation of the operation behavior. Supports CommonMark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Unique string used to identify the operation. The id MUST be unique among
    /// all operations described in the API. Case-sensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    /// A list of parameters applicable for this operation. If a parameter is
    /// already defined at the Path Item, the new definition overrides it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RefOr<Parameter>>>,

    /// The request body applicable for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBodyRef>,

    /// **REQUIRED.** The list of possible responses as they are returned from
    /// executing this operation.
    pub responses: Responses,

    /// A map of possible out-of-band callbacks related to the parent operation.
    /// The key is a unique identifier for the Callback Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<IndexMap<String, RefOr<crate::callback::Callback>>>,

    /// Declares this operation to be deprecated. Default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// A declaration of which security mechanisms can be used for this operation.
    /// Overrides any declared top-level `security`. An empty array removes
    /// top-level security declarations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,

    /// An alternative servers array to service this operation.
    /// Overrides Path Item or OpenAPI Object level servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            responses: Responses::default(),
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            callbacks: None,
            deprecated: None,
            security: None,
            servers: None,
        }
    }
}

impl Operation {
    /// Create a new Operation with the given responses.
    pub fn new(responses: Responses) -> Self {
        Self {
            responses,
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            callbacks: None,
            deprecated: None,
            security: None,
            servers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_minimal() {
        let json = r#"{"responses": {"200": {"description": "OK"}}}"#;
        let op: Operation = serde_json::from_str(json).unwrap();
        assert!(op.responses.responses.contains_key("200"));
    }

    #[test]
    fn test_operation_full() {
        let json = r#"{
            "tags": ["pets"],
            "summary": "List pets",
            "operationId": "listPets",
            "responses": {
                "200": {"description": "A list of pets"}
            }
        }"#;
        let op: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(op.operation_id.as_deref(), Some("listPets"));
        assert_eq!(op.tags.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_path_item_get() {
        let json = r#"{
            "get": {
                "summary": "List all",
                "responses": {"200": {"description": "OK"}}
            }
        }"#;
        let pi: PathItem = serde_json::from_str(json).unwrap();
        assert!(pi.get.is_some());
        assert_eq!(pi.get.as_ref().unwrap().summary.as_deref(), Some("List all"));
    }

    #[test]
    fn test_path_item_ref() {
        let json = r###"{"$ref": "#/components/pathItems/Foo"}"###;
        let pi: PathItem = serde_json::from_str(json).unwrap();
        assert_eq!(pi.ref_path.as_deref(), Some("#/components/pathItems/Foo"));
    }

    #[test]
    fn test_paths() {
        let json = r#"{
            "/pets": {
                "get": {
                    "summary": "List pets",
                    "responses": {"200": {"description": "OK"}}
                }
            }
        }"#;
        let paths: Paths = serde_json::from_str(json).unwrap();
        assert!(paths.contains_key("/pets"));
    }
}
