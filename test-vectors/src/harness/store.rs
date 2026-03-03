//! The store module provides facilities for using values across multiple test
//! steps.

use std::collections::HashMap;

/// A key-value store for sharing data between test commands.
///
/// The `Store` is used to capture values from TPM responses and make them
/// available for use in subsequent commands. Each test case gets its own
/// `Store`, which is discarded at the end of the test.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Store {
    values: HashMap<String, String>,
}

impl Store {
    /// Creates a new, empty `Store`.
    pub fn new() -> Self {
        Store::default()
    }

    /// Inserts a key-value pair into the store.
    ///
    /// If the store did not have this key present, `None` is returned.
    /// If the store did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.values.insert(key.into(), value.into())
    }

    /// Returns the value corresponding to the key, if it exists.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}
