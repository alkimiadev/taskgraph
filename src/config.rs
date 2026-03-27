//! Configuration file support.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// TaskGraph configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Project configuration.
    #[serde(default)]
    pub project: ProjectConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Path to tasks directory.
    #[serde(default = "default_tasks_dir")]
    pub tasks_dir: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            tasks_dir: default_tasks_dir(),
        }
    }
}

fn default_tasks_dir() -> String {
    "tasks".to_string()
}

impl Config {
    /// Load configuration from a file.
    pub fn from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| crate::Error::Graph(format!("Config parse error: {}", e)))?;
        Ok(config)
    }

    /// Find and load config file, searching up directory tree.
    pub fn find_and_load() -> Option<Self> {
        let mut current_dir = std::env::current_dir().ok()?;

        loop {
            let config_path = current_dir.join(".taskgraph.toml");
            if config_path.exists() {
                return Self::from_file(&config_path).ok();
            }

            // Try parent directory
            match current_dir.parent() {
                Some(parent) => current_dir = parent.to_path_buf(),
                None => break,
            }
        }

        None
    }

    /// Get the tasks directory path.
    pub fn tasks_path(&self) -> PathBuf {
        PathBuf::from(&self.project.tasks_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.project.tasks_dir, "tasks");
    }

    #[test]
    fn test_load_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[project]
tasks_dir = "my-tasks"
"#
        )
        .unwrap();

        let config = Config::from_file(file.path()).unwrap();
        assert_eq!(config.project.tasks_dir, "my-tasks");
    }
}
