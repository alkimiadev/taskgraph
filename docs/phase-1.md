# Phase 1: Core CLI & Validation

## Objective

Establish the foundation: task file parsing, validation, and basic discovery. Skip full CRUD - files are the source of truth, edited directly.

## Status: ✅ COMPLETE

## Philosophy

> "Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away."

The CLI adds value where manual methods fail (graph analysis). CRUD is just file operations - use your editor.

## Tasks

### 1.1 Project Setup ✅
- [x] Cargo.toml with dependencies
- [x] Dual MIT/Apache-2.0 license
- [x] AGENTS.md with coding guidelines
- [x] Architecture document

### 1.2 Task Parsing ✅
- [x] Implement `Task::from_file()` with gray_matter
- [x] Implement `Task::to_markdown()` for serialization (used internally)
- [x] Handle missing/invalid frontmatter gracefully
- [x] Add tests for parsing edge cases

### 1.3 Task Discovery ✅
- [x] Scan directory for `*.md` files (recursive? configurable?)
- [x] Filter files with valid frontmatter containing `id` field
- [x] Error on duplicate IDs
- [x] Default task directory: `./tasks`
- [x] Configurable via `--path`

### 1.4 Commands ✅

#### `init <id>` (scaffolding helper) ✅
- [x] Create template task file with given ID
- [x] Optional flags: `--name`, `--scope`, `--risk`
- [x] User fills in body manually
- [x] Not required - just convenience

#### `validate` ✅
- [x] Check all files parse correctly
- [x] Check for duplicate IDs
- [x] Check for missing dependencies (referenced but don't exist)
- [x] Output: list of issues or "OK"

#### `list` ✅
- [x] List all tasks with basic info
- [x] Filter by status, tag
- [x] Output: plain text table, JSON

#### `show <id>` ✅
- [x] Display single task (frontmatter + body)
- [x] Output: formatted plain text, JSON

### 1.5 Output Formatting ✅
- [x] Plain text (default)
- [ ] JSON (`--json` flag) - not implemented
- [x] Consistent error messages

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

- [x] Parse valid task file
- [x] Parse task with missing frontmatter
- [x] Parse task with invalid YAML
- [x] Parse task with missing `id` field
- [x] Detect duplicate IDs
- [x] Detect missing dependencies
- [x] List with filters
- [x] `init` creates valid template

## Success Criteria

- [x] Can parse all valid task files
- [x] Validates integrity of task set
- [x] Clear error messages
- [x] Tests pass