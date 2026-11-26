use parcel::storage::buffer_pool::BufferPool;
use parcel::storage::file::FileManager;

fn main() -> std::io::Result<()> {
    // Open the database file
    let storage = FileManager::open("parcel.db")?;

    // Create a buffer pool
    let mut buffer_pool = BufferPool::new(storage);

    // Retrieve Page 0 and modify its first byte also print its value
    let page = buffer_pool.get_page_mut(0)?;
    page.data[0] = 12;

    println!("Page 0 value in RAM {}", page.data[0]);

    // Save the modified page to disk
    buffer_pool.flush_page(0)?;

    // Open the database file again
    let mut storage = FileManager::open("parcel.db")?;

    // Read Page 0 directly from disk
    let disk_page = storage.read_page(0)?;

    println!("Page 0 value on SSD: {}", disk_page.data[0]);

    Ok(())
}
