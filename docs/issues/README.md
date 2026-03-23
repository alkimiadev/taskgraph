# Issues

This directory documents blocking issues encountered during implementation.

## Purpose

When an agent encounters a breaking issue that cannot be resolved with available tools/knowledge:

1. **Document the issue** - Create a file describing the problem
2. **Pause and move on** - Don't force impossible tasks
3. **Wait for resolution** - Human or research can resolve

This is the "safe exit" mechanism - forcing agents to complete impossible tasks causes problems.

## Issue Template

```markdown
# Issue: [Brief Description]

## Context
- Phase: [1/2/3/4]
- Task: [task name from phase doc]
- Date: [YYYY-MM-DD]

## Problem
[What went wrong]

## Attempted Solutions
1. [What you tried]
2. [What you tried]

## Blocked By
- [ ] Missing dependency
- [ ] Tool limitation
- [ ] Environment issue
- [ ] Need clarification
- [ ] Other: ___

## Suggested Resolution
[If you have ideas]

## Workaround
[If any exists]
```

## When to Use

- Tool/dependency not working as expected
- Cannot find solution via web search or source code
- Environment configuration issues
- Ambiguous requirements needing human input
- Any situation where forcing completion would cause problems

## Resolution

Once resolved, update the issue file:
```markdown
## Resolution
- Date: [YYYY-MM-DD]
- How: [How it was fixed]
```

Then move to `docs/issues/resolved/` for reference.