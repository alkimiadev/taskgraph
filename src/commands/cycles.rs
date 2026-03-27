//! Check for cycles command.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

#[derive(Serialize)]
struct CyclesOutput {
    has_cycles: bool,
    cycle_count: usize,
    cycles: Vec<Vec<String>>,
}

pub fn execute(collection: &TaskCollection, format: OutputFormat) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);

    let cycles_found = graph.find_cycles();
    let has_cycles = !cycles_found.is_empty();

    match format {
        OutputFormat::Plain => {
            if has_cycles {
                println!("Found {} cycle(s):", cycles_found.len());

                for (i, cycle) in cycles_found.iter().enumerate() {
                    let path = cycle.join(" → ");
                    println!("  {}. {}", i + 1, path);
                }
            } else {
                println!("No cycles detected.");
            }
        }
        OutputFormat::Json => {
            let output = CyclesOutput {
                has_cycles,
                cycle_count: cycles_found.len(),
                cycles: cycles_found,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}
