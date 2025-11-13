# Collection Congruence System - Completion Plan

## Problem Statement

The congruence-driven projection system works correctly for rules where the collection pattern is at the TOP LEVEL (e.g., `(PPar {...}) => RHS`). However, rules with NESTED collection patterns (e.g., `(PAmb M (PPar {...})) => RHS`) still generate old-style indexed projections using `.iter().next()` and `.nth(i)`, making them order-dependent and buggy.

**Symptoms:**
- Ambient `exit` rule doesn't match when elements are in different orders
- Generated code shows `if elem_f0.len() >= 2usize` and `.iter().next()` patterns for nested collections
- RhoCalc rules work (all have top-level PPar), but Ambient rules with nested collections fail

**Root Cause:**
Rules like `(PAmb M (PPar {(PAmb N ...), R})) => RHS` have `PAmb` at the top level, NOT `PPar`. So they are:
1. NOT analyzed by the `PPar` collection congruence (which only looks at rules with `Proc` element type)
2. Processed by the old `generate_rewrite_clause` path
3. That path encounters the nested `PPar` collection and uses indexed matching via `generate_ascent_pattern`

The issue is in `generate_ascent_pattern` - it generates indexed `.iter().next()` code for ANY collection pattern it encounters, regardless of nesting level.

## Current State Analysis

### What Works (RhoCalc)
- Communication rule: `(PPar {(PInput chan x P), (POutput chan Q)})` generates TWO projections
- Drop-quote rule: `(PDrop (NQuote P))` generates ONE projection with nested matching
- Projections use full pattern matching, not indexing
- Variables are extracted by name, not position

### What's Broken (Ambient)
- Rules with collection patterns in LHS are generating BOTH:
  1. New congruence-driven projections (correct)
  2. Old indexed projections (buggy)
- The old path creates order-dependent code
- Example: `(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R)})`

## Architecture Review

### Correct Flow (What Should Happen)
```
User writes: (PPar {(PAmb N ...), (PAmb M R)}) => RHS

1. `extract_collection_congruence_info`: Identifies PPar congruence
2. `find_base_rewrites_for_category`: Finds this rewrite (element type = Proc)
3. `extract_element_patterns_from_base_rewrite`: Extracts TWO patterns:
   - Pattern 0: PAmb N (PPar {...})
   - Pattern 1: PAmb M R
4. `generate_base_rewrite_projection`: For each pattern, generates projection
   using `generate_ascent_pattern` for full pattern matching
5. `generate_joined_base_rewrite_clause`: Joins projections, reconstructs RHS
6. `generate_rewrite_clauses`: SKIPS this rewrite (congruence handles it)
```

### Current Broken Flow (What's Actually Happening)
```
Same as above, BUT ALSO:

7. `generate_rewrite_clauses`: DOES NOT SKIP, generates old-style code:
   - Calls `analyze_collection_pattern`
   - Calls `requires_indexed_projection` 
   - Generates indexed matching with .iter().next()
```

## The Fix

The solution is to make `generate_ascent_pattern` use projection-based matching for collection patterns at ANY nesting level, not just top-level ones.

### Option 1: Generate Inline Projections (Recommended)

When `generate_ascent_pattern` encounters a collection pattern, instead of generating indexed extraction, it should generate projection-style matching:

**Before (indexed, buggy):**
```rust
if elem_f0.len() >= 2usize,
let elem_f0_elem_0 = elem_f0.iter().next().unwrap().0.clone(),
let elem_f0_elem_1 = elem_f0.iter().nth(1usize).unwrap().0.clone(),
```

**After (projection, correct):**
```rust
for (nested_elem_0, _count) in elem_f0.iter(),
if let Proc::PAmb(...) = nested_elem_0,
for (nested_elem_1, _count) in elem_f0.iter(),
if nested_elem_0 != nested_elem_1,  // Different elements
if let Proc::PAmb(...) = nested_elem_1,
```

### Option 2: Lift ALL Collection Patterns to Congruence (Complex)

Alternatively, we could extend the congruence system to analyze rules with nested collections and generate helper projections. This is more complex and may not be necessary.

### Implementation: Modify `generate_ascent_collection_pattern`

**File:** `mettail-macros/src/rewrite_gen.rs`, function `generate_ascent_collection_pattern`

**Current approach** (lines 311-420): Uses `.iter().next()` and `.nth(i)` to extract specific elements.

**New approach**: Generate a loop for EACH element pattern, with distinctness checks:

```rust
fn generate_ascent_collection_pattern(
    constructor: &Option<Ident>,
    elements: &[Expr],
    rest: &Option<Ident>,
    term_name: &Ident,
    expected_category: &Ident,
    theory: &TheoryDef,
    bindings: &mut HashMap<String, TokenStream>,
    variable_categories: &mut HashMap<String, Ident>,
    clauses: &mut Vec<TokenStream>,
    duplicate_vars: &std::collections::HashSet<String>,
    equational_checks: &mut Vec<TokenStream>,
) {
    // ... (constructor and grammar rule lookup - same as before) ...
    
    // Generate loop-based matching for each element
    let mut elem_vars = Vec::new();
    
    for (elem_idx, elem_pattern) in elements.iter().enumerate() {
        let elem_var = quote::format_ident!("{}_elem_{}", term_name, elem_idx);
        elem_vars.push(elem_var.clone());
        
        // Generate: for (elem_var, _count) in bag_field.iter()
        clauses.push(quote! {
            for (#elem_var, _count) in #bag_var.iter()
        });
        
        // Add distinctness checks (ensure we don't match the same element twice)
        for prev_elem_var in &elem_vars[..elem_idx] {
            clauses.push(quote! {
                if &#elem_var != &#prev_elem_var
            });
        }
        
        // Recursively generate pattern for this element
        generate_ascent_pattern(
            elem_pattern,
            &elem_var,
            &element_category,
            theory,
            bindings,
            variable_categories,
            clauses,
            duplicate_vars,
            equational_checks,
        );
    }
    
    // Bind rest variable if present
    if let Some(rest_var) = rest {
        let rest_ident = quote::format_ident!("{}_rest", term_name);
        
        // Build rest by removing matched elements
        clauses.push(quote! {
            let #rest_ident = {
                let mut bag = #bag_var.clone();
                #(bag.remove(&#elem_vars);)*
                bag
            }
        });
        
        bindings.insert(rest_var.to_string(), quote! { #rest_ident });
    }
}
```

This approach:
1. Uses `for (elem, _count) in bag.iter()` for each element (order-independent)
2. Adds `if &elem_i != &elem_j` checks to ensure distinctness
3. Recursively calls `generate_ascent_pattern` for nested matching
4. No `.next()` or `.nth()` calls!

## Complete Scope of Changes

ALL instances of indexed collection matching (`.iter().next()`, `.nth(i)`) must be replaced with loop-based matching across the entire codebase:

### Files and Functions to Fix:

1. **`mettail-macros/src/rewrite_gen.rs`**:
   - `generate_ascent_collection_pattern` (lines ~339-399) - TOP-LEVEL collection patterns
   - `generate_ascent_binder_pattern` (lines ~750-850) - NESTED collection patterns within binder constructors

2. **`mettail-macros/src/ascent_gen.rs`**:
   - `generate_equation_pattern` (lines ~1100-1350) - Collection patterns in EQUATIONS

3. **`mettail-macros/src/congruence_analysis.rs`**:
   - Review: ensure no indexed patterns generated here (likely clean)

### Why This Matters

Every place that generates Ascent Datalog code for matching collection patterns must use the loop-based approach:

```rust
// BAD (order-dependent):
let elem_0 = bag.iter().next().unwrap().0.clone()
let elem_1 = bag.iter().nth(1).unwrap().0.clone()

// GOOD (order-independent):
for (elem_0, _count) in bag.iter()
for (elem_1, _count) in bag.iter()
if &elem_0 != &elem_1  // distinctness check
```

### Test 1: Ambient Enter Rule (Order Independence)
```rust
// Both should work:
"{n[{in(m,p), q}], m[r], c}"  // n first
"{m[r], n[{in(m,p), q}], c}"  // m first

// Should generate: m[{n[{p, q}], r}]
```

### Test 2: Ambient Exit Rule
```rust
"{m[{n[{out(m,p), q}], r}], c}"
// Should generate: {n[{p, q}], m[r], c}
```

### Test 3: Verify No Indexed Patterns in Generated Code
Search generated code for:
- `.iter().next()` - should ONLY appear in collection pattern matching, never in projections
- `.nth(` - should NOT appear anywhere
- `if.*len.*>=.*usize` - should NOT appear (this was the old heuristic)

### Test 4: RhoCalc Still Works
Ensure RhoCalc tests still pass - no regressions.

## Implementation Steps

1. **Add helper functions** to `rewrite_gen.rs`:
   - `should_skip_for_congruence`
   - `get_collection_element_category` 
   - Verify `contains_collection_pattern`

2. **Update `generate_rewrite_clauses`**:
   - Replace skip condition with `should_skip_for_congruence`
   - Ensure it runs BEFORE any other rewrite generation logic

3. **Search and destroy** any remaining indexed projection code:
   - Search for `.iter().next()` in generated output
   - Search for `.nth(` in `rewrite_gen.rs`
   - Remove `requires_indexed_projection` if it exists

4. **Test thoroughly**:
   - Run ambient examples with different element orders
   - Verify generated code structure
   - Ensure no regressions in RhoCalc

## Success Criteria

✅ Ambient rules work regardless of element order
✅ No `.iter().next()` or `.nth()` in projection code
✅ All collection rewrites handled by congruence system
✅ RhoCalc examples still pass
✅ Generated code is clean and maintainable

## Files to Modify

1. `mettail-macros/src/rewrite_gen.rs` - Primary changes
2. `mettail-macros/src/congruence_analysis.rs` - May need to expose helpers
3. Test files - Add order-independence tests

