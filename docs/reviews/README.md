# Reviews

This directory contains code reviews conducted during implementation.

## Purpose

Structured reviews at natural breakpoints catch issues early and maintain quality.

## When to Review

Per SDD framework:
- After each "community" of related tasks
- Before critical path tasks
- Before parallel work groups merge
- At explicit review tasks in the task graph

## Review Template

```markdown
# Review: [Scope]

## Date
[YYYY-MM-DD]

## Scope
- Files changed: [list]
- Tasks completed: [list]
- Phase: [1/2/3/4]

## Code Quality
- [ ] Follows project conventions
- [ ] No obvious bugs
- [ ] Handles edge cases
- [ ] Error messages clear

## Architecture
- [ ] Follows ARCHITECTURE.md
- [ ] No hidden assumptions
- [ ] Dependencies appropriate

## Tests
- [ ] Happy path covered
- [ ] Error path covered
- [ ] Edge cases covered
- [ ] Tests pass

## Issues Found
1. [Issue] - [Severity: low/medium/high]

## Recommendations
1. [Recommendation]

## Verdict
- [ ] Approve
- [ ] Approve with fixes
- [ ] Needs rework
```

## Review Types

| Type | When | Focus |
|------|------|-------|
| Self-review | After each task | Correctness |
| Checkpoint | After task groups | Quality, patterns |
| Architecture | Phase boundaries | Design adherence |
| Final | Before release | Security, performance |

## Process

1. Agent completes tasks
2. Agent switches to review mode (self-review)
3. Document findings here
4. Fix issues or proceed
5. Mark review complete in task status