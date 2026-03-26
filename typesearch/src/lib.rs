//! # TypeSearch
//!
//! Type-safe schema and query builder for OpenSearch, Elasticsearch, and Tantivy.
//!
//! ## Features
//!
//! - 🦀 **Type-safe queries** - Compile-time validation of field names and types
//! - 📝 **Derive macros** - Generate schemas from Rust structs
//! - 🔌 **Multi-backend** - Works with OpenSearch, Elasticsearch, and Tantivy
//! - ⚡ **Zero-cost** - No runtime overhead
//!
//! ## Quick Start
//!
//! ```rust
//! use typesearch::Index;
//!
//! #[derive(Index)]
//! #[index(name = "products")]
//! struct Product {
//!     #[field(text, stored)]
//!     name: String,
//!
//!     #[field(numeric, sortable)]
//!     price: f64,
//!
//!     #[field(keyword)]
//!     category: String,
//! }
//!
//! fn main() {
//!     // Generate OpenSearch mapping
//!     let mapping = Product::mapping();
//!     println!("{}", serde_json::to_string_pretty(&mapping).unwrap());
//! }
//! ```
//!
//! ## Field Types
//!
//! - `text` - Full-text searchable field
//! - `keyword` - Exact match field
//! - `numeric` - Numeric field (i64, f64, etc.)
//! - `date` - Date/time field
//! - `geo` - Geospatial field
//!
//! ## Field Options
//!
//! - `stored` - Store the field value (returned in search results)
//! - `sortable` - Enable sorting on this field
//! - `prefix` - Enable prefix search (generates edge n-gram analyzer)
//! - `case_insensitive` - Case-insensitive matching

// Re-export the derive macro
pub use typesearch_derive::Index;

// Re-export serde_json for user convenience
pub use serde_json::Value as JsonValue;

pub mod field;
pub mod schema;
pub mod query;

// Re-export commonly used types
pub use field::{FieldType, FieldOptions, FieldMetadata};
pub use schema::{IndexSchema, FieldMapping};

/// Core trait implemented by the `#[derive(Index)]` macro
///
/// This trait provides methods to introspect the index schema and generate
/// backend-specific configurations.
pub trait Index {
    /// Get the index name
    fn index_name() -> &'static str;

    /// Generate the OpenSearch/Elasticsearch mapping
    fn mapping() -> serde_json::Value;

    /// Get metadata for all fields
    fn fields() -> &'static [FieldMetadata];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Index)]
    #[index(name = "test_index")]
    struct TestDoc {
        #[field(text, stored)]
        title: String,
    }

    #[test]
    fn test_index_name() {
        assert_eq!(TestDoc::index_name(), "test_index");
    }

    #[test]
    fn test_mapping_generation() {
        let mapping = TestDoc::mapping();
        assert!(mapping.is_object());
    }
}
