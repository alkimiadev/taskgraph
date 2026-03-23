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

### Future: Reactive Server Mode

Phase 4+ could add a lightweight server with pubsub for fully reactive workflows:

```
taskgraph server --watch

# Agents subscribe to events:
# - task_completed → trigger downstream tasks
# - task_blocked → notify coordinator
# - community_completed → trigger review
```

This enables "automagically reactive" orchestration where the graph drives agent behavior.

## Implementation Phases

### Phase 1: Core CLI & Validation

- Project setup with dual MIT/Apache-2.0 license
- Task file parsing (frontmatter + body)
- Commands: `init`, `validate`, `list`, `show`
- Output formatting (plain, JSON)
- **No full CRUD** - files are source of truth, edited directly

### Phase 2: Graph Operations

- Graph building from task files
- Cache system (`.taskgraph/cache.json`)
- Graph commands: `deps`, `dependents`, `topo`, `cycles`, `parallel`, `critical`, `bottleneck`
- DOT format output for visualization

### Phase 3: Semantic Search (Feature-gated)

- Rolling window embeddings
- Safetensor storage for embeddings + index
- `search` command with similarity scoring
- Integration with model2vec-rs

### Phase 4: Polish & Extensions

- File watching (`--watch`)
- TUI mode (optional)
- MCP server for LLM integration
- Performance optimization

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
    ├── embeddings.safetensors    # Phase 3
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

**Scope → Token Estimates** (environment-dependent):
| Scope | Approximate Tokens |
|-------|-------------------|
| `single` | ~500 |
| `narrow` | ~1,500 |
| `moderate` | ~3,000 |
| `broad` | ~6,000 |
| `system` | ~10,000+ |

**Risk → Success Probability** (for cost-benefit analysis):
| Risk | Implied p |
|------|-----------|
| `trivial` | 0.98 |
| `low` | 0.90 |
| `medium` | 0.80 |
| `high` | 0.65 |
| `critical` | 0.50 |

**Impact → Criticality Weight**:
| Impact | Weight |
|--------|--------|
| `isolated` | 1.0 |
| `component` | 1.5 |
| `phase` | 2.0 |
| `project` | 3.0 |

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

### Workflow Analysis

```
taskgraph risk                   # Show risk distribution across tasks
taskgraph risk-path              # Highest risk path through graph
taskgraph decompose-check        # Flag tasks that should be split (risk > medium)
taskgraph workflow-cost          # Expected cost using cost-benefit framework
```

### Semantic Search (Phase 3)

```
taskgraph search <query>         # Semantic search across task descriptions
taskgraph embed --rebuild        # Rebuild embedding index
taskgraph embed --status         # Show embedding index info
```

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
  "file_path_hashes": {
    "tasks/auth-setup.md": 18446744073709551615
  },
  "graph": {
    "nodes": [...],
    "edges": [...]
  }
}
```

**Note**: Uses mtime only for invalidation (fast, good enough). `file_path_hashes` is for semantic search index lookups.

### Cache Validation

1. On graph operation, check if `.taskgraph/cache.json` exists
2. Compare file mtimes against cached values
3. If any file changed (or new files exist), rebuild graph
4. If all match, use cached graph

### Cache Invalidation

- Manual: `taskgraph cache clear`
- Automatic: File modification detected

## Semantic Search (Phase 3)

### Feature Flag

```toml
[features]
default = []
semantic = ["model2vec-rs", "safetensors", "ndarray", "twox-hash"]
```

### Rolling Window Embeddings

For tasks with long descriptions, create overlapping token windows:

```
Task body: "Implement auth system with OAuth2, SAML, and JWT support..."

Window 0: tokens[0:512]   → embedding_0
Window 1: tokens[256:768] → embedding_1  (50% overlap)
Window 2: tokens[512:1024] → embedding_2
```

**Benefits:**
- No truncation loss for long descriptions
- Query matches specific sections of a task
- More granular similarity scoring

### Safetensor Storage Format

Store embeddings and index as pure tensors for memory-mapped access:

```
.taskgraph/embeddings.safetensors
```

**File structure:**
```
[8 bytes: header size (u64 LE)]
[header_size bytes: JSON header]
[tensor data: embeddings + index]

Header JSON:
{
  "__metadata__": {
    "model_id": "minishlab/potion-base-8M",
    "embedding_dim": "256",
    "window_size": "512",
    "overlap": "0.5",
    "created_at": "2026-03-23T12:00:00Z",
    "file_count": "42"
  },
  "embeddings": {
    "dtype": "F32",
    "shape": [N, D],
    "data_offsets": [0, N*D*4]
  },
  "index": {
    "dtype": "U8",
    "shape": [N, 24],
    "data_offsets": [N*D*4, N*D*4 + N*24]
  }
}
```

**Note:** Embedding dimension D depends on model (256 for potion-base-8M, 384 for potion-base-32M, 768 for multilingual). Never hardcode - read from metadata or tensor shape.

### Index Struct Layout

24 bytes per window, C-style packed struct:

```rust
#[repr(C, packed)]
struct WindowIndex {
    file_path_hash: u64,       // 8 bytes - xxHash3 of file path
    window_start_token: u32,   // 4 bytes - token start position
    window_end_token: u32,     // 4 bytes - token end position
    window_start_char: u32,    // 4 bytes - character offset in file
    window_end_char: u32,      // 4 bytes - character offset in file
}
```

**Why both token and char positions:**

| Use Case | Position Type | Benefit |
|----------|---------------|---------|
| Token consistency | `*_token` | Verify window is exactly 512 tokens |
| User display | `*_char` | Direct text slice: `text[start..end]` |
| Cross-model compat | `*_char` | Works even if tokenization differs |

**Storage cost:** 24 bytes per window vs 16 bytes. For 1000 windows = 24KB overhead - negligible.

**File path hash vs task ID hash:**
- Using file path hash is more general
- Works for tasks, memories, notes, any markdown collection
- Same storage format can serve multiple use cases
- Hash → path mapping stored in `cache.json` for reverse lookup

### Search Algorithm

1. Compute query embedding using model2vec
2. Calculate cosine similarity against all window embeddings
3. Return top-k windows, grouped/deduped by task_id
4. Optionally show context: "matched in tokens 256-768 of auth-setup.md"

### Model Options

Default: `minishlab/potion-base-8M` (7.5M params, 256 dims, fast)

Alternatives via config:
- `potion-base-4M` - smaller, faster
- `potion-base-32M` - higher quality
- `potion-multilingual-128M` - non-English tasks

## Dependencies (Rust Crates)

### Core Dependencies (Phase 1-2)

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

### Semantic Search Dependencies (Phase 3)

| Crate | Purpose |
|-------|---------|
| `model2vec-rs` | Static embedding model inference |
| `safetensors` | Safetensor file format |
| `ndarray` | Matrix operations for similarity |
| `twox-hash` | Fast xxHash3 for task ID hashing |

### Optional/Future

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (if needed for file watching) |
| `notify` | File system watching (for `--watch` mode) |
| `ratatui` | TUI interface (for interactive mode) |

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

All graph commands support:

| Flag | Format | Use Case |
|------|--------|----------|
| `--json` | JSON | Scripting, tooling |
| `--dot` | Graphviz DOT | Visualization |
| `--plain` | Plain text | Human reading (default) |

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
│   ├── search.rs           # Semantic search (feature-gated)
│   ├── embedding.rs        # Embedding storage/encoding (feature-gated)
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs
│   │   ├── show.rs
│   │   ├── create.rs
│   │   ├── edit.rs
│   │   ├── delete.rs
│   │   ├── deps.rs
│   │   ├── topo.rs
│   │   ├── search.rs       # Feature-gated
│   │   └── ...
│   └── output.rs           # Output formatting
└── tests/
    ├── integration/
    └── fixtures/
```

## Future Considerations

- **File watching**: `--watch` flag for live updates
- **TUI mode**: Interactive terminal interface
- **Web UI**: Optional web interface for visualization
- **MCP server**: Model Context Protocol for LLM integration
- **Plugins**: Extension system for custom commands