# Phase 4: Cleanup & Review

## Objective

Address technical debt, improve test coverage, and ensure production readiness before advanced features.

## Tasks

### 4.1 Test Coverage

**Current: 68.40% | Target: 80%**

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

### 4.2 Code Review

- [ ] Review `commands/*.rs` for error handling consistency
- [ ] Review `graph.rs` for algorithm correctness
- [ ] Review all public API documentation

### 4.3 Missing Features Review

Per `docs/issues/incomplete-workflow-commands.md`:
- [ ] Review if `risk` command is needed
- [ ] Review if `risk-path` command is needed
- [ ] Review if `decompose-check` command is needed
- [ ] Review if `workflow-cost` command is needed

### 4.4 Documentation Review

- [x] Verify README accuracy
- [x] Check ARCHITECTURE.md reflects current state
- [ ] Update phase docs with completion status
- [ ] Add inline code documentation where missing

### 4.5 Code Quality

- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo fmt --check`
- [ ] Remove dead code (if any)
- [ ] Consolidate duplicate code patterns

### 4.6 Performance Baseline

- [ ] Benchmark task discovery (100, 1000 tasks)
- [ ] Document performance characteristics

## Success Criteria

- [ ] Test coverage ≥ 80%
- [ ] All clippy warnings resolved
- [ ] All documented issues reviewed
- [ ] Performance baselines documented
- [ ] Ready for Phase 5 (Polish & Extensions)

## Notes

- Semantic search has been extracted to [taskgraph-semantic](../taskgraph-semantic/)
- Test coverage improved from 20.89% to 68.40% during initial Phase 4 work