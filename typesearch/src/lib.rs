extern crate self as typesearch;

pub use typesearch_derive::Index;

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

    /// Generate the opensearch schema for the given struct
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
        _title: String,
    }

    #[test]
    fn test_index_name() {
        assert_eq!(TestDoc::index_name(), "test_index");
    }

    #[test]
    fn test_mapping_generation() {
        let mapping = TestDoc::mapping();
        assert!(mapping.is_object());
        assert_eq!(mapping["mappings"]["properties"]["_title"]["type"], "text");
        assert_eq!(mapping["mappings"]["properties"]["_title"]["store"], true);
    }
}
