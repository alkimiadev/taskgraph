//! Calculate relative workflow cost based on structural risk.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;

/// Task cost data for JSON output.
#[derive(Serialize)]
struct TaskCost {
    id: String,
    name: String,
    cost: f64,
}

/// Default parameters for EV calculation (relative units).
const DEFAULT_RETRIES: i32 = 2;
const DEFAULT_FALLBACK_COST: f64 = 20.0;
const DEFAULT_TIME_LOST: f64 = 0.5;
const DEFAULT_VALUE_RATE: f64 = 100.0;

pub fn execute(collection: &TaskCollection, format: OutputFormat) -> crate::Result<()> {
    let mut task_costs: Vec<(String, String, f64)> = Vec::new();
    let mut total_ev = 0.0;

    for task in collection.tasks() {
        let p = task
            .frontmatter
            .risk
            .map(|r| r.success_probability())
            .unwrap_or(0.80);

        let scope_cost = task
            .frontmatter
            .scope
            .map(|s| s.cost_estimate())
            .unwrap_or(2.0);

        let impact_weight = task.frontmatter.impact.map(|i| i.weight()).unwrap_or(1.0);

        let task_ev = calculate_task_ev(p, scope_cost, impact_weight);

        task_costs.push((task.id().to_string(), task.name().to_string(), task_ev));
        total_ev += task_ev;
    }

    if task_costs.is_empty() {
        match format {
            OutputFormat::Plain => println!("No tasks found."),
            OutputFormat::Json => println!("[]"),
        }
        return Ok(());
    }

    task_costs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    match format {
        OutputFormat::Plain => {
            println!("Workflow Cost Analysis ({} tasks):", task_costs.len());
            println!();
            println!("{:<20} {:>10} NAME", "ID", "COST");
            println!("{}", "-".repeat(60));

            for (id, name, cost) in task_costs.iter().take(15) {
                println!("{:<20} {:>10.2} {}", id, cost, name);
            }

            if task_costs.len() > 15 {
                println!("... and {} more", task_costs.len() - 15);
            }

            println!("{}", "-".repeat(60));
            println!("{:<20} {:>10.2} (relative units)", "TOTAL", total_ev);
            println!();

            let avg_ev = total_ev / task_costs.len() as f64;
            println!("Average per task: {:.2}", avg_ev);
            println!();
            println!("Note: Values are relative units for comparison, not dollars.");
        }
        OutputFormat::Json => {
            let costs: Vec<TaskCost> = task_costs
                .into_iter()
                .map(|(id, name, cost)| TaskCost { id, name, cost })
                .collect();
            let json = serde_json::json!({
                "tasks": costs,
                "total": total_ev,
                "count": costs.len(),
                "average": total_ev / costs.len() as f64,
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
    }

    Ok(())
}

fn calculate_task_ev(p: f64, scope_cost: f64, impact_weight: f64) -> f64 {
    let r = DEFAULT_RETRIES;
    let fallback = DEFAULT_FALLBACK_COST;
    let time_lost = DEFAULT_TIME_LOST;
    let value_rate = DEFAULT_VALUE_RATE;

    let c = scope_cost;
    let f = c;

    let p_success = 1.0 - (1.0 - p).powi(r + 1);

    let e_r_given_success = if p_success > 0.0 && p < 1.0 {
        let q = 1.0 - p;
        let k = (r + 1) as f64;
        let numerator = 1.0 - q.powi(r + 1) - k * p * q.powi(r);
        (q / p) * numerator / p_success
    } else {
        0.0
    };

    let c_success = c + f * e_r_given_success;
    let c_fail = c + r as f64 * f + fallback + time_lost * value_rate;

    let ev = p_success * c_success + (1.0 - p_success) * c_fail;

    ev * impact_weight
}
