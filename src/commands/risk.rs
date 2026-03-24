//! Show risk distribution across tasks.

use crate::discovery::TaskCollection;
use crate::task::TaskRisk;

pub fn execute(collection: &TaskCollection) -> crate::Result<()> {
    let mut trivial = Vec::new();
    let mut low = Vec::new();
    let mut medium = Vec::new();
    let mut high = Vec::new();
    let mut critical = Vec::new();
    let mut unspecified = Vec::new();

    for task in collection.tasks() {
        let name = task.name().to_string();
        match task.frontmatter.risk {
            Some(TaskRisk::Trivial) => trivial.push((task.id().to_string(), name)),
            Some(TaskRisk::Low) => low.push((task.id().to_string(), name)),
            Some(TaskRisk::Medium) => medium.push((task.id().to_string(), name)),
            Some(TaskRisk::High) => high.push((task.id().to_string(), name)),
            Some(TaskRisk::Critical) => critical.push((task.id().to_string(), name)),
            None => unspecified.push((task.id().to_string(), name)),
        }
    }

    let total = collection.tasks().count();

    if total == 0 {
        println!("No tasks found.");
        return Ok(());
    }

    println!("Risk Distribution ({} tasks):", total);
    println!();

    print_risk_level("CRITICAL", &critical, total);
    print_risk_level("HIGH", &high, total);
    print_risk_level("MEDIUM", &medium, total);
    print_risk_level("LOW", &low, total);
    print_risk_level("TRIVIAL", &trivial, total);

    if !unspecified.is_empty() {
        let pct = (unspecified.len() as f64 / total as f64) * 100.0;
        println!();
        println!("UNSPECIFIED: {} ({:.0}%)", unspecified.len(), pct);
        for (id, name) in &unspecified {
            println!("  - {} ({})", id, name);
        }
    }

    Ok(())
}

fn print_risk_level(label: &str, tasks: &[(String, String)], total: usize) {
    if tasks.is_empty() {
        println!("{}: 0", label);
        return;
    }

    let pct = (tasks.len() as f64 / total as f64) * 100.0;
    println!("{}: {} ({:.0}%)", label, tasks.len(), pct);
    for (id, name) in tasks {
        println!("  - {} ({})", id, name);
    }
}
