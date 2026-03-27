# TaskGraph

A CLI tool for managing task dependencies using markdown files.

Each task is a single `.md` file with YAML frontmatter, making tasks version-controllable, LLM-editable, and naturally concurrent.

## Installation

```bash
cargo install taskgraph
```

## Quick Start

```bash
# Initialize a new task
taskgraph init my-task --name "My Task"

# List all tasks
taskgraph list

# Validate task files
taskgraph validate

# Show topological order
taskgraph topo

# Check for circular dependencies
taskgraph cycles

# Show parallel work groups
taskgraph parallel
```

## Task File Format

```markdown
---
id: my-task
name: My Task
status: pending
depends_on: [other-task]
scope: narrow
risk: low
---

Detailed description of the task goes here.
```

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier |
| `name` | string | Human-readable name |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `status` | enum | pending, in-progress, completed, failed, blocked |
| `depends_on` | string[] | Tasks that must complete first |
| `scope` | enum | single, narrow, moderate, broad, system |
| `risk` | enum | trivial, low, medium, high, critical |
| `impact` | enum | isolated, component, phase, project |
| `level` | enum | planning, decomposition, implementation, review, research |
| `tags` | string[] | Arbitrary tags for filtering |

## Commands

### Task Management

| Command | Description |
|---------|-------------|
| `init <id>` | Scaffold a new task file |
| `validate` | Validate all task files |
| `list` | List all tasks |
| `show <id>` | Show task details |

### Graph Operations

| Command | Description |
|---------|-------------|
| `deps <id>` | Show what a task depends on |
| `dependents <id>` | Show what depends on a task |
| `topo` | Show topological order |
| `cycles` | Check for circular dependencies |
| `parallel` | Show parallel work groups |
| `critical` | Show critical path |
| `bottleneck` | Show bottleneck tasks |
| `graph` | Output graph in DOT format |

### Workflow Analysis

| Command | Description |
|---------|-------------|
| `risk` | Show risk distribution across tasks |
| `decompose` | Flag tasks that should be split |
| `workflow-cost` | Calculate relative workflow cost |
| `risk-path` | Find path with highest cumulative risk |

## Project Structure

```
project/
└── tasks/
    ├── task-one.md
    ├── task-two.md
    └── ...
```

## Learn More

- [Workflow Guide](docs/workflow.md) — Practical usage patterns and phases
- [Framework](docs/framework.md) — Why categorical estimates over numeric costs
- [Architecture](docs/ARCHITECTURE.md) — Technical design decisions

## Performance

TaskGraph is designed for speed. No caching needed—graph rebuilds are instant:

| Tasks | Load Time | Topo Sort | Cycles | Critical Path |
|-------|-----------|-----------|---------|---------------|
| 50 | 3ms | 3ms | 2ms | 8ms |
| 500 | 19ms | 21ms | 14ms | 52ms |
| 1,000 | 34ms | 42ms | 26ms | 82ms |

*Benchmarked on AMD EPYC 9004 series. Run `./scripts/benchmark.sh` to verify on your system.*

## Configuration

Create `.taskgraph.toml` in your project root:

```toml
[project]
tasks_dir = "tasks"
```

TaskGraph searches up the directory tree for config files (like git).

## Output Formats

All commands support `--format json` for programmatic use:

```bash
taskgraph list --format json
taskgraph show my-task --format json
taskgraph validate --format json
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.