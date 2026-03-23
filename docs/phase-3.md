# Phase 3: Semantic Search

## Objective

Add embedding-based semantic search across task descriptions. Feature-gated.

## Tasks

### 3.1 Feature Flag Setup
- [ ] Add `semantic` feature to Cargo.toml
- [ ] Conditional compilation with `#[cfg(feature = "semantic")]`
- [ ] Document feature requirements in README
- [ ] Graceful error if used without feature

### 3.2 Embedding Infrastructure

#### Model Loading
- [ ] Load model2vec model (default: potion-base-8M)
- [ ] Load model per command invocation (8MB, fast enough)
- [ ] `--model` flag for override (HuggingFace ID or local path)
- [ ] Handle model not found with clear error message

#### Rolling Window Embeddings
- [ ] Tokenize task body using model's tokenizer
- [ ] Create windows of 512 tokens
- [ ] 50% overlap between windows
- [ ] Generate embedding per window (256 dims for potion-base-8M)
- [ ] Track window positions (start token, end token)

### 3.3 Safetensor Storage

#### Storage Format
- [ ] Create `.taskgraph/embeddings.safetensors`
- [ ] Tensor: `embeddings` [N, D] F32 where D = model embedding dim
- [ ] Tensor: `index` [N, 16] U8 (struct tensor)
- [ ] Memory-mapped access for fast reads

#### Metadata (in safetensor `__metadata__` field)
```json
{
  "model_id": "minishlab/potion-base-8M",
  "embedding_dim": "256",
  "window_size": "512",
  "overlap": "0.5",
  "created_at": "2026-03-23T12:00:00Z",
  "file_count": "42"
}
```
- [ ] Store model info for validation and status display
- [ ] Detect model mismatch (rebuild needed?)
- [ ] Enable `embed --status` to show model used

#### Index Struct Layout (16 bytes)
```
[file_path_hash: u64][window_start: u32][window_end: u32]
```
- [ ] Hash **file path** with xxHash3 (not task_id - more general)
- [ ] Enables reuse for memories, notes, any markdown collection
- [ ] Store hash → path mapping in cache.json
- [ ] Pack/unpack struct tensor

### 3.4 Commands

#### `search <query>`
- [ ] Embed query text
- [ ] Cosine similarity against all window embeddings
- [ ] Return top-k results (default 10, `--top-k` override)
- [ ] Group by task_id (deduplicate windows from same task)
- [ ] Show matched context (which window matched)
- [ ] `--json` output

#### `embed`
- [ ] `taskgraph embed` - build/rebuild embeddings
- [ ] `taskgraph embed --status` - show info (count, model, size, last build)
- [ ] Detect stale embeddings (mtimes changed)
- [ ] Full rebuild (no incremental for v1)
- [ ] Store file_path_hash → path mapping in cache.json

### 3.5 CLI Integration
- [ ] Search subcommand under main CLI
- [ ] Clear error if feature not enabled: "Enable with `cargo install taskgraph --features semantic`"

## Assumption Points (Resolved)

| Question | Decision |
|----------|----------|
| Window size | Fixed 512 tokens (matches model) |
| Overlap | Fixed 50% (no content lost) |
| Model choice | Default potion-base-8M, `--model` override |
| Top-k | Default 10, `--top-k` flag |
| Incremental vs rebuild | Full rebuild (simpler for v1) |
| Model download | HuggingFace or local path (both supported) |
| Memory | Per command invocation (8MB model is fine) |

## Dependencies (Feature-gated)

| Crate | Purpose |
|-------|---------|
| `model2vec-rs` | Embedding generation (git: alkimiadev fork) |
| `safetensors` | Storage format |
| `ndarray` | Matrix operations (cosine similarity) |
| `twox-hash` | xxHash3 for file path hashing |

### Reference Sources (after cargo build)

```
~/.cargo/git/checkouts/model2vec-rs-*/           - our fork with encode_with_stats
~/.cargo/registry/src/*/safetensors-*/           - safetensor read/write
~/.cargo/registry/src/*/ndarray-*/               - matrix operations
```

### Key model2vec-rs API (our fork)

```rust
use model2vec_rs::model::{StaticModel, EncodeResult};

let model = StaticModel::from_pretrained("minishlab/potion-base-8M", None, None, None)?;

// Single-pass: embeddings + token counts
let result: EncodeResult = model.encode_with_stats(&sentences, Some(512), 1024);
// result.embeddings: Vec<Vec<f32>>
// result.token_counts: Vec<usize>

// Access tokenizer for rolling window logic
let tokenizer = model.tokenizer();
```

## Tests Required

- Embed single text
- Rolling window generation (verify overlap)
- Safetensor write/read roundtrip
- Struct tensor pack/unpack
- Search returns relevant results
- Top-k limiting
- Grouping by task_id
- Feature flag disables correctly

## Success Criteria

- `search` returns semantically relevant tasks
- Embeddings stored in ~D*4 bytes per window (D = model dim, e.g., 1024 bytes for 256 dims)
- Search is fast (sub-second for hundreds of tasks)
- Feature can be disabled without affecting core
- Clear error messages when feature not enabled
- Model-agnostic: works with any model2vec model