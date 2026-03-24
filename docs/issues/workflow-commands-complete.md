# Complete: Workflow Analysis Commands

## Context
- Phase: 2
- Task: Workflow analysis commands
- Date: 2026-03-23

## Resolution

All four workflow analysis commands implemented:

| Command | Description | Status |
|---------|-------------|--------|
| `risk` | Show risk distribution across tasks | ✅ Implemented |
| `risk-path` | Find path with highest cumulative risk | ✅ Implemented |
| `decompose` | Flag tasks that should be split | ✅ Implemented |
| `workflow-cost` | Relative workflow cost comparison | ✅ Implemented |

## Implementation Commits

- `5141a93` feat: add risk command
- `352d6f2` feat: add decompose command
- `5ef25a2` feat: add workflow-cost command
- `2bd3b66` feat: add risk-path command

## Design Philosophy

### Structural Analysis, Not Cost Calculation

These commands surface **structural risk patterns** rather than precise dollar amounts:

| Command | Structural Question |
|---------|---------------------|
| `risk` | Where is risk concentrated in the graph? |
| `decompose-check` | Which upstream tasks should be split? |
| `risk-path` | Which failure chain has most downstream damage? |
| `workflow-cost` | Relative comparison - path A vs path B |

### Why Relative, Not Absolute?

1. **Token counts vary**: Interface bloat, model verbosity, context window usage
2. **Prices change**: Model costs vary by provider
3. **LLM calibration**: Models are reliable at "high vs medium" but not "$3.42 vs $3.50"
4. **Structural insight matters more**: Upstream failures multiply downstream surface area

### The Key Insight

Failures at higher levels multiply downstream surface area:

```
planning failure → wrong decomposition → wasted implementation
decomposition failure → unclear tasks → rework
review failure → bugs shipped → rework
```

This means `risk: critical` at planning level > `risk: critical` at implementation level.

## Design Notes

### Mappings (for relative ordering)

Defined in `ARCHITECTURE.md`. These preserve ordering, not precision.

**Risk → Failure Likelihood:**
| Risk | Ordering | p |
|------|----------|---|
| trivial | 1 | 0.98 |
| low | 2 | 0.90 |
| medium | 3 | 0.80 |
| high | 4 | 0.65 |
| critical | 5 | 0.50 |

**Impact → Downstream Damage:**
| Impact | Ordering | Weight |
|--------|----------|--------|
| isolated | 1 | 1.0 |
| component | 2 | 1.5 |
| phase | 3 | 2.0 |
| project | 4 | 3.0 |

**Scope → Size:**
| Scope | Ordering | Approx Cost |
|-------|----------|-------------|
| single | 1 | 0.10 |
| narrow | 2 | 0.30 |
| moderate | 3 | 0.60 |
| broad | 4 | 1.20 |
| system | 5 | 2.00 |

### EV Formula (Reference)

From `cost_benefit_analysis_framework.py`. Used for relative comparison, not absolute $.

```
EV_task = P_success × C_success + (1 - P_success) × C_fail
```

The formula captures the intuition that failure cost compounds, but the numeric output should be interpreted as relative units, not dollars.

## Implementation Notes

### `risk` (simple)
- Aggregate tasks by risk level
- Output: count + list per level
- No graph needed

### `risk-path` (medium)
- Find path with highest cumulative risk
- Weight edges by risk/impact
- Use existing critical path algorithm with weights

### `decompose-check` (simple)
- Filter tasks where `risk > medium` OR `scope > moderate`
- Output: list with reasons
- No graph needed

### `workflow-cost` (medium)
- Implement EV calculation per task
- Sum across workflow (optionally weighted by impact)
- Output: relative units (not $)

## Next Steps

1. Implement `risk` command (simple aggregation)
2. Implement `decompose-check` (filter + flag)
3. Implement `workflow-cost` (EV calculation)
4. Implement `risk-path` (path analysis with weights)