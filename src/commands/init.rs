//! Initialize/scaffold a new task file.

use std::path::PathBuf;

use crate::task::{Task, TaskFrontmatter, TaskRisk, TaskScope, TaskStatus};
use chrono::Utc;

pub fn execute(
    id: &str,
    name: Option<&str>,
    scope: Option<&str>,
    risk: Option<&str>,
) -> crate::Result<()> {
    let tasks_dir = PathBuf::from("./tasks");
    if !tasks_dir.exists() {
        std::fs::create_dir_all(&tasks_dir)?;
    }

    let file_path = tasks_dir.join(format!("{}.md", id));

    if file_path.exists() {
        return Err(crate::Error::TaskAlreadyExists(id.to_string()));
    }

    let frontmatter = TaskFrontmatter {
        id: id.to_string(),
        name: name.unwrap_or(id).to_string(),
        status: TaskStatus::Pending,
        depends_on: vec![],
        priority: None,
        tags: vec![],
        created: Some(Utc::now()),
        modified: Some(Utc::now()),
        assignee: None,
        due: None,
        scope: scope.and_then(|s| parse_scope(s)),
        risk: risk.and_then(|r| parse_risk(r)),
        impact: None,
        level: None,
    };

    let task = Task {
        frontmatter,
        body: "\n# Description\n\n[Add task description here]\n".to_string(),
        source: Some(file_path.display().to_string()),
    };

    let content = task.to_markdown()?;
    std::fs::write(&file_path, content)?;

    println!("Created: {}", file_path.display());
    println!("Edit the file to add dependencies and details.");

    Ok(())
}

fn parse_scope(s: &str) -> Option<TaskScope> {
    match s.to_lowercase().as_str() {
        "single" => Some(TaskScope::Single),
        "narrow" => Some(TaskScope::Narrow),
        "moderate" => Some(TaskScope::Moderate),
        "broad" => Some(TaskScope::Broad),
        "system" => Some(TaskScope::System),
        _ => None,
    }
}

fn parse_risk(s: &str) -> Option<TaskRisk> {
    match s.to_lowercase().as_str() {
        "trivial" => Some(TaskRisk::Trivial),
        "low" => Some(TaskRisk::Low),
        "medium" => Some(TaskRisk::Medium),
        "high" => Some(TaskRisk::High),
        "critical" => Some(TaskRisk::Critical),
        _ => None,
    }
}
