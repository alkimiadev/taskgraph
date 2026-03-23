# Issue: Safetensor Update Strategy

## Context
- Phase: 3 (semantic search)
- Task: Efficient index updates
- Date: 2026-03-23

## Problem

Current approach: **Full rewrite** on every `taskgraph embed`

```rust
// Current: serialize entire index to file
index.save(&embeddings_path)?;
```

For large indexes (1000+ tasks, 10k+ windows), this is:
- Slow (rewrite megabytes of embeddings)
- Wasteful (most embeddings unchanged)
- Blocks concurrent reads

## Options

### Option A: Full Rewrite (Current)

```rust
pub fn save(&self, path: &Path) -> Result<()>;
```

Pros:
- Simple
- Always consistent

Cons:
- O(N) for any change
- Slow for large indexes

### Option B: Append-Only

```
[header v1] [embeddings v1] [index v1]
[header v2] [embeddings v2] [index v2]  <- appended
```

On read: use latest header/embeddings.

Pros:
- Fast writes (append only)
- History preserved

Cons:
- File grows unbounded
- Need compaction
- More complex read logic

### Option C: Chunked Files

```
.taskgraph/
├── embeddings/
│   ├── shard-000.safetensors
│   ├── shard-001.safetensors
│   └── manifest.json
```

Pros:
- Update one shard at a time
- Parallel reads possible
- Natural grouping (by file hash?)

Cons:
- Multiple files to manage
- Cross-shard search
- Manifest consistency

### Option D: Memory-Mapped + Copy-on-Write

Research from metatensor project:
- mmap the safetensor for reads
- Copy-on-write for updates
- Only write changed regions

Pros:
- Fast reads (mmap)
- Efficient partial updates
- Memory efficient

Cons:
- Complex implementation
- Need to handle alignment
- Platform-specific behavior?

## Research Needed

- [ ] Check metatensor project patterns
- [ ] Benchmark current approach with 1000+ tasks
- [ ] Evaluate safetensors-rs capabilities for partial updates

## Priority

Low - Optimization, not blocking. Profile first.

## Related

- `docs/issues/index-invalidation-strategy.md`
- `docs/issues/path-map-storage.md`