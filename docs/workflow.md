# Workflow Guide

A practical guide to using TaskGraph effectively. For the reasoning behind categorical estimates and structural analysis, see [framework.md](./framework.md).

## The Problem

AI-assisted development often fails in predictable ways:

1. **Planning failures** → Wrong decomposition → Wasted implementation
2. **Large tasks** → Higher failure rate → More retries → Higher cost
3. **Missing dependencies** → Blocked work → Context switching
4. **No review checkpoints** → Bugs compound → Expensive rework

The cost-benefit framework shows that **upstream failures multiply downstream damage**. A `risk: critical` task at planning level costs more than the same risk at implementation level.

## Solution: Structured Task Graphs

TaskGraph provides:

- **Source of truth**: Tasks as markdown files (git-friendly, LLM-editable)
- **Dependency analysis**: Topological ordering, cycle detection, parallel groups
- **Structural insights**: Bottlenecks, critical paths, risk concentration
- **Review injection**: Identify where reviews should happen

## Workflow Phases

### Phase 1: Planning & Decomposition

**Goal**: Create a well-structured task graph before implementation.

1. **Create tasks** as markdown files in `tasks/` directory:

```markdown
---
id: auth-setup
title: Setup Authentication
status: pending
depends_on: []
scope: moderate
risk: medium
impact: component
level: decomposition
---

Implement OAuth2 authentication with provider abstraction.
```

2. **Validate structure**:

```bash
taskgraph validate    # Check for missing deps, duplicate IDs
taskgraph cycles      # Detect circular dependencies
```

3. **Analyze the graph**:

```bash
taskgraph topo        # See execution order
taskgraph parallel    # Identify parallelizable work
taskgraph critical    # Find longest path (completion blockers)
taskgraph bottleneck  # Find high-betweenness tasks
```

4. **Assess risk distribution**:

```bash
taskgraph risk        # Where is risk concentrated?
taskgraph decompose   # Which tasks should be split?
taskgraph risk-path   # Which failure chain is most damaging?
```

5. **Refine based on analysis**:
   - Tasks flagged by `decompose` → Split into smaller tasks
   - High-risk paths → Add review tasks or reduce scope
   - Bottlenecks → Consider parallel alternatives

### Phase 2: Implementation

**Goal**: Execute tasks in dependency order with verification.

1. **Get next tasks**:

```bash
taskgraph topo --status pending    # Tasks ready to work on
taskgraph parallel                 # Groups that can run concurrently
```

2. **Implement and update status**:

Edit task file:
```markdown
status: in-progress
```

Implement, then:
```markdown
status: completed
```

3. **Check dependencies before moving on**:

```bash
taskgraph deps <id>        # What did this task depend on?
taskgraph dependents <id>  # What's waiting on this task?
```

### Phase 3: Review

**Goal**: Validate work at natural checkpoints.

Use graph analysis to determine review timing:

| Analysis | Review Trigger |
|----------|----------------|
| `parallel` groups | Review before groups merge |
| `bottleneck` tasks | Review before critical path |
| `risk` hotspots | Review high-risk tasks before dependents |
| `critical` path | Review before critical tasks |

### Phase 4: Blocked Tasks

**Goal**: Don't get stuck. Use the safe exit mechanism.

When a task becomes untendable:

1. Create a blocker task:
```bash
taskgraph init resolve-auth-config --name "Resolve: Missing OAuth config"
```

2. Mark original as blocked:
```markdown
status: blocked
depends_on: [resolve-auth-config]
```

3. Move to next unblocked task:
```bash
taskgraph topo --status pending
```

## Command Reference

### Discovery & Validation

| Command | Purpose |
|---------|---------|
| `init <id>` | Scaffold a new task file |
| `validate` | Check all tasks valid, no duplicates/missing deps |
| `list` | List all tasks |
| `show <id>` | Display task details |

### Graph Analysis

| Command | Purpose | Use When |
|---------|---------|----------|
| `deps <id>` | What this task depends on | Understanding a task |
| `dependents <id>` | What depends on this task | Before changing a task |
| `topo` | Topological sort | Planning execution order |
| `cycles` | Detect circular dependencies | After adding dependencies |
| `parallel` | Groups of parallelizable tasks | Optimizing workflow |
| `critical` | Longest path through graph | Identifying blockers |
| `bottleneck` | High-betweenness tasks | Finding review points |
| `graph` | DOT format output | Visualization |

### Workflow Analysis

| Command | Purpose | Use When |
|---------|---------|----------|
| `risk` | Risk distribution | Assessing project health |
| `decompose` | Tasks that should be split | During decomposition |
| `risk-path` | Highest cumulative risk path | Prioritizing attention |
| `workflow-cost` | Relative cost comparison | Comparing approaches |

## Example Session

```bash
# Start a new project
taskgraph init project-setup --name "Project Setup" --scope narrow --risk low
taskgraph init core-module --name "Core Module" --scope moderate --risk medium
taskgraph init api-layer --name "API Layer" --scope moderate --risk low
taskgraph init tests --name "Integration Tests" --scope narrow --risk low

# Add dependencies (edit files directly)
# core-module: depends_on: [project-setup]
# api-layer: depends_on: [core-module]
# tests: depends_on: [api-layer]

# Validate and analyze
taskgraph validate
taskgraph cycles
taskgraph topo

# Assess risk
taskgraph risk
taskgraph decompose

# Visualize
taskgraph graph | dot -Tpng -o graph.png

# Start implementation
taskgraph topo --status pending
# ... edit task files to update status ...
```

## Task Metadata

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier |
| `title` | string | Human-readable name |

### Recommended Fields

| Field | Type | Values | Description |
|-------|------|--------|-------------|
| `status` | enum | pending, in-progress, completed, failed, blocked | Task state |
| `depends_on` | string[] | Task IDs | Dependencies |
| `scope` | enum | single, narrow, moderate, broad, system | Size/complexity |
| `risk` | enum | trivial, low, medium, high, critical | Failure likelihood |
| `impact` | enum | isolated, component, phase, project | Failure consequence |
| `level` | enum | planning, decomposition, implementation, review, research | Task type |
| `tags` | string[] | Arbitrary | Filtering |

## Integration with Development Process

### With Git

Task files are markdown → natural version control:

```bash
git add tasks/
git commit -m "Add authentication tasks"
```

### With CI/CD

```bash
# In CI pipeline
taskgraph validate
taskgraph cycles
taskgraph topo --status pending  # Check for blocked work
```

### With LLM Agents

Agents can:
- Read task files directly (markdown is LLM-friendly)
- Edit task files to update status
- Run TaskGraph commands via bash
- Use graph analysis to plan work

## Further Reading

- [framework.md](./framework.md) - Why categorical estimates, cost-benefit analysis
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Technical architecture
- [research/cost_benefit_analysis_framework.py](./research/cost_benefit_analysis_framework.py) - Full framework implementation