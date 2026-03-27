//! Initialize/scaffold a new task file.

use std::path::Path;

use crate::task::{Task, TaskFrontmatter, TaskRisk, TaskScope, TaskStatus};
use chrono::Utc;

pub fn execute(
    id: &str,
    name: Option<&str>,
    scope: Option<&str>,
    risk: Option<&str>,
    tasks_dir: &Path,
) -> crate::Result<()> {
    if !tasks_dir.exists() {
        std::fs::create_dir_all(tasks_dir)?;
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
        scope: scope.and_then(parse_scope),
        risk: risk.and_then(parse_risk),
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
    match s {
        s if s.eq_ignore_ascii_case("single") => Some(TaskScope::Single),
        s if s.eq_ignore_ascii_case("narrow") => Some(TaskScope::Narrow),
        s if s.eq_ignore_ascii_case("moderate") => Some(TaskScope::Moderate),
        s if s.eq_ignore_ascii_case("broad") => Some(TaskScope::Broad),
        s if s.eq_ignore_ascii_case("system") => Some(TaskScope::System),
        _ => None,
    }
}

fn parse_risk(s: &str) -> Option<TaskRisk> {
    match s {
        s if s.eq_ignore_ascii_case("trivial") => Some(TaskRisk::Trivial),
        s if s.eq_ignore_ascii_case("low") => Some(TaskRisk::Low),
        s if s.eq_ignore_ascii_case("medium") => Some(TaskRisk::Medium),
        s if s.eq_ignore_ascii_case("high") => Some(TaskRisk::High),
        s if s.eq_ignore_ascii_case("critical") => Some(TaskRisk::Critical),
        _ => None,
    }
}
