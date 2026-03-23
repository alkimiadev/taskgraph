//! Task discovery and collection.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::task::Task;
use crate::Error;

/// Collection of tasks discovered from a directory.
#[derive(Debug, Default)]
pub struct TaskCollection {
    /// Tasks indexed by ID.
    tasks: HashMap<String, Task>,
    /// File paths indexed by ID.
    paths: HashMap<String, PathBuf>,
    /// Parse errors encountered during discovery.
    errors: Vec<DiscoveryError>,
}

/// Error encountered during task discovery.
#[derive(Debug, Clone)]
pub struct DiscoveryError {
    /// File path where error occurred.
    pub path: PathBuf,
    /// Error message.
    pub message: String,
}

impl TaskCollection {
    /// Create an empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Discover all tasks in a directory.
    ///
    /// Scans recursively for `*.md` files and attempts to parse each.
    /// Files without valid frontmatter are skipped.
    ///
    /// # Arguments
    /// * `path` - Directory to scan for tasks.
    ///
    /// # Returns
    /// Collection with all valid tasks and any parse errors.
    pub fn from_directory(path: &Path) -> Self {
        let mut collection = Self::new();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_path = entry.path();

            // Only process .md files
            if file_path.extension().map_or(false, |ext| ext == "md") {
                match Task::from_file(file_path) {
                    Ok(task) => {
                        let id = task.id().to_string();

                        // Check for duplicate IDs
                        if collection.tasks.contains_key(&id) {
                            collection.errors.push(DiscoveryError {
                                path: file_path.to_path_buf(),
                                message: format!("Duplicate task ID: {}", id),
                            });
                        } else {
                            collection.tasks.insert(id.clone(), task);
                            collection.paths.insert(id, file_path.to_path_buf());
                        }
                    }
                    Err(Error::InvalidFrontmatter { message, .. }) => {
                        // Skip files without valid frontmatter (not an error)
                        if message != "No frontmatter found" {
                            collection.errors.push(DiscoveryError {
                                path: file_path.to_path_buf(),
                                message,
                            });
                        }
                    }
                    Err(e) => {
                        collection.errors.push(DiscoveryError {
                            path: file_path.to_path_buf(),
                            message: e.to_string(),
                        });
                    }
                }
            }
        }

        collection
    }

    /// Get a task by ID.
    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    /// Get the file path for a task ID.
    pub fn path(&self, id: &str) -> Option<&PathBuf> {
        self.paths.get(id)
    }

    /// Get all tasks.
    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.values()
    }

    /// Get all task IDs.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.tasks.keys().map(|s| s.as_str())
    }

    /// Get the number of tasks.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Check if collection is empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Get discovery errors.
    pub fn errors(&self) -> &[DiscoveryError] {
        &self.errors
    }

    /// Check for missing dependencies.
    ///
    /// Returns a map from task ID to list of missing dependency IDs.
    pub fn missing_dependencies(&self) -> HashMap<String, Vec<String>> {
        let mut missing = HashMap::new();

        for (id, task) in &self.tasks {
            let task_missing: Vec<String> = task
                .depends_on()
                .iter()
                .filter(|dep| !self.tasks.contains_key(*dep))
                .cloned()
                .collect();

            if !task_missing.is_empty() {
                missing.insert(id.clone(), task_missing);
            }
        }

        missing
    }

    /// Validate the collection.
    ///
    /// Returns validation issues found:
    /// - Duplicate IDs (already caught during discovery)
    /// - Missing dependencies
    /// - Parse errors
    pub fn validate(&self) -> ValidationResult {
        ValidationResult {
            task_count: self.tasks.len(),
            errors: self.errors.clone(),
            missing_dependencies: self.missing_dependencies(),
        }
    }
}

/// Result of validating a task collection.
#[derive(Debug)]
pub struct ValidationResult {
    /// Number of valid tasks.
    pub task_count: usize,
    /// Parse errors encountered.
    pub errors: Vec<DiscoveryError>,
    /// Missing dependencies (task_id -> missing_dep_ids).
    pub missing_dependencies: HashMap<String, Vec<String>>,
}

impl ValidationResult {
    /// Check if validation passed (no errors).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() && self.missing_dependencies.is_empty()
    }

    /// Get total issue count.
    pub fn issue_count(&self) -> usize {
        self.errors.len() + self.missing_dependencies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_temp_tasks(dir: &Path, files: &[(&str, &str)]) {
        for (name, content) in files {
            fs::write(dir.join(name), content).unwrap();
        }
    }

    #[test]
    fn test_discover_single_task() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_tasks(
            dir.path(),
            &[(
                "test.md",
                "---\nid: test\nname: Test Task\n---\nBody content",
            )],
        );

        let collection = TaskCollection::from_directory(dir.path());

        assert_eq!(collection.len(), 1);
        assert!(collection.get("test").is_some());
        assert!(collection.errors().is_empty());
    }

    #[test]
    fn test_skip_files_without_frontmatter() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_tasks(
            dir.path(),
            &[
                ("valid.md", "---\nid: valid\nname: Valid\n---\nBody"),
                ("invalid.md", "No frontmatter here"),
                ("readme.md", "# README\n\nNo frontmatter"),
            ],
        );

        let collection = TaskCollection::from_directory(dir.path());

        assert_eq!(collection.len(), 1);
        assert!(collection.get("valid").is_some());
        assert!(collection.errors().is_empty()); // Skipped files are not errors
    }

    #[test]
    fn test_detect_duplicate_ids() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_tasks(
            dir.path(),
            &[
                ("task1.md", "---\nid: same-id\nname: First\n---\n"),
                ("task2.md", "---\nid: same-id\nname: Second\n---\n"),
            ],
        );

        let collection = TaskCollection::from_directory(dir.path());

        assert_eq!(collection.len(), 1); // Only one kept
        assert_eq!(collection.errors().len(), 1); // Duplicate detected
        assert!(collection.errors()[0].message.contains("Duplicate"));
    }

    #[test]
    fn test_missing_dependencies() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_tasks(
            dir.path(),
            &[(
                "task1.md",
                "---\nid: task-1\nname: Task 1\ndepends_on: [missing-task]\n---\n",
            )],
        );

        let collection = TaskCollection::from_directory(dir.path());
        let missing = collection.missing_dependencies();

        assert_eq!(missing.len(), 1);
        assert!(missing.contains_key("task-1"));
        assert!(missing["task-1"].contains(&"missing-task".to_string()));
    }

    #[test]
    fn test_validation_result() {
        let dir = tempfile::tempdir().unwrap();
        create_temp_tasks(
            dir.path(),
            &[("valid.md", "---\nid: valid\nname: Valid\n---\n")],
        );

        let collection = TaskCollection::from_directory(dir.path());
        let result = collection.validate();

        assert!(result.is_valid());
        assert_eq!(result.task_count, 1);
    }
}
