# Implementation Guide

## Available Tools

### Core Tools (Always Available)
| Tool | Description |
|------|-------------|
| `read` | Read files/directories |
| `write` | Write files |
| `edit` | Edit files with string replacement |
| `bash` | Execute shell commands |
| `glob` | Find files by pattern |
| `grep` | Search file contents |
| `task` | Spawn sub-agents for research/parallel work |

### Web Research
| Tool | Description |
|------|-------------|
| `webSearch_search` | Search the web (DuckDuckGo, Bing, Exa) |
| `webFetch` / `webSearch_fetchWebContent` | Fetch content from URLs |
| `webSearch_fetchGithubReadme` | Fetch GitHub READMEs |

### Git Worktrees (open-trees plugin)
Enable with `worktree_mode { "action": "on" }`:

| Tool | Description |
|------|-------------|
| `worktree_mode` | Enable/disable worktree mode |
| `worktree_overview` | List/status worktrees |
| `worktree_make` | Create/open/fork worktrees |
| `worktree_cleanup` | Remove/prune worktrees |

**For parallel development:**
```bash
# Create worktree for Phase 1-2
worktree_make { "action": "create", "name": "phase-1-2" }

# Create worktree for Phase 3
worktree_make { "action": "create", "name": "phase-3-semantic" }
```

### Optional CLI Tools
```bash
# Check if installed:
which glimpse  # Fast codebase peeking

# If not, can install:
cargo install glimpse
```

## Sub-Agent Spawning

Use `task` tool for research or parallel work:

```
task {
  "subagent_type": "explore",  // or "general"
  "description": "Research petgraph API",
  "prompt": "Search ~/.cargo/registry/src/*/petgraph-*/ for betweenness centrality implementation..."
}
```

**Good use cases:**
- Researching dependency implementations
- Exploring unfamiliar codebases
- Parallel file operations

## Available Models

Configured in `opencode.json`:

| Model | Best For | Context |
|-------|----------|---------|
| `glm-5` | Complex reasoning (default) | 202K |
| `glm-4.7` | Fast coding | 202K |
| `qwen3-coder-plus` | Code generation | 1M |
| `MiniMax-M2.5` | Balanced | 196K |
| `kimi-k2.5` | Code review | 262K |

All are open models (MIT/Apache-2.0) hosted by Alibaba Cloud.

## Document Organization

```
docs/
├── ARCHITECTURE.md     # Full architecture (source of truth)
├── phase-1.md          # Phase 1 implementation details
├── phase-2.md          # Phase 2 implementation details
├── phase-3.md          # Phase 3 implementation details (feature-gated)
├── phase-4.md          # Phase 4 implementation details
├── implementation.md   # This file - tools and guidelines
└── research/           # Research notes, reference implementations
    └── context_branching.md
```

**When adding docs:**
- Implementation details → `phase-N.md`
- Research/reference → `docs/research/`
- Architecture changes → `ARCHITECTURE.md`

## Dependency Sources

All dependency sources available locally after `cargo build`:

```
~/.cargo/registry/src/     # crates.io dependencies
~/.cargo/git/checkouts/    # git dependencies
```

See `AGENTS.md` for phase-specific reference paths.

## External Repositories

Can clone if needed:
```bash
git clone <url> /workspace/<name>
```

Useful for reference implementations or debugging dependencies.

## Communication Style

- Be concise - output displays on CLI
- 1-3 sentences for simple answers
- No unnecessary preamble/postamble
- Use tables for structured info
- Reference files as `path/to/file.rs:42`