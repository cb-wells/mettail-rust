# Term Generation with Collection Types - TODO

**Date**: November 9, 2025  
**Status**: 🔴 BROKEN - Needs Implementation  
**Priority**: Medium (not critical for rewrite engine)

---

## Problem Statement

Term generation (exhaustive and random) currently **skips collection constructors** entirely. This means:

1. **Exhaustive generation** cannot generate terms with collection fields
2. **Random generation** cannot generate terms with collection fields
3. **Testing** cannot automatically generate collection-based test cases

### Current Workaround

In `termgen_gen.rs`:
```rust
// Lines 124-126 and 166-171
.filter(|item| matches!(item, GrammarItem::NonTerminal(_) | GrammarItem::Binder(_)))
// Collections are filtered out!
```

This prevents compilation errors but makes collection constructors unavailable in generated terms.

---

## Impact

### Low Impact on Core Functionality ✅
- Rewrite engine works fine (doesn't need term generation)
- Parser works fine
- Manual test cases work fine

### Medium Impact on Testing ⚠️
- Cannot auto-generate `PPar {P, Q}` terms
- Cannot use property-based testing with collections
- Reduces test coverage for collection-heavy theories

### Example

**Rho Calculus with Collections:**
```rust
PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}";
```

**Current behavior:**
```rust
// generate_terms::<Proc>(2) will NOT include:
PPar({PZero, PZero})         // ❌ Skipped
PPar({PInput(...), PZero})   // ❌ Skipped

// Only generates:
PZero                        // ✅ Works
PInput(...)                  // ✅ Works
POutput(...)                 // ✅ Works
```

---

## Design Considerations

### Challenge 1: How Many Elements?

Collections have **variable size**. For exhaustive generation at depth `d`:

**Option A: Fixed sizes** (0, 1, 2 elements)
```rust
// Depth 0:
PPar({})           // Empty collection

// Depth 1:
PPar({PZero})      // One depth-0 element

// Depth 2:
PPar({PZero, PZero})              // Two depth-0 elements
PPar({PInput(NVar, PZero)})       // One depth-1 element
```

**Option B: Bounded sizes** (0 to N elements where N grows with depth)
```rust
// Depth 2, max_elements = 3:
PPar({})
PPar({PZero})
PPar({PZero, PZero})
PPar({PZero, PZero, PZero})
PPar({PInput(...)})
PPar({PInput(...), PZero})
// ... many more
```

**Option C: Single element only** (simplest)
```rust
// Only generate collections with 1 element
PPar({elem})   where elem is a generated Proc
```

**Recommendation**: Start with **Option C** (single element), can enhance later

### Challenge 2: Combinatorial Explosion

With collections, term count explodes:

**Without collections (depth 2, 3 constructors):**
- ~10-20 terms

**With collections (depth 2, up to 2 elements):**
- ~100-500 terms (combinations of pairs)

**Mitigation**: 
- Limit collection size to 1-2 elements
- Make max collection size configurable
- Use random generation for larger tests

### Challenge 3: Empty Collections

Should we generate `PPar({})`?

**Pro**: 
- It's a valid term
- Identity element in some theories

**Con**:
- Often normalized away by equations (e.g., `{} == PZero`)
- Adds noise to test cases

**Recommendation**: 
- Generate empty collections at depth 0
- Respect theory's identity equation if present

### Challenge 4: Duplicate Elements

Should we generate `PPar({P, P})`?

**Pro**:
- It's a valid bag (multiset allows duplicates)
- Tests duplicate handling

**Con**:
- Often semantically meaningless (e.g., `P | P` = `P` in many calculi)
- Increases test case count

**Recommendation**:
- Allow duplicates in exhaustive generation (it's correct)
- Filter in random generation if theory has idempotence equation

---

## Proposed Implementation

### Phase 1: Single-Element Collections (Simplest)

**Goal**: Generate collections with exactly 1 element

**Changes to `termgen_gen.rs`:**

```rust
// generate_depth_0_cases (lines 124-126)
for item in &rule.grammar_rule.items {
    match item {
        GrammarItem::NonTerminal(field_cat) | GrammarItem::Binder(field_cat) => {
            // Existing code for regular fields
        }
        GrammarItem::Collection { element_type, .. } => {
            // NEW: Generate single-element collection at depth 0
            let empty_bag = quote! {
                {
                    let mut bag = mettail_runtime::HashBag::new();
                    bag
                }
            };
            field_options.push(vec![empty_bag]);
        }
        _ => {} // Skip terminals
    }
}

// generate_depth_d_cases (lines 166-171)
for item in &rule.grammar_rule.items {
    match item {
        GrammarItem::NonTerminal(field_cat) | GrammarItem::Binder(field_cat) => {
            // Existing code for regular fields
        }
        GrammarItem::Collection { element_type, .. } => {
            // NEW: Generate single-element collection
            let element_terms = quote! {
                <#element_type as mettail_runtime::GenTerms>::generate_terms(d - 1)
            };
            let single_elem_bags = quote! {
                #element_terms.into_iter().map(|elem| {
                    let mut bag = mettail_runtime::HashBag::new();
                    bag.insert(elem);
                    bag
                }).collect::<Vec<_>>()
            };
            field_options.push(single_elem_bags);
        }
        _ => {}
    }
}
```

**Result**: 
- `generate_terms::<Proc>(0)` includes `PPar({})`
- `generate_terms::<Proc>(1)` includes `PPar({PZero})`
- `generate_terms::<Proc>(2)` includes `PPar({PInput(...)})`, etc.

**Complexity**: Simple, minimal code changes

---

### Phase 2: Multi-Element Collections (Future)

**Goal**: Generate collections with 0-2 elements

**Additional complexity**:
- Need to generate all pairs: `(elem1, elem2)` where `elem1, elem2 in generate_terms(d-1)`
- Combinatorial explosion: if `n` terms at depth `d-1`, then `n²` pairs
- Need to handle duplicates: `(P, P)` is valid

**Implementation sketch**:
```rust
GrammarItem::Collection { element_type, .. } => {
    let element_terms = quote! {
        <#element_type as mettail_runtime::GenTerms>::generate_terms(d - 1)
    };
    
    let multi_elem_bags = quote! {
        {
            let elems = #element_terms;
            let mut result = Vec::new();
            
            // Empty bag
            result.push(mettail_runtime::HashBag::new());
            
            // Single elements
            for elem in &elems {
                let mut bag = mettail_runtime::HashBag::new();
                bag.insert(elem.clone());
                result.push(bag);
            }
            
            // Pairs (limit to 2 elements for now)
            for e1 in &elems {
                for e2 in &elems {
                    let mut bag = mettail_runtime::HashBag::new();
                    bag.insert(e1.clone());
                    bag.insert(e2.clone());
                    result.push(bag);
                }
            }
            
            result
        }
    };
    field_options.push(multi_elem_bags);
}
```

**Caveat**: This will explode term count. Needs testing and tuning.

---

### Phase 3: Random Generation (Complementary)

**Goal**: Generate random collection sizes for property-based testing

**Changes to `random_generation.rs`:**

```rust
// Line 158-164: extend match to include Collection
GrammarItem::Collection { element_type, .. } => {
    // Randomly choose collection size (0-3 elements)
    let size = rng.gen_range(0..=3);
    
    let mut bag = HashBag::new();
    for _ in 0..size {
        let elem = generate_random_rec(element_type, depth - 1, rng, theory);
        bag.insert(elem);
    }
    
    quote! { #bag }
}
```

**Benefit**: Can test with larger collections without exponential cost

---

## Sorting Collections (Related Issue)

Collections also need **total ordering** for `Ord` trait.

### Current Implementation

In `mettail-runtime/src/hashbag.rs`, we already have:
```rust
impl<T: Ord> Ord for HashBag<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by sorted vector of (elem, count) pairs
        let mut self_vec: Vec<_> = self.map.iter().collect();
        self_vec.sort();
        let mut other_vec: Vec<_> = other.map.iter().collect();
        other_vec.sort();
        self_vec.cmp(&other_vec)
    }
}
```

**Status**: ✅ **Already working!**

No changes needed for sorting - it's already implemented and used by the generated AST's `#[derive(Ord)]`.

---

## Action Items

### Immediate (Before Push)
- [x] Document the limitation (this file)
- [x] Note it's not critical for current functionality

### Short-Term (Next 1-2 weeks)
- [ ] Implement Phase 1 (single-element collections)
- [ ] Add tests for generated collection terms
- [ ] Verify no compilation errors

### Medium-Term (Q1 2026)
- [ ] Implement Phase 2 (multi-element collections)
- [ ] Add configurability (max collection size)
- [ ] Benchmark term generation performance

### Long-Term (Q2 2026)
- [ ] Implement Phase 3 (random generation)
- [ ] Property-based testing with collections
- [ ] Tune generation parameters based on theory

---

## Related Files

- `mettail-macros/src/termgen_gen.rs` - Exhaustive term generation
- `mettail-macros/src/random_generation.rs` - Random term generation
- `mettail-runtime/src/hashbag.rs` - HashBag with Ord trait ✅

---

## Summary

**Current Status**: Collection constructors skipped in term generation  
**Impact**: Low (rewrite engine unaffected)  
**Quick Fix**: Implement single-element collections (Phase 1)  
**Full Solution**: Multi-element + random generation (Phases 2-3)  
**Sorting**: ✅ Already works via HashBag's Ord implementation

Not a blocker for the current push, but should be addressed for better testing coverage.

