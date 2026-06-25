# openapi3-rs

> ⚠️ **This is a work-in-progress, not a battle-tested library.**
> Expect missing fields, API churn, and rough edges. Use at your own risk.

Strongly-typed [OpenAPI 3.x](https://spec.openapis.org/oas/latest.html) document types with **serde** support.

This crate provides idiomatic Rust structs for every object defined in the OpenAPI Specification, enabling type-safe deserialization, serialization, and programmatic construction of OpenAPI documents.

## Features

- Typed structs for all OAS 3.x objects (`OpenAPI`, `Info`, `Schema`, `Operation`, `PathItem`, `Parameter`, `Response`, `Components`, etc.)
- `Serialize` + `Deserialize` on every type, with `camelCase` field naming
- `RefOr<T>` untagged enum for fields that accept either a `$ref` reference or an inline object
- Schema Object stores JSON Schema keywords as raw `serde_json::Value` — no need for a separate JSON Schema types crate
- Optional JSON Schema validation via `jsonschema` (behind the `validate` feature flag)
- Patterned/dynamic-key fields use `IndexMap` to preserve insertion order

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
openapi3-rs = "0.1"
serde_json = "1"
```

### Deserialize an OpenAPI document

```rust
use openapi3_rs::OpenAPI;

let json = std::fs::read_to_string("my-api.json")?;
let api: OpenAPI = serde_json::from_str(&json)?;
println!("{} v{}", api.info.title, api.info.version);
```

### Build and serialize

```rust
use openapi3_rs::*;

let api = OpenAPI {
    openapi: "3.1.0".into(),
    info: Info::new("My API", "1.0.0"),
    paths: Some(Paths::new()),
    ..Default::default()
};

println!("{}", serde_json::to_string_pretty(&api)?);
```

More examples in [`examples/`](examples/).

## What's implemented

A rough map of OAS sections to crate types:

| OAS Section | Crate Type(s) | Description |
|---|---|---|
| §4.1 | `OpenAPI` | Root object — `openapi`, `info`, `paths`, `webhooks`, `components`, `security`, `tags`, `servers`, `externalDocs` |
| §4.2 | `Info` | API metadata — `title`, `version`, `summary`, `description`, `termsOfService`, `contact`, `license` |
| §4.3 | `Contact` | Contact info — `name`, `url`, `email` |
| §4.4 | `License` | License info — `name`, `identifier`, `url` |
| §4.5 | `Server`, `ServerVariable` | Server connectivity info with variable templating |
| §4.6 | `Components` | Reusable objects — `schemas`, `responses`, `parameters`, `examples`, `requestBodies`, `headers`, `securitySchemes`, `links`, `callbacks`, `pathItems`, `encodings` |
| §4.7 | `Paths`, `PathItem`, `Operation` | API paths and HTTP operations (GET, PUT, POST, DELETE, OPTIONS, HEAD, PATCH, TRACE, QUERY + custom) |
| §4.8 | `ExternalDocumentation` | External docs reference |
| §4.9 | `Parameter`, `ParameterIn`, `Style` | Operation/path parameters — location, serialization style, `explode`, `allowReserved`, `allowEmptyValue` |
| §4.10 | `RequestBody`, `RequestBodyRef` | Request body with `content` media type map |
| §4.11-4.14 | `MediaType`, `Encoding`, `Header` | Content negotiation, encoding options, header definitions |
| §4.15-4.16 | `Response`, `Responses` | HTTP responses with headers, content, links |
| §4.17 | `Callback` | Out-of-band webhook callbacks keyed by runtime expression |
| §4.18 | `Example` | Named examples — `value`, `summary`, `description`, `externalValue` |
| §4.19 | `Link` | Runtime link with `operationRef`/`operationId`, parameters, description |
| §4.20 | `Tag` | Tag metadata — `name`, `description`, `externalDocs`, `parent`, `kind` |
| §4.21-4.22 | `SecurityScheme`, `OAuthFlows`, `OAuthFlow`, `SecurityRequirement` | Security: apiKey, http, oauth2, mutualTLS, openIdConnect |
| §4.23 | `Reference` | `$ref` reference object |
| §4.24 | `Schema`, `SchemaObject` | JSON Schema 2020-12 superset — `discriminator`, `xml`, `externalDocs`, `example` + raw JSON keywords |
| §4.25 | `Discriminator` | Polymorphism discriminator — `propertyName`, `mapping` |
| §4.26 | `XML` | XML serialization hints — `name`, `namespace`, `prefix`, `attribute`, `wrapped` |
| §5 | `Extensions` | Vendor/specification extensions (patterned fields) |

## `$ref` handling (`RefOr<T>`)

Many OAS fields accept either an inline value or a `$ref` reference. `RefOr<T>` models this as an untagged enum:

```rust
use openapi3_rs::{RefOr, Reference, Schema, SchemaObject};

// Direct schema
let schema = RefOr::Item(Schema::Object(SchemaObject::default()));

// $ref to a component
let schema = RefOr::Ref(Reference::new("#/components/schemas/Pet"));

match schema {
    RefOr::Item(obj) => { /* inline definition */ }
    RefOr::Ref(r) => println!("→ {}", r.ref_path),
}
```

## Schema Object

OAS-specific fields (`discriminator`, `xml`, `externalDocs`, `example`) are typed struct fields. Everything else — `type`, `properties`, `items`, `oneOf`, `allOf`, `$ref`, etc. — lives in `schema_data: Map<String, Value>`. This means:

- No dependency on a separate JSON Schema crate
- Arbitrary JSON Schema keywords round-trip without being typed
- OAS-specific fields are still accessible in a type-safe way
- Boolean schemas (`true` / `false`) are supported

### Validation (optional)

Enable the `validate` feature to check schema data against the JSON Schema 2020-12 meta-schema:

```toml
[dependencies]
openapi3-rs = { version = "0.1", features = ["validate"] }
```

```rust
use openapi3_rs::Schema;

let json = serde_json::json!({
    "type": "object",
    "properties": {
        "id": { "type": "integer" },
        "name": { "type": "string" }
    }
});
let schema = Schema::from_value(json)?;
// With "validate" feature: validates internal JSON Schema structure
```

## Builders (optional)

Enable the `builder` feature for `derive_builder` support:

```toml
[dependencies]
openapi3-rs = { version = "0.1", features = ["builder"] }
```

## Feature flags

| Feature | Default | Description |
|---|---|---|
| `default` | ✓ | No optional features enabled |
| `validate` | — | Schema validation via `jsonschema` |
| `builder` | — | `derive_builder` support |

## Related

- [`openapiv3`](https://crates.io/crates/openapiv3) — OpenAPI 3.0 types (the `RefOr` pattern is borrowed from here)
- [`schemars`](https://crates.io/crates/schemars) — JSON Schema generation from Rust types (used in integration tests)
- [`jsonschema`](https://crates.io/crates/jsonschema) — JSON Schema validator (used by the `validate` feature)

## Contributing

PRs welcome. The goal is OAS 3.x coverage. Check the [spec](https://spec.openapis.org/oas/latest.html) before adding fields or types.

## License

MIT OR Apache-2.0
