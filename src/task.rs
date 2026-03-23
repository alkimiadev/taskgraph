//! Task definition and parsing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    /// Task is pending.
    Pending,
    /// Task is in progress.
    InProgress,
    /// Task is completed.
    Completed,
    /// Task has failed.
    Failed,
    /// Task is blocked.
    Blocked,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::InProgress => write!(f, "in-progress"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Blocked => write!(f, "blocked"),
        }
    }
}

/// Parsed frontmatter from a task file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFrontmatter {
    /// Unique task identifier.
    pub id: String,
    /// Human-readable task name.
    pub name: String,
    /// Task status.
    #[serde(default)]
    pub status: TaskStatus,
    /// Tasks this task depends on.
    #[serde(default, rename = "depends_on")]
    pub depends_on: Vec<String>,
    /// Task priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Task tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Creation timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    /// Last modification timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
    /// Task assignee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// Due date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,
}

/// A task with its content.
#[derive(Debug, Clone)]
pub struct Task {
    /// Frontmatter metadata.
    pub frontmatter: TaskFrontmatter,
    /// Markdown body content.
    pub body: String,
    /// Source file path (if loaded from file).
    pub source: Option<String>,
}

impl Task {
    /// Get the task ID.
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the task name.
    pub fn name(&self) -> &str {
        &self.frontmatter.name
    }

    /// Get the task status.
    pub fn status(&self) -> TaskStatus {
        self.frontmatter.status
    }

    /// Get the task dependencies.
    pub fn depends_on(&self) -> &[String] {
        &self.frontmatter.depends_on
    }

    /// Parse a task from a markdown file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_markdown(&content, Some(path.display().to_string()))
    }

    /// Parse a task from markdown content.
    ///
    /// # Errors
    /// Returns an error if the frontmatter is invalid or missing required fields.
    pub fn from_markdown(content: &str, source: Option<String>) -> crate::Result<Self> {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let result = matter.parse(content);

        let frontmatter: TaskFrontmatter = result
            .data
            .ok_or_else(|| crate::Error::InvalidFrontmatter {
                file: source.clone().unwrap_or_default(),
                message: "No frontmatter found".to_string(),
            })?
            .deserialize()
            .map_err(|e| crate::Error::InvalidFrontmatter {
                file: source.clone().unwrap_or_default(),
                message: e.to_string(),
            })?;

        Ok(Task {
            frontmatter,
            body: result.content,
            source,
        })
    }

    /// Serialize the task to markdown.
    pub fn to_markdown(&self) -> Result<String, serde_yaml::Error> {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)?;
        Ok(format!("---\n{}---\n{}", frontmatter_yaml, self.body))
    }
}
