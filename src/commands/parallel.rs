//! Parallel work groups command.

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);
    let groups = graph.parallel_groups();

    if groups.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    println!("Parallel work groups (same group = can run concurrently):");
    println!();

    for (i, group) in groups.iter().enumerate() {
        println!("Generation {}:", i + 1);
        for task_id in group {
            let status = collection
                .get(task_id)
                .map(|t| t.status().to_string())
                .unwrap_or_default();
            println!("  - {} ({})", task_id, status);
        }
        println!();
    }

    println!(
        "Total: {} generations, {} tasks",
        groups.len(),
        collection.len()
    );

    Ok(())
}
