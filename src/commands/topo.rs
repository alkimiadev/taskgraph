//! Topological sort command.

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection, status_filter: Option<&str>) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);

    match graph.topological_order() {
        Some(order) => {
            println!("Topological order:");
            for (i, id) in order.iter().enumerate() {
                if let Some(task) = collection.get(id) {
                    let status = task.status().to_string();

                    // Filter by status if specified
                    if let Some(filter) = status_filter {
                        if status != filter {
                            continue;
                        }
                    }

                    println!("  {}. {} ({})", i + 1, id, status);
                }
            }
        }
        None => {
            println!("Cannot compute topological order: cycle detected.");
            println!("Run 'taskgraph cycles' to see the cycle.");
        }
    }

    Ok(())
}
