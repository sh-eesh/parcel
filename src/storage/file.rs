// Provides file operations and page positioning
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::storage::page::{PAGE_SIZE, Page};

// Handles reading and writing database pages
pub struct FileManager {
    file: File,
}

impl FileManager {
    // Open an existing database file or create a new one
    pub fn open(path: &str) -> std::io::Result<FileManager> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        Ok(FileManager { file })
    }

    // Write to a page at its assigned position in the database file
    pub fn write_page(&mut self, page_id: u64, page: &Page) -> std::io::Result<()> {
        let offset = page_id * PAGE_SIZE as u64;

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&page.data)?;

        Ok(())
    }

    // Read a page from its assigned position in the database file
    pub fn read_page(&mut self, page_id: u64) -> std::io::Result<Page> {
        let offset = page_id * PAGE_SIZE as u64;

        self.file.seek(SeekFrom::Start(offset))?;

        let mut page = Page::new();
        self.file.read_exact(&mut page.data)?;

        Ok(page)
    }

    // Read the current database file size in bytes
    pub fn file_size(&self) -> std::io::Result<u64> {
        let metadata = self.file.metadata()?;

        Ok(metadata.len())
    }
}
