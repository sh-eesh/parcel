use std::collections::HashMap;

use crate::storage::file::FileManager;
use crate::storage::page::Page;

// Manage database pages cached in RAM
pub struct BufferPool {
    // Maps page IDs to their cached pages
    pages: HashMap<u64, Page>,

    // Handles reading pages from the database file
    storage: FileManager,
}

impl BufferPool {
    // Create a new buffer pool with an empty page cache
    pub fn new(storage: FileManager) -> BufferPool {
        BufferPool {
            pages: HashMap::new(),
            storage,
        }
    }

    // Retrieve a page from RAM or load it from disk
    pub fn get_page(&mut self, page_id: u64) -> std::io::Result<&Page> {
        // Check whether the page is already cached
        if !self.pages.contains_key(&page_id) {
            // Load the page from the database file
            let page = self.storage.read_page(page_id)?;

            // Store the loaded page in the RAM cache
            self.pages.insert(page_id, page);
        }

        // Return a reference to the cached page
        Ok(self.pages.get(&page_id).unwrap())
    }
}
