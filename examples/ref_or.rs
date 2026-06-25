//! # Working with `$ref` references and `RefOr<T>`
//!
//! Demonstrates the `RefOr<T>` pattern — how to use inline definitions and
//! `$ref` references interchangeably throughout an OpenAPI document.
//!
//! Run: `cargo run --example ref_or`

use openapi3_rs::*;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Schema: inline vs $ref -----------------------------------------------
    //
    // An inline schema — all JSON Schema keywords provided directly:
    let inline_schema: RefOr<Schema> = RefOr::Item(Schema::Object(SchemaObject {
        schema_data: {
            let mut m = serde_json::Map::new();
            m.insert("type".into(), "string".into());
            m.insert("maxLength".into(), Value::Number(255.into()));
            m
        },
        ..Default::default()
    }));

    // A $ref reference — points to a component defined elsewhere:
    let ref_schema: RefOr<Schema> = RefOr::Ref(Reference::new("#/components/schemas/Pet"));

    println!("Inline schema is_item: {}", inline_schema.is_item());
    println!("Ref schema is_ref:      {}\n", ref_schema.is_ref());

    // -- Parameters can also use RefOr ----------------------------------------
    let params = vec![
        // Inline parameter definition
        RefOr::Item(Parameter {
            name: "petId".into(),
            location: ParameterIn::Path,
            required: Some(true),
            schema: Some(RefOr::Item(Schema::Object(SchemaObject {
                schema_data: {
                    let mut m = serde_json::Map::new();
                    m.insert("type".into(), "integer".into());
                    m
                },
                ..Default::default()
            }))),
            description: None,
            deprecated: None,
            allow_empty_value: None,
            style: None,
            explode: None,
            allow_reserved: None,
            example: None,
            examples: None,
            content: None,
            extensions: Extensions::default(),
        }),
        // Parameter defined via reference
        RefOr::Ref(Reference::new("#/components/parameters/OffsetParam")),
    ];

    for (i, p) in params.iter().enumerate() {
        match p {
            RefOr::Item(param) => {
                println!(
                    "Param {i}: inline  name={} in={}",
                    param.name,
                    serde_json::to_string(&param.location).unwrap()
                );
            }
            RefOr::Ref(r) => {
                println!("Param {i}: $ref    → {}", r.ref_path);
            }
        }
    }

    // -- Responses with mixed inline/$ref ------------------------------------
    let mut responses = Responses::new();
    responses.responses.insert(
        "200".into(),
        RefOr::Item(Response {
            description: "OK".into(),
            summary: None,
            headers: None,
            content: None,
            links: None,
            extensions: Extensions::default(),
        }),
    );
    responses.responses.insert(
        "404".into(),
        RefOr::Ref(Reference::new("#/components/responses/NotFound")),
    );

    println!("\nResponses map:");
    for (code, resp) in &responses.responses {
        match resp {
            RefOr::Item(r) => println!("  {code}: inline  — \"{}\"", r.description),
            RefOr::Ref(r)  => println!("  {code}: $ref    → {}", r.ref_path),
        }
    }

    // -- Serialize to show the difference -------------------------------------
    let ref_only: Vec<RefOr<Schema>> = vec![ref_schema];
    let inline_only: Vec<RefOr<Schema>> = vec![inline_schema];

    println!("\nSerialized $ref schema:");
    println!("  {}", serde_json::to_string(&ref_only)?);
    println!("\nSerialized inline schema:");
    println!("  {}", serde_json::to_string(&inline_only)?);

    // -- Round-trip check ----------------------------------------------------
    for schemas in [ref_only, inline_only] {
        let json = serde_json::to_string(&schemas)?;
        let back: Vec<RefOr<Schema>> = serde_json::from_str(&json)?;
        assert_eq!(schemas, back);
    }
    println!("\n✅ All round-trips passed");

    Ok(())
}
