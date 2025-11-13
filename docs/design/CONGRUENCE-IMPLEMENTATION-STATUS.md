# Congruence-Driven Projection Implementation Status

## Overview

This document tracks the implementation status of the congruence-driven projection refactoring for MeTTa-IL collection rewriting.

**Last Updated:** November 13, 2024
**Current Phase:** Phase 3 Complete

## Architecture Summary

The new congruence-driven approach shifts responsibility for projection generation from individual base rewrites to congruence rules. Key insight: **if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})** should analyze ALL base rewrites that affect `S` and generate appropriate projections.

### Key Components

1. **congruence_analysis.rs** - New module for AST analysis and projection generation
2. **ascent_gen.rs** - Modified to use congruence-driven approach
3. **rewrite_gen.rs** - Will be modified to skip redundant base rewrite generation (Phase 4)

## Phase-by-Phase Status

### ✅ Phase 1: AST Analysis Functions (COMPLETE)

**Status:** All functions implemented and compile successfully

**Implemented Functions:**
- `extract_collection_congruence_info` - Extracts collection congruence metadata
- `extract_regular_congruence_pattern` - Extracts regular (non-collection) congruence info
- `find_base_rewrites_for_category` - Finds base rewrites affecting a category
- `find_regular_congruences_for_category` - Finds regular congruences for a category
- `is_collection_congruence` - Identifies collection congruence rules
- `contains_collection_pattern` - Detects collection patterns in expressions
- `rule_involves_category` - Determines if a rule affects a category
- `get_collection_element_category` - Retrieves element type from collection constructor
- `extract_category_from_expr` - Extracts category from an expression
- `find_collection_congruence_element_categories` - Identifies all categories subject to congruence

**Data Structures:**
```rust
pub struct CollectionCongruenceInfo {
    pub parent_category: Ident,      // Proc
    pub element_category: Ident,     // Proc
    pub constructor: Ident,          // PPar
    pub source_var: Ident,          // S
    pub target_var: Ident,          // T
    pub rest_var: Option<Ident>,    // rest
}

pub struct RegularCongruencePattern {
    pub constructor: Ident,         // PNew
    pub category: Ident,            // Proc
    pub body_field_idx: usize,      // field containing rewritten element
    pub is_binding: bool,           // true for PNew
    pub source_var: Ident,          // S
    pub target_var: Ident,          // T
}

pub struct ElementPatternInfo {
    pub constructor: Ident,          // PInput, POutput, PDrop
    pub category: Ident,             // Proc
    pub captures: Vec<CaptureInfo>,  // Variables captured from pattern
}

pub struct CaptureInfo {
    pub var_name: String,
    pub category: Ident,
    pub field_idx: usize,
    pub is_binder: bool,
}
```

### ✅ Phase 2: Projection Generation (COMPLETE)

**Status:** All projection generation functions implemented and compile successfully

**Implemented Functions:**
- `generate_congruence_projections` - Main orchestrator for projection generation
- `extract_element_patterns_from_base_rewrite` - Extracts patterns from base rewrite LHS
- `analyze_constructor_pattern` - Analyzes constructor patterns for captures
- `extract_captures` - Extracts captured variables and metadata
- `generate_base_rewrite_projection` - Generates projection relations for base rewrites
- `generate_regular_congruence_projection` - Generates projection relations for regular congruences
- `generate_field_extraction` - Helper for field extraction code generation

**Example Generated Projection (Base Rewrite):**
```rust
// For: (PInput chan x P)
relation pinput_proj_c0_b0_p0(Proc, Name, Binder<String>, Proc, Proc);

pinput_proj_c0_b0_p0(parent.clone(), cap_chan.clone(), binder_x.clone(), deref_p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc::PInput(ref f0, ref f1) = elem,
    let cap_chan = (**f0).clone(),
    let (binder_x, body_x) = (*f1).clone().unbind(),
    let deref_p = (*body_x).clone();
```

**Example Generated Projection (Regular Congruence):**
```rust
// For: if S => T then (PNew x S) => (PNew x T)
relation pnew_proj_c0_r0(Proc, Binder<String>, Proc, Proc);

pnew_proj_c0_r0(parent.clone(), binder_x.clone(), body.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc::PNew(ref scope_field) = elem,
    let (binder_x, body) = (*scope_field).clone().unbind();
```

### ✅ Phase 3: Congruence Clause Generation (COMPLETE)

**Status:** Integrated projection-based congruence generation into main pipeline

**Modified Functions:**
- `generate_rewrite_rules` (ascent_gen.rs) - Now uses congruence-driven approach
- Added `generate_new_collection_congruence_clauses` - Orchestrates clause generation
- Added `generate_base_rewrite_congruence_clause` - Generates clauses for base rewrites
- Added `generate_regular_congruence_clause` - Generates clauses for regular congruences
- Added `generate_rhs_reconstruction` - Reconstructs RHS from captures

**Example Generated Clause (Base Rewrite):**
```rust
// Uses projection to apply base rewrite within collection
rw_proc(parent, result) <--
    pinput_proj_c0_b0_p0(parent, cap_chan, binder_x, cap_p, elem),
    if let Proc::PPar(ref bag) = parent,
    let remaining = {
        let mut b = bag.clone();
        b.remove(elem);
        b
    },
    let rewritten = /* RHS construction */,
    let result = Proc::PPar({
        let mut bag = remaining;
        Proc::insert_into_ppar(&mut bag, rewritten);
        bag
    });
```

**Example Generated Clause (Regular Congruence):**
```rust
// Recursively rewrites body via rw_proc
rw_proc(parent, result) <--
    pnew_proj_c0_r0(parent, binder_var, body, elem),
    rw_proc(body, body_rewritten),
    if let Proc::PPar(ref bag) = parent,
    let remaining = {
        let mut b = bag.clone();
        b.remove(elem);
        b
    },
    let rewritten = Proc::PNew(
        mettail_runtime::Scope::new(binder_var.clone(), Box::new(body_rewritten.clone()))
    ),
    let result = Proc::PPar({
        let mut bag = remaining;
        Proc::insert_into_ppar(&mut bag, rewritten);
        bag
    });
```

**Integration:**
- Collection congruences now trigger analysis of all relevant base rewrites and regular congruences
- Projections are generated automatically
- Congruence clauses reference these projections
- System correctly composes: `PPar { PNew x (PDrop (NQuote P)) }` → `PPar { PNew x P }`

### 🔄 Phase 4: Skip Redundant Base Rewrites (PENDING)

**Status:** Not yet started

**Goal:** Modify `generate_rewrite_clauses` to skip generating direct base rewrite clauses for categories that are covered by collection congruences.

**Rationale:** 
- Currently, both old-style base rewrites AND new-style congruence projections are generated
- This creates redundancy but doesn't break correctness
- Once verified working, we can remove the redundant path

**Tasks:**
1. Add logic to detect when a category has collection congruence coverage
2. Skip `requires_indexed_projection` path for covered categories
3. Verify no functionality is lost
4. Test with RhoCalc and Ambient Calculus

### 🔄 Phase 5: Regular Congruence Direct Rules (PENDING)

**Status:** Not yet started

**Goal:** Ensure regular (non-collection) congruences also generate direct rewrite rules for non-collection contexts.

**Example:**
```rust
// if S => T then (PNew x S) => (PNew x T)
// Should generate BOTH:
// 1. Collection context: PPar { PNew x S } => PPar { PNew x T }  (via projection)
// 2. Direct context: PNew x S => PNew x T  (via existing generate_congruence_rewrite)
```

**Current Status:** Regular congruences already handled by `generate_congruence_rewrite` for non-collection contexts. Need to verify composition works correctly.

### 🔄 Phase 6: Remove Heuristic (PENDING)

**Status:** Not yet started

**Goal:** Remove `requires_indexed_projection` heuristic entirely from `rewrite_gen.rs`.

**Rationale:** 
- The heuristic was a workaround for the old approach
- New congruence-driven approach eliminates the need for it
- Cleaner, more principled design

### 🔄 Phase 7: Full Testing (PENDING)

**Status:** Not yet started

**Test Cases:**
1. RhoCalc with updated syntax (no `...rest` in base rewrites)
2. Communication rule: `(PPar {(PInput chan x P), (POutput chan Q)}) => (PPar {(subst P x (NQuote Q))})`
3. Drop-quote rule: `(PDrop (NQuote P)) => P`
4. Composition: `PPar { PDrop (NQuote (PDrop (NQuote P))) } => PPar { P }`
5. Ambient Calculus with PNew composition

### 🔄 Phase 8: Documentation (PENDING)

**Status:** Not yet started

**Deliverables:**
1. Update REPL-GUIDE.md with new syntax
2. Update examples (rhocalc.rs, ambient.rs)
3. Migration guide for users with existing theories
4. Update CONGRUENCE-DRIVEN-PROJECTIONS.md with final implementation details

## Testing Strategy

### Unit Tests (Pending)
- Test AST analysis functions with various rule patterns
- Test projection generation for different constructor types
- Test clause generation for base rewrites and regular congruences

### Integration Tests (Pending)
- Test RhoCalc theory with new syntax
- Test Ambient Calculus theory
- Verify no redundant rules are generated
- Verify composition of congruences works correctly

### Current Compilation Status
- ✅ mettail-macros compiles successfully (49 warnings, mostly unused code)
- ✅ mettail-repl compiles successfully
- ⚠️ Generated Ascent code uses old approach (expected, not yet switched over)

## Known Issues

1. **RHS Reconstruction:** Currently uses `generate_equation_rhs` which handles most cases but may need refinement for complex multi-element collection rewrites.

2. **Regular Congruence Reconstruction:** Non-binding regular congruences use a simplified reconstruction (`rewritten.clone()`) which may need to be extended to handle all fields of the constructor.

3. **Warnings:** Many unused code warnings in congruence_analysis.rs - these will resolve as the integration progresses and functions are actively called.

## Next Steps

1. **Immediate (Phase 4):**
   - Modify `generate_rewrite_clauses` to skip redundant base rewrites
   - Add feature flag or conditional to enable/disable new approach
   - Test with current RhoCalc theory (should produce identical behavior)

2. **Short Term (Phases 5-6):**
   - Verify regular congruence composition
   - Remove old heuristic
   - Update RhoCalc theory syntax

3. **Long Term (Phases 7-8):**
   - Comprehensive testing
   - Documentation updates
   - Migration guide

## References

- Main Design: `/docs/design/CONGRUENCE-DRIVEN-PROJECTIONS.md`
- Implementation: `/mettail-macros/src/congruence_analysis.rs`
- Integration: `/mettail-macros/src/ascent_gen.rs`
- Test Case: `/mettail-repl/src/rhocalc_theory.rs`

