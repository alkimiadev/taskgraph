# Phase 4: Polish & Extensions

## Objective

Production-ready CLI with distribution.

## Status: 🔄 IN PROGRESS

## Priority Order

Based on the SDD framework use case:

1. **Documentation** - Required for release
2. **Testing & Quality** - Confidence in correctness
3. **Distribution** - Final step

## Tasks

### 4.1 Documentation ✅

- [x] README with installation, quick start, examples
- [x] Shell completions (bash, zsh, fish) via clap_complete
- [x] Fix doc issues (field name typo, missing fields, phantom dirs)
- [x] Add "Learn More" links to docs/ in README

### 4.2 Testing & Quality

**Coverage: 89% (from Phase 3)**

- [ ] Unit tests for `task.rs` parsing (`from_markdown`, `from_file`, `to_markdown`, roundtrip)
- [ ] Unit tests for `calculate_task_ev()` math verification
- [ ] Unit tests for CLI arg parsing (`cli.rs`)
- [ ] Property-based tests for parsers (optional — consider value vs dependency cost)
- [ ] Realistic multi-phase workflow fixture for integration tests
- [ ] Shared test helpers module (`tests/common/mod.rs`)

### 4.3 Distribution

- [ ] Cargo publish (dry-run verified)
- [ ] GitHub releases with binaries (Linux, macOS, Windows)
- [ ] Homebrew formula (optional)
- [ ] Verify published crate README renders correctly on crates.io

### 4.4 Extensions (Backlog)

- [ ] Configuration system (TOML)
- [ ] `--json` output format for scripting
- [ ] File watching (`--watch`)
- [ ] Performance optimization (parallel parsing)

## Success Criteria

- Alpha 0.1.0 published to crates.io
- Documentation covers common workflows
- Easy installation via `cargo install taskgraph`
- Shell completions for bash, zsh, fish

## Design Notes

### Why CLI over MCP?

TaskGraph is designed as a CLI tool that agents use via their native bash/shell tools:

1. **Zero overhead when not needed** - MCP servers add latency and cognitive overhead
2. **Natural discovery** - Good `--help` makes CLI self-documenting
3. **Works everywhere** - No MCP client setup required
4. **Composable** - Pipes, redirects, shell scripting

MCP is useful when you need persistent state or event streams. TaskGraph operations are typically stateless queries on a file-based graph, making CLI the better fit.

### Why No Cache?

Graph rebuild is instant (<50ms for 1000 tasks). Cache adds complexity without benefit:
- Cache invalidation is error-prone
- Fresh state avoids stale data bugs
- LLM response times dwarf parse times (seconds vs milliseconds)