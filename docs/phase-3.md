# Phase 3: Cleanup & Review

## Objective

Address technical debt, improve test coverage, and ensure production readiness before advanced features.

## Tasks

### 3.1 Test Coverage

**Current: ~80% | Target: 80% (soft)**

- [x] Add integration tests using `assert_cmd`
  - [x] `validate` command
  - [x] `list` command with filters
  - [x] `topo` command
  - [x] `show` command
  - [x] Error cases (missing task, invalid path)

- [x] Add unit tests for `graph.rs`
  - [x] Topological sort
  - [x] Cycle detection
  - [x] Parallel groups
  - [x] Critical path
  - [x] Betweenness centrality

- [x] Add unit tests for `cache.rs`
  - [x] Has changed detection
  - [x] Mtime tracking

- [ ] Add more integration tests for edge cases
- [ ] Improve task.rs coverage (more Task parsing tests)

### 3.2 Code Review

- [ ] Review `commands/*.rs` for error handling consistency
- [ ] Review `graph.rs` for algorithm correctness
- [ ] Review all public API documentation

### 3.3 Documentation Review

- [x] Verify README accuracy
- [x] Check ARCHITECTURE.md reflects current state
- [ ] Update phase docs with completion status
- [ ] Add inline code documentation where missing

### 3.4 Code Quality

- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo fmt --check`
- [ ] Remove dead code (if any)
- [ ] Consolidate duplicate code patterns

### 3.5 Performance Baseline

- [ ] Benchmark task discovery (100, 1000 tasks)
- [ ] Document performance characteristics

## Success Criteria

- [x] Test coverage ~80% (soft target met)
- [ ] All clippy warnings resolved
- [ ] All documented issues reviewed
- [ ] Performance baselines documented
- [ ] Ready for Phase 4 (Polish & Extensions)

## Notes

- Test coverage improved from 20.89% → 68.40% → ~80%