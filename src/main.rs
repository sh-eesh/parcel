use parcel::Database;
use parcel::storage::page::Page;

fn main() {
    let db = Database::new(String::from("Parcel"));
    let page = Page::new();

    println!("{} db engine.", db.name());
    println!("Page size: {}", page.data.len());
}
