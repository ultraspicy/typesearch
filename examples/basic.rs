use typesearch::Index;

#[derive(Index)]
#[index(name = "products")]
struct Product {
    #[field(text, stored, prefix)]
    name: String,

    #[field(text, stored)]
    description: String,

    #[field(numeric, sortable)]
    price: f64,

    #[field(keyword)]
    category: String,

    #[field(keyword, case_insensitive)]
    brand: String,

    #[field(date, sortable)]
    created_at: String,

    #[field(boolean)]
    in_stock: bool,
}

fn main() {
    println!("Index name: {}", Product::index_name());
    println!("\nFields:");
    for field in Product::fields() {
        println!("  - {} ({:?})", field.name, field.field_type);
    }

    println!("\nOpenSearch Mapping:");
    let mapping = Product::mapping();
    println!("{}", serde_json::to_string_pretty(&mapping).unwrap());
}
