# Incomplete: Workflow Analysis Commands

## Context
- Phase: 2
- Task: Workflow analysis commands
- Date: 2026-03-23

## Problem

Four workflow analysis commands from phase-2.md were not implemented:

| Command | Description | Status |
|---------|-------------|--------|
| `risk` | Show risk distribution across tasks | Not implemented |
| `risk-path` | Find path with highest cumulative risk | Not implemented |
| `decompose-check` | Flag tasks that should be split | Not implemented |
| `workflow-cost` | Calculate expected cost using EV framework | Not implemented |

## Status

Not blocking - these are independent features that can be added later.

## Design Notes

### Categorical → Continuous Mappings

Already defined in `ARCHITECTURE.md`:

**Risk → Success Probability (p):**
| Risk | p |
|------|---|
| trivial | 0.98 |
| low | 0.90 |
| medium | 0.80 |
| high | 0.65 |
| critical | 0.50 |

**Impact → Criticality Weight:**
| Impact | Weight |
|--------|--------|
| isolated | 1.0 |
| component | 1.5 |
| phase | 2.0 |
| project | 3.0 |

### EV Formula

From `cost_benefit_analysis_framework.py`:

```
EV_task = P_success × C_success + (1 - P_success) × C_fail
```

Where:
- `P_success = 1 - (1-p)^(r+1)` — probability of success within r+1 attempts
- `C_success = c + f × E[R|succ]` — cost given eventual success
- `C_fail = c + r×f + F + t×v` — cost on total failure

**Parameters:**
| Symbol | Meaning | Default |
|--------|---------|---------|
| p | success probability | from risk mapping |
| r | max retries | 2 |
| c | cost per attempt | derived from scope |
| f | retry cost | ≈ c |
| F | fallback cost | 20 (configurable) |
| t | hours lost on failure | 0.5 (configurable) |
| v | value of time ($/hr) | 100 (configurable) |

**Scope → Cost (c):**
| Scope | Approx Tokens | Est. Cost |
|-------|---------------|-----------|
| single | ~500 | $0.10 |
| narrow | ~1,500 | $0.30 |
| moderate | ~3,000 | $0.60 |
| broad | ~6,000 | $1.20 |
| system | ~10,000+ | $2.00 |

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
- Sum across workflow (optionally weighted by criticality)
- CLI overrides: `--fallback-cost`, `--time-lost`, `--value-rate`
- Output: total EV in $

## Next Steps

1. Implement `risk` command (simple aggregation)
2. Implement `decompose-check` (filter + flag)
3. Implement `workflow-cost` (EV calculation)
4. Implement `risk-path` (path analysis with weights)