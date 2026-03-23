# Issue: Path Map Storage Location

## Context
- Phase: 3 (semantic search)
- Task: Embedding index structure
- Date: 2026-03-23

## Problem

The `path_map` (hash → file path) is stored inside `embeddings.safetensors`:

```rust
pub struct EmbeddingIndex {
    pub embeddings: Array2<f32>,      // tensor data
    pub indices: Vec<WindowIndex>,    // tensor data
    pub metadata: EmbeddingMetadata,  // safetensor metadata
    pub path_map: HashMap<u64, String>, // stored as tensor?!
}
```

**Issues:**

1. **Coupling** - Path map changes require rewriting entire embeddings file
2. **Corruption risk** - If safetensor corrupted, path map lost
3. **Separation of concerns** - Embeddings are data, paths are metadata
4. **Updates** - Cannot update path_map without rewriting embeddings

## Options

### Option A: Keep in Safetensor (Current)

Pros:
- Single file to manage
- Atomic read

Cons:
- Cannot update independently
- Larger file size (path strings in tensor)

### Option B: Separate JSON File

```
.taskgraph/
├── embeddings.safetensors  # Just tensors
├── path_map.json           # { "hash1": "path1", ... }
└── cache.json              # (future)
```

Pros:
- Can update path_map without touching embeddings
- Faster to read/write
- Survives embedding file corruption

Cons:
- Two files to keep in sync
- Need to handle missing/stale path_map

### Option C: EmbeddingMetadata per File

```rust
pub struct EmbeddingMetadata {
    pub model_id: String,
    pub embedding_dim: usize,
    // ...
    pub files: Vec<FileRecord>,  // NEW
}

pub struct FileRecord {
    pub path: String,
    pub path_hash: u64,
    pub content_hash: u64,  // for invalidation
    pub window_count: usize,
}
```

Pros:
- All metadata in one place
- Enables per-file invalidation
- Clear separation from tensor data

Cons:
- Metadata grows with file count
- Still stored in safetensor (same issues)

### Option D: SQLite/Cache Integration

Store in the same `cache.json` that TaskGraph cache would use:

```json
{
  "mtimes": { ... },
  "task_hashes": { ... },
  "embedding_paths": {
    "12345": "tasks/foo.md",
    "67890": "tasks/bar.md"
  },
  "embedding_content_hashes": {
    "12345": "abc123",
    "67890": "def456"
  }
}
```

Pros:
- Unified cache management
- Already planning cache persistence
- Easy to update

Cons:
- Ties semantic feature to cache feature
- Cache is per-project, embeddings might be shared?

## Recommendation

**Option B** for now (separate JSON), migrate to **Option D** when cache is implemented.

## Priority

Low - Works correctly, just inefficient for updates.