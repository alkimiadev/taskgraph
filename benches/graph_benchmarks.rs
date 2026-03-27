use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use taskgraph::{DependencyGraph, TaskCollection};
use tempfile::TempDir;

fn generate_tasks(count: usize) -> TempDir {
    let dir = TempDir::new().unwrap();
    for i in 0..count {
        let deps = if i > 0 {
            format!("depends_on: [task-{}]", i - 1)
        } else {
            "depends_on: []".to_string()
        };

        let content = format!(
            "---\nid: task-{}\nname: Task {}\nstatus: pending\n{}\n---\n\nDescription of task {}\n",
            i, i, deps, i
        );
        fs::write(dir.path().join(format!("task-{}.md", i)), content).unwrap();
    }
    dir
}

fn benchmark_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_tasks");

    // Test different sizes
    for size in [50, 100, 500, 1000].iter() {
        let dir = generate_tasks(*size);

        group.bench_with_input(format!("{}_tasks", size), size, |b, _| {
            b.iter(|| {
                let collection = TaskCollection::from_directory(dir.path());
                let _graph = DependencyGraph::from_collection(black_box(&collection));
            })
        });
    }

    group.finish();
}

fn benchmark_graph_ops(c: &mut Criterion) {
    let dir = generate_tasks(1000);
    let collection = TaskCollection::from_directory(dir.path());
    let graph = DependencyGraph::from_collection(&collection);

    c.bench_function("topological_sort_1000", |b| {
        b.iter(|| {
            black_box(graph.topological_order());
        })
    });

    c.bench_function("cycle_detection_1000", |b| {
        b.iter(|| {
            black_box(graph.has_cycles());
        })
    });

    c.bench_function("critical_path_1000", |b| {
        b.iter(|| {
            black_box(graph.critical_path());
        })
    });

    c.bench_function("bottlenecks_1000", |b| {
        b.iter(|| {
            black_box(graph.bottlenecks());
        })
    });
}

criterion_group!(benches, benchmark_load, benchmark_graph_ops);
criterion_main!(benches);
