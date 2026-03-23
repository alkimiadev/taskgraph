# Phase 2: Graph Operations

## Objective

Build and analyze dependency graphs. Implement cache for performance.

## Status: ✅ MOSTLY COMPLETE (see gaps below)

## Tasks

### 2.1 Graph Building ✅
- [x] Scan tasks directory
- [x] Parse all task files
- [x] Build `petgraph::DiGraph`
- [x] Warn on missing dependencies (don't block)
- [x] Report missing dependencies in `validate`

### 2.2 Cache System ⚠️ PARTIAL
- [ ] Create `.taskgraph/` directory
- [ ] Store graph state in `cache.json`
- [ ] Store file mtimes
- [ ] Rebuild on file changes (mtime comparison)
- [x] `taskgraph cache clear` (stub)
- [x] `taskgraph cache status` (stub)

**Gap**: Cache commands exist but don't persist data. See `docs/issues/cache-not-persisted.md`.

**Recommendation**: Cache not critical for current scale. Revisit if performance becomes an issue.

### 2.3 Graph Commands ✅

#### `deps <id>` ✅
- [x] Show what a task depends on (direct)
- [x] No graph needed - just read frontmatter

#### `dependents <id>` ✅
- [x] Show what depends on a task (direct)
- [x] Requires full graph

#### `topo` ✅
- [x] Topological sort
- [x] Filter by status (`--status pending`)
- [x] Output: ordered list
- [x] Handle cycles: show cycle, partial order up to cycle

#### `cycles` ✅
- [x] Detect circular dependencies
- [x] Output: all cycle paths found
- [x] Format: `A → B → C → A`

#### `parallel` ✅
- [x] Group tasks by generation (Kahn's algorithm)
- [x] Output: generation 1, generation 2, etc.
- [x] Same generation = can run concurrently

#### `critical` ✅
- [x] Find longest path through graph
- [x] These tasks block completion if delayed
- [x] Output: path with task IDs

#### `bottleneck` ✅
- [x] Find high betweenness centrality tasks
- [x] These tasks are on many dependency paths
- [x] Output: ranked list with scores

#### `graph` ✅
- [x] Output in DOT format
- [x] `--output file.dot` to save
- [x] Pipe to graphviz: `taskgraph graph | dot -Tpng -o graph.png`

### 2.4 Workflow Analysis Commands ⏳ NOT IMPLEMENTED

See `docs/issues/incomplete-workflow-commands.md` for details.

#### `risk` ❌
- [ ] Show risk distribution (count by level)
- [ ] List high/critical tasks
- [ ] Output: summary + details

#### `risk-path` ❌
- [ ] Find path with highest cumulative risk
- [ ] Combine risk + impact along paths
- [ ] Output: path with risk score

#### `decompose-check` ❌
- [ ] Flag tasks where `risk > medium` OR `scope > moderate`
- [ ] These should be decomposed further
- [ ] Output: list with reasons

#### `workflow-cost` ❌
- [ ] Implement EV calculation from cost-benefit framework
- [ ] Use categorical → numeric mappings
- [ ] Default params: F=20, t=0.5, v=100
- [ ] CLI overrides: `--fallback-cost`, `--time-lost`, `--value-rate`
- [ ] Output: expected cost in $

**Status**: Not blocking. These are independent features that can be added later.

## Assumption Points (Resolved)

| Question | Decision |
|----------|----------|
| Cache invalidation | mtime only - fast, good enough |
| Missing dependency | Warning (don't block iterative building) |
| Cycle display | Full cycle path: `A → B → C → A` |
| Parallel grouping | By generation (Kahn's algorithm) |
| Critical vs bottleneck | Separate: `critical` = longest path, `bottleneck` = betweenness |
| Cost-benefit params | Defaults + CLI overrides. No config required. |

## Dependencies

| Crate | Purpose |
|-------|---------|
| `petgraph` | Graph data structure, algorithms |
| `serde_json` | Cache serialization |
| `walkdir` | Directory scanning |

### Reference Sources (after cargo build)

```
~/.cargo/registry/src/*/petgraph-*/      - graph algorithms (toposort, cycles, betweenness)
~/.cargo/registry/src/*/serde_json-*/    - JSON serialization patterns
```

### Key petgraph algorithms used

```rust
petgraph::algo::toposort(&graph, None)           // topological sort
petgraph::algo::is_cyclic_directed(&graph)       // cycle detection
petgraph::algo::dijkstra(...)                     // shortest paths
petgraph::dot::Dot::new(&graph)                   // DOT format output
```

## Tests Required

- [x] Build graph from tasks
- [x] Detect cycle and show path
- [x] Topological sort (with and without cycles)
- [x] Dependencies and dependents (direct)
- [x] Parallel generation grouping
- [x] Critical path calculation
- [x] Bottleneck identification
- [ ] Cache rebuild detection (cache not implemented)
- [ ] Risk distribution calculation (not implemented)
- [ ] Workflow EV calculation (not implemented)

## Success Criteria

- [x] Core graph commands work
- [x] Cycle detection shows actionable paths
- [ ] Cache reduces rebuild overhead (not critical for current scale)
- [ ] Workflow analysis provides useful insights (deferred)