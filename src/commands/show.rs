//! Show task details.

use crate::discovery::TaskCollection;
use crate::Error;

pub fn execute(collection: &TaskCollection, id: &str) -> crate::Result<()> {
    let task = collection
        .get(id)
        .ok_or_else(|| Error::TaskNotFound(id.to_string()))?;

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

    Ok(())
}
