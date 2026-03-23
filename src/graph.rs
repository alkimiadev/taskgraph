//! Dependency graph operations.

use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use std::collections::{HashMap, HashSet};

use crate::discovery::TaskCollection;
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

    /// Build a graph from a task collection.
    pub fn from_collection(collection: &TaskCollection) -> Self {
        Self::from_tasks(collection.tasks().collect())
    }

    /// Build a graph from a list of tasks.
    pub fn from_tasks(tasks: Vec<&Task>) -> Self {
        let mut graph = Self::new();

        // Add all tasks as nodes first
        for task in &tasks {
            graph.add_task(task.id().to_string());
        }

        // Add edges for dependencies
        for task in &tasks {
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

    /// Find all cycles in the graph.
    /// Returns a list of cycles, where each cycle is a list of task IDs.
    pub fn find_cycles(&self) -> Vec<Vec<TaskId>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in self.graph.node_indices() {
            if !visited.contains(&node) {
                self.dfs_cycles(node, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycles(
        &self,
        node: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        rec_stack: &mut HashSet<NodeIndex>,
        path: &mut Vec<TaskId>,
        cycles: &mut Vec<Vec<TaskId>>,
    ) {
        visited.insert(node);
        rec_stack.insert(node);
        path.push(self.graph[node].clone());

        for neighbor in self.graph.neighbors_directed(node, Direction::Outgoing) {
            if !visited.contains(&neighbor) {
                self.dfs_cycles(neighbor, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(&neighbor) {
                // Found a cycle - extract it from path
                let cycle_start = path.iter().position(|id| id == &self.graph[neighbor]);
                if let Some(start) = cycle_start {
                    let cycle: Vec<TaskId> = path[start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        path.pop();
        rec_stack.remove(&node);
    }

    /// Get topological order of tasks.
    /// Returns None if there are cycles.
    pub fn topological_order(&self) -> Option<Vec<TaskId>> {
        match toposort(&self.graph, None) {
            Ok(order) => Some(
                order
                    .into_iter()
                    .map(|idx| self.graph[idx].clone())
                    .collect(),
            ),
            Err(_) => None,
        }
    }

    /// Get tasks that this task depends on (direct).
    pub fn dependencies(&self, task_id: &str) -> Vec<TaskId> {
        if let Some(&idx) = self.index_map.get(task_id) {
            self.graph
                .neighbors_directed(idx, Direction::Incoming)
                .map(|n| self.graph[n].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get tasks that depend on this task (direct).
    pub fn dependents(&self, task_id: &str) -> Vec<TaskId> {
        if let Some(&idx) = self.index_map.get(task_id) {
            self.graph
                .neighbors_directed(idx, Direction::Outgoing)
                .map(|n| self.graph[n].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get parallel work groups (generations).
    /// Tasks in the same group have no dependencies on each other.
    pub fn parallel_groups(&self) -> Vec<Vec<TaskId>> {
        let mut groups = Vec::new();
        let mut completed: HashSet<TaskId> = HashSet::new();
        let remaining: HashSet<TaskId> = self.index_map.keys().cloned().collect();

        while completed.len() < remaining.len() {
            // Find tasks whose dependencies are all completed
            let mut next_group = Vec::new();

            for task_id in &remaining {
                if completed.contains(task_id) {
                    continue;
                }

                let deps = self.dependencies(task_id);
                if deps.iter().all(|d| completed.contains(d)) {
                    next_group.push(task_id.clone());
                }
            }

            if next_group.is_empty() {
                // Cycle detected - remaining tasks are in a cycle
                break;
            }

            next_group.sort();
            groups.push(next_group.clone());
            completed.extend(next_group);
        }

        groups
    }

    /// Get critical path (longest path through graph).
    /// Returns the sequence of tasks on the critical path.
    pub fn critical_path(&self) -> Vec<TaskId> {
        // Find tasks with no dependents (endpoints)
        let endpoints: Vec<_> = self
            .index_map
            .keys()
            .filter(|id| self.dependents(id).is_empty())
            .collect();

        // Find longest path to each endpoint
        let mut best_path = Vec::new();

        for endpoint in endpoints {
            let path = self.longest_path_to(endpoint);
            if path.len() > best_path.len() {
                best_path = path;
            }
        }

        best_path
    }

    fn longest_path_to(&self, target: &str) -> Vec<TaskId> {
        let mut cache: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
        self.longest_path_to_recursive(target, &mut cache)
    }

    fn longest_path_to_recursive(
        &self,
        target: &str,
        cache: &mut HashMap<TaskId, Vec<TaskId>>,
    ) -> Vec<TaskId> {
        if let Some(cached) = cache.get(target) {
            return cached.clone();
        }

        let deps = self.dependencies(target);

        let result = if deps.is_empty() {
            vec![target.to_string()]
        } else {
            let mut best_dep_path = Vec::new();
            for dep in &deps {
                let dep_path = self.longest_path_to_recursive(dep, cache);
                if dep_path.len() > best_dep_path.len() {
                    best_dep_path = dep_path;
                }
            }
            let mut result = best_dep_path;
            result.push(target.to_string());
            result
        };

        cache.insert(target.to_string(), result.clone());
        result
    }

    /// Get bottleneck tasks (high betweenness centrality).
    /// Returns tasks sorted by how many paths they're on.
    pub fn bottlenecks(&self) -> Vec<(TaskId, usize)> {
        let mut path_counts: HashMap<TaskId, usize> = HashMap::new();

        // For each pair of nodes, find paths and count
        for start in self.index_map.keys() {
            for end in self.index_map.keys() {
                if start != end {
                    let paths = self.all_paths(start, end);
                    for path in paths {
                        for task_id in &path {
                            *path_counts.entry(task_id.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        let mut result: Vec<_> = path_counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }

    fn all_paths(&self, start: &str, end: &str) -> Vec<Vec<TaskId>> {
        let mut paths = Vec::new();
        let mut current_path = vec![start.to_string()];
        let mut visited = HashSet::new();
        visited.insert(start.to_string());

        self.dfs_paths(start, end, &mut current_path, &mut visited, &mut paths);

        paths
    }

    fn dfs_paths(
        &self,
        current: &str,
        target: &str,
        current_path: &mut Vec<TaskId>,
        visited: &mut HashSet<TaskId>,
        paths: &mut Vec<Vec<TaskId>>,
    ) {
        if current == target {
            paths.push(current_path.clone());
            return;
        }

        for neighbor in self.dependents(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                current_path.push(neighbor.clone());
                self.dfs_paths(&neighbor, target, current_path, visited, paths);
                current_path.pop();
                visited.remove(&neighbor);
            }
        }
    }

    /// Export graph in DOT format for visualization.
    pub fn to_dot(&self) -> String {
        let mut output = String::from("digraph taskgraph {\n");
        output.push_str("  rankdir=LR;\n");

        for node in self.graph.node_indices() {
            let id = &self.graph[node];
            output.push_str(&format!("  \"{}\";\n", id));
        }

        for edge in self.graph.edge_indices() {
            let (source, target) = self.graph.edge_endpoints(edge).unwrap();
            output.push_str(&format!(
                "  \"{}\" -> \"{}\";\n",
                self.graph[source], self.graph[target]
            ));
        }

        output.push_str("}\n");
        output
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_task("c".to_string());
        graph.add_task("d".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");
        graph.add_dependency("a", "d");
        graph
    }

    #[test]
    fn test_empty_graph() {
        let graph = DependencyGraph::new();
        assert!(!graph.has_cycles());
        assert_eq!(graph.topological_order(), Some(Vec::<String>::new()));
        assert_eq!(graph.parallel_groups(), Vec::<Vec<String>>::new());
    }

    #[test]
    fn test_add_task() {
        let mut graph = DependencyGraph::new();
        graph.add_task("task-1".to_string());
        assert_eq!(graph.dependencies("task-1"), Vec::<String>::new());
        assert_eq!(graph.dependents("task-1"), Vec::<String>::new());
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_dependency("a", "b");

        assert_eq!(graph.dependencies("b"), vec!["a".to_string()]);
        assert_eq!(graph.dependents("a"), vec!["b".to_string()]);
    }

    #[test]
    fn test_missing_dependency_ignored() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_dependency("missing", "a");
        graph.add_dependency("a", "missing");

        assert_eq!(graph.dependencies("a"), Vec::<String>::new());
        assert_eq!(graph.dependents("a"), Vec::<String>::new());
    }

    #[test]
    fn test_no_cycles() {
        let graph = create_test_graph();
        assert!(!graph.has_cycles());
        assert!(graph.find_cycles().is_empty());
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_task("c".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");
        graph.add_dependency("c", "a");

        assert!(graph.has_cycles());
        let cycles = graph.find_cycles();
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_topological_order() {
        let graph = create_test_graph();
        let order = graph.topological_order().unwrap();

        assert!(
            order.iter().position(|x| x == "a").unwrap()
                < order.iter().position(|x| x == "b").unwrap()
        );
        assert!(
            order.iter().position(|x| x == "b").unwrap()
                < order.iter().position(|x| x == "c").unwrap()
        );
        assert!(
            order.iter().position(|x| x == "a").unwrap()
                < order.iter().position(|x| x == "d").unwrap()
        );
    }

    #[test]
    fn test_topological_order_with_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "a");

        assert!(graph.topological_order().is_none());
    }

    #[test]
    fn test_parallel_groups() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_task("c".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("a", "c");

        let groups = graph.parallel_groups();

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], vec!["a".to_string()]);
        assert!(groups[1].contains(&"b".to_string()));
        assert!(groups[1].contains(&"c".to_string()));
    }

    #[test]
    fn test_parallel_groups_linear() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_task("c".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");

        let groups = graph.parallel_groups();

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0], vec!["a".to_string()]);
        assert_eq!(groups[1], vec!["b".to_string()]);
        assert_eq!(groups[2], vec!["c".to_string()]);
    }

    #[test]
    fn test_critical_path() {
        let graph = create_test_graph();
        let path = graph.critical_path();

        assert!(path.contains(&"a".to_string()));
        assert!(path.contains(&"b".to_string()));
        assert!(path.contains(&"c".to_string()));
        assert_eq!(path.len(), 3);
    }

    #[test]
    fn test_critical_path_single_node() {
        let mut graph = DependencyGraph::new();
        graph.add_task("alone".to_string());

        let path = graph.critical_path();
        assert_eq!(path, vec!["alone".to_string()]);
    }

    #[test]
    fn test_bottlenecks() {
        let mut graph = DependencyGraph::new();
        graph.add_task("a".to_string());
        graph.add_task("b".to_string());
        graph.add_task("c".to_string());
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");

        let bottlenecks = graph.bottlenecks();

        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks.iter().any(|(id, _)| id == "b"));
    }

    #[test]
    fn test_to_dot() {
        let graph = create_test_graph();
        let dot = graph.to_dot();

        assert!(dot.contains("digraph taskgraph"));
        assert!(dot.contains("\"a\""));
        assert!(dot.contains("\"b\""));
        assert!(dot.contains("\"a\" -> \"b\""));
    }

    #[test]
    fn test_dependencies_unknown_task() {
        let graph = DependencyGraph::new();
        assert!(graph.dependencies("unknown").is_empty());
    }

    #[test]
    fn test_dependents_unknown_task() {
        let graph = DependencyGraph::new();
        assert!(graph.dependents("unknown").is_empty());
    }
}
