//! Show dependencies command.

use crate::discovery::TaskCollection;
use crate::Error;

pub fn execute(collection: &TaskCollection, id: &str) -> crate::Result<()> {
    let task = collection
        .get(id)
        .ok_or_else(|| Error::TaskNotFound(id.to_string()))?;

    let deps = task.depends_on();

    if deps.is_empty() {
        println!("Task '{}' has no dependencies.", id);
    } else {
        println!("Dependencies of '{}':", id);
        for dep_id in deps {
            let status = collection
                .get(dep_id)
                .map(|t| t.status().to_string())
                .unwrap_or_else(|| "MISSING".to_string());
            println!("  - {} ({})", dep_id, status);
        }
    }

    Ok(())
}
