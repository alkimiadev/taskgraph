# TaskGraph

A CLI tool for managing task dependencies using markdown files.

Each task is a single `.md` file with YAML frontmatter, making tasks version-controllable, LLM-editable, and naturally concurrent.

## Installation

```bash
# Basic installation
cargo install taskgraph

# With semantic search support
cargo install taskgraph --features semantic
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

### Semantic Search (requires `--features semantic`)

| Command | Description |
|---------|-------------|
| `search <query>` | Search tasks by semantic similarity |
| `embed` | Build the embedding index |
| `embed --status` | Show embedding index info |

## Semantic Search

The semantic search feature enables finding tasks by meaning, not just keywords.

```bash
# Build the embedding index
taskgraph embed

# Search for tasks
taskgraph search "authentication and login flow"
taskgraph search "database schema changes" --top-k 5
```

### Requirements

- Enable with `cargo install taskgraph --features semantic`
- First run of `embed` will download the model (~8MB)
- Uses `minishlab/potion-base-8M` by default

### Custom Models

```bash
# Use a different model
taskgraph embed --model minishlab/potion-base-32M
taskgraph search "query" --model minishlab/potion-base-32M
```

## Project Structure

```
project/
├── tasks/
│   ├── task-one.md
│   ├── task-two.md
│   └── ...
└── .taskgraph/
    ├── embeddings.safetensors  # Semantic search index
    └── cache.json              # Graph cache
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.