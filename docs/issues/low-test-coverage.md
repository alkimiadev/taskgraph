# Issue: Low Test Coverage

## Context
- Phase: 4
- Task: Test coverage improvement
- Date: 2026-03-23

## Problem

Test coverage is 20.89%, well below the 80% target.

**Coverage breakdown:**

| Module | Lines | Coverage | Issue |
|--------|-------|----------|-------|
| `discovery.rs` | 160 | 83.75% | ✅ Acceptable |
| `embedding.rs` | 279 | 45.88% | ⚠️ Missing save/load tests |
| `task.rs` | 104 | 26.92% | ⚠️ Indirect coverage only |
| `graph.rs` | 228 | 0.00% | ❌ No tests |
| `cache.rs` | 26 | 0.00% | ❌ No tests |
| `commands/*.rs` | ~500 | 0.00% | ❌ No integration tests |
| `cli.rs` | 75 | 0.00% | ❌ No integration tests |

## Root Cause

Only unit tests exist. No integration tests exercising CLI command paths.

## Suggested Resolution

1. Add integration tests with `assert_cmd` (already in dev-dependencies)
2. Add graph module unit tests
3. Add embedding roundtrip tests
4. Add cache tests

## Priority

High - Blocking Phase 5 release readiness.