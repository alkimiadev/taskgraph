//! Initialize/scaffold a new task file.

pub fn execute(
    id: &str,
    name: Option<&str>,
    scope: Option<&str>,
    risk: Option<&str>,
) -> crate::Result<()> {
    let _ = (id, name, scope, risk);
    // TODO: Create template task file with given ID
    // User fills in body and other frontmatter manually
    Ok(())
}
