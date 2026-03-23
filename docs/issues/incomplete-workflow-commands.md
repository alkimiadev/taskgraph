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

## Notes

- Core graph commands (`topo`, `cycles`, `parallel`, `critical`, `bottleneck`) are complete
- These require the `risk`, `scope`, `impact` fields from task frontmatter
- `workflow-cost` requires cost-benefit parameter handling (defaults + CLI overrides)

## Next Steps

1. Implement `risk` command (simple aggregation)
2. Implement `risk-path` (path analysis with weights)
3. Implement `decompose-check` (filter + flag)
4. Implement `workflow-cost` (EV calculation)