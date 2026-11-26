// Import the database and storage components.
use parcel::Database;
use parcel::storage::buffer_pool::BufferPool;
use parcel::storage::file::FileManager;

fn main() -> std::io::Result<()> {
    // Initialize the Parcel database.
    let db = Database::new(String::from("Parcel"));

    // Open the database file
    let storage = FileManager::open("parcel.db")?;

    // Create a buffer pool connected to the database file
    let mut buffer_pool = BufferPool::new(storage);

    // Retrieve Page 0 from the buffer pool
    let page = buffer_pool.get_page_mut(0)?;

    println!("{} database engine", db.name());
    println!("Page size: {} bytes", page.data.len());
    println!("Stored value: {}", page.data[0]);

    // Change the page value in RAM
    page.data[0] = 12;

    // Request Page 0 again
    let page = buffer_pool.get_page(0)?;

    println!("Retrieved Page 0 again: {}", page.data[0]);

    Ok(())
}
