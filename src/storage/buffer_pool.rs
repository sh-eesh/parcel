use std::collections::{HashMap, HashSet};

use crate::storage::file::FileManager;
use crate::storage::page::{PAGE_SIZE, Page};

// Manage database pages cached in RAM
pub struct BufferPool {
    // Maps page IDs to their cached pages
    pages: HashMap<u64, Page>,

    // Handles reading pages from the database file
    storage: FileManager,

    // Tracks pages that have been modified in RAM
    dirty_pages: HashSet<u64>,
}

impl BufferPool {
    // Create a new buffer pool with an empty page cache
    pub fn new(storage: FileManager) -> BufferPool {
        BufferPool {
            pages: HashMap::new(),
            storage,
            dirty_pages: HashSet::new(),
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

    // Retrieve a mutable page from RAM or load it from disk
    pub fn get_page_mut(&mut self, page_id: u64) -> std::io::Result<&mut Page> {
        // Check whether the page is already cached
        if !self.pages.contains_key(&page_id) {
            // Load the page from the database file
            let page = self.storage.read_page(page_id)?;

            // Store the loaded page in the RAM cache
            self.pages.insert(page_id, page);
        }

        // Mark the page as modified
        self.dirty_pages.insert(page_id);

        // Return a mutable reference to the cached page
        Ok(self.pages.get_mut(&page_id).unwrap())
    }

    // Save a modified page back to the database file
    pub fn flush_page(&mut self, page_id: u64) -> std::io::Result<()> {
        // Check whether the page has been modified
        if self.dirty_pages.contains(&page_id) {
            // Get page from RAM
            let page = self.pages.get(&page_id).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Page not found in buffer pool.",
                )
            })?;

            // Write page back to disk
            self.storage.write_page(page_id, page)?;

            // Mark the page as clean
            self.dirty_pages.remove(&page_id);
        }

        Ok(())
    }

    // Save all modified pages to the database file
    pub fn flush_all(&mut self) -> std::io::Result<()> {
        // Collect IDs of all dirty pages
        let page_ids: Vec<u64> = self.dirty_pages.iter().copied().collect();

        // Save each dirty page
        for page_id in page_ids {
            self.flush_page(page_id)?;
        }

        Ok(())
    }

    // Create a new page and add it to the buffer pool
    pub fn new_page(&mut self) -> std::io::Result<u64> {
        // Get the current database file size
        let file_size = self.storage.file_size()?;

        // Calculate how many pages already exist on disk
        let disk_page_count = file_size / PAGE_SIZE as u64;

        // Account for pages created in RAM but not yet saved
        let cached_next_id = self.pages.keys().max().map_or(0, |id| id + 1);

        // Determine the next available page ID
        let page_id = disk_page_count.max(cached_next_id);

        // Create an empty page
        let page = Page::new();

        // Store the page in the buffer pool
        self.pages.insert(page_id, page);

        // Mark the new page as dirty
        self.dirty_pages.insert(page_id);

        // Return the new page ID
        Ok(page_id)
    }
}
