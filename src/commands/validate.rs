//! Validate all tasks in directory.

use serde::Serialize;
use std::collections::HashMap;

use crate::cli::OutputFormat;
use crate::discovery::{DiscoveryError, TaskCollection};

/// Validation result for JSON output.
#[derive(Serialize)]
struct ValidationOutput {
    valid: bool,
    task_count: usize,
    error_count: usize,
    errors: Vec<ValidationError>,
    missing_dependencies: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct ValidationError {
    path: String,
    message: String,
}

pub fn execute(
    collection: &TaskCollection,
    strict: bool,
    format: OutputFormat,
) -> crate::Result<()> {
    let result = collection.validate();

    let output = ValidationOutput {
        valid: result.is_valid(),
        task_count: result.task_count,
        error_count: result.issue_count(),
        errors: result
            .errors
            .iter()
            .map(|e| ValidationError {
                path: e.path.display().to_string(),
                message: e.message.clone(),
            })
            .collect(),
        missing_dependencies: result.missing_dependencies.clone(),
    };

    match format {
        OutputFormat::Plain => {
            if result.is_valid() {
                println!("✓ Valid: {} tasks", result.task_count);
            } else {
                println!("✗ Found {} issue(s)", result.issue_count());
                println!();

                if !result.errors.is_empty() {
                    println!("Errors:");
                    for err in &result.errors {
                        println!("  - {}: {}", err.path.display(), err.message);
                    }
                }

                if !result.missing_dependencies.is_empty() {
                    println!();
                    println!("Missing dependencies:");
                    for (task_id, missing) in &result.missing_dependencies {
                        println!("  - {} depends on missing: {}", task_id, missing.join(", "));
                    }
                }
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    if strict && !result.is_valid() {
        return Err(crate::Error::Graph(format!(
            "Validation failed with {} issues",
            result.issue_count()
        )));
    }

    Ok(())
}
