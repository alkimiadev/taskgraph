//! CLI argument definitions using clap.

use clap::{Parser, Subcommand};

/// CLI tool for managing task dependencies using markdown files.
#[derive(Parser, Debug)]
#[command(name = "taskgraph")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to tasks directory (default: ./tasks)
    #[arg(short, long, global = true)]
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all tasks
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Show details of a specific task
    Show {
        /// Task ID
        id: String,
    },

    /// Create a new task
    Create {
        /// Task ID
        id: String,
        /// Task name
        #[arg(short, long)]
        name: String,
        /// Tasks this depends on
        #[arg(short, long)]
        depends_on: Vec<String>,
        /// Task status
        #[arg(short, long, default_value = "pending")]
        status: String,
    },

    /// Edit an existing task
    Edit {
        /// Task ID
        id: String,
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        /// New status
        #[arg(short, long)]
        status: Option<String>,
        /// Add dependencies
        #[arg(long)]
        add_dep: Vec<String>,
        /// Remove dependencies
        #[arg(long)]
        remove_dep: Vec<String>,
    },

    /// Delete a task
    Delete {
        /// Task ID
        id: String,
    },

    /// Show what a task depends on
    Deps {
        /// Task ID
        id: String,
    },

    /// Show what depends on a task
    Dependents {
        /// Task ID
        id: String,
    },

    /// Show tasks in topological order
    Topo {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },

    /// Check for circular dependencies
    Cycles,

    /// Show groups of tasks that can be done in parallel
    Parallel,

    /// Show critical path / bottleneck tasks
    Critical,

    /// Visualize the dependency graph (DOT format)
    Graph {
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Manage the cache
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum CacheCommands {
    /// Clear the cache
    Clear,
    /// Show cache status
    Status,
}

impl Cli {
    /// Execute the CLI command.
    pub fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::List { status, tag } => {
                println!("List tasks (status={:?}, tag={:?})", status, tag);
            }
            Commands::Show { id } => {
                println!("Show task: {}", id);
            }
            Commands::Create {
                id,
                name,
                depends_on,
                status,
            } => {
                println!(
                    "Create task {} ({}) deps={:?} status={}",
                    id, name, depends_on, status
                );
            }
            Commands::Edit {
                id,
                name,
                status,
                add_dep,
                remove_dep,
            } => {
                println!(
                    "Edit task {} name={:?} status={:?} +{:?} -{:?}",
                    id, name, status, add_dep, remove_dep
                );
            }
            Commands::Delete { id } => {
                println!("Delete task: {}", id);
            }
            Commands::Deps { id } => {
                println!("Dependencies of: {}", id);
            }
            Commands::Dependents { id } => {
                println!("Dependents of: {}", id);
            }
            Commands::Topo { status } => {
                println!("Topological order (status={:?})", status);
            }
            Commands::Cycles => {
                println!("Check for cycles");
            }
            Commands::Parallel => {
                println!("Parallel work groups");
            }
            Commands::Critical => {
                println!("Critical path");
            }
            Commands::Graph { output } => {
                println!("Graph (output={:?})", output);
            }
            Commands::Cache { command } => match command {
                CacheCommands::Clear => println!("Cache cleared"),
                CacheCommands::Status => println!("Cache status"),
            },
        }
        Ok(())
    }
}
