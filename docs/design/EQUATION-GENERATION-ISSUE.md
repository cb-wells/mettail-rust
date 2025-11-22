# Equation Generation Issue - Summary

**Date**: November 19, 2025  
**Severity**: 🚨 CRITICAL - Semantic Correctness  
**Status**: Discovered, Plan Created

---

## TL;DR

**All user-defined equations are silently failing to generate.** Tests pass because they only check rewrites, not equational equivalence. This is a **correctness issue**, not just a missing feature.

**Ambient Calculus**: 0/6 equations working
- Zero identity (`P == {P, 0}`) - Not enforced
- Scope extrusion (moving `new` in/out) - Not working

---

## The Problem

### What We Discovered

While reviewing equation generation for the ambient calculus test plan, we found that **NONE of the 6 equations are being generated**:

```
=== GENERATING EQUATION CLAUSES ===
Total equations to process: 6

Equation 0: P == {P, 0}
  ❌ Failed to generate (returned None)

Equation 1: {P, (PNew x C)} == (PNew x {P, C})
  ❌ Failed to generate (returned None)

... (4 more failures)
```

### Root Causes

**File**: `mettail-macros/src/ascent_gen.rs`, function `generate_equation_pattern`

1. **Bare Variable LHS** (Line 1329-1342)
   ```rust
   P == {P, 0}  // Can't pattern match on bare variable!
   ```
   - Returns `None` for `Expr::Var` that isn't a constructor
   - No Datalog clause can be generated without a pattern

2. **Complex Patterns in Collections** (Line 1507-1510)
   ```rust
   {P, (PNew x C)} == ...  // Apply pattern inside collection
   ```
   - Only simple variables supported: `match elem { Expr::Var(...) => ..., _ => None }`
   - ALL 6 ambient equations have complex patterns: `(PNew ...)`, `(PIn ...)`, `(POut ...)`, etc.

3. **Rest Patterns Not Supported** (Line 1458-1462)
   - Not relevant for ambient (no rest in equations)
   - But still a limitation for future use

### Why Tests Pass Anyway

- Rewrite rules (`entry`, `exit`, `open`) work independently
- Congruences work independently  
- Tests only check: "Can we rewrite A to B?"
- Tests don't check: "Are A and B equivalent?"
- **False sense of completeness!**

---

## The Impact

### Semantic Incorrectness

**Zero Identity Not Enforced**:
```rust
{a[0], 0}  // Should equal a[0], but doesn't!
```

**Scope Extrusion Not Working**:
```rust
{a[0], new(x, b[x])}  // Should equal new(x, {a[0], b[x]}), but doesn't!
```

### What's Still Working

✅ **Structural Congruence**: Constructors have congruence equations
- `(PIn N1 P1) == (PIn N2 P2)` if `N1 == N2` and `P1 == P2`
- Generated automatically for all constructors

✅ **Rewrites**: Base rewrite rules fire correctly
- Entry, exit, open all work
- All 17 ambient tests pass

❌ **User-Defined Equations**: Completely missing
- Zero identity
- Scope extrusion
- Any theory-specific equivalences

---

## The Solution

**Comprehensive implementation plan**: [EQUATION-IMPLEMENTATION-PLAN.md](EQUATION-IMPLEMENTATION-PLAN.md)

### Quick Fixes (Week 1)

**Fix 1: Auto-Flip Bare Variable LHS**
```rust
// User writes:  P == {P, 0}
// Generate as: {P, 0} == P   (flip LHS/RHS)
```
- 5 lines of code
- Symmetry of equality makes this valid

**Fix 2: Support Complex Patterns in Collections**
```rust
// For: {P, (PNew x C)}
// Generate nested if-let patterns (same as rewrite rules):
for (elem_1, _) in bag.iter(),
if let Proc::PNew(ref scope_1) = elem_1,
let x = scope_1.inner().unsafe_pattern.clone(),
let c = scope_1.inner().unsafe_body.as_ref().clone(),
```
- Reuse existing pattern matching logic
- Handle binders with `unsafe_pattern`/`unsafe_body`

### Full Implementation (2 Weeks)

1. **Week 1**: Core pattern matching + RHS construction
2. **Week 2**: Freshness conditions + comprehensive testing

---

## Testing Strategy

### Verify Equation Generation

```bash
cargo build --bin ambient 2>&1 | grep "✅ Generated"
# Should see: ✅ Generated successfully (x6)
```

### Test Semantic Equivalence

```rust
#[test]
fn test_zero_identity() {
    let p = parse("a[0]");
    let p_with_zero = parse("{a[0], 0}");
    
    let prog = run_ascent();
    
    // Check they're in same equivalence class
    assert!(prog.are_equivalent(&p, &p_with_zero),
        "Zero identity should hold");
}
```

---

## Priority

**HIGH - Correctness Issue**

This is not a "nice to have" feature - it's a fundamental correctness problem. The semantic meaning of the language is incomplete without equations.

**Start Phase 1 immediately** after user approval.

---

## References

- **Implementation Plan**: [EQUATION-IMPLEMENTATION-PLAN.md](EQUATION-IMPLEMENTATION-PLAN.md)
- **Test Plan**: [AMBIENT-TEST-PLAN.md](AMBIENT-TEST-PLAN.md) (updated with blockers)
- **Code Location**: `mettail-macros/src/ascent_gen.rs` lines 1321-1521
- **Debug Output**: Added temporarily to show equation generation failures

---

**Bottom Line**: We have excellent rewrite rule support, but equation support is broken. This needs to be fixed for semantic correctness.

