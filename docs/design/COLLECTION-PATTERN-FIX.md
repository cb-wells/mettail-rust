# Collection Pattern Matching Bug & Fix Plan

**Date:** November 11, 2025  
**Priority:** High - Correctness Issue

---

## The Bug

### Current Behavior
When generating Ascent code for collection rewrite patterns like:
```rust
(PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});
```

The codegen produces:
```rust
rw_proc(s, t) <-- proc(s), 
    if let Proc::PPar(s_f0) = s, 
    if s_f0.len() >= 1usize,
    let s_f0_elem_0 = s_f0.iter().next().unwrap().0.clone(),  // ← BUG!
    // ... pattern match on s_f0_elem_0 ...
```

**Problem:** Uses `.iter().next()` to get an **arbitrary** first element, then checks if it matches. If the first element doesn't match, the rule fails even if other elements would match.

### Why This Happens
The indexed projection optimization (which iterates over ALL matching elements) is only triggered when there are **shared variables** between multiple patterns in the collection. For single-pattern rules with no shared variables, it falls back to naive iteration.

### Example Failure
```rust
{b!(0), *@(a!(0))}  // Term with 2 elements
```

- Iterator might return `b!(0)` first
- Pattern `(PDrop (NQuote P))` doesn't match `b!(0)`
- Rule fails, even though `*@(a!(0))` would match!
- Result: **No rewrite found** (incorrectly reported as normal form)

---

## Root Cause Analysis

### File: `mettail-macros/src/rewrite_gen.rs`

The function `generate_ascent_collection_pattern` makes a decision:

```rust
fn generate_ascent_collection_pattern(...) -> TokenStream {
    // Check if we need indexed projection
    if needs_indexed_projection(&elements) {
        generate_indexed_projection(...)  // ✅ Correct: iterates all matches
    } else {
        generate_naive_extraction(...)     // ❌ Bug: only checks first element
    }
}

fn needs_indexed_projection(elements: &[ElementPattern]) -> bool {
    // Only returns true if there are SHARED VARIABLES between patterns
    let shared_vars = find_shared_variables(elements);
    shared_vars.len() > 0
}
```

**The flaw:** Indexed projection is an **optimization** for joining, but it's also the **only correct way** to iterate over all potential matches!

---

## The Fix: Always Use Indexed Projection

### Principle
**Every** collection pattern should generate a projection relation, even single patterns. This ensures we check **all** elements, not just an arbitrary first one.

### Design Changes

#### 1. Remove the "needs projection" check
**Before:**
```rust
if needs_indexed_projection(&elements) {
    generate_indexed_projection(...)
} else {
    generate_naive_extraction(...)
}
```

**After:**
```rust
// ALWAYS use projection for correctness
generate_indexed_projection(...)
```

#### 2. Simplify single-pattern projection
For patterns with no shared variables, the projection doesn't need joins:

**Generated code for `{(PDrop (NQuote P)), ...}`:**
```rust
// Projection relation
relation pdrop_proj_r0_p0(Proc, Proc, Proc);  // (parent, P, elem)

pdrop_proj_r0_p0(parent.clone(), cap_p.clone(), elem.clone()) 
    <-- proc(parent), 
        if let Proc::PPar(ref bag_field) = parent,
        for (elem, _count) in bag_field.iter(),  // ✅ ALL elements
        if let Proc::PDrop(ref f0) = elem,
        if let Name::NQuote(ref f1) = f0.as_ref(),
        let cap_p = (*f1).clone();

// Rewrite rule using projection
rw_proc(parent.clone(), result) 
    <-- pdrop_proj_r0_p0(parent, p, elem),
        if let Proc::PPar(ref bag) = parent,
        let rest = { 
            let mut b = bag.clone(); 
            b.remove(&elem); 
            b 
        },
        let result = Proc::PPar({
            let mut bag = rest.clone();
            Proc::insert_into_ppar(&mut bag, p);
            bag
        });
```

**Key difference:** The `for (elem, _count) in bag_field.iter()` in the projection ensures we generate facts for **every** matching element, not just one.

---

## Implementation Plan

### Phase 1: Refactor Collection Pattern Generation (1-2 days)

#### Step 1: Update `rewrite_gen.rs`
**File:** `mettail-macros/src/rewrite_gen.rs`

1. **Remove naive extraction path:**
   ```rust
   // DELETE this function entirely
   fn generate_naive_extraction(...) -> TokenStream { ... }
   ```

2. **Always use projection:**
   ```rust
   fn generate_ascent_collection_pattern(...) -> TokenStream {
       // OLD:
       // if needs_indexed_projection(&elements) { ... } else { ... }
       
       // NEW: Always project
       generate_indexed_projection(...)
   }
   ```

3. **Update projection metadata:**
   ```rust
   fn create_projection_spec(...) -> ProjectionSpec {
       // Handle single-pattern case (no join keys)
       let join_keys = find_shared_variables(elements);
       
       ProjectionSpec {
           elements: pattern_elements,
           join_keys,  // May be empty!
           requires_join: join_keys.len() > 1,  // ← NEW field
           ...
       }
   }
   ```

4. **Simplify projection for non-joining patterns:**
   ```rust
   fn generate_projection_relation(..., spec: &ProjectionSpec) -> TokenStream {
       if spec.requires_join {
           // Generate indexed facts with join keys
           generate_indexed_projection_with_join(...)
       } else {
           // Generate simpler projection (no join needed)
           generate_simple_projection(...)
       }
   }
   ```

#### Step 2: Test Cases
**File:** `mettail-macros/tests/rewrite_gen_tests.rs`

```rust
#[test]
fn test_single_pattern_projection() {
    theory! {
        name: TestTheory,
        exports { T },
        terms {
            TZero . T ::= "0";
            TWrap . T ::= "wrap" "(" T ")";
            TPar . T ::= HashBag(T) sep "," delim "{" "}";
        },
        rewrites {
            (TPar {(TWrap P), ...rest}) => (TPar {P, ...rest});
        }
    }
    
    // Test that {wrap(0), other} reduces to {0, other}
    let term = TPar(hashbag![TWrap(Box::new(TZero)), TZero]);
    let prog = ascent_run! {
        testheory_source!();
        t(term);
    };
    
    assert!(prog.rw_t.iter().any(|(from, to)| {
        from == &term && to.contains(&TZero)
    }));
}

#[test]
fn test_order_independence() {
    // Test that pattern matches regardless of iteration order
    let term1 = TPar(hashbag![TWrap(Box::new(TZero)), TZero]);
    let term2 = TPar(hashbag![TZero, TWrap(Box::new(TZero))]);
    
    // Both should have same rewrites (order-independent)
    // ...
}
```

---

### Phase 2: Optimize Performance (1 day)

The fix adds projection relations for all collection patterns. This increases the number of Ascent relations, which could impact performance.

#### Optimization: Relation Deduplication
If multiple patterns in the same theory generate identical projections, share the relation:

```rust
fn deduplicate_projection_specs(specs: Vec<ProjectionSpec>) -> Vec<ProjectionSpec> {
    // Hash by: constructor, captures, join keys
    // Reuse relation name if patterns are equivalent
    ...
}
```

#### Optimization: Selective Inlining
For very simple patterns, inline instead of projecting:

```rust
if is_trivial_pattern(pattern) {
    // Pattern like {X, ...} with no nested constructors
    // Can safely use iter().next() because X matches anything
    generate_inline_extraction(...)
} else {
    generate_indexed_projection(...)
}
```

---

### Phase 3: Update Documentation (0.5 days)

#### Update Design Docs
**File:** `docs/design/COLLECTION-MATCHING-SOLUTION.md`

Add section:
```markdown
## Correctness Fix: Always Project

Prior to November 2025, collection pattern matching had a correctness bug:
single-pattern rules would only check the first element in iteration order.

**Fix:** All collection patterns now generate projection relations, ensuring
every element is checked for matches. This guarantees order-independent
and exhaustive pattern matching.

**Performance:** Adds one relation per collection pattern, but correctness
is paramount. Optimization opportunities exist for trivial patterns.
```

#### Update Known Limitations
**File:** `docs/KNOWN-LIMITATIONS.md`

Remove or update:
```markdown
## Fixed Issues

### Collection Pattern Matching Order Dependence (FIXED ✅)
**Was:** Single-pattern collection rules only checked arbitrary first element
**Now:** All elements are checked via projection relations
**Fixed:** November 2025
```

---

## Testing Strategy

### Unit Tests
1. **Single pattern in collection** - `{wrap(x), y}` should match
2. **Order independence** - `{a, wrap(b)}` vs `{wrap(b), a}` same rewrites
3. **Multiple matches** - `{wrap(a), wrap(b), c}` finds both
4. **Empty rest** - `{wrap(x)}` rewrites to `{x}`
5. **Nested patterns** - `{drop(@(p)), q}` matches correctly

### Integration Tests
1. **RhoCalc `*@(P)` reduction** - Must work in all positions
2. **Communication with side effects** - Ensure no regressions
3. **Performance benchmarks** - Measure impact of extra relations

### Real-World Test
```bash
# This MUST work after the fix:
rhocalc> term: {*@(a!(0)), b!(0)}
rhocalc> rewrites
# Should show: → {a!(0), b!(0)}
```

---

## Estimated Impact

### Correctness
- **Critical fix** - Ensures pattern matching is exhaustive
- **Eliminates false normal forms** - Terms won't get stuck incorrectly
- **Order-independent** - HashBag iteration order no longer matters

### Performance
- **Increased relations:** +1 relation per collection pattern (~10-20 for typical theory)
- **Increased facts:** Proportional to (# collection patterns) × (avg elements per collection)
- **Estimated overhead:** 5-15% increase in Ascent runtime for typical workloads
- **Acceptable trade-off:** Correctness > minor performance cost

### Compatibility
- **No syntax changes** - Theory definitions unchanged
- **Semantic fix only** - Existing theories become more correct
- **May find new rewrites** - Patterns that silently failed will now work

---

## Migration Notes

### For Existing Theories
After this fix, theories may exhibit **new behaviors**:

1. **More rewrites found** - Patterns that were silently failing now match
2. **Different normal forms** - Terms that were stuck may now reduce
3. **Performance changes** - More relations = slightly slower, but correct

### Recommended Actions
1. **Retest all examples** - Verify expected behavior
2. **Update documentation** - Note any changed normal forms
3. **Performance audit** - Measure if impact is acceptable
4. **Add regression tests** - Prevent reintroduction of bug

---

## Success Criteria

### Must Pass
1. ✅ `{*@(a!(0)), b!(0)}` reduces to `{a!(0), b!(0)}`
2. ✅ `replicated_input` example completes reduction
3. ✅ All existing tests still pass
4. ✅ Order independence verified with property tests

### Should Pass
1. 📊 Performance overhead < 20%
2. 📈 Compile time increase < 10%
3. 📝 Documentation updated

---

## Timeline

**Week 1:** Implementation
- Day 1-2: Refactor rewrite_gen.rs
- Day 3: Add test cases
- Day 4: Debug and fix issues
- Day 5: Performance optimization

**Week 2:** Testing & Documentation
- Day 1-2: Integration testing
- Day 3: Real-world examples
- Day 4: Performance benchmarking
- Day 5: Documentation updates

**Total:** ~2 weeks for full implementation and validation

---

## Open Questions

1. **Should we add a lint?** Warn users if they rely on order-dependent behavior?
2. **Projection sharing?** Can we deduplicate identical projection specs across rules?
3. **Incremental migration?** Add feature flag to enable gradually?
4. **Performance monitoring?** Add metrics to track projection relation sizes?

---

## Related Issues

- **Indexed Projection Automation** (Phase 6.1) - Built foundation for this
- **Collection Types** (Phase 6) - Introduced HashBag, exposed this bug
- **Order-Independent Matching** - Core goal, now fully achieved

---

**Status:** Planning Complete ✅  
**Next Action:** Begin implementation in `rewrite_gen.rs`

