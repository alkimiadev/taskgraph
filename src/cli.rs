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
    /// Initialize/scaffold a new task file
    Init {
        /// Task ID
        id: String,
        /// Task name
        #[arg(short, long)]
        name: Option<String>,
        /// Task scope
        #[arg(short, long)]
        scope: Option<String>,
        /// Task risk
        #[arg(short, long)]
        risk: Option<String>,
    },

    /// Validate all task files
    Validate,

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

    /// Show critical path (longest path through graph)
    Critical,

    /// Show bottleneck tasks (high betweenness centrality)
    Bottleneck,

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
            Commands::Init {
                id,
                name,
                scope,
                risk,
            } => {
                println!(
                    "Init task {} name={:?} scope={:?} risk={:?}",
                    id, name, scope, risk
                );
            }
            Commands::Validate => {
                println!("Validate tasks");
            }
            Commands::List { status, tag } => {
                println!("List tasks (status={:?}, tag={:?})", status, tag);
            }
            Commands::Show { id } => {
                println!("Show task: {}", id);
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
            Commands::Bottleneck => {
                println!("Bottleneck tasks");
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
