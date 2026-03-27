//! Show task details.

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::discovery::TaskCollection;
use crate::Error;

/// Task details for JSON output.
#[derive(Serialize)]
struct TaskDetails {
    id: String,
    name: String,
    status: String,
    depends_on: Vec<String>,
    scope: Option<String>,
    risk: Option<String>,
    impact: Option<String>,
    level: Option<String>,
    tags: Vec<String>,
    body: String,
}

pub fn execute(collection: &TaskCollection, id: &str, format: OutputFormat) -> crate::Result<()> {
    let task = collection
        .get(id)
        .ok_or_else(|| Error::TaskNotFound(id.to_string()))?;

    match format {
        OutputFormat::Plain => {
            println!("ID:     {}", task.id());
            println!("Name:   {}", task.name());
            println!("Status: {}", task.status());

            if !task.depends_on().is_empty() {
                println!("Depends on: {}", task.depends_on().join(", "));
            }

            if let Some(scope) = task.frontmatter.scope {
                println!("Scope:  {}", scope);
            }
            if let Some(risk) = task.frontmatter.risk {
                println!("Risk:   {}", risk);
            }
            if let Some(impact) = task.frontmatter.impact {
                println!("Impact: {}", impact);
            }
            if let Some(level) = task.frontmatter.level {
                println!("Level:  {}", level);
            }
            if !task.frontmatter.tags.is_empty() {
                println!("Tags:   {}", task.frontmatter.tags.join(", "));
            }

            if !task.body.trim().is_empty() {
                println!();
                println!("---");
                println!("{}", task.body.trim());
            }
        }
        OutputFormat::Json => {
            let details = TaskDetails {
                id: task.id().to_string(),
                name: task.name().to_string(),
                status: task.status().to_string(),
                depends_on: task.depends_on().to_vec(),
                scope: task.frontmatter.scope.map(|s| s.to_string()),
                risk: task.frontmatter.risk.map(|r| r.to_string()),
                impact: task.frontmatter.impact.map(|i| i.to_string()),
                level: task.frontmatter.level.map(|l| l.to_string()),
                tags: task.frontmatter.tags.clone(),
                body: task.body.trim().to_string(),
            };
            println!("{}", serde_json::to_string_pretty(&details)?);
        }
    }

    Ok(())
}
