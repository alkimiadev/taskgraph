//! Task definition and parsing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Default)]
pub enum TaskStatus {
    /// Task is pending.
    #[default]
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

/// Task scope - how many files/components affected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TaskScope {
    /// Single file, one function.
    Single,
    /// 2-3 files, related changes.
    #[default]
    Narrow,
    /// One component/module.
    Moderate,
    /// Cross-cutting, multiple modules.
    Broad,
    /// Architecture-level change.
    System,
}

/// Task risk - likelihood of failure or iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TaskRisk {
    /// Pattern match, no thinking required.
    Trivial,
    /// Clear path, minimal unknowns.
    #[default]
    Low,
    /// Some decisions needed.
    Medium,
    /// Multiple unknowns, may iterate.
    High,
    /// Architectural implications.
    Critical,
}

/// Task impact - consequence if task fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TaskImpact {
    /// Only affects this task.
    #[default]
    Isolated,
    /// Blocks related tasks.
    Component,
    /// Blocks entire phase.
    Phase,
    /// Requires re-planning.
    Project,
}

/// Task level - type of work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TaskLevel {
    /// Architecture, design.
    Planning,
    /// Breaking down work.
    Decomposition,
    /// Writing code.
    #[default]
    Implementation,
    /// Verification.
    Review,
    /// Information gathering.
    Research,
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

impl TaskScope {
    /// Get approximate token count for this scope.
    pub fn token_estimate(&self) -> u32 {
        match self {
            Self::Single => 500,
            Self::Narrow => 1500,
            Self::Moderate => 3000,
            Self::Broad => 6000,
            Self::System => 10000,
        }
    }

    /// Get relative cost estimate for this scope (ordering value).
    pub fn cost_estimate(&self) -> f64 {
        match self {
            Self::Single => 1.0,
            Self::Narrow => 2.0,
            Self::Moderate => 3.0,
            Self::Broad => 4.0,
            Self::System => 5.0,
        }
    }
}

impl TaskRisk {
    /// Get implied success probability for this risk level.
    pub fn success_probability(&self) -> f64 {
        match self {
            Self::Trivial => 0.98,
            Self::Low => 0.90,
            Self::Medium => 0.80,
            Self::High => 0.65,
            Self::Critical => 0.50,
        }
    }
}

impl TaskImpact {
    /// Get criticality weight for cost-benefit analysis.
    pub fn weight(&self) -> f64 {
        match self {
            Self::Isolated => 1.0,
            Self::Component => 1.5,
            Self::Phase => 2.0,
            Self::Project => 3.0,
        }
    }
}

impl std::fmt::Display for TaskScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Narrow => write!(f, "narrow"),
            Self::Moderate => write!(f, "moderate"),
            Self::Broad => write!(f, "broad"),
            Self::System => write!(f, "system"),
        }
    }
}

impl std::fmt::Display for TaskRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trivial => write!(f, "trivial"),
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

impl std::fmt::Display for TaskImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Isolated => write!(f, "isolated"),
            Self::Component => write!(f, "component"),
            Self::Phase => write!(f, "phase"),
            Self::Project => write!(f, "project"),
        }
    }
}

impl std::fmt::Display for TaskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Decomposition => write!(f, "decomposition"),
            Self::Implementation => write!(f, "implementation"),
            Self::Review => write!(f, "review"),
            Self::Research => write!(f, "research"),
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
    /// Task scope - how many files/components affected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TaskScope>,
    /// Task risk - likelihood of failure or iteration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<TaskRisk>,
    /// Task impact - consequence if task fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<TaskImpact>,
    /// Task level - type of work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<TaskLevel>,
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
