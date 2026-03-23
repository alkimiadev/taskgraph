# Phase 5: Polish & Extensions

## Objective

Production-ready CLI with advanced features.

## Priority Order

Based on the SDD framework use case and "assumption points" analysis:

1. **Configuration** - Needed for any non-trivial use
2. **MCP Server** - Directly serves the SDD use case
3. **Documentation** - Required for release
4. **Performance** - Needed for large task sets
5. **File Watching** - Enables reactive workflows
6. **TUI Mode** - Nice to have, lower priority
7. **Distribution** - Final step

## Tasks

### 5.1 Configuration
- [ ] Global config: `~/.config/taskgraph/config.toml`
- [ ] Project config: `.taskgraph/config.toml`
- [ ] Settings: default path, model, output format
- [ ] Config precedence: project > global > defaults

### 5.2 MCP Server
- [ ] Model Context Protocol server mode
- [ ] Expose as MCP tools for LLM integration
- [ ] `taskgraph mcp` subcommand
- [ ] Stdio transport (same as other MCP servers)
- [ ] Tools: list, show, topo, cycles, parallel, critical

### 5.3 Documentation
- [ ] README with installation, quick start, examples
- [ ] Shell completion (bash, zsh, fish) via clap
- [ ] Example task files
- [ ] Example workflows (SDD integration)

### 5.4 Performance
- [ ] Profile hot paths
- [ ] Parallel file parsing with rayon
- [ ] Handle 1000+ tasks gracefully
- [ ] Cache efficiency metrics

### 5.5 File Watching
- [ ] `taskgraph watch` subcommand
- [ ] Monitor task directory
- [ ] Emit events on changes
- [ ] Optional: rebuild cache automatically
- [ ] Integration with reactive workflows (pubsub?)

### 5.6 TUI Mode (Optional)
- [ ] `taskgraph tui` subcommand
- [ ] Interactive task list
- [ ] Dependency graph visualization
- [ ] Status updates in real-time
- [ ] Lower priority - editors exist

### 5.7 Testing & Quality
- [ ] Integration tests with example task directories
- [ ] Property-based tests for parsers
- [ ] Coverage > 80%
- [ ] Fuzz testing for edge cases

### 5.8 Distribution
- [ ] Release builds (Linux, macOS, Windows)
- [ ] Cargo publish
- [ ] GitHub releases with binaries
- [ ] Homebrew formula (optional)
- [ ] AUR package (optional)

## Assumption Points (Resolved)

| Question | Decision |
|----------|----------|
| Watch implementation | `notify` crate - standard, well-maintained |
| TUI necessity | Lower priority. Editors + CLI covers most use cases |
| MCP priority | High - directly serves SDD use case |
| Config format | TOML - human-friendly, standard for Rust tools |
| Distribution | Cargo primary, GitHub releases for binaries |

## Dependencies (New)

| Crate | Purpose |
|-------|---------|
| `notify` | File watching |
| `toml` | Config parsing |
| `ratatui` | TUI (optional) |

### Reference Sources (after cargo build)

```
~/.cargo/registry/src/*/notify-*/      - file system watching patterns
~/.cargo/registry/src/*/toml-*/        - TOML parsing
~/.cargo/registry/src/*/ratatui-*/     - TUI building (if implementing)

## Success Criteria

- Stable 1.0 release
- Documentation covers common workflows
- Easy installation via cargo
- Handles 1000+ tasks
- MCP server works with common LLM tools