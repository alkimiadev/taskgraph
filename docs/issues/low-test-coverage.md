# Issue: Low Test Coverage

## Context
- Phase: 4
- Task: Test coverage improvement
- Date: 2026-03-23

## Problem

Test coverage was 20.89%, well below the 80% target.

**Initial coverage breakdown:**

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

Only unit tests existed. No integration tests exercising CLI command paths.

## Resolution

**Progress (2026-03-23):**

1. Added 16 graph module unit tests
2. Added 6 cache module unit tests  
3. Added 10 embedding module tests (including roundtrip)
4. Added 14 integration tests with `assert_cmd`

**Current coverage: 68.40%** (up from 20.89%)

| Module | Lines | Coverage | Status |
|--------|-------|----------|--------|
| `graph.rs` | 379 | 98.42% | ✅ |
| `cache.rs` | 57 | 100.00% | ✅ |
| `embedding.rs` | 329 | 87.23% | ✅ |
| `discovery.rs` | 160 | 85.62% | ✅ |
| `cli.rs` | 75 | 61.33% | ⚠️ |
| `task.rs` | 104 | 37.50% | ⚠️ |
| `commands/*.rs` | ~400 | ~60% | ⚠️ Mixed |

## Remaining Work

- [ ] Add more integration tests for edge cases
- [ ] Improve task.rs coverage (more Task parsing tests)
- [ ] Add semantic feature integration tests (requires model)