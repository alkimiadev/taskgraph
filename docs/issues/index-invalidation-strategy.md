# Issue: Index Invalidation Strategy

## Context
- Phase: 3 (semantic search)
- Task: Embedding index lifecycle
- Date: 2026-03-23

## Problem

The embedding index has no strategy for detecting or handling changes:

1. **No content hash** - Cannot detect if file was modified
2. **No stale detection** - Deleted files leave orphaned embeddings
3. **No incremental updates** - Full rebuild every time
4. **No parameter change detection** - Model/window size changes ignored

## Current State

```rust
WindowIndex {
    file_path_hash: u64,  // hash of path STRING, not content
    start_token: u32,
    end_token: u32,
    start_char: u32,
    end_char: u32,
}
```

## Missing

### Content Hash

```rust
WindowIndex {
    file_path_hash: u64,   // hash of "tasks/foo.md"
    content_hash: u64,     // hash of file body at embed time
    // ...
}
```

Would enable:
- Detect modified files (same path, different content)
- Skip unchanged files in incremental rebuild

### File Walk Comparison

On `embed`:
1. Walk current files, collect `(path_hash, content_hash)`
2. Compare against index
3. Remove embeddings for deleted files
4. Add embeddings for new files
5. Update embeddings for modified files

## Invalidation Detection Matrix

| Event | Detectable Now | With content_hash |
|-------|----------------|-------------------|
| File modified | ❌ | ✅ |
| File deleted | ❌ | ✅ (after file walk) |
| File added | ✅ | ✅ |
| File renamed | ❌ | ❌ (orphan + duplicate) |
| Model changed | ❌ | ❌ (metadata only) |

## Options

### Option A: Full Rebuild (Current)
- Simple, always correct
- Slow for large indexes
- User manually triggers via `taskgraph embed`

### Option B: Incremental Update
- Detect changes via content_hash + file walk
- Only update changed files
- Need to handle:
  - Removing stale embeddings (requires index rewrite or marking)
  - Window recalculation for modified files
  - Path map updates

### Option C: Hybrid
- Default: incremental if index exists
- Flag: `--full` for complete rebuild
- Safety: rebuild if model/params mismatch

## Questions

1. Should incremental be the default or opt-in?
2. How to handle renamed files? (same content, new path)
3. Should we store `content_hash` in `EmbeddingMetadata` per file?
4. How to efficiently remove embeddings from safetensor?

## Priority

Medium - Feature works, but inefficient for repeated use.

## Related

- `docs/issues/path-map-storage.md`
- `docs/issues/safetensor-update-strategy.md` (to be created)