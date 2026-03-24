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
| Task status | `TaskStatus` | `src/task.rs` |
| Task estimation | `TaskScope`, `TaskRisk`, `TaskImpact`, `TaskLevel` | `src/task.rs` |
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
4. **CLI only, no server** - Simplicity first
5. **CLI over MCP** - Agents use bash tools, no protocol overhead

## Implementation Plan

### Phase 1: Core CLI & Validation ✅
- [x] Project setup with dual MIT/Apache-2.0 license
- [x] Task file parsing (gray_matter + serde_yaml)
- [x] Commands: `init`, `validate`, `list`, `show`
- [x] Output formatting (plain, JSON)

### Phase 2: Graph Operations ✅
- [x] Graph building from task files
- [ ] Cache system (`.taskgraph/cache.json`) - stub only, not critical
- [x] Graph commands: `deps`, `dependents`, `topo`, `cycles`, `parallel`, `critical`, `bottleneck`
- [x] Workflow analysis: `risk`, `risk-path`, `decompose`, `workflow-cost`
- [x] DOT format output for visualization

### Phase 3: Cleanup & Review (in progress)
- [x] Test coverage ~80% (target met)
- [ ] Code review and documentation
- [ ] Resolve documented issues

### Phase 4: Polish & Extensions
- [ ] Configuration system
- [ ] Documentation and shell completion
- [ ] Performance optimization
- [ ] File watching (`--watch`)
- [ ] Distribution (cargo publish, binaries)

## Available Tools

### Core
| Tool | Description |
|------|-------------|
| `read`, `write`, `edit` | File operations |
| `bash` | Shell commands |
| `glob`, `grep` | File search |
| `task` | Spawn sub-agents for research |

### Web Research
| Tool | Description |
|------|-------------|
| `webSearch_search` | Web search (DuckDuckGo, Bing, Exa) |
| `webFetch` | Fetch URL content |

### Git Worktrees (open-trees plugin)
Enable: `worktree_mode { "action": "on" }`

| Tool | Description |
|------|-------------|
| `worktree_make` | Create/open worktrees |
| `worktree_overview` | List worktrees |
| `worktree_cleanup` | Remove worktrees |

See `docs/implementation.md` for full details.

## Development Patterns

### Testing Standards

**Coverage Target:** 80% line coverage (soft - meaningful coverage over raw numbers)

```bash
cargo llvm-cov --all-features          # Run with coverage
cargo llvm-cov --all-features --html   # Generate HTML report
```

**Test alongside implementation:** When adding a new command or feature, add tests in the same session. Don't defer testing.

### Test Categories Required

1. **Happy Path** - Normal usage with valid inputs
2. **Error Path** - Invalid inputs, missing files, malformed frontmatter
3. **Graph Operations** - Cycles, missing dependencies, empty graphs
4. **Roundtrip** - Parse task file → write → parse again → values match

### Test Placement

| Type | Location |
|------|----------|
| Unit tests | `#[cfg(test)] mod tests` in same file |
| Integration tests | `tests/integration/` using `assert_cmd` |
| Fixtures | `tests/fixtures/` |

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

### Code Quality

**Required before commits:**
```bash
cargo clippy -- -D warnings   # Treat warnings as errors
cargo fmt --check             # Verify formatting
```

**Fix all warnings** before committing. Do not suppress warnings.

### CLI Consistency

**Path handling:** Commands must use `Cli::tasks_path()` for the tasks directory, not hardcode `./tasks`.

```rust
// Correct
let path = cli.tasks_path();
let collection = TaskCollection::from_directory(&path);

// Wrong
let collection = TaskCollection::from_directory(PathBuf::from("./tasks"));
```

**Error handling:** Use `anyhow::Result` and `?` operator. Provide actionable error messages.

### Commit Strategy

**Make frequent commits** during development:
- Natural breakpoints (after each command, after each module)
- Easier to revert if issues arise
- Smaller diffs are easier to review

## Pre-Commit Checklist

Before committing changes:

- [ ] `cargo test --all-features` passes (includes unit + doc tests)
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] New code has tests
- [ ] Public API has documentation

## Build & Test Commands

```bash
cargo build
cargo test --all-features          # Unit + integration tests
cargo test --doc --all-features    # Doc tests only
cargo clippy -- -D warnings
cargo fmt --check
```

## Code Conventions

- Rust edition 2024 (or 2021 if 2024 not stable)
- `anyhow` for error handling in CLI
- `thiserror` for library error types
- `#![warn(missing_docs)]` after initial implementation

## Dependencies

### Core

| Crate | Purpose |
|-------|---------|
| `petgraph` | Graph data structure & algorithms |
| `gray_matter` | Frontmatter extraction |
| `serde` + `serde_yaml` | Serialization |
| `clap` | CLI argument parsing |
| `chrono` | Date/time handling |
| `anyhow` | Error handling |
| `dirs` | Platform directories |
| `walkdir` | Directory scanning |
| `tracing` | Logging |

### Dependency Source Reference

All dependency sources are available locally after `cargo build`:

```
~/.cargo/registry/src/     # crates.io dependencies
~/.cargo/git/checkouts/    # git dependencies
```

When implementing, you can reference the source code for any dependency. For example:
- `~/.cargo/registry/src/*/petgraph-*/` - graph algorithms
- `~/.cargo/registry/src/*/gray_matter-*/` - frontmatter parsing

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
- `/workspace/@alkminer/reference/spec-driven-dev/cost_benefit_analysis_framework.py` - EV formula for workflow-cost

## Directory Structure

```
taskgraph/
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── AGENTS.md              # This file
├── docs/
│   ├── ARCHITECTURE.md    # Full architecture spec
│   ├── implementation.md  # Tools, models, guidelines
│   ├── phase-1.md         # Phase 1 tasks
│   ├── phase-2.md         # Phase 2 tasks
│   ├── phase-3.md         # Phase 3 tasks
│   ├── phase-4.md         # Phase 4 tasks
│   ├── issues/            # Blocking issues (safe exit)
│   ├── reviews/           # Code review documentation
│   └── research/          # Reference implementations
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

**Phase 1-2: Complete.** Phase 3 in progress (coverage at 88%).

## Implementation Guide

See `docs/implementation.md` for:
- Available tools (bash, webSearch, task, worktree)
- Sub-agent spawning for research
- Available models
- Document organization
- External repository access

## License

Dual-licensed under MIT OR Apache-2.0. Both license files must be present at repository root.