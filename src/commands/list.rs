//! List all tasks.

use crate::discovery::TaskCollection;
use crate::task::TaskStatus;

pub fn execute(
    collection: &TaskCollection,
    status_filter: Option<&str>,
    tag_filter: Option<&str>,
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
        println!("No tasks found.");
        return Ok(());
    }

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

    Ok(())
}
