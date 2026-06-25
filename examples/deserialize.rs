//! # Deserialize an OpenAPI document from JSON
//!
//! Demonstrates reading an OpenAPI description from a JSON string (or file),
//! then inspecting its structure — paths, operations, schemas, etc.
//!
//! Run: `cargo run --example deserialize`

use openapi3_rs::{OpenAPI, RefOr, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A minimal but realistic OpenAPI document inline
    let json = r###"{
        "openapi": "3.1.0",
        "info": {
            "title": "Petstore API",
            "version": "1.0.0",
            "description": "A sample API that uses a petstore as an example",
            "contact": {
                "name": "API Support",
                "email": "support@example.com"
            },
            "license": {
                "name": "Apache 2.0",
                "url": "https://www.apache.org/licenses/LICENSE-2.0.html"
            }
        },
        "servers": [
            { "url": "https://api.example.com/v1" }
        ],
        "paths": {
            "/pets": {
                "get": {
                    "summary": "List all pets",
                    "operationId": "listPets",
                    "parameters": [
                        {
                            "name": "limit",
                            "in": "query",
                            "schema": { "type": "integer", "format": "int32" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "A list of pets",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": { "$ref": "#/components/schemas/Pet" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "Pet": {
                    "type": "object",
                    "required": ["id", "name"],
                    "properties": {
                        "id":   { "type": "integer", "format": "int64" },
                        "name": { "type": "string" },
                        "tag":  { "type": "string" }
                    }
                },
                "Error": {
                    "type": "object",
                    "required": ["code", "message"],
                    "properties": {
                        "code":    { "type": "integer", "format": "int32" },
                        "message": { "type": "string" }
                    }
                }
            }
        }
    }"###;

    // -- Deserialize ---------------------------------------------------------
    let api: OpenAPI = serde_json::from_str(json)?;
    println!("✅ Parsed OpenAPI document successfully!\n");

    // -- Inspect -------------------------------------------------------------
    println!("📋 Info");
    println!("   Title:   {}", api.info.title);
    println!("   Version: {}", api.info.version);
    if let Some(ref desc) = api.info.description {
        println!("   Description: {desc}");
    }
    if let Some(ref contact) = api.info.contact {
        if let Some(ref name) = contact.name {
            println!("   Contact: {name}");
        }
        if let Some(ref email) = contact.email {
            println!("   Email:   {email}");
        }
    }

    println!("\n🌐 Servers");
    if let Some(ref servers) = api.servers {
        for s in servers {
            println!("   {}", s.url);
        }
    }

    println!("\n🛤️  Paths");
    if let Some(ref paths) = api.paths {
        for (path, path_item) in paths {
            match path_item {
                RefOr::Item(item) => {
                    println!("   {path}");
                    if let Some(ref get) = item.get {
                        println!("     GET → {}", get.summary.as_deref().unwrap_or("(no summary)"));
                        if let Some(ref op_id) = get.operation_id {
                            println!("     operationId: {op_id}");
                        }
                    }
                    if item.put.is_some()    { println!("     PUT"); }
                    if item.post.is_some()   { println!("     POST"); }
                }
                RefOr::Ref(r) => {
                    println!("   {path} → $ref: {}", r.ref_path);
                }
            }
        }
    }

    println!("\n🧩 Components / Schemas");
    if let Some(ref components) = api.components {
        if let Some(ref schemas) = components.schemas {
            for (name, schema) in schemas {
                print!("   {name}: ");
                match schema {
                    Schema::Object(obj) => {
                        // Read the JSON Schema 'type' keyword from schema_data
                        let stype = obj.schema_data
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("(untyped)");
                        let props = obj.schema_data
                            .get("properties")
                            .and_then(|v| v.as_object())
                            .map(|o| o.len())
                            .unwrap_or(0);
                        println!("{stype} ({props} properties)");
                    }
                    Schema::Bool(true)  => println!("(allows everything)"),
                    Schema::Bool(false) => println!("(allows nothing)"),
                }
            }
        }
    }

    // -- Round-trip ----------------------------------------------------------
    let output = serde_json::to_string_pretty(&api)?;
    let api2: OpenAPI = serde_json::from_str(&output)?;
    assert_eq!(api, api2, "round-trip should be lossless");
    println!("\n✅ Round-trip serialization: identity preserved");

    Ok(())
}
