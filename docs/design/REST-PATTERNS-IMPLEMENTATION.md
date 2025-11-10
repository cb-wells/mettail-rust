# Implementing Rest Patterns in Equations

**Date**: November 9, 2025  
**Priority**: HIGH - Blocking correctness  
**Complexity**: Medium (1-2 days)

---

## Problem

Equations with rest patterns in the LHS are silently skipped:

```rust
// This is NOT generated:
(PPar {P, (PPar {Q, ...rest})}) == (PPar {P, Q, ...rest});
```

Because of lines 979-983 in `ascent_gen.rs`:
```rust
if rest.is_some() {
    return None;  // Skip!
}
```

---

## What Needs to Work

### Example 1: Flattening
```rust
(PPar {P, (PPar {Q, ...rest})}) == (PPar {P, Q, ...rest});
```

**Semantics**:
- Match a collection with at least 2 elements
- One element is `P` (any term)
- Another element is a nested `PPar` containing `Q` and `...rest`
- RHS: Create new collection with `P`, `Q`, and all of `rest` merged to top level

### Example 2: Identity with Rest
```rust
(PPar {P, ...rest}) == (PPar {rest, P});  // Commutativity (if we need it)
```

### Example 3: Nested Pattern with Rest
```rust
(PPar {(PInput chan x P), ...rest}) == ...
```

---

## Design

### Phase 1: Pattern Generation

**In `generate_equation_pattern` (lines 975-1045)**:

Current code for `CollectionPattern`:
```rust
if rest.is_some() {
    return None;  // ← REMOVE THIS
}
```

**New logic**:

1. **Extract elements** (same as current for non-rest)
2. **Bind rest variable**:
   ```rust
   let rest_var = format_ident!("rest_{}", rest_var_name);
   let rest_binding = quote! {
       let #rest_var = {
           let mut rest_bag = #bag_var.clone();
           #(rest_bag.remove(&#element_vars);)*  // Remove matched elements
           rest_bag
       };
   };
   ```

3. **Return pattern with rest binding**

**Example output** for `{P, (PPar {Q, ...rest})}`:
```rust
if let Proc::PPar(ref bag) = p0,
if bag.len() >= 2,  // At least P and nested PPar
let p = bag.iter().next().unwrap().0.clone(),
// Now need to match another element that's a PPar
let nested = /* extract PPar from bag */, 
if let Proc::PPar(ref inner_bag) = nested,
let q = inner_bag.iter().next().unwrap().0.clone(),
let rest = {
    let mut r = inner_bag.clone();
    r.remove(&q);
    r
},
// Bind outer rest too
let outer_rest = {
    let mut r = bag.clone();
    r.remove(&p);
    r.remove(&nested);
    r
};
```

**Challenge**: How to "extract a PPar from bag"?

---

### Phase 2: Handling Nested Constructor Patterns

**Problem**: `(PPar {Q, ...rest})` is a constructor pattern, not just a variable.

**Current approach for nested constructors** (lines 998-1039):
- Only handles single-element collections with one variable
- Doesn't handle nested constructors with rest

**New approach needed**:

1. **Iterate over collection** to find matching constructor
2. **Try each element** against the pattern
3. **Bind if match succeeds**

**Pseudo-code**:
```rust
let nested_var = None;
for elem in bag.iter() {
    if let Proc::PPar(inner) = elem.0 {
        nested_var = Some((elem.0.clone(), inner));
        break;
    }
}
let (nested, inner_bag) = nested_var?;
```

**In Ascent syntax**:
```rust
// This is tricky - Ascent doesn't have "find first that matches"
// We might need multiple clauses or a different approach
```

**Alternative**: Use separate relation for matching

```rust
// Helper relation: find PPar in a collection
relation ppar_in_collection(Proc, Proc);  // (parent_bag, found_ppar)

ppar_in_collection(parent, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (elem, _) in bag.iter(),
    if let Proc::PPar(_) = elem;

// Then in equation:
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc::PPar(bag) = p0,
    ppar_in_collection(p0, nested),
    if let Proc::PPar(inner_bag) = nested,
    // Extract Q and rest from inner_bag
    let q = inner_bag.iter().next().unwrap().0.clone(),
    let inner_rest = { /* ... */ },
    // Build RHS
    let p1 = Proc::PPar({
        let mut new_bag = bag.clone();
        new_bag.remove(&nested);  // Remove the nested PPar
        // Add Q
        new_bag.insert(q.clone());
        // Add all from inner_rest
        for (e, count) in inner_rest.iter() {
            for _ in 0..*count {
                new_bag.insert(e.clone());
            }
        }
        new_bag
    });
```

**Problem with this approach**:
- Requires helper relations
- More complex code generation
- But it works!

---

### Phase 3: RHS Construction with Rest

**In `generate_equation_rhs` or `generate_collection_equation_rhs`**:

For RHS like `(PPar {P, Q, ...rest})`:

```rust
Proc::PPar({
    let mut bag = HashBag::new();
    bag.insert(p.clone());
    bag.insert(q.clone());
    // Merge rest
    for (elem, count) in rest.iter() {
        for _ in 0..*count {
            bag.insert(elem.clone());
        }
    }
    bag
})
```

**Challenge**: Need to track which variables are bags vs single values

---

## Simplified Approach (Pragmatic)

Instead of full general support, handle the **specific pattern we need**:

```rust
(PPar {P, (PPar nested)}) == (PPar {P, ...nested});
```

Where `nested` is bound as a **bag variable**, and `...nested` means "expand the bag".

**Changes needed**:

1. **New syntax**: `...varname` in RHS means "expand this bag"
2. **Pattern matching**: `(PPar nested)` binds the inner bag
3. **RHS generation**: Detect `...var` and expand

**Example**:
```rust
// User writes:
(PPar {P, (PPar nested)}) == (PPar {P, ...nested});

// We generate:
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc::PPar(bag) = p0,
    if bag.len() >= 2,
    let p = bag.iter().next().unwrap().0.clone(),
    // Find a nested PPar
    let nested_opt = bag.iter()
        .find(|(e, _)| matches!(e, Proc::PPar(_)))
        .map(|(e, _)| e.clone()),
    if let Some(nested_term) = nested_opt,
    if let Proc::PPar(nested_bag) = nested_term,
    // Build RHS
    let p1 = Proc::PPar({
        let mut new_bag = bag.clone();
        new_bag.remove(&nested_term);
        // Merge nested_bag into new_bag
        for (elem, count) in nested_bag.iter() {
            for _ in 0..*count {
                new_bag.insert(elem.clone());
            }
        }
        new_bag
    });
```

---

## Implementation Plan

### Step 1: Add `...var` syntax to AST (1 hour)

In `ast.rs`:
```rust
pub enum Expr {
    // ...
    Spread(String),  // ...varname
}
```

Update parser to recognize `...` prefix.

### Step 2: Update equation pattern generation (3-4 hours)

In `ascent_gen.rs`:
- Remove the `rest.is_some() → None` check
- Add logic to bind rest variables
- Handle nested constructor matching (use helper relation approach)

### Step 3: Update RHS generation (2-3 hours)

In `ascent_gen.rs`:
- Detect `Spread(var)` in RHS
- Generate bag expansion code
- Merge contents into parent bag

### Step 4: Testing (2 hours)

Test cases:
- `(PPar {P}) == P` (already works)
- `(PPar {(PPar nested)}) == nested` (unwrap)
- `(PPar {P, (PPar {Q, R})}) == (PPar {P, Q, R})` (flatten 2-element)
- `(PPar {P, (PPar nested)}) == (PPar {P, ...nested})` (general flatten)

### Step 5: Documentation (1 hour)

Update docs with new equation capabilities.

---

## Alternative: Just Fix the Specific Case

For **immediate unblocking**, hardcode support for the flattening pattern:

In `ascent_gen.rs`, add **before** the `rest.is_some()` check:

```rust
// Special case: flattening equation
if is_flattening_pattern(constructor, elements, rest) {
    return generate_flattening_equation(...);
}
```

Where `is_flattening_pattern` detects:
- 2 elements: `P` and `(PPar nested)`
- Rest pattern in the nested PPar
- RHS is `(PPar {P, ...nested})`

**Time**: 2-3 hours  
**Scope**: Just this one pattern  
**Trade-off**: Not general, but solves the immediate problem

---

## Recommendation

For **next session**:
1. Implement the **alternative** (hardcode flattening support) - 2-3 hours
2. Then tackle **performance** (remove eager deconstruction) - 1 hour
3. Test both together - 30 min

**Total**: ~4 hours to unblock both correctness and performance

For **Q1 2026**:
- Implement general rest pattern support (Steps 1-5 above)
- Full testing and documentation

---

## Files to Modify

1. `mettail-macros/src/ascent_gen.rs`
   - Lines 979-983: Rest pattern handling
   - Add `generate_flattening_equation` function
   - Update `generate_equation_rhs` for spread syntax

2. `mettail-macros/src/ast.rs` (if doing general solution)
   - Add `Expr::Spread` variant

3. `mettail-macros/src/parser_gen.rs` (if doing general solution)
   - Parse `...var` syntax

4. `examples/rhocalc.rs`
   - Keep the flattening equation as-is
   - Test that it now works

---

## Success Criteria

After implementation:
- [ ] Flattening equation generates Ascent code
- [ ] No nested collections in normal forms
- [ ] All 9+ rewrite paths found
- [ ] Execution time < 5 seconds (with performance fix)
- [ ] Tests pass

---

**Next**: Implement the alternative (hardcoded flattening) to unblock immediately, then plan the general solution.

