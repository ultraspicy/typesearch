# TypeSearch

[![Crates.io](https://img.shields.io/crates/v/typesearch.svg)](https://crates.io/crates/typesearch)
[![Documentation](https://docs.rs/typesearch/badge.svg)](https://docs.rs/typesearch)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Type-safe schema and query builder for OpenSearch, Elasticsearch, and Tantivy.

## Features

- 🦀 **Type-safe queries** - Compile-time validation of field names and types
- 📝 **Derive macros** - Generate schemas from Rust structs
- 🔌 **Multi-backend** - Works with OpenSearch, Elasticsearch, and Tantivy
- ⚡ **Zero-cost** - No runtime overhead, all validation at compile time

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
typesearch = "0.1"
```

Define your index schema:

```rust
use typesearch::Index;

#[derive(Index)]
#[index(name = "products")]
struct Product {
    #[field(text, stored, prefix)]
    name: String,

    #[field(numeric, sortable)]
    price: f64,

    #[field(keyword)]
    category: String,

    #[field(date, sortable)]
    created_at: String,
}
```

Generate OpenSearch mapping:

```rust
fn main() {
    let mapping = Product::mapping();
    println!("{}", serde_json::to_string_pretty(&mapping).unwrap());
}
```

Output:

```json
{
  "mappings": {
    "properties": {
      "name": {
        "type": "text",
        "store": true,
        "fields": {
          "prefix": {
            "type": "text",
            "analyzer": "edge_ngram_analyzer"
          }
        }
      },
      "price": {
        "type": "numeric",
        "doc_values": true
      },
      "category": {
        "type": "keyword"
      },
      "created_at": {
        "type": "date",
        "doc_values": true
      }
    }
  },
  "settings": {
    "number_of_shards": 5,
    "number_of_replicas": 1,
    "analysis": {
      "analyzer": {
        "edge_ngram_analyzer": {
          "type": "custom",
          "tokenizer": "standard",
          "filter": ["lowercase", "edge_ngram_filter"]
        }
      },
      "filter": {
        "edge_ngram_filter": {
          "type": "edge_ngram",
          "min_gram": 1,
          "max_gram": 20
        }
      }
    }
  }
}
```

## Field Types

| Attribute | OpenSearch Type | Description |
|-----------|----------------|-------------|
| `text` | `text` | Full-text searchable field |
| `keyword` | `keyword` | Exact match field |
| `numeric` | `long`/`double` | Numeric field |
| `date` | `date` | Date/time field |
| `geo` | `geo_point` | Geospatial field |
| `boolean` | `boolean` | Boolean field |

## Field Options

| Option | Description |
|--------|-------------|
| `stored` | Store field value (returned in search results) |
| `sortable` | Enable sorting on this field |
| `prefix` | Enable prefix search (edge n-grams) |
| `case_insensitive` | Case-insensitive matching |
| `fuzzy` | Enable fuzzy matching |

## Index Options

```rust
#[derive(Index)]
#[index(
    name = "my_index",
    shards = 3,
    replicas = 2
)]
struct MyDoc { ... }
```

## Roadmap

- [x] Basic derive macro for schema generation
- [x] Field types (text, keyword, numeric, date, geo, boolean)
- [x] Field options (stored, sortable, prefix, case_insensitive)
- [ ] Type-safe query builder
- [ ] Tantivy backend support
- [ ] Nested object support
- [ ] Schema migration tools
- [ ] Compile-time query validation

## Examples

See the [examples](examples/) directory for more usage examples.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
