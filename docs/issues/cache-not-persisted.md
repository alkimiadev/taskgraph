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

## Analysis

### Why Cache?

Cache helps when:
1. Graph rebuild is expensive (1000+ tasks)
2. Frequent invocations (watch mode, MCP server)

### When Cache is Unnecessary

- < 100 tasks: rebuild is ~instant
- One-off queries: cache overhead > savings
- CI/CD: fresh state each run

### Current Performance

**Unknown** - needs benchmarking.

## Options

1. **Benchmark first** - Measure reparse time at 100/500/1000 tasks before deciding
2. **Implement cache** - Add persistence if benchmarks show need
3. **Remove cache commands** - YAGNI until proven otherwise

## Recommendation

**Option 1: Benchmark first.**

```bash
# Create test datasets
for n in 100 500 1000; do
  # generate $n task files
  time taskgraph topo
done
```

If reparse is < 100ms for 1000 tasks, cache is premature optimization.

## Priority

Low - Not blocking any functionality. Revisit in Phase 4 (Performance) with data.