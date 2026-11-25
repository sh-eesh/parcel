// Expose the storage module to the rest of the project
pub mod storage;

// Represent a database instance
pub struct Database {
    name: String,
}

impl Database {
    // Create a database instance
    pub fn new(name: String) -> Database {
        Database { name }
    }

    // Return database name without transferring ownership
    pub fn name(&self) -> &str {
        &self.name
    }
}
