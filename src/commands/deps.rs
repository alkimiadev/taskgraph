//! Show dependencies command.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;
use crate::Error;

/// Dependency info for JSON output.
#[derive(Serialize)]
struct DependencyInfo {
    id: String,
    status: String,
    exists: bool,
}

#[derive(Serialize)]
struct DependenciesOutput {
    task_id: String,
    dependencies: Vec<DependencyInfo>,
}

pub fn execute(collection: &TaskCollection, id: &str, format: OutputFormat) -> crate::Result<()> {
    let task = collection
        .get(id)
        .ok_or_else(|| Error::TaskNotFound(id.to_string()))?;

    let deps = task.depends_on();

    match format {
        OutputFormat::Plain => {
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
        }
        OutputFormat::Json => {
            let dep_infos: Vec<DependencyInfo> = deps
                .iter()
                .map(|dep_id| {
                    let dep_task = collection.get(dep_id);
                    DependencyInfo {
                        id: dep_id.clone(),
                        status: dep_task
                            .map(|t| t.status().to_string())
                            .unwrap_or_else(|| "MISSING".to_string()),
                        exists: dep_task.is_some(),
                    }
                })
                .collect();

            let output = DependenciesOutput {
                task_id: id.to_string(),
                dependencies: dep_infos,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}
