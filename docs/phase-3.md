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
- [ ] Tensor: `embeddings` [N, 256] F32
- [ ] Tensor: `index` [N, 16] U8 (struct tensor)
- [ ] Memory-mapped access for fast reads

#### Index Struct Layout (16 bytes)
```
[task_hash: u64][window_start: u32][window_end: u32]
```
- [ ] Hash task IDs with xxHash3
- [ ] Store hash → id mapping in cache.json
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
| `model2vec-rs` | Embedding generation |
| `safetensors` | Storage format |
| `ndarray` | Matrix operations (cosine similarity) |
| `twox-hash` | xxHash3 for task ID hashing |

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
- Embeddings stored in ~4KB per task (256 dims × 4 bytes)
- Search is fast (sub-second for hundreds of tasks)
- Feature can be disabled without affecting core
- Clear error messages when feature not enabled