//! Find path with highest cumulative risk through the graph.

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let graph = DependencyGraph::from_collection(collection);

    if collection.tasks().count() == 0 {
        println!("No tasks found.");
        return Ok(());
    }

    let risk_path = graph.weighted_critical_path(|task_id| {
        let task = collection.get(task_id);
        if let Some(task) = task {
            let risk_weight = task
                .frontmatter
                .risk
                .map(|r| 1.0 - r.success_probability())
                .unwrap_or(0.2);

            let impact_weight = task.frontmatter.impact.map(|i| i.weight()).unwrap_or(1.0);

            risk_weight * impact_weight
        } else {
            0.2
        }
    });

    if risk_path.is_empty() {
        println!("No risk path found (graph may have cycles).");
        return Ok(());
    }

    println!("Highest Risk Path ({} tasks):", risk_path.len());
    println!();
    println!("{:<20} {:<10} {:<10} NAME", "ID", "RISK", "IMPACT");
    println!("{}", "-".repeat(70));

    let mut total_risk = 0.0;
    for task_id in &risk_path {
        if let Some(task) = collection.get(task_id) {
            let risk_str = task
                .frontmatter
                .risk
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".to_string());
            let impact_str = task
                .frontmatter
                .impact
                .map(|i| i.to_string())
                .unwrap_or_else(|| "-".to_string());

            let risk_value = task
                .frontmatter
                .risk
                .map(|r| 1.0 - r.success_probability())
                .unwrap_or(0.2);
            let impact_value = task.frontmatter.impact.map(|i| i.weight()).unwrap_or(1.0);
            total_risk += risk_value * impact_value;

            println!(
                "{:<20} {:<10} {:<10} {}",
                task_id,
                risk_str,
                impact_str,
                task.name()
            );
        }
    }

    println!("{}", "-".repeat(70));
    println!("Total risk score: {:.2}", total_risk);
    println!();
    println!("This path has the highest cumulative risk in the workflow.");
    println!("Consider decomposing high-risk tasks or adding safeguards.");

    Ok(())
}
