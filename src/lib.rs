//! TaskGraph - CLI tool for managing task dependencies using markdown files.
//!
//! See [`taskgraph::Task`] for the core task structure.

pub mod cache;
pub mod cli;
pub mod commands;
pub mod error;
pub mod graph;
pub mod output;
pub mod task;

pub use cache::Cache;
pub use error::{Error, Result};
pub use graph::DependencyGraph;
pub use task::{Task, TaskFrontmatter, TaskImpact, TaskLevel, TaskRisk, TaskScope, TaskStatus};
