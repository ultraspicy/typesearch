//! Procedural macros for the `typesearch` crate
//!
//! This crate provides derive macros for generating search index schemas.
//! Users should depend on the `typesearch` crate instead, which re-exports
//! these macros.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod expand;

/// Derive macro for generating search index schema
///
/// # Attributes
///
/// ## Container attributes (`#[index(...)]`)
///
/// - `name = "index_name"` - Set the index name (defaults to struct name in snake_case)
/// - `shards = N` - Number of primary shards (default: 5)
/// - `replicas = N` - Number of replica shards (default: 1)
///
/// ## Field attributes (`#[field(...)]`)
///
/// **Field types** (mutually exclusive):
/// - `text` - Full-text searchable field
/// - `keyword` - Exact match field
/// - `numeric` - Numeric field
/// - `date` - Date/time field
/// - `geo` - Geospatial field
/// - `boolean` - Boolean field
///
/// **Field options** (can be combined):
/// - `stored` - Store field value (returned in results)
/// - `sortable` - Enable sorting on this field
/// - `prefix` - Enable prefix search (edge n-grams)
/// - `case_insensitive` - Case-insensitive matching
/// - `fuzzy` - Enable fuzzy matching
///
/// # Example
///
/// ```ignore
/// use typesearch::Index;
///
/// #[derive(Index)]
/// #[index(name = "products", shards = 3)]
/// struct Product {
///     #[field(text, stored, prefix)]
///     name: String,
///
///     #[field(numeric, sortable)]
///     price: f64,
///
///     #[field(keyword)]
///     category: String,
/// }
/// ```
#[proc_macro_derive(Index, attributes(index, field))]
pub fn derive_index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand::expand_derive_index(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
