# Phase 4: Polish & Extensions

## Objective

Production-ready CLI with advanced features.

## Status: ⏳ NOT STARTED

## Priority Order

Based on the SDD framework use case:

1. **Configuration** - Needed for any non-trivial use
2. **Documentation** - Required for release
3. **Performance** - Needed for large task sets
4. **File Watching** - Enables reactive workflows
5. **Distribution** - Final step

## Tasks

### 4.1 Configuration
- [ ] Global config: `~/.config/taskgraph/config.toml`
- [ ] Project config: `.taskgraph/config.toml`
- [ ] Settings: default path, output format
- [ ] Config precedence: project > global > defaults

### 4.2 Documentation
- [ ] README with installation, quick start, examples
- [ ] Shell completion (bash, zsh, fish) via clap
- [ ] Example task files
- [ ] Example workflows (SDD integration)

### 4.3 Performance
- [ ] Profile hot paths
- [ ] Parallel file parsing with rayon
- [ ] Handle 1000+ tasks gracefully
- [ ] Benchmark: measure reparse time vs cache overhead

### 4.4 File Watching
- [ ] `taskgraph watch` subcommand
- [ ] Monitor task directory
- [ ] Emit events on changes
- [ ] Integration with reactive workflows

### 4.5 Testing & Quality
- [ ] Integration tests with example task directories
- [ ] Property-based tests for parsers
- [x] Coverage > 80% (achieved in Phase 3)
- [ ] Fuzz testing for edge cases

### 4.6 Distribution
- [ ] Release builds (Linux, macOS, Windows)
- [ ] Cargo publish
- [ ] GitHub releases with binaries
- [ ] Homebrew formula (optional)

## Dependencies (New)

| Crate | Purpose |
|-------|---------|
| `notify` | File watching |
| `toml` | Config parsing |

## Success Criteria

- Stable 1.0 release
- Documentation covers common workflows
- Easy installation via cargo
- Handles 1000+ tasks

## Design Notes

### Why CLI over MCP?

TaskGraph is designed as a CLI tool that agents use via their native bash/shell tools:

1. **Zero overhead when not needed** - MCP servers add latency and cognitive overhead
2. **Natural discovery** - Good `--help` makes CLI self-documenting
3. **Works everywhere** - No MCP client setup required
4. **Composable** - Pipes, redirects, shell scripting

MCP is useful when you need persistent state or event streams. TaskGraph operations are typically stateless queries on a file-based graph, making CLI the better fit.