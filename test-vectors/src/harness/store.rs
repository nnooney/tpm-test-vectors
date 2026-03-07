//! The store module provides facilities for using values across multiple test
//! steps.

use std::collections::HashMap;

/// A trait for a key-value store for sharing data between test commands.
pub trait Store {
    /// Inserts a key-value pair into the store, returning the old value if one existed.
    fn insert(&mut self, key: &str, value: &str) -> Option<String>;
    /// Returns the value corresponding to the key, if it exists.
    fn get(&self, key: &str) -> Option<&str>;
}

/// A key-value store for sharing data between test commands.
///
/// The `Store` is used to capture values from TPM responses and make them
/// available for use in subsequent commands. Each test case gets its own
/// `Store`, which is discarded at the end of the test.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InMemoryStore {
    values: HashMap<String, String>,
}

impl InMemoryStore {
    /// Creates a new, empty `Store`.
    pub fn new() -> Self {
        InMemoryStore::default()
    }
}

impl Store for InMemoryStore {
    fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.values.insert(key.to_string(), value.to_string())
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}
