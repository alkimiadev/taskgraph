//! Graph visualization command.

use std::path::PathBuf;

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection, output: Option<&str>) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);
    let dot = graph.to_dot();

    if let Some(path) = output {
        let path = PathBuf::from(path);
        std::fs::write(&path, &dot)?;
        println!("Graph written to: {}", path.display());
        println!(
            "Render with: dot -Tpng {} -o {}.png",
            path.display(),
            path.display()
        );
    } else {
        println!("{}", dot);
    }

    Ok(())
}
