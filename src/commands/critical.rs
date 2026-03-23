//! Critical path command.

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);
    let path = graph.critical_path();

    if path.is_empty() {
        println!("No critical path found (empty graph or cycle).");
        return Ok(());
    }

    println!("Critical path (longest path through graph):");
    println!();

    for (i, task_id) in path.iter().enumerate() {
        let task = collection.get(task_id);
        let name = task.map(|t| t.name()).unwrap_or("?");
        let status = task.map(|t| t.status().to_string()).unwrap_or_default();

        if i > 0 {
            println!("  ↓");
        }
        println!("  {} - {} ({})", task_id, name, status);
    }

    println!();
    println!("Length: {} tasks", path.len());

    Ok(())
}
