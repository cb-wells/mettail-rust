# Known Limitations & TODOs

**Last Updated**: November 10, 2025

---

## ✅ Recently Fixed

### Automatic Collection Flattening (FIXED ✅)

**Status**: ✅ COMPLETE  
**Date Fixed**: November 10, 2025  

**Problem**: Nested collections required complex flattening equations with rest patterns.

**Solution**: Implemented automatic flattening via generated helper functions. Collections now flatten automatically during construction - no equations needed!

**Details**: See `docs/AUTO-FLATTEN-COMPLETE.md`

---

### Performance Issue (FIXED ✅)

**Status**: ✅ COMPLETE  
**Date Fixed**: November 9, 2025  
**Speedup**: 42x (18.5s → 435ms)

**Problem**: Eager collection deconstruction caused exponential fact explosion.

**Solution**: Disabled eager deconstruction - indexed projection already iterates on-demand.

**Details**: See `docs/PERFORMANCE-FIX-COMPLETE.md`

---

## 🔴 Critical (Blocking Correctness)

### NONE! 🎉

All critical correctness issues have been resolved via automatic collection flattening.

---

## ⚠️ Medium Priority (Functionality Gaps)

### 1. Term Generation Skips Collections

**Status**: Documented  
**Priority**: MEDIUM  
**Timeline**: Q1 2026

**Problem**: Collection constructors are skipped in exhaustive and random term generation.

**Impact**: Cannot auto-generate test cases with collections.

**Details**: See `docs/TODO-TERM-GENERATION.md`

---

### 2. Deep Projection Not Implemented

**Status**: Designed  
**Priority**: MEDIUM  
**Timeline**: Q1 2026

**Problem**: Indexed projection only works for top-level shared variables in collection patterns.

**Impact**: Deeply nested shared variables fall back to order-dependent matching.

**Example**:
```rust
// Doesn't work optimally:
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R), ...rest})
//                         ^                 ^
//                    nested            top-level
```

**Details**: See `docs/design/DEEP-PROJECTION-DESIGN.md`

---

### 3. Ascent Parallel Mode Incompatible

**Status**: Investigated  
**Priority**: LOW  
**Timeline**: Q1 2026 (after benchmarking)

**Problem**: `ascent_run_par!` expects different reference levels than serial mode.

**Impact**: Cannot use parallel execution for performance.

**Workaround**: Use `ascent_run!` (serial mode) which is fast enough for now.

**Details**: See `docs/ASCENT-PARALLEL-ISSUE.md`

---

## 📝 Low Priority (Quality of Life)

### 4. Compiler Warnings

**Status**: Known  
**Priority**: LOW

**Warnings**:
- Unused imports in `mettail-macros/src`
- Unused variables in generated code
- Non-snake_case warnings from `moniker` derives

**Impact**: None (compilation succeeds)

**Fix**: Clean up when convenient

---

### 5. Nested Constructor Patterns in Collections (Equations)

**Status**: Not implemented  
**Priority**: LOW

**Problem**: Equations can't match nested constructors in collections:
```rust
// This won't work:
(PPar {(PInput chan x P), Q}) == something
```

**Workaround**: Use rewrites for complex patterns, equations for simple ones.

---

## ✅ Fixed / Not Issues

### Empty Collections
- Status: ✅ Handled correctly
- `(PPar {}) == PZero` works

### Single-Element Collections  
- Status: ✅ Handled correctly
- `(PPar {P}) == P` works

### Order-Independent Matching (Flat)
- Status: ✅ Working perfectly
- Indexed projection handles 2-3 element collections optimally

### Nested Collection Flattening
- Status: ✅ Automatic!
- No equations needed - handled during construction

---

## Priority Order for Fixes

1. **Deep projection** (important for complex patterns)
2. **Term generation** (useful for testing)
3. **Parallel support** (performance optimization)
4. **Compiler warnings** (code quality)

---

## How to Contribute Fixes

See individual TODO/design docs for details on each issue. Most have detailed analysis and proposed solutions.

