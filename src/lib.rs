//! # openapi3-rs
//!
//! Strongly-typed [OpenAPI 3.2](https://spec.openapis.org/oas/latest.html) document
//! types with serde support.
//!
//! This crate provides Rust structs for all objects defined in the OpenAPI
//! Specification, enabling type-safe deserialization, serialization, and
//! manipulation of OpenAPI documents.
//!
//! ## Features
//!
//! - **Strongly typed**: All OAS objects have corresponding Rust types
//! - **serde support**: Serialize/deserialize from JSON or YAML
//! - **Optional validation**: Validate Schema Objects via `jsonschema` (feature: `validate`)
//! - **Optional builders**: Construct objects ergonomically (feature: `builder`)
//!
//! ## Example
//!
//! ```rust
//! use openapi3_rs::OpenAPI;
//!
//! let json = r#"{"openapi":"3.2.0","info":{"title":"Demo","version":"1.0"}}"#;
//! let api: OpenAPI = serde_json::from_str(json).unwrap();
//! println!("API: {} v{}", api.info.title, api.info.version);
//! ```

pub mod callback;
pub mod components;
pub mod discriminator;
pub mod encoding;
pub mod example;
pub mod extensions;
pub mod external_docs;
pub mod header;
pub mod info;
pub mod link;
pub mod media_type;
pub mod openapi;
pub mod parameter;
pub mod paths;
pub mod reference;
pub mod request_body;
pub mod response;
pub mod schema;
pub mod security;
pub mod server;
pub mod tag;
pub mod xml;

// Re-exports
pub use callback::Callback;
pub use components::Components;
pub use discriminator::Discriminator;
pub use encoding::Encoding;
pub use example::Example;
pub use extensions::Extensions;
pub use external_docs::ExternalDocumentation;
pub use header::Header;
pub use info::{Contact, Info, License};
pub use link::Link;
pub use media_type::MediaType;
pub use openapi::OpenAPI;
pub use parameter::{Parameter, ParameterIn, Style};
pub use paths::{Operation, PathItem, Paths};
pub use reference::{RefOr, Reference};
pub use request_body::{RequestBody, RequestBodyRef};
pub use response::{Response, Responses};
pub use schema::{Schema, SchemaObject};
pub use security::{
    ApiKeyLocation, OAuthFlow, OAuthFlows, SecurityRequirement, SecurityScheme,
    SecuritySchemeType, anonymous_security, security_requirement,
};
pub use server::{Server, ServerVariable};
pub use tag::Tag;
pub use xml::{XML, XmlNodeType};

