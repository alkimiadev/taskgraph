//! Validate all tasks in directory.

use crate::discovery::TaskCollection;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let result = collection.validate();

    if result.is_valid() {
        println!("✓ Valid: {} tasks", result.task_count);
    } else {
        println!("✗ Found {} issue(s)", result.issue_count());
        println!();

        if !result.errors.is_empty() {
            println!("Errors:");
            for err in &result.errors {
                println!("  - {}: {}", err.path.display(), err.message);
            }
        }

        if !result.missing_dependencies.is_empty() {
            println!();
            println!("Missing dependencies:");
            for (task_id, missing) in &result.missing_dependencies {
                println!("  - {} depends on missing: {}", task_id, missing.join(", "));
            }
        }
    }

    Ok(())
}
