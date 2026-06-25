//! # Build and serialize an OpenAPI document from scratch
//!
//! Demonstrates programmatic construction of an OpenAPI document using the
//! crate's types, then serializing to JSON and YAML.
//!
//! Run: `cargo run --example serialize`

use indexmap::IndexMap;
use openapi3_rs::*;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Define schemas ------------------------------------------------------
    let mut schemas = IndexMap::new();

    schemas.insert(
        "Pet".into(),
        Schema::Object(SchemaObject {
            schema_data: {
                let mut m = serde_json::Map::new();
                m.insert("type".into(), "object".into());
                m.insert("required".into(), Value::Array(vec![
                    "id".into(), "name".into()
                ]));
                m.insert("properties".into(), serde_json::json!({
                    "id":   { "type": "integer", "format": "int64" },
                    "name": { "type": "string" },
                    "tag":  { "type": "string" }
                }));
                m
            },
            ..Default::default()
        }),
    );

    // -- Define path operations ----------------------------------------------
    let mut paths = Paths::new();

    paths.insert(
        "/pets".into(),
        RefOr::Item(PathItem {
            get: Some(Operation {
                summary: Some("List all pets".into()),
                operation_id: Some("listPets".into()),
                parameters: Some(vec![RefOr::Item(Parameter {
                    name: "limit".into(),
                    location: ParameterIn::Query,
                    schema: Some(RefOr::Item(Schema::Object(SchemaObject {
                        schema_data: {
                            let mut m = serde_json::Map::new();
                            m.insert("type".into(), "integer".into());
                            m.insert("format".into(), "int32".into());
                            m
                        },
                        ..Default::default()
                    }))),
                    description: None,
                    required: None,
                    deprecated: None,
                    allow_empty_value: None,
                    style: None,
                    explode: None,
                    allow_reserved: None,
                    example: None,
                    examples: None,
                    content: None,
                    extensions: Extensions::default(),
                })]),
                responses: {
                    let mut r = Responses::new();
                    r.responses.insert("200".into(), RefOr::Item(Response {
                        description: "A list of pets.".into(),
                        content: Some({
                            let mut c = IndexMap::new();
                            c.insert("application/json".into(), RefOr::Item(MediaType {
                                schema: Some(RefOr::Item(Schema::Object(SchemaObject {
                                    schema_data: {
                                        let mut m = serde_json::Map::new();
                                        m.insert("type".into(), "array".into());
                                        m.insert("items".into(), serde_json::json!({
                                            "$ref": "#/components/schemas/Pet"
                                        }));
                                        m
                                    },
                                    ..Default::default()
                                }))),
                                ..Default::default()
                            }));
                            c
                        }),
                        summary: None,
                        headers: None,
                        links: None,
                        extensions: Extensions::default(),
                    }));
                    r
                },
                tags: None,
                description: None,
                external_docs: None,
                request_body: None,
                callbacks: None,
                deprecated: None,
                security: None,
                servers: None,
                extensions: Extensions::default(),
            }),
            ..Default::default()
        }),
    );

    // -- Assemble the root object ---------------------------------------------
    let api = OpenAPI {
        openapi: "3.1.0".into(),
        info: Info {
            title: "Petstore API".into(),
            version: "1.0.0".into(),
            summary: None,
            description: Some("A sample petstore API".into()),
            terms_of_service: None,
            contact: Some(Contact {
                name: Some("API Support".into()),
                url: None,
                email: Some("support@example.com".into()),
                extensions: Extensions::default(),
            }),
            license: Some(License {
                name: "Apache 2.0".into(),
                identifier: None,
                url: Some("https://www.apache.org/licenses/LICENSE-2.0.html".into()),
                extensions: Extensions::default(),
            }),
            extensions: Extensions::default(),
        },
        servers: Some(vec![Server {
            url: "https://api.example.com/v1".into(),
            description: None,
            name: None,
            variables: None,
            extensions: Extensions::default(),
        }]),
        paths: Some(paths),
        components: Some(Components {
            schemas: Some(schemas),
            ..Default::default()
        }),
        self_uri: None,
        json_schema_dialect: None,
        webhooks: None,
        security: None,
        tags: None,
        external_docs: None,
        extensions: Extensions::default(),
    };

    // -- Serialize to JSON ---------------------------------------------------
    println!("═══ JSON ═══");
    let json = serde_json::to_string_pretty(&api)?;
    println!("{json}");

    // -- Serialize to YAML (serde_yaml is a dev-dependency) -------------------
    println!("\n═══ YAML ═══");
    {
        let yaml = serde_yaml::to_string(&api)?;
        println!("{yaml}");
    }

    // -- Round-trip verification ---------------------------------------------
    let api2: OpenAPI = serde_json::from_str(&json)?;
    assert_eq!(api, api2, "round-trip should be lossless");
    println!("\n✅ Round-trip verification passed");

    Ok(())
}
