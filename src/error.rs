//! Error types for TaskGraph.

use thiserror::Error;

/// TaskGraph error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Task not found.
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// Task already exists.
    #[error("Task already exists: {0}")]
    TaskAlreadyExists(String),

    /// Circular dependency detected.
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    /// Invalid frontmatter.
    #[error("Invalid frontmatter in {file}: {message}")]
    InvalidFrontmatter {
        /// File path.
        file: String,
        /// Error message.
        message: String,
    },

    /// Missing required field.
    #[error("Missing required field '{field}' in {file}")]
    MissingField {
        /// File path.
        file: String,
        /// Field name.
        field: String,
    },

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing error.
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// Graph error.
    #[error("Graph error: {0}")]
    Graph(String),
}

/// Result type for TaskGraph operations.
pub type Result<T> = std::result::Result<T, Error>;
