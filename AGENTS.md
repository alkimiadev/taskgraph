# AGENTS.md

Context for AI agents working on TaskGraph.

## Project Overview

TaskGraph is a CLI tool for managing task dependencies using markdown files. Each task is a single `.md` file with YAML frontmatter, making tasks version-controllable, LLM-editable, and naturally concurrent.

**Core Principle:** Markdown files are the source of truth. The graph is derived, not stored.

**Key insight:** Most operations are CRUD on individual files. Graph operations (topo, cycles, parallel groups) are only needed for planning and analysis.

## Architecture Summary

```
tasks/*.md (source of truth)
    ↓ parse
Task structs (in-memory)
    ↓ build
petgraph::DiGraph
    ↓ analyze
topo order, cycles, communities, bottlenecks
```

### Core Types

| Concept | Rust Type | File |
|---------|-----------|------|
| Task definition | `Task` | `src/task.rs` |
| Parsed frontmatter | `TaskFrontmatter` | `src/task.rs` |
| Dependency graph | `DiGraph<TaskId, ()>` | `src/graph.rs` |
| Cache + graph state | `Cache` | `src/cache.rs` |
| CLI command | `Commands` enum | `src/cli.rs` |

### Data Flow

```
CLI args → Command → 
  ├─ CRUD: Read/Edit single file (no graph)
  └─ Graph: Build graph from files → Analyze → Output
```

### Key Decisions

1. **One task = one file** - Natural concurrency, git-friendly
2. **Frontmatter = YAML** - Human/LLM editable, typed via serde
3. **Cache is optional** - Graph can be rebuilt on every query
4. **CLI only, no server (Phase 1-3)** - Simplicity first
5. **Semantic search = feature flag** - Optional heavy dependency

## Implementation Plan

### Phase 1: Core CLI & CRUD
- [ ] Project setup with dual MIT/Apache-2.0 license
- [ ] Task file parsing (gray_matter + serde_yaml)
- [ ] Basic commands: `list`, `show`, `create`, `edit`, `delete`
- [ ] Output formatting (plain, JSON)

### Phase 2: Graph Operations
- [ ] Graph building from task files
- [ ] Cache system (`.taskgraph/cache.json`)
- [ ] Graph commands: `deps`, `dependents`, `topo`, `cycles`, `parallel`, `critical`
- [ ] DOT format output for visualization

### Phase 3: Semantic Search (Feature-gated)
- [ ] Rolling window embeddings
- [ ] Safetensor storage (embeddings + struct tensor index)
- [ ] `search` command with similarity scoring

### Phase 4: Polish & Extensions
- [ ] File watching (`--watch`)
- [ ] TUI mode (optional)
- [ ] MCP server for LLM integration

## Development Patterns

### Testing Standards

**Coverage Target:** 80% line coverage (soft - meaningful coverage over raw numbers)

```bash
cargo llvm-cov --all-features          # Run with coverage
cargo llvm-cov --all-features --html   # Generate HTML report
```

### Test Categories Required

1. **Happy Path** - Normal usage with valid inputs
2. **Error Path** - Invalid inputs, missing files, malformed frontmatter
3. **Graph Operations** - Cycles, missing dependencies, empty graphs
4. **Roundtrip** - Create task → read task → values match

### Documentation Standards

Every public item must have:
1. **Summary Line** - One sentence starting with a verb
2. **Examples** - For non-trivial functions (doc tests)
3. **Errors Section** - `# Errors` listing possible failures

```rust
/// Parses a task from a markdown file.
///
/// # Examples
/// ```
/// use taskgraph::Task;
/// let task = Task::from_file("tasks/example.md")?;
/// ```
///
/// # Errors
/// Returns `Error::ParseError` if frontmatter is invalid.
pub fn from_file(path: &Path) -> Result<Task> { ... }
```

```bash
cargo doc --all-features --no-deps     # Build docs
cargo test --doc --all-features        # Run doc tests
```

### Commit Strategy

**Make frequent commits** during development:
- Natural breakpoints (after each command, after each module)
- Easier to revert if issues arise
- Smaller diffs are easier to review

## Build & Test Commands

```bash
cargo build
cargo test
cargo test --all-features
cargo clippy --all-features
cargo fmt --check
```

## Code Conventions

- Rust edition 2024 (or 2021 if 2024 not stable)
- `anyhow` for error handling in CLI
- `thiserror` for library error types
- Feature flags for optional functionality (`semantic`)
- `#![warn(missing_docs)]` after initial implementation

## Dependencies

### Core (Phase 1-2)

| Crate | Purpose |
|-------|---------|
| `petgraph` | Graph data structure & algorithms |
| `gray_matter` | Frontmatter extraction |
| `serde` + `serde_yaml` | Serialization |
| `clap` | CLI argument parsing |
| `chrono` | Date/time handling |
| `anyhow` | Error handling |
| `dirs` | Platform directories |
| `tracing` | Logging |

### Semantic Search (Phase 3, feature-gated)

| Crate | Purpose |
|-------|---------|
| `model2vec-rs` | Static embedding model inference |
| `safetensors` | Safetensor file format |
| `ndarray` | Matrix operations |
| `twox-hash` | Fast xxHash3 for task ID hashing |

## Relevant Files

### Architecture & Plans
- `docs/ARCHITECTURE.md` - Full architecture spec (this is the source of truth)

### Previous Iterations (Reference)
- `/workspace/tools/ade_mcp/` - Original MCP-based POC (TypeScript/Deno)
- `/workspace/tools/ade_mcp/src/core/TaskGraphManager.ts` - Graph operations reference
- `/workspace/tools/ade_mcp/src/persistence/` - Persistence patterns to avoid

### End Use Case Reference
- `/workspace/@alkminer/reference/spec-driven-dev/README.md` - SDD framework that uses TaskGraph
- `/workspace/@alkminer/reference/spec-driven-dev/prompts/task-decomposer.md` - Agent role spec

### Related Projects
- `/workspace/embedding_service/` - model2vec-rs embedding service
- `/workspace/model2vec-rs/` - Forked model2vec with token counting

## Directory Structure

```
taskgraph/
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── AGENTS.md              # This file
├── docs/
│   └── ARCHITECTURE.md    # Full architecture spec
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── task.rs
│   ├── graph.rs
│   ├── cache.rs
│   └── commands/
│       └── ...
└── tests/
    ├── integration/
    └── fixtures/
```

## Current Status

**Phase 0: Setup**

Architecture document complete. Ready to initialize project.

## License

Dual-licensed under MIT OR Apache-2.0. Both license files must be present at repository root.