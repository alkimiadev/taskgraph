//! Flag tasks that should be decomposed further.

use crate::discovery::TaskCollection;
use crate::task::{TaskRisk, TaskScope};

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let mut flagged: Vec<(String, String, Vec<String>)> = Vec::new();

    for task in collection.tasks() {
        let mut reasons = Vec::new();

        if let Some(risk) = task.frontmatter.risk {
            if matches!(risk, TaskRisk::High | TaskRisk::Critical) {
                reasons.push(format!("risk: {}", risk));
            }
        }

        if let Some(scope) = task.frontmatter.scope {
            if matches!(scope, TaskScope::Broad | TaskScope::System) {
                reasons.push(format!("scope: {}", scope));
            }
        }

        if !reasons.is_empty() {
            flagged.push((task.id().to_string(), task.name().to_string(), reasons));
        }
    }

    if flagged.is_empty() {
        println!("No tasks need decomposition.");
        println!("All tasks have risk <= medium and scope <= moderate.");
        return Ok(());
    }

    println!("Tasks that should be decomposed ({}):", flagged.len());
    println!();
    println!("{:<20} REASONS", "ID");
    println!("{}", "-".repeat(60));

    for (id, _name, reasons) in &flagged {
        println!("{:<20} {}", id, reasons.join(", "));
    }

    println!();
    println!("Recommendation: Split these tasks into smaller, lower-risk tasks.");

    Ok(())
}
