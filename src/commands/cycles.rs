//! Check for cycles command.

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);

    if graph.has_cycles() {
        let cycles = graph.find_cycles();
        println!("Found {} cycle(s):", cycles.len());

        for (i, cycle) in cycles.iter().enumerate() {
            let path = cycle.join(" → ");
            println!("  {}. {}", i + 1, path);
        }
    } else {
        println!("No cycles detected.");
    }

    Ok(())
}
