# Phase 2: Graph Operations

## Objective

Build and analyze dependency graphs. Implement cache for performance.

## Tasks

### 2.1 Graph Building
- [ ] Scan tasks directory
- [ ] Parse all task files
- [ ] Build `petgraph::DiGraph`
- [ ] Warn on missing dependencies (don't block)
- [ ] Report missing dependencies in `validate`

### 2.2 Cache System
- [ ] Create `.taskgraph/` directory
- [ ] Store graph state in `cache.json`
- [ ] Store file mtimes
- [ ] Store task_id → hash mapping (for semantic search later)
- [ ] Rebuild on file changes (mtime comparison)
- [ ] `taskgraph cache clear`
- [ ] `taskgraph cache status`

### 2.3 Graph Commands

#### `deps <id>`
- [ ] Show what a task depends on (direct)
- [ ] Optional `--transitive` flag for full chain
- [ ] No graph needed - just read frontmatter

#### `dependents <id>`
- [ ] Show what depends on a task (direct)
- [ ] Optional `--transitive` flag for full tree
- [ ] Requires full graph

#### `topo`
- [ ] Topological sort
- [ ] Filter by status (`--status pending`)
- [ ] Output: ordered list
- [ ] Handle cycles: show cycle, partial order up to cycle

#### `cycles`
- [ ] Detect circular dependencies
- [ ] Output: all cycle paths found
- [ ] Format: `A → B → C → A`

#### `parallel`
- [ ] Group tasks by generation (Kahn's algorithm)
- [ ] Output: generation 1, generation 2, etc.
- [ ] Same generation = can run concurrently

#### `critical`
- [ ] Find longest path through graph
- [ ] These tasks block completion if delayed
- [ ] Output: path with task IDs

#### `bottleneck`
- [ ] Find high betweenness centrality tasks
- [ ] These tasks are on many dependency paths
- [ ] Output: ranked list with scores

#### `graph`
- [ ] Output in DOT format
- [ ] `--output file.dot` to save
- [ ] Pipe to graphviz: `taskgraph graph | dot -Tpng -o graph.png`

### 2.4 Workflow Analysis Commands

#### `risk`
- [ ] Show risk distribution (count by level)
- [ ] List high/critical tasks
- [ ] Output: summary + details

#### `risk-path`
- [ ] Find path with highest cumulative risk
- [ ] Combine risk + impact along paths
- [ ] Output: path with risk score

#### `decompose-check`
- [ ] Flag tasks where `risk > medium` OR `scope > moderate`
- [ ] These should be decomposed further
- [ ] Output: list with reasons

#### `workflow-cost`
- [ ] Implement EV calculation from cost-benefit framework
- [ ] Use categorical → numeric mappings
- [ ] Default params: F=20, t=0.5, v=100
- [ ] CLI overrides: `--fallback-cost`, `--time-lost`, `--value-rate`
- [ ] Output: expected cost in $

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

## Tests Required

- Build graph from tasks
- Detect cycle and show path
- Topological sort (with and without cycles)
- Dependencies and dependents (direct and transitive)
- Parallel generation grouping
- Critical path calculation
- Bottleneck identification
- Cache rebuild detection
- Risk distribution calculation
- Workflow EV calculation

## Success Criteria

- All graph commands work
- Cycle detection shows actionable paths
- Cache reduces rebuild overhead
- Workflow analysis provides useful insights
- No config required for basic use