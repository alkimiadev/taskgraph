//! TaskGraph - CLI tool for managing task dependencies using markdown files.
//!
//! See [`taskgraph::Task`] for the core task structure.

pub mod cache;
pub mod cli;
pub mod commands;
pub mod discovery;
pub mod error;
pub mod graph;
pub mod output;
pub mod task;

#[cfg(feature = "semantic")]
pub mod embedding;

pub use cache::Cache;
pub use discovery::{DiscoveryError, TaskCollection, ValidationResult};
pub use error::{Error, Result};
pub use graph::DependencyGraph;
pub use task::{Task, TaskFrontmatter, TaskImpact, TaskLevel, TaskRisk, TaskScope, TaskStatus};
