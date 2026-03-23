//! Cache management.

use std::collections::HashMap;
use std::path::PathBuf;

/// Cache storing graph state and file metadata.
#[derive(Debug, Clone)]
pub struct Cache {
    /// Path to the cache directory.
    path: PathBuf,
    /// File modification times.
    mtimes: HashMap<String, u64>,
    /// Task ID to hash mapping.
    task_hashes: HashMap<String, u64>,
}

impl Cache {
    /// Create a new cache at the given path.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            mtimes: HashMap::new(),
            task_hashes: HashMap::new(),
        }
    }

    /// Get the cache directory path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the cache file path.
    pub fn cache_file(&self) -> PathBuf {
        self.path.join("cache.json")
    }

    /// Check if a file has changed since last cache.
    pub fn has_changed(&self, file: &str, mtime: u64) -> bool {
        self.mtimes
            .get(file)
            .is_none_or(|&cached| cached != mtime)
    }

    /// Update the mtime for a file.
    pub fn update_mtime(&mut self, file: String, mtime: u64) {
        self.mtimes.insert(file, mtime);
    }

    /// Get the hash for a task ID.
    pub fn get_hash(&self, task_id: &str) -> Option<u64> {
        self.task_hashes.get(task_id).copied()
    }

    /// Set the hash for a task ID.
    pub fn set_hash(&mut self, task_id: String, hash: u64) {
        self.task_hashes.insert(task_id, hash);
    }
}
