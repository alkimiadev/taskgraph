//! Dependency graph operations.

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

use crate::task::Task;

/// Task ID type.
pub type TaskId = String;

/// Dependency graph built from tasks.
pub struct DependencyGraph {
    /// The underlying directed graph.
    graph: DiGraph<TaskId, ()>,
    /// Map from task ID to node index.
    index_map: HashMap<TaskId, NodeIndex>,
}

impl DependencyGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    /// Build a graph from a list of tasks.
    pub fn from_tasks(tasks: &[Task]) -> Self {
        let mut graph = Self::new();

        // Add all tasks as nodes first
        for task in tasks {
            graph.add_task(task.id().to_string());
        }

        // Add edges for dependencies
        for task in tasks {
            for dep in task.depends_on() {
                // Edge: dep -> task (dependency must complete first)
                graph.add_dependency(dep, task.id());
            }
        }

        graph
    }

    /// Add a task to the graph.
    pub fn add_task(&mut self, id: TaskId) {
        let idx = self.graph.add_node(id.clone());
        self.index_map.insert(id, idx);
    }

    /// Add a dependency edge (from must complete before to).
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        if let (Some(&from_idx), Some(&to_idx)) = (self.index_map.get(from), self.index_map.get(to))
        {
            self.graph.add_edge(from_idx, to_idx, ());
        }
    }

    /// Check if the graph has cycles.
    pub fn has_cycles(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }

    /// Get topological order of tasks.
    /// Returns None if there are cycles.
    pub fn topological_order(&self) -> Option<Vec<TaskId>> {
        match petgraph::algo::toposort(&self.graph, None) {
            Ok(order) => Some(
                order
                    .into_iter()
                    .map(|idx| self.graph[idx].clone())
                    .collect(),
            ),
            Err(_) => None,
        }
    }

    /// Get tasks that this task depends on.
    pub fn dependencies(&self, task_id: &str) -> Vec<TaskId> {
        if let Some(&idx) = self.index_map.get(task_id) {
            self.graph
                .neighbors_directed(idx, petgraph::Direction::Incoming)
                .map(|n| self.graph[n].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get tasks that depend on this task.
    pub fn dependents(&self, task_id: &str) -> Vec<TaskId> {
        if let Some(&idx) = self.index_map.get(task_id) {
            self.graph
                .neighbors_directed(idx, petgraph::Direction::Outgoing)
                .map(|n| self.graph[n].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Export graph in DOT format for visualization.
    pub fn to_dot(&self) -> String {
        format!("{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
