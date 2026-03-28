//! CLI argument definitions using clap.

use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use crate::discovery::TaskCollection;
use crate::graph::DependencyGraph;

/// Output format for command results.
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum OutputFormat {
    /// Plain text output (default).
    #[default]
    Plain,
    /// JSON output.
    Json,
}

/// CLI tool for managing task dependencies using markdown files.
#[derive(Parser, Debug)]
#[command(name = "taskgraph")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to tasks directory (default: ./tasks)
    #[arg(short, long, global = true)]
    pub path: Option<String>,

    /// Output format
    #[arg(short, long, global = true, value_enum, default_value = "plain")]
    pub format: OutputFormat,

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
    Validate {
        /// Fail on any validation error
        #[arg(short, long)]
        strict: bool,
    },

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

    /// Show risk distribution across tasks
    Risk,

    /// Flag tasks that should be decomposed
    Decompose,

    /// Calculate relative workflow cost
    WorkflowCost {
        /// Include completed tasks in analysis
        #[arg(long)]
        include_completed: bool,
        /// Maximum number of tasks to display
        #[arg(short, long, default_value = "15")]
        limit: usize,
    },

    /// Find path with highest cumulative risk
    RiskPath,

    /// Visualize the dependency graph (DOT format)
    Graph {
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: ShellName,
    },
}

/// Supported shell names for completion generation.
#[derive(Clone, Debug, ValueEnum)]
pub enum ShellName {
    Bash,
    Zsh,
    Fish,
}

impl Cli {
    /// Get the tasks directory path.
    /// Priority: CLI arg > config file > default
    pub fn tasks_path(&self) -> PathBuf {
        if let Some(path) = &self.path {
            return PathBuf::from(path);
        }

        // Check for config file
        if let Some(config) = crate::config::Config::find_and_load() {
            return config.tasks_path();
        }

        // Default
        PathBuf::from("./tasks")
    }

    /// Execute the CLI command.
    pub fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Init {
                id,
                name,
                scope,
                risk,
            } => {
                crate::commands::init::execute(
                    id,
                    name.as_deref(),
                    scope.as_deref(),
                    risk.as_deref(),
                    &self.tasks_path(),
                )?;
            }
            Commands::Validate { strict } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::validate::execute(&collection, *strict, self.format)?;
            }
            Commands::List { status, tag } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::list::execute(
                    &collection,
                    status.as_deref(),
                    tag.as_deref(),
                    self.format,
                )?;
            }
            Commands::Show { id } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::show::execute(&collection, id, self.format)?;
            }
            Commands::Deps { id } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::deps::execute(&collection, id, self.format)?;
            }
            Commands::Dependents { id } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                let graph = DependencyGraph::from_collection(&collection);
                let dependents = graph.dependents(id);

                if dependents.is_empty() {
                    println!("No tasks depend on '{}'.", id);
                } else {
                    println!("Tasks that depend on '{}':", id);
                    for dep_id in dependents {
                        let status = collection
                            .get(&dep_id)
                            .map(|t| t.status().to_string())
                            .unwrap_or_default();
                        println!("  - {} ({})", dep_id, status);
                    }
                }
            }
            Commands::Topo { status } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::topo::execute(&collection, status.as_deref(), self.format)?;
            }
            Commands::Cycles => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::cycles::execute(&collection, self.format)?;
            }
            Commands::Parallel => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::parallel::execute(&collection)?;
            }
            Commands::Critical => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::critical::execute(&collection)?;
            }
            Commands::Bottleneck => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::bottleneck::execute(&collection)?;
            }
            Commands::Risk => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::risk::execute(&collection)?;
            }
            Commands::Decompose => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::decompose::execute(&collection)?;
            }
            Commands::WorkflowCost {
                include_completed,
                limit,
            } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::workflow_cost::execute(
                    &collection,
                    self.format,
                    *include_completed,
                    *limit,
                )?;
            }
            Commands::RiskPath => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::risk_path::execute(&collection)?;
            }
            Commands::Graph { output } => {
                let collection = TaskCollection::from_directory(&self.tasks_path());
                crate::commands::graph_cmd::execute(&collection, output.as_deref())?;
            }
            Commands::Completions { shell } => {
                let shell = match shell {
                    ShellName::Bash => Shell::Bash,
                    ShellName::Zsh => Shell::Zsh,
                    ShellName::Fish => Shell::Fish,
                };
                clap_complete::generate(
                    shell,
                    &mut Cli::command(),
                    "taskgraph",
                    &mut std::io::stdout(),
                );
            }
        }
        Ok(())
    }
}
