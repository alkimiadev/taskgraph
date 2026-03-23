# Phase 4: Cleanup & Review

## Objective

Address technical debt, improve test coverage, and ensure production readiness before advanced features.

## Tasks

### 4.1 Test Coverage

**Current: 20.89% | Target: 80%**

- [ ] Add integration tests using `assert_cmd`
  - [ ] `validate` command
  - [ ] `list` command with filters
  - [ ] `topo` command
  - [ ] `show` command
  - [ ] Error cases (missing task, invalid path)

- [ ] Add unit tests for `graph.rs`
  - [ ] Topological sort
  - [ ] Cycle detection
  - [ ] Parallel groups
  - [ ] Critical path
  - [ ] Betweenness centrality

- [ ] Add unit tests for `cache.rs`
  - [ ] Has changed detection
  - [ ] Mtime tracking

- [ ] Add roundtrip test for embedding index
  - [ ] Create index → save → load → verify

### 4.2 Code Review

- [ ] Review `commands/*.rs` for error handling consistency
- [ ] Review `embedding.rs` for edge cases
- [ ] Review `graph.rs` for algorithm correctness
- [ ] Review all public API documentation

### 4.3 Missing Features Review

Per `docs/issues/incomplete-workflow-commands.md`:
- [ ] Review if `risk` command is needed
- [ ] Review if `risk-path` command is needed
- [ ] Review if `decompose-check` command is needed
- [ ] Review if `workflow-cost` command is needed

### 4.4 Documentation Review

- [ ] Verify README accuracy
- [ ] Check ARCHITECTURE.md reflects current state
- [ ] Update phase docs with completion status
- [ ] Add inline code documentation where missing

### 4.5 Code Quality

- [ ] Run `cargo clippy --all-features -- -D warnings`
- [ ] Run `cargo fmt --check`
- [ ] Remove dead code (if any)
- [ ] Consolidate duplicate code patterns

### 4.6 Performance Baseline

- [ ] Benchmark task discovery (100, 1000 tasks)
- [ ] Benchmark embedding generation
- [ ] Benchmark search query time
- [ ] Document performance characteristics

## Success Criteria

- [ ] Test coverage ≥ 80%
- [ ] All clippy warnings resolved
- [ ] All documented issues reviewed
- [ ] Performance baselines documented
- [ ] Ready for Phase 5 (Polish & Extensions)