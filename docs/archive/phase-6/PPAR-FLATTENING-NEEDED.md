# PPar Flattening - Theory Enhancement Needed

## Problem Statement

7 ambient calculus tests are failing because nested parallel compositions aren't being automatically flattened:

```
Input:   {{}, p, {{}, q}}
Expected: {p, q}
Actual:   {{}, {{}, q}, p}  (no flattening occurs)
```

## Root Cause

This is **not a bug** in the equation implementation. The issue is that the theory definition is missing equations to express the **associative** and **unit** properties of parallel composition (`PPar`).

## Current Equations

```rust
equations {
    // MISSING: (PPar {P}) == P;  // Commented out
    if x # C then (PPar {P, (PNew x C)}) == (PNew x (PPar {P, C}));
    if x # N then (PNew x (PPar {P, (PIn N Q)})) == (PPar {P, (PIn N (PNew x Q))});
    // ... other scope extrusion equations
}
```

The zero identity equation `(PPar {P, PZero}) == P` was removed because `PZero` was replaced with `{}` (empty parallel composition).

## Required Equations

To properly handle parallel composition, we need:

### 1. Unit Law (Identity Element)
```rust
(PPar {P}) == P;  // Single-element bag equals the element itself
```

Or equivalently with empty:
```rust
(PPar {P, (PPar {})}) == P;  // Empty parallel is the identity
```

### 2. Associativity / Flattening
```rust
// Flatten nested bags - this is the key missing piece!
(PPar {P, (PPar {...rest})}) == (PPar {P, ...rest});
```

This says: a parallel composition containing another parallel composition should be flattened.

### Challenge with Current Syntax

The current equation syntax doesn't support:
1. Destructuring a nested `PPar` in the pattern
2. Splicing its contents into the parent `PPar`

This would require something like:
```rust
// Hypothetical syntax:
if {Q, ...inner} = nested_par then
    (PPar {P, nested_par, ...outer}) == (PPar {P, Q, ...inner, ...outer})
```

## Workarounds

### Option 1: Extend Equation Syntax

Add support for nested collection pattern matching:

```rust
equations {
    // Match nested PPar and flatten
    (PPar {P, (PPar {Q, ...inner}), ...outer}) == (PPar {P, Q, ...inner, ...outer});
}
```

This would require:
- Extending the equation parser to handle nested collection constructors
- Updating `generate_equation_clause` to handle nested patterns
- Generating appropriate Ascent code to destructure nested bags

### Option 2: Built-in PPar Normalization

Add special handling for `PPar` in the code generator to automatically:
1. Recognize `HashBag` collections as associative
2. Generate flattening rules automatically
3. Treat `{}` (empty bag) as the unit element

```rust
// In ascent_gen.rs, for each collection constructor marked as `associative`:
eq_proc(flat, nested) <--
    proc(nested),
    if let Proc::PPar(bag) = nested,
    // ... flatten any nested PPar elements in bag ...
    let flat = Proc::PPar(flattened_bag);
```

### Option 3: Rewrite Rules (Current Approach)

Instead of equations, use congruence rewrites:

```rust
rewrites {
    // Flatten nested parallel
    (PPar {P, (PPar {...rest}), ...outer}) => (PPar {P, ...rest, ...outer});

    // Remove empty parallel
    (PPar {P, (PPar {}), ...rest}) => (PPar {P, ...rest});

    // Single element unwrapping
    (PPar {P}) => P;
}
```

**Problem**: These are **directed** rewrites, not equations. They establish a reduction order rather than equivalence. This could interfere with equation-based reasoning.

## Recommended Solution

**Option 1** (Extend Equation Syntax) is the most principled approach:

1. Add support for nested collection patterns in equations
2. Generate appropriate flattening clauses in Ascent
3. Preserve the declarative, equation-based semantics

This maintains the distinction between:
- **Equations**: establish equivalence (bidirectional, no ordering)
- **Rewrites**: establish reduction (directional, with ordering)

## Implementation Steps

If pursuing Option 1:

1. **Parser**: Extend equation LHS/RHS to allow nested `CollectionPattern`
2. **Type Checker**: Validate that nested patterns are well-typed
3. **Pattern Generator**: Update `generate_equation_pattern_via_rewrite_logic` to handle nesting
4. **RHS Generator**: Update `generate_equation_rhs_simple` to handle nested reconstruction
5. **Tests**: Add equation tests for associativity and flattening

## Impact on Current Tests

Once PPar flattening is properly supported, the 7 failing tests should all pass:

1. `equation_then_rewrite_extrusion_in` - needs flattening after extrusion
2. `equation_zero_then_rewrite` - needs `{}` elimination
3. `extrusion_enables_out` - nested bags after extrusion
4. `parallel_with_extrusion` - nested structure flattening
5. `zero_in_multiple_contexts` - multiple `{}` to eliminate
6. `extrusion_amb_then_open` - nested after equation application
7. `open_after_amb_extrusion` - similar nested structure

## Conclusion

The equation system implementation is **correct and complete**. The failing tests expose a **theory-level gap**: missing equations for the algebraic properties of parallel composition (associativity and unit).

This requires either:
- Extending the equation language to express these properties, OR
- Adding built-in support for associative collections

This is a **feature enhancement**, not a bug fix. The current system correctly implements what is specified in the theory.

