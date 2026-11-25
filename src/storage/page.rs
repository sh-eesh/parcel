// Every database page occupies 4096 bytes
pub const PAGE_SIZE: usize = 4096;

// Represent a fixed size block of database storage
pub struct Page {
    pub data: [u8; PAGE_SIZE],
}

impl Page {
    // Create a new page initialized with zeros
    pub fn new() -> Page {
        Page {
            data: [0; PAGE_SIZE],
        }
    }
}
