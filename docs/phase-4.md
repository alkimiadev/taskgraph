# Phase 4: Polish & Extensions

## Objective

Production-ready CLI with distribution.

## Status: ⏳ NOT STARTED

## Priority Order

Based on the SDD framework use case:

1. **Documentation** - Required for release
2. **Distribution** - Final step

## Tasks

### 4.1 Documentation
- [ ] README with installation, quick start, examples
- [ ] Shell completion (bash, zsh, fish) via clap
- [ ] Example task files
- [ ] Example workflows (SDD integration)

### 4.2 Testing & Quality
- [ ] Integration tests with example task directories
- [ ] Property-based tests for parsers
- [x] Coverage > 80% (achieved in Phase 3)

### 4.3 Distribution
- [ ] Release builds (Linux, macOS, Windows)
- [ ] Cargo publish
- [ ] GitHub releases with binaries
- [ ] Homebrew formula (optional)

## Success Criteria

- Stable 1.0 release
- Documentation covers common workflows
- Easy installation via cargo

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