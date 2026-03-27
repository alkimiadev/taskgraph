//! Topological sort command.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

/// Task in topological order.
#[derive(Serialize)]
struct TopoTask {
    position: usize,
    id: String,
    name: String,
    status: String,
}

#[derive(Serialize)]
struct TopoOutput {
    order: Vec<TopoTask>,
    has_cycles: bool,
}

pub fn execute(
    collection: &TaskCollection,
    status_filter: Option<&str>,
    format: OutputFormat,
) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);

    match graph.topological_order() {
        Some(order) => {
            let mut tasks = Vec::new();

            for (i, id) in order.iter().enumerate() {
                if let Some(task) = collection.get(id) {
                    let status = task.status().to_string();

                    // Filter by status if specified
                    if let Some(filter) = status_filter {
                        if status != filter {
                            continue;
                        }
                    }

                    tasks.push(TopoTask {
                        position: i + 1,
                        id: id.clone(),
                        name: task.name().to_string(),
                        status,
                    });
                }
            }

            match format {
                OutputFormat::Plain => {
                    println!("Topological order:");
                    for task in &tasks {
                        println!("  {}. {} ({})", task.position, task.id, task.status);
                    }
                }
                OutputFormat::Json => {
                    let output = TopoOutput {
                        order: tasks,
                        has_cycles: false,
                    };
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
            }
        }
        None => match format {
            OutputFormat::Plain => {
                println!("Cannot compute topological order: cycle detected.");
                println!("Run 'taskgraph cycles' to see the cycle.");
            }
            OutputFormat::Json => {
                let output = TopoOutput {
                    order: vec![],
                    has_cycles: true,
                };
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        },
    }

    Ok(())
}
