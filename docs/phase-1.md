# Phase 1: Core CLI & Validation

## Objective

Establish the foundation: task file parsing, validation, and basic discovery. Skip full CRUD - files are the source of truth, edited directly.

## Philosophy

> "Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away."

The CLI adds value where manual methods fail (graph analysis). CRUD is just file operations - use your editor.

## Tasks

### 1.1 Project Setup ✅
- [x] Cargo.toml with dependencies
- [x] Dual MIT/Apache-2.0 license
- [x] AGENTS.md with coding guidelines
- [x] Architecture document

### 1.2 Task Parsing
- [ ] Implement `Task::from_file()` with gray_matter
- [ ] Implement `Task::to_markdown()` for serialization (used internally)
- [ ] Handle missing/invalid frontmatter gracefully
- [ ] Add tests for parsing edge cases

### 1.3 Task Discovery
- [ ] Scan directory for `*.md` files (recursive? configurable?)
- [ ] Filter files with valid frontmatter containing `id` field
- [ ] Error on duplicate IDs
- [ ] Default task directory: `./tasks`
- [ ] Configurable via `--path`

### 1.4 Commands

#### `init <id>` (scaffolding helper)
- [ ] Create template task file with given ID
- [ ] Optional flags: `--name`, `--scope`, `--risk`, `--impact`, `--level`
- [ ] User fills in body manually
- [ ] Not required - just convenience

#### `validate`
- [ ] Check all files parse correctly
- [ ] Check for duplicate IDs
- [ ] Check for missing dependencies (referenced but don't exist)
- [ ] Output: list of issues or "OK"

#### `list`
- [ ] List all tasks with basic info
- [ ] Filter by status, tag, risk, scope
- [ ] Output: plain text table, JSON

#### `show <id>`
- [ ] Display single task (frontmatter + body)
- [ ] Output: formatted plain text, JSON

### 1.5 Output Formatting
- [ ] Plain text (default)
- [ ] JSON (`--json` flag)
- [ ] Consistent error messages

## What We're NOT Building

| Operation | How to do it |
|-----------|--------------|
| Create task | Write a file |
| Edit task | Edit the file |
| Delete task | Delete the file |
| Rename task | Rename file, update `id` field |

These are file operations. The CLI doesn't add value here.

## Assumption Points (Resolved)

| Question | Decision |
|----------|----------|
| Task directory | Default `./tasks`, `--path` override |
| File naming | Any `*.md` with `id` in frontmatter |
| Duplicate IDs | Error - fail fast |
| Recursive scan | Yes, but configurable |

## Dependencies

| Crate | Purpose |
|-------|---------|
| `gray_matter` | Frontmatter extraction |
| `serde` + `serde_yaml` | Serialization |
| `clap` | CLI parsing |
| `chrono` | Timestamps |
| `anyhow` | Error handling |
| `walkdir` | Directory scanning |

### Reference Sources (after cargo build)

```
~/.cargo/registry/src/*/gray_matter-*/   - frontmatter parsing implementation
~/.cargo/registry/src/*/serde_yaml-*/    - YAML parsing details
~/.cargo/registry/src/*/walkdir-*/       - directory traversal patterns
```

## Tests Required

- Parse valid task file
- Parse task with missing frontmatter
- Parse task with invalid YAML
- Parse task with missing `id` field
- Detect duplicate IDs
- Detect missing dependencies
- List with filters
- `init` creates valid template

## Success Criteria

- Can parse all valid task files
- Validates integrity of task set
- Clear error messages
- Tests pass