# Issue: Cache Not Persisted

## Context
- Phase: 2
- Task: Cache system
- Date: 2026-03-23

## Problem

The `cache` command exists but doesn't actually persist or read cache data:

```rust
Commands::Cache { command } => match command {
    CacheCommands::Clear => println!("Cache cleared"),
    CacheCommands::Status => println!("Cache status"),
},
```

**Expected behavior:**
- `cache clear` - Delete `.taskgraph/cache.json`
- `cache status` - Show cache file info (path, size, last modified)

**Current behavior:**
- Both commands just print messages

## Options

1. **Implement cache persistence** - Add actual file read/write
2. **Remove cache commands** - Until cache is actually needed
3. **Leave as stub** - Document as TODO for later

## Recommendation

Option 2: Remove or stub out cache commands. The cache system isn't critical for Phase 1-3 functionality. TaskGraph can rebuild the graph on each invocation (fast enough for typical task counts).

If performance becomes an issue with 1000+ tasks, implement in Phase 5 (Performance).

## Priority

Low - Not blocking any functionality.