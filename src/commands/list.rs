//! List all tasks.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;
use crate::task::TaskStatus;

/// Task summary for JSON output.
#[derive(Serialize)]
struct TaskSummary {
    id: String,
    name: String,
    status: String,
    scope: Option<String>,
}

pub fn execute(
    collection: &TaskCollection,
    status_filter: Option<&str>,
    tag_filter: Option<&str>,
    format: OutputFormat,
) -> crate::Result<()> {
    let tasks: Vec<_> = collection
        .tasks()
        .filter(|task| {
            if let Some(status) = status_filter {
                let task_status = match task.status() {
                    TaskStatus::Pending => "pending",
                    TaskStatus::InProgress => "in-progress",
                    TaskStatus::Completed => "completed",
                    TaskStatus::Failed => "failed",
                    TaskStatus::Blocked => "blocked",
                };
                if task_status != status {
                    return false;
                }
            }
            if let Some(tag) = tag_filter {
                if !task.frontmatter.tags.iter().any(|t| t == tag) {
                    return false;
                }
            }
            true
        })
        .collect();

    if tasks.is_empty() {
        match format {
            OutputFormat::Plain => println!("No tasks found."),
            OutputFormat::Json => println!("[]"),
        }
        return Ok(());
    }

    match format {
        OutputFormat::Plain => {
            println!("{:<20} {:<12} {:<10} NAME", "ID", "STATUS", "SCOPE");
            println!("{}", "-".repeat(70));

            for task in tasks {
                let scope = task
                    .frontmatter
                    .scope
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                println!(
                    "{:<20} {:<12} {:<10} {}",
                    task.id(),
                    task.status(),
                    scope,
                    task.name()
                );
            }
        }
        OutputFormat::Json => {
            let summaries: Vec<TaskSummary> = tasks
                .into_iter()
                .map(|task| TaskSummary {
                    id: task.id().to_string(),
                    name: task.name().to_string(),
                    status: task.status().to_string(),
                    scope: task.frontmatter.scope.map(|s| s.to_string()),
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
    }

    Ok(())
}
