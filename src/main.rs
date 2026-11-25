// Import the database and storage components
use parcel::Database;
use parcel::storage::file::FileManager;
use parcel::storage::page::Page;

fn main() -> std::io::Result<()> {
    // Initialize the Parcel database
    let db = Database::new(String::from("Parcel"));

    // Create a new empty page
    let mut page = Page::new();

    // Store a test value in the first byte
    page.data[0] = 42;
    page.data[1] = 69;

    // Open or create the database file
    let mut storage = FileManager::open("parcel.db")?;

    // Save the page as Page 0
    storage.write_page(0, &page)?;

    // Read Page 0 back from the database file
    let loaded_page = storage.read_page(0)?;

    println!("{} database engine", db.name());
    println!("Page size: {} bytes", loaded_page.data.len());
    println!("Stored value: {}", loaded_page.data[0]);
    println!("Stored value: {}", loaded_page.data[1]);

    Ok(())
}
