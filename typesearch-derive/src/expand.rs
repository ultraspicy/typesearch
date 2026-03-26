//! Code generation for derive macros

use crate::attrs::{FieldArgs, IndexArgs};
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

/// Convert identifier to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && prev_is_lower {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
            prev_is_lower = false;
        } else {
            result.push(ch);
            prev_is_lower = ch.is_lowercase();
        }
    }

    result
}

pub fn expand_derive_index(input: &DeriveInput) -> Result<TokenStream> {
    let args = IndexArgs::from_derive_input(input)?;

    let struct_name = &args.ident;
    let index_name = args
        .name
        .unwrap_or_else(|| to_snake_case(&struct_name.to_string()));

    // Extract fields
    let fields = args.data.take_struct().ok_or_else(|| {
        syn::Error::new_spanned(input, "Index can only be derived for structs")
    })?;

    // Generate field metadata
    let field_metadata = generate_field_metadata(&fields)?;

    // Generate mapping JSON
    let mapping_json = generate_mapping(&fields)?;

    // Generate the trait implementation
    Ok(quote! {
        impl typesearch::Index for #struct_name {
            fn index_name() -> &'static str {
                #index_name
            }

            fn mapping() -> typesearch::JsonValue {
                #mapping_json
            }

            fn fields() -> &'static [typesearch::FieldMetadata] {
                &[#(#field_metadata),*]
            }
        }
    })
}

fn generate_field_metadata(fields: &[FieldArgs]) -> Result<Vec<TokenStream>> {
    fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let name_str = name.to_string();
            let field_type_str = field.field_type()?;

            let field_type = match field_type_str {
                "text" => quote! { typesearch::FieldType::Text },
                "keyword" => quote! { typesearch::FieldType::Keyword },
                "numeric" => quote! { typesearch::FieldType::Numeric },
                "date" => quote! { typesearch::FieldType::Date },
                "geo" => quote! { typesearch::FieldType::Geo },
                "boolean" => quote! { typesearch::FieldType::Boolean },
                _ => quote! { typesearch::FieldType::Keyword },
            };

            let stored = field.stored;
            let sortable = field.sortable;
            let prefix = field.prefix;
            let case_insensitive = field.case_insensitive;
            let fuzzy = field.fuzzy;

            let analyzer = if let Some(ref analyzer) = field.analyzer {
                quote! { Some(#analyzer.to_string()) }
            } else {
                quote! { None }
            };

            let min_gram = if let Some(min) = field.min_gram {
                quote! { Some(#min) }
            } else {
                quote! { None }
            };

            let max_gram = if let Some(max) = field.max_gram {
                quote! { Some(#max) }
            } else {
                quote! { None }
            };

            Ok(quote! {
                typesearch::FieldMetadata {
                    name: #name_str,
                    field_type: #field_type,
                    options: typesearch::FieldOptions {
                        stored: #stored,
                        sortable: #sortable,
                        prefix: #prefix,
                        case_insensitive: #case_insensitive,
                        fuzzy: #fuzzy,
                        analyzer: #analyzer,
                        min_gram: #min_gram,
                        max_gram: #max_gram,
                    },
                }
            })
        })
        .collect()
}

fn generate_mapping(fields: &[FieldArgs]) -> Result<TokenStream> {
    let mut properties = Vec::new();

    for field in fields {
        let name = field.ident.as_ref().unwrap().to_string();
        let field_type = field.field_type()?;

        let mut field_props = vec![
            quote! { "type": #field_type }
        ];

        if field.stored {
            field_props.push(quote! { "store": true });
        }

        if field.sortable {
            field_props.push(quote! { "doc_values": true });
        }

        if field.case_insensitive {
            field_props.push(quote! { "normalizer": "lowercase_normalizer" });
        }

        // Add sub-fields for prefix search
        if field.prefix {
            field_props.push(quote! {
                "fields": {
                    "prefix": {
                        "type": "text",
                        "analyzer": "edge_ngram_analyzer"
                    }
                }
            });
        }

        properties.push(quote! {
            #name: { #(#field_props),* }
        });
    }

    Ok(quote! {
        serde_json::json!({
            "mappings": {
                "properties": {
                    #(#properties),*
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
                    },
                    "normalizer": {
                        "lowercase_normalizer": {
                            "type": "custom",
                            "filter": ["lowercase"]
                        }
                    }
                }
            }
        })
    })
}
