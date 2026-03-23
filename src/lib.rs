//! TaskGraph - CLI tool for managing task dependencies using markdown files.
//!
//! See [`taskgraph::Task`] for the core task structure.

pub mod cli;
pub mod task;
pub mod graph;
pub mod cache;
pub mod commands;
pub mod output;
pub mod error;

pub use task::{Task, TaskFrontmatter, TaskStatus};
pub use graph::DependencyGraph;
pub use cache::Cache;
pub use error::{Error, Result};