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
                    ..Default::default()
                })]),
                responses: {
                    let mut r = Responses::default();
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
                        ..Default::default()
                    }));
                    r
                },
                ..Default::default()
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
                ..Default::default()
            }),
            license: Some(License {
                name: "Apache 2.0".into(),
                identifier: None,
                url: Some("https://www.apache.org/licenses/LICENSE-2.0.html".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        servers: Some(vec![Server {
            url: "https://api.example.com/v1".into(),
            ..Default::default()
        }]),
        paths: Some(paths),
        components: Some(Components {
            schemas: Some(schemas),
            ..Default::default()
        }),
        ..Default::default()
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
