//! Find bottleneck tasks (high betweenness centrality).

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);
    let bottlenecks = graph.bottlenecks();

    if bottlenecks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    println!("Bottleneck tasks (ranked by path centrality):");
    println!();
    println!("{:<20} {:>10} {}", "ID", "SCORE", "NAME");
    println!("{}", "-".repeat(60));

    for (task_id, score) in bottlenecks.iter().take(10) {
        let name = collection.get(task_id).map(|t| t.name()).unwrap_or("?");
        println!("{:<20} {:>10} {}", task_id, score, name);
    }

    if bottlenecks.len() > 10 {
        println!("... and {} more", bottlenecks.len() - 10);
    }

    Ok(())
}
