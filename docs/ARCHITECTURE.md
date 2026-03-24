# TaskGraph Architecture

## Overview

TaskGraph is a CLI tool for managing task dependencies using markdown files as the source of truth. Unlike traditional task managers that use databases or JSON files, TaskGraph stores tasks as individual markdown files, making them:

- Version-controllable (git-friendly)
- Easy for humans and LLMs to read/edit
- Naturally concurrent (one file per task)

## Primary Use Case: Agent-Driven Development Workflows

TaskGraph is designed for AI agent orchestration in software development. Instead of bloating agent context with MCP tools, agents use a simple CLI via their existing bash capabilities.

### Design Philosophy

**Problem with MCP-based approaches:**
- MCP servers add cognitive overhead (tools must be discovered, configured)
- mcp-cli starts servers on each request (~6s latency for some servers)
- Tools are "thrown at agents" regardless of relevance
- Amortized cognitive cost is excessive

**CLI-based approach:**
- Agents already have bash tools by default
- Good `--help` makes discovery natural
- Instructions in role specs where relevant
- Zero overhead when not needed

### SDD Framework Integration

TaskGraph supports the [Spec-Driven Development (SDD) framework](/workspace/@alkminer/reference/spec-driven-dev/README.md) where agents:

1. **Task Decomposition Specialist**: Creates tasks from architecture
    ```bash
    # Create task files directly (or use init as scaffold)
    taskgraph init auth-setup --name "Setup Authentication" --scope narrow --risk low
    # Then edit file to add depends_on, body content
    ```

2. **Implementation Specialist**: Selects and executes tasks
    ```bash
    taskgraph topo --status pending           # Get next executable tasks
    # Edit file directly: status: in-progress
    # ... implement ...
    # Edit file directly: status: completed
    taskgraph validate                         # Verify no issues
    ```

3. **Safe Exit Mechanism**: Agents can unblock themselves
    ```bash
    # Edit file directly: status: blocked
    taskgraph init resolve-env-issue --name "Resolve: Missing env vars"
    # Edit new file to add: depends_on: [resolve-env-issue]
    ```

4. **Review Injection**: Based on graph analysis
    ```bash
    taskgraph risk           # Review high-risk tasks
    taskgraph bottleneck     # Review before critical tasks
    taskgraph parallel       # Review before parallel groups merge
    ```

## Implementation Phases

### Phase 1: Core CLI & Validation ✅

- Project setup with dual MIT/Apache-2.0 license
- Task file parsing (frontmatter + body)
- Commands: `init`, `validate`, `list`, `show`
- Output formatting (plain, JSON)
- **No full CRUD** - files are source of truth, edited directly

### Phase 2: Graph Operations ✅ (mostly)

- Graph building from task files
- Cache system (`.taskgraph/cache.json`) - stub only, not critical
- Graph commands: `deps`, `dependents`, `topo`, `cycles`, `parallel`, `critical`, `bottleneck`
- Workflow analysis commands: deferred
- DOT format output for visualization

### Phase 3: Cleanup & Review (in progress)

- Test coverage ~80% (target met)
- Code review and documentation
- Resolve documented issues

### Phase 4: Polish & Extensions

- Configuration system (TOML)
- Documentation and shell completion
- Performance optimization (benchmark, parallel parsing)
- File watching (`--watch`)
- Distribution (cargo publish, binaries)

### Future: Reactive Server Mode (Optional)

A lightweight server with pubsub could enable fully reactive workflows:

```
taskgraph server --watch

# Agents subscribe to events:
# - task_completed → trigger downstream tasks
# - task_blocked → notify coordinator
# - community_completed → trigger review
```

This is optional - CLI-first design covers most use cases.

## Core Concept

**Tasks as Files**: Each task is a markdown file with YAML frontmatter for structured data and a body for free-form description.

```
project/
├── tasks/
│   ├── auth-setup.md
│   ├── api-implementation.md
│   └── deployment.md
└── .taskgraph/
    ├── cache.json
    └── logs/
```

## Task File Format

```markdown
---
id: auth-setup
name: Setup Authentication
status: in-progress
depends_on: [database-schema]
priority: high
tags: [backend, security]
created: 2026-03-23
modified: 2026-03-23
---

Detailed description of the task. This can be any markdown content.

## Notes
- Use any markdown formatting
- LLMs can edit this directly
- No special tools needed for basic CRUD
```

### Required Frontmatter Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier (required) |
| `name` | string | Human-readable name (required) |
| `status` | enum | pending, in-progress, completed, failed, blocked |
| `depends_on` | string[] | Tasks that must complete first |

### Optional Frontmatter Fields

| Field | Type | Description |
|-------|------|-------------|
| `priority` | string | low, medium, high, critical |
| `tags` | string[] | Arbitrary tags for filtering |
| `created` | date | Creation timestamp |
| `modified` | date | Last modification timestamp |
| `assignee` | string | Person or agent responsible |
| `due` | date | Due date |

### Task Estimation Fields (Categorical)

LLMs are well-calibrated to categorical estimates, not numeric ones. Use these for workflow analysis:

| Field | Type | Values | Description |
|-------|------|--------|-------------|
| `scope` | enum | single, narrow, moderate, broad, system | How many files/components affected |
| `risk` | enum | trivial, low, medium, high, critical | Likelihood of failure or iteration |
| `impact` | enum | isolated, component, phase, project | Consequence if task fails |
| `level` | enum | planning, decomposition, implementation, review, research | Task type |

#### Why Categorical?

1. **LLM calibration**: Models reliably distinguish "high risk" vs "medium risk" but struggle with numeric estimates
2. **Environment independence**: Token counts, costs, and model verbosity vary widely
3. **Relative ordering matters more than precision**: We need to know A > B, not A = $3.42

#### Structural Insight: Upstream Failures Multiply

The key insight from cost-benefit analysis: **failures at higher levels multiply downstream surface area.**

```
planning failure → wrong decomposition → wasted implementation
decomposition failure → unclear tasks → rework  
review failure → bugs shipped → rework
```

This means:
- `risk: critical` at planning level > `risk: critical` at implementation level
- The `level` field captures position in the hierarchy
- Upstream tasks should be decomposed more carefully

The workflow analysis commands surface this structure rather than calculating precise dollar amounts.

#### Mappings (for relative comparison)

**Scope → Size Ordering:**
| Scope | Ordering | Approximate Tokens |
|-------|----------|-------------------|
| `single` | 1 | ~500 |
| `narrow` | 2 | ~1,500 |
| `moderate` | 3 | ~3,000 |
| `broad` | 4 | ~6,000 |
| `system` | 5 | ~10,000+ |

**Risk → Failure Likelihood:**
| Risk | Ordering | Implied p |
|------|----------|-----------|
| `trivial` | 1 | 0.98 |
| `low` | 2 | 0.90 |
| `medium` | 3 | 0.80 |
| `high` | 4 | 0.65 |
| `critical` | 5 | 0.50 |

**Impact → Downstream Damage:**
| Impact | Ordering | Weight |
|--------|----------|--------|
| `isolated` | 1 | 1.0 |
| `component` | 2 | 1.5 |
| `phase` | 3 | 2.0 |
| `project` | 4 | 3.0 |

**Note**: The numeric values are reasonable defaults for relative comparison. They should not be interpreted as precise measurements.

## CLI Commands

### Task Discovery & Validation

```
taskgraph init <id> [--name <name>] [--scope <scope>] [--risk <risk>]  # Scaffold new task
taskgraph validate                     # Check all tasks valid, no duplicates/missing deps
taskgraph list [--status <status>] [--tag <tag>]  # List tasks
taskgraph show <id>                    # Display task details
```

**Note**: No full CRUD. Files are the source of truth. Edit files directly.

### Graph Operations (Cache Helps)

These require building the full graph:

```
taskgraph deps <id>              # What must complete before this task?
taskgraph dependents <id>        # What's waiting on this task?
taskgraph topo                   # Topological sort (execution order)
taskgraph cycles                 # Detect circular dependencies
taskgraph parallel               # Groups of tasks that can run together
taskgraph critical               # Longest path (completion blockers)
taskgraph bottleneck             # High betweenness (on many paths)
taskgraph graph                  # Visualize dependency graph (DOT format)
```

### Cache Management

```
taskgraph cache clear            # Clear the cache
taskgraph cache status           # Show cache info (size, age, file count)
```

### Workflow Analysis (Structural Risk)

These commands surface structural risk patterns rather than precise cost calculations.

```
taskgraph risk                   # Show risk distribution across tasks
taskgraph risk-path              # Highest risk path through graph
taskgraph decompose-check        # Flag tasks that should be split (risk > medium)
taskgraph workflow-cost          # Relative workflow cost comparison
```

| Command | Structural Question |
|---------|---------------------|
| `risk` | Where is risk concentrated in the graph? |
| `decompose-check` | Which upstream tasks should be split? |
| `risk-path` | Which failure chain has most downstream damage? |
| `workflow-cost` | Relative comparison - path A vs path B |

**Key insight**: The goal is to identify structural problems (upstream tasks that should be decomposed, high-risk paths) rather than calculate absolute dollar amounts.

## Dependency Model

**Direction**: `depends_on` means "before me, this must be done"

```
task-a.md: depends_on: []
task-b.md: depends_on: [task-a]
task-c.md: depends_on: [task-a, task-b]
```

Graph edges: task-a → task-b, task-a → task-c, task-b → task-c

This direction gives correct topological order: A, B, C

### Query Implications

| Query | Implementation |
|-------|----------------|
| "What does B depend on?" | Read B's frontmatter only |
| "What depends on A?" | Scan all tasks for `depends_on: [A]` - needs full graph |

## Cache Strategy

### Cache Location

```
<project>/.taskgraph/cache.json
```

### Cache Contents

```json
{
  "version": 1,
  "built_at": "2026-03-23T10:00:00Z",
  "files": {
    "tasks/auth-setup.md": {
      "mtime": 1711185600
    }
  },
  "graph": {
    "nodes": [...],
    "edges": [...]
  }
}
```

**Note**: Uses mtime only for invalidation (fast, good enough).

### Cache Validation

1. On graph operation, check if `.taskgraph/cache.json` exists
2. Compare file mtimes against cached values
3. If any file changed (or new files exist), rebuild graph
4. If all match, use cached graph

### Cache Invalidation

- Manual: `taskgraph cache clear`
- Automatic: File modification detected

## Dependencies (Rust Crates)

### Core Dependencies

| Crate | Purpose | Notes |
|-------|---------|-------|
| `petgraph` | Graph data structure & algorithms | Stable, well-maintained |
| `gray_matter` | Frontmatter extraction | Handles YAML/TOML/JSON frontmatter |
| `serde` | Serialization | For frontmatter, cache, JSON output |
| `serde_yaml` | YAML parsing | Typed frontmatter deserialization |
| `clap` | CLI argument parsing | Derive-based for clean code |
| `chrono` | Date/time handling | For timestamps |
| `anyhow` | Error handling | Ergonomic error types |
| `dirs` | Platform directories | For future global config |
| `tracing` | Logging/tracing | Structured logging |

### Optional/Future

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (if needed for file watching) |
| `notify` | File system watching (for `--watch` mode) |

## Graph Operations Detail

### Topological Sort

```rust
petgraph::algo::toposort(&graph, None)
```

Returns `Result<Vec<NodeIndex>, CycleError>`. If cycle exists, error contains the cycle.

### Cycle Detection

```rust
petgraph::algo::is_cyclic_directed(&graph)
```

Returns `bool`. Use `toposort` for cycle details.

### Parallel Work Groups

Group tasks by "generation" - tasks with no pending dependencies can run in parallel.

Algorithm:
1. Topological sort
2. Group by distance from source (tasks with no dependencies)
3. Each generation can run in parallel

### Critical Path / Bottlenecks

Use betweenness centrality to find tasks that are on many dependency chains:

```rust
petgraph::algo::dijkstra // For path finding
// Custom centrality calculation
```

## Output Formats

Current output is plain text only.

| Flag | Format | Status |
|------|--------|--------|
| `--plain` | Plain text | Default, implemented |
| `--json` | JSON | Not implemented |
| `--dot` | Graphviz DOT | Graph command outputs DOT by default |

JSON output could be added later if needed for scripting/tooling integration.

## Error Handling

| Scenario | Behavior |
|----------|----------|
| Task not found | Clear error with suggestion |
| Circular dependency | Show the cycle path |
| Invalid frontmatter | Parse error with file:line |
| Missing `id` field | Skip file, warn in logs |
| Duplicate `id` | Error on cache build |

## Project Structure

```
taskgraph/
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── AGENTS.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs              # CLI definition (clap)
│   ├── task.rs             # Task struct & parsing
│   ├── graph.rs            # Graph building & operations
│   ├── cache.rs            # Cache management
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── show.rs
│   │   ├── deps.rs
│   │   ├── topo.rs
│   │   └── ...
│   └── output.rs           # Output formatting
└── tests/
    ├── integration/
    └── fixtures/
```

## Future Considerations

- **File watching**: `--watch` flag for live updates (Phase 4)
- **Reactive server**: Optional pubsub mode for event-driven workflows
- **Plugins**: Extension system for custom commands