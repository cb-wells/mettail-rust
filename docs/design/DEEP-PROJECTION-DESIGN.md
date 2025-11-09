# Deep Indexed Projection Design

**Date**: November 9, 2025  
**Status**: 📋 DESIGN PHASE

## Problem Statement

Current indexed projection only works for **top-level shared variables**. It fails for patterns like:

```rust
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R), ...rest})
```

Where `M` appears:
- In pattern 1: **deeply nested** inside `PAmb` → `PPar` → `PIn`
- In pattern 2: directly as first argument of `PAmb`

**Current Limitation**: `extract_captures_from_args` only looks at direct arguments of the element constructor, missing deeply nested variables.

**Impact**: Rules with deeply nested shared variables fall back to order-dependent matching, which is unreliable with HashBags.

---

## Root Cause Analysis

### Current Architecture

1. **Detection** (`requires_indexed_projection`):
   - ✅ Correctly detects shared variables at any depth using `collect_vars_in_expr`
   - ✅ Returns `true` for the ambient calculus rule

2. **Analysis** (`analyze_collection_pattern`):
   - ❌ Calls `extract_captures_from_args` which only extracts **direct arguments**
   - ❌ Misses `M` nested inside `PAmb(N, PPar({PIn(M, P), Q}))`
   - ❌ Returns `None` because no shared variables found
   - ❌ Falls back to old order-dependent code

3. **Result**: Order-dependent matching with `eq_name` check, which only works if elements are in the "right" order

### Why Simple Fix Doesn't Work

We can't just use `collect_vars_in_expr` for analysis because we need to know:
1. **Extraction path**: How to get to the variable (which fields to traverse)
2. **Join key type**: What category/type the variable has
3. **Binder status**: Is it a binder variable or regular variable?

For `M` in `PAmb(N, PPar({PIn(M, P), Q}))`:
- **Path**: field[1] → PPar.bag → PIn.field[0]
- **Type**: `Name`
- **Not a binder**

---

## Design Goals

1. ✅ **Detect** shared variables at any depth (already working)
2. ✅ **Analyze** nested patterns to extract join keys and their paths
3. ✅ **Generate** efficient Ascent code with multi-level projections
4. ✅ **Maintain** type safety and correctness
5. ✅ **Optimize** for common cases (avoid unnecessary complexity)

---

## Proposed Solution: Recursive Projection

### High-Level Approach

Instead of flat projections, use **recursive descent** to extract join keys:

**Current (Flat Projection)**:
```rust
// For (PPar {(PIn chan x P), (POutput chan Q)})
pinput_proj(parent, chan, x, p, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (elem, _) in bag.iter(),
    if let Proc::PInput(ref f0, ref f1) = elem,
    let chan = (**f0).clone(),
    ...
```

**Proposed (Deep Projection)**:
```rust
// For (PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R)})

// Step 1: Extract all PAmb ambients from outer PPar
pamb_in_ppar(parent, name, proc, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (elem, _) in bag.iter(),
    if let Proc::PAmb(n, p) = elem,
    let name = (**n).clone(),
    let proc = (**p).clone();

// Step 2: From those PAmbs, extract PIn from nested PPars
pin_in_pamb_nested(parent, outer_name, inner_chan, p_body, ...) <--
    pamb_in_ppar(parent, outer_name, inner_proc, amb_elem),
    if let Proc::PPar(inner_bag) = &inner_proc,
    for (inner_elem, _) in inner_bag.iter(),
    if let Proc::PIn(chan, p) = inner_elem,
    let inner_chan = (**chan).clone(),
    ...;

// Step 3: Join on the nested channel
rw_proc(parent, result) <--
    pin_in_pamb_nested(parent, n, m, p, q, amb1_elem, pin_elem),
    pamb_in_ppar(parent, m, r, amb2_elem),  // Join on m!
    ...;
```

### Key Insight

**Decompose extraction into stages**:
1. Extract outer constructors (e.g., `PAmb` from `PPar`)
2. For nested patterns, extract inner constructors (e.g., `PIn` from inner `PPar`)
3. Join at the appropriate level where shared variables meet

---

## Detailed Design

### Phase 1: Enhanced Analysis

#### 1.1 Deep Variable Discovery

Extend `CaptureInfo` to include **extraction path**:

```rust
struct CaptureInfo {
    var_name: String,
    category: Ident,
    extraction_path: Vec<ExtractionStep>,
    is_binder: bool,
}

enum ExtractionStep {
    /// Direct field access (dereference Box)
    Field { 
        field_idx: usize,
        category: Ident,
    },
    /// Collection iteration and pattern match
    CollectionMatch {
        field_idx: usize,        // Which field is the collection
        constructor: Ident,       // What constructor to match
        element_category: Ident,
    },
}
```

**Example** for `M` in `PAmb(N, PPar({PIn(M, P), Q}))`:
```rust
CaptureInfo {
    var_name: "M",
    category: Name,
    extraction_path: [
        Field { field_idx: 1, category: Proc },           // Access second field of PAmb
        CollectionMatch {                                  // It's a collection
            field_idx: 0,
            constructor: PIn,
            element_category: Proc,
        },
        Field { field_idx: 0, category: Name },           // Access first field of PIn
    ],
    is_binder: false,
}
```

#### 1.2 Recursive Pattern Analyzer

```rust
fn analyze_element_pattern_deep(
    pattern: &Expr,
    theory: &TheoryDef,
    current_path: Vec<ExtractionStep>,
) -> Vec<CaptureInfo> {
    match pattern {
        Expr::Var(v) => {
            vec![CaptureInfo {
                var_name: v.to_string(),
                category: infer_from_context(),
                extraction_path: current_path,
                is_binder: false,
            }]
        }
        Expr::Apply { constructor, args } => {
            let mut captures = Vec::new();
            let grammar = find_grammar_rule(constructor, theory);
            
            for (arg_idx, arg) in args.iter().enumerate() {
                let field_idx = map_arg_to_field(arg_idx, grammar);
                
                match arg {
                    Expr::Var(v) => {
                        let mut path = current_path.clone();
                        path.push(ExtractionStep::Field {
                            field_idx,
                            category: get_arg_category(arg_idx, grammar),
                        });
                        captures.push(CaptureInfo { 
                            var_name: v.to_string(),
                            extraction_path: path,
                            ...
                        });
                    }
                    Expr::Apply { .. } => {
                        // Nested Apply - recurse
                        let mut path = current_path.clone();
                        path.push(ExtractionStep::Field { field_idx, ... });
                        captures.extend(
                            analyze_element_pattern_deep(arg, theory, path)
                        );
                    }
                    Expr::CollectionPattern { elements, .. } => {
                        // Nested collection - analyze each possible element
                        for nested_elem in elements {
                            if let Expr::Apply { constructor: nested_cons, .. } = nested_elem {
                                let mut path = current_path.clone();
                                path.push(ExtractionStep::CollectionMatch {
                                    field_idx,
                                    constructor: nested_cons.clone(),
                                    element_category: get_element_category(nested_cons, theory),
                                });
                                captures.extend(
                                    analyze_element_pattern_deep(nested_elem, theory, path)
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            captures
        }
        _ => vec![]
    }
}
```

### Phase 2: Multi-Level Projection Generation

#### 2.1 Projection Decomposition

For each unique extraction path to a shared variable, generate an intermediate projection:

```rust
struct ProjectionLevel {
    relation_name: Ident,
    /// Variables captured at this level
    captures: Vec<CaptureInfo>,
    /// Path from parent to this level
    extraction_steps: Vec<ExtractionStep>,
    /// Which deeper projections feed into this one
    child_projections: Vec<ProjectionLevel>,
}
```

#### 2.2 Code Generation Strategy

**Bottom-Up Generation**:

1. **Deepest level first**: Generate projections for the most nested patterns
2. **Compose upward**: Each level uses results from deeper levels
3. **Join at top**: Final join uses the shallowest common ancestor

**Example for Ambient Rule**:

```rust
// Level 2 (deepest): Extract PIn from inner collections
relation pin_from_inner_ppar_r0(
    outer_elem: Proc,      // The PAmb that contains this
    inner_chan: Name,      // M - the join key
    p: Proc,               // P
    inner_elem: Proc,      // The PIn itself
);

pin_from_inner_ppar_r0(outer_elem, inner_chan, p, inner_elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (outer_elem, _) in bag.iter(),
    if let Proc::PAmb(n, inner_proc) = outer_elem,
    if let Proc::PPar(inner_bag) = (**inner_proc).as_ref(),
    for (inner_elem, _) in inner_bag.iter(),
    if let Proc::PIn(chan, p_box) = inner_elem,
    let inner_chan = (**chan).clone(),
    let p = (**p_box).clone();

// Level 1: Extract PAmb from outer collection  
relation pamb_simple_r0(
    parent: Proc,
    name: Name,           // M or N
    proc: Proc,
    elem: Proc,
);

pamb_simple_r0(parent, name, proc, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (elem, _) in bag.iter(),
    if let Proc::PAmb(n, p) = elem,
    let name = (**n).clone(),
    let proc = (**p).clone();

// Level 0 (top): Join on shared variable M
rw_proc(parent, result) <--
    // Pattern 1: Get PAmb(N, ...) that contains PIn(M, P)
    pamb_simple_r0(parent, n, nested_proc, amb1_elem),
    pin_from_inner_ppar_r0(amb1_elem, m, p, pin_elem),
    // Pattern 2: Get PAmb(M, R) where M is the same
    pamb_simple_r0(parent, m, r, amb2_elem),
    amb1_elem != amb2_elem,  // Must be different elements
    // Extract Q from the nested collection
    if let Proc::PPar(inner_bag) = &nested_proc,
    // ... rest construction and RHS
```

#### 2.3 Path Merging Optimization

When multiple patterns have **common extraction prefixes**, share the intermediate projections:

```rust
// Both patterns extract PAmb from outer PPar - SHARE this projection
pamb_simple_r0(parent, name, proc, elem) <-- ...

// Pattern 1 then goes deeper
pin_from_inner_ppar_r0(...) <-- 
    pamb_simple_r0(parent, n, nested_proc, amb1_elem),
    ...

// Pattern 2 uses the same base projection
// Join at pamb_simple_r0 level on M
```

---

## Implementation Plan

### Stage 1: Analysis Enhancement (Week 1)

**Goal**: Detect deeply nested shared variables and compute extraction paths

1. ✅ Implement `ExtractionStep` enum
2. ✅ Implement `analyze_element_pattern_deep` 
3. ✅ Extend `ElementPattern` with deep capture info
4. ✅ Update `analyze_collection_pattern` to use deep analysis
5. ✅ Add tests for path extraction

**Success Criteria**: 
- Ambient rule analysis returns shared variable `M` with correct path
- Both top-level and nested variables are captured

### Stage 2: Multi-Level Projection Generation (Week 2)

**Goal**: Generate intermediate relations for nested extractions

1. ✅ Implement `ProjectionLevel` structure
2. ✅ Build projection tree from capture paths
3. ✅ Generate bottom-up relation declarations
4. ✅ Generate extraction rules for each level
5. ✅ Test with manually constructed paths

**Success Criteria**:
- Generates `pin_from_inner_ppar_r0` relation
- Generates `pamb_simple_r0` relation
- Relations properly parameterized

### Stage 3: Join Generation (Week 3)

**Goal**: Generate top-level join that uses multi-level projections

1. ✅ Identify join point (common ancestor in extraction tree)
2. ✅ Generate join clause with intermediate relations
3. ✅ Handle element distinctness checks
4. ✅ Generate rest construction
5. ✅ Generate RHS with nested reconstructions

**Success Criteria**:
- Ambient rule generates working join
- `{m[r], n[{in(m,p), q}]} ~> {m[{n[{p, q}], r}]}` works in any order

### Stage 4: Optimization & Edge Cases (Week 4)

**Goal**: Handle edge cases and optimize generated code

1. ✅ Handle mixed depth (one shallow, one deep)
2. ✅ Handle multiple shared variables at different depths
3. ✅ Optimize away unnecessary projections for simple cases
4. ✅ Handle binders in nested positions
5. ✅ Handle rest patterns in nested collections

**Success Criteria**:
- All test cases pass
- Performance comparable to manual code
- No unnecessary intermediate relations

---

## Complexity Analysis

### Worst Case Scenarios

1. **Exponential Nesting**: Pattern with N levels of nesting
   - **Cost**: O(N) intermediate relations
   - **Mitigation**: Share common prefixes

2. **Many Shared Variables**: K shared variables at different depths
   - **Cost**: K separate extraction paths
   - **Mitigation**: Group by common extraction prefix

3. **Large Collections**: M elements in nested collection
   - **Cost**: O(M²) for pairwise joins (same as current)
   - **Mitigation**: Use Ascent's optimized joins

### Performance Expectations

- **Best case** (top-level shared vars): Same as current indexed projection
- **Nested case** (1-2 levels): 2-3x overhead from intermediate relations
- **Deep nesting** (3+ levels): May be slower, but still O(N) vs O(N!) without projection

---

## Alternative Approaches Considered

### 1. **Flatten Nested Patterns**

Transform `(PAmb N (PPar {(PIn M P), Q}))` into multiple patterns:
- Match `PAmb` in outer collection
- Match `PPar` in ambient's body
- Match `PIn` in inner collection

**Rejected**: Loses structural information, makes RHS reconstruction complex

### 2. **Path Expressions in Ascent**

Extend Ascent with path query syntax: `parent.field[1].collection{PIn}.field[0]`

**Rejected**: Requires changes to Ascent, not feasible

### 3. **Hybrid Approach**

Use indexed projection for top-level, fall back to enumeration for nested

**Rejected**: Still order-dependent for nested cases, doesn't solve the problem

### 4. **Lazy Extraction**

Don't extract nested variables into relations, just traverse in the join

**Rejected**: Loses Ascent's join optimization, essentially same as old approach

---

## Migration Strategy

### Backward Compatibility

✅ **Old code still works**: Current flat projection is a special case (zero nesting depth)

✅ **Gradual rollout**: Can enable/disable deep projection with feature flag

✅ **Fallback path**: If deep analysis fails, fall back to current approach

### Testing Strategy

1. **Unit tests**: Each stage of analysis and generation
2. **Integration tests**: Full rules with various nesting depths
3. **Performance tests**: Compare with manual projections
4. **Correctness tests**: All orderings produce same result

### Validation Criteria

Before considering complete:
- [ ] Ambient calculus rule works in all orderings
- [ ] RhoCalc rules still work (regression test)
- [ ] Performance within 2x of manual code
- [ ] No false positives (generating deep projection unnecessarily)
- [ ] Clear error messages when analysis fails

---

## Future Enhancements

### Beyond Phase 1

1. **Cross-collection joins**: Shared variables across different collection arguments
2. **Recursive patterns**: Self-referential nesting (e.g., nested Par inside Par)
3. **Conditional extraction**: Only extract if certain conditions hold
4. **Index selection**: Automatically choose best join order

### Integration with Theory Composition

When composing theories, deep projections might span theory boundaries:
- Extract from Theory A's constructors
- Join on Theory B's relations
- Requires careful handling of category boundaries

---

## Risk Assessment

### High Risk

- **Complexity explosion**: Too many intermediate relations for complex patterns
  - **Mitigation**: Aggressive sharing of common prefixes, limits on nesting depth
  
- **Type errors**: Mismatch between expected and actual types in deep extraction
  - **Mitigation**: Careful type tracking through extraction paths

### Medium Risk

- **Performance regression**: Deep projection slower than needed
  - **Mitigation**: Benchmark against manual code, optimize hot paths

- **Analysis failures**: Complex patterns that analysis can't handle
  - **Mitigation**: Clear error messages, fallback to old approach

### Low Risk

- **Backward compatibility**: Breaking existing rules
  - **Mitigation**: Extensive regression testing, staged rollout

---

## Success Metrics

### Correctness
- ✅ Ambient rule works in all element orderings
- ✅ All existing tests still pass
- ✅ No false matches or missed matches

### Performance
- ✅ Within 2x of manual projection code
- ✅ Better than O(N!) enumeration for N>2 elements

### Usability
- ✅ Users don't need to think about depth
- ✅ Automatic optimization selection
- ✅ Clear error messages for unsupported patterns

### Maintainability
- ✅ Code well-documented with examples
- ✅ Clear separation between analysis and generation
- ✅ Easy to extend for new pattern types

---

## Conclusion

**Deep indexed projection** extends MeTTaIL's automatic optimization to handle the full generality of nested collection patterns. While more complex than flat projection, the modular design allows incremental implementation and maintains backward compatibility.

**Key Innovation**: Decompose extraction into stages, generate intermediate projections, and leverage Ascent's join optimization at multiple levels.

**Timeline**: 4 weeks for full implementation, 2 weeks for core functionality

**Next Step**: Implement Stage 1 (Analysis Enhancement) and validate with ambient calculus example

---

**Status**: 📋 **DESIGN APPROVED - READY FOR IMPLEMENTATION**

