use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod expand;

/// ```rust,ignore
/// use typesearch::Index;
///
/// #[derive(Index)]
/// #[index(name = "products", shards = 3)]
/// struct Product {
///     #[field(text, stored, prefix)]
///     name: String,
///     #[field(numeric, sortable)]
///     price: f64,
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
