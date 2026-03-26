//! Attribute parsing for derive macros

use darling::{FromDeriveInput, FromField};
use syn::{Ident, Type};

/// Container attributes: #[index(...)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(index), supports(struct_named))]
pub struct IndexArgs {
    pub ident: Ident,
    pub data: darling::ast::Data<(), FieldArgs>,

    /// Index name (defaults to struct name in snake_case)
    #[darling(default)]
    pub name: Option<String>,

    /// Number of shards
    #[darling(default)]
    pub shards: Option<u32>,

    /// Number of replicas
    #[darling(default)]
    pub replicas: Option<u32>,
}

/// Field attributes: #[field(...)]
#[derive(Debug, FromField)]
#[darling(attributes(field))]
pub struct FieldArgs {
    pub ident: Option<Ident>,
    pub ty: Type,

    // Field types (mutually exclusive)
    #[darling(default)]
    pub text: bool,

    #[darling(default)]
    pub keyword: bool,

    #[darling(default)]
    pub numeric: bool,

    #[darling(default)]
    pub date: bool,

    #[darling(default)]
    pub geo: bool,

    #[darling(default)]
    pub boolean: bool,

    // Field options
    #[darling(default)]
    pub stored: bool,

    #[darling(default)]
    pub sortable: bool,

    #[darling(default)]
    pub prefix: bool,

    #[darling(default)]
    pub case_insensitive: bool,

    #[darling(default)]
    pub fuzzy: bool,

    #[darling(default)]
    pub analyzer: Option<String>,

    #[darling(default)]
    pub min_gram: Option<u32>,

    #[darling(default)]
    pub max_gram: Option<u32>,
}

impl FieldArgs {
    /// Determine the field type from the attributes
    pub fn field_type(&self) -> Result<&'static str, darling::Error> {
        let types: Vec<&str> = [
            (self.text, "text"),
            (self.keyword, "keyword"),
            (self.numeric, "numeric"),
            (self.date, "date"),
            (self.geo, "geo"),
            (self.boolean, "boolean"),
        ]
        .iter()
        .filter_map(|(flag, name)| if *flag { Some(*name) } else { None })
        .collect();

        match types.len() {
            0 => Ok("keyword"), // Default to keyword if no type specified
            1 => Ok(types[0]),
            _ => Err(darling::Error::custom(
                "Multiple field types specified. Use only one: text, keyword, numeric, date, geo, or boolean"
            )),
        }
    }
}
