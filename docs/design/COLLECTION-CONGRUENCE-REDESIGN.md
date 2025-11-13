# Collection Congruence Redesign

**Date**: November 11, 2025  
**Status**: Design Phase  
**Priority**: Critical (Correctness Bug)

## Problem Statement

The current approach to collection pattern matching in rewrite rules is fundamentally flawed. When a user writes:

```rust
rewrites {
    (PDrop (NQuote P)) => P;
}
```

The system should apply this rewrite **wherever** `(PDrop (NQuote P))` appears, including inside collections like `PPar {...}`. However, the current implementation has several architectural problems:

### Current Issues

1. **Mixing Concerns**: The rewrite generation tries to handle both:
   - Base rewrites on terms
   - Congruence through collections
   
   These should be separate concerns.

2. **Order-Dependent Matching**: When we write:
   ```rust
   (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});
   ```
   
   The generated code uses `.iter().next()` which only checks the **first** element in the arbitrary iteration order of the `HashBag`. This misses matches when the pattern isn't first.

3. **Heuristic-Based Decision**: The `requires_indexed_projection()` function tries to guess when to use indexed projection based on shared variables, but this is a heuristic that fails for single-pattern cases.

4. **User Burden**: Users must explicitly write collection variants of every rewrite, which is both tedious and error-prone.

## The Core Insight

Looking at the RhoCalc example:

```rust
rewrites {
    // Base rewrite - should work anywhere
    (PDrop (NQuote P)) => P;
    
    // Generic congruence - automatically lifts any rewrite into collections
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

The **congruence rule** is the key! It says: "if any term S can rewrite to T, then we can rewrite S inside a PPar collection to T inside that collection."

But currently, this congruence rule is **not** generating the right code.

## Proposed Architecture

### Separation of Concerns

1. **Base Rewrites** (no collections)
   - Written without collection syntax: `(PDrop (NQuote P)) => P`
   - Generate simple pattern matching on individual terms
   - Result: `rw_proc(PDrop(NQuote(p)), p) <-- proc(p)`

2. **Collection Congruence** (automatic lifting)
   - Written as explicit congruence: `if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})`
   - Generate **projection relations** that connect collections to their elements
   - Use **indexed joins** to match rewrites within collections
   - Result: Efficient, order-independent matching

### How It Should Work

#### Step 1: Base Rewrite Generation

For base rewrite `(PDrop (NQuote P)) => P`:

```rust
// Simple direct matching
rw_proc(s, t) <--
    proc(s),
    if let Proc::PDrop(n) = s,
    if let Name::NQuote(p) = &**n,
    let t = (**p).clone();
```

#### Step 2: Collection Projection Relations

For constructor `PPar(HashBag<Proc>)`, automatically generate:

```rust
// Generic projection: relates a PPar to each contained Proc
relation ppar_contains(Proc, Proc);  // (parent, element)

ppar_contains(parent.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag) = parent,
    for (elem, _count) in bag.iter();
```

This creates a **database** of all collection-element relationships.

#### Step 3: Collection Congruence via Projection

For congruence `if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})`:

```rust
rw_proc(parent, result) <--
    ppar_contains(parent, elem),  // Find an element in the collection
    rw_proc(elem, elem_rewritten),  // That element can be rewritten
    if let Proc::PPar(ref bag) = parent,
    let rest = {
        let mut b = bag.clone();
        b.remove(&elem);
        b
    },
    let result = Proc::PPar({
        let mut bag = rest;
        Proc::insert_into_ppar(&mut bag, elem_rewritten);
        bag
    });
```

**Key Properties:**
- Uses projection relation, so ALL elements are checked
- Order-independent: `ppar_contains` iterates exhaustively
- Efficient: Ascent's indexing optimizes the join
- Automatic: Works for any base rewrite, no special cases

## Implementation Plan

### Phase 1: Collection Projection Relations (Automatic)

**Goal**: For every constructor with a collection field, automatically generate projection relations.

**Changes to `ascent_gen.rs`**:

```rust
fn generate_collection_projection_relations(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut relations = Vec::new();
    
    for rule in &theory.terms {
        if let Some((coll_idx, elem_cat)) = find_collection_field(rule) {
            let parent_cat = &rule.category;
            let constructor = &rule.label;
            
            // Relation name: <constructor>_contains
            let rel_name = format_ident!("{}_contains", 
                                         constructor.to_string().to_lowercase());
            let parent_cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
            let elem_cat_lower = format_ident!("{}", elem_cat.to_string().to_lowercase());
            
            relations.push(quote! {
                relation #rel_name(#parent_cat, #elem_cat);
                
                #rel_name(parent.clone(), elem.clone()) <--
                    #parent_cat_lower(parent),
                    if let #parent_cat::#constructor(ref bag_field) = parent,
                    for (elem, _count) in bag_field.iter();
            });
        }
    }
    
    relations
}
```

### Phase 2: Congruence Rewrite Generation (Enhanced)

**Goal**: Make `generate_congruence_rewrite` handle collection constructors properly.

**Changes to `ascent_gen.rs`**:

Currently, `generate_congruence_rewrite` handles:
```rust
if S => T then (POutput chan S) => (POutput chan T)
```

We need to extend it to handle:
```rust
if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})
```

**Detection**: Check if the congruence LHS contains a `CollectionPattern`.

**Generation**: Use the projection relation instead of direct pattern matching.

```rust
fn generate_collection_congruence(
    category: &Ident,
    rw_rel: &Ident,
    constructor: &Ident,
    source_var: &Ident,
    rest_var: &Option<Ident>,
) -> TokenStream {
    let contains_rel = format_ident!("{}_contains", 
                                     constructor.to_string().to_lowercase());
    let source_lower = format_ident!("{}", source_var.to_string().to_lowercase());
    let source_cat = /* infer from theory */;
    let source_rw = format_ident!("rw_{}", source_cat.to_string().to_lowercase());
    
    quote! {
        #rw_rel(parent, result) <--
            #contains_rel(parent, elem),
            #source_rw(elem, elem_rewritten),
            if let #category::#constructor(ref bag) = parent,
            let rest = {
                let mut b = bag.clone();
                b.remove(&elem);
                b
            },
            let result = #category::#constructor({
                let mut bag = rest;
                #category::insert_into_#constructor_lower(&mut bag, elem_rewritten);
                bag
            });
    }
}
```

### Phase 3: Deprecate Collection Syntax in Base Rewrites

**Goal**: Simplify the user experience by disallowing collection patterns in base rewrites.

Users should write:
```rust
rewrites {
    // Base rewrite
    (PDrop (NQuote P)) => P;
    
    // Explicit congruence
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

NOT:
```rust
rewrites {
    // DEPRECATED: Don't mix concerns
    (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});
}
```

**Validation**: Add a check in `validator.rs`:

```rust
fn validate_rewrite_rule(rule: &RewriteRule) -> Result<(), String> {
    if rule.premise.is_none() {
        // Base rewrite - must not contain collection patterns
        if contains_collection_pattern(&rule.left) {
            return Err(format!(
                "Base rewrites cannot contain collection patterns. \
                 Write the base rewrite without collections, then use \
                 a congruence rule to lift it: \
                 'if S => T then (Constructor {{S, ...rest}}) => (Constructor {{T, ...rest}})'"
            ));
        }
    }
    Ok(())
}
```

### Phase 4: Communication Rewrite (Special Case)

**Challenge**: The communication rewrite is special:
```rust
(PPar {(PInput chan x P), (POutput chan Q), ...rest})
    => (PPar {(subst P x (NQuote Q)), ...rest});
```

This matches **two** elements simultaneously with a shared variable `chan`.

**Solution**: This is still a base rewrite, but it operates on the PPar constructor itself. It's not a congruence rule - it's a genuine multi-element match.

Keep the indexed projection approach for this case:
- It has shared variables (`chan`)
- It matches multiple patterns simultaneously
- It's a fundamental rewrite of the PPar structure, not lifting an inner rewrite

**Detection**: A rewrite is a "collection-level base rewrite" if:
1. LHS is `Apply(Constructor, [CollectionPattern])`
2. CollectionPattern has multiple element patterns (not just one)
3. OR: CollectionPattern elements share variables

For these cases, continue using the indexed projection approach.

## Summary of Changes

### ascent_gen.rs

1. Add `generate_collection_projection_relations()` - called automatically
2. Enhance `generate_congruence_rewrite()` to detect and handle collection congruence
3. Add `generate_collection_congruence()` for the new pattern

### rewrite_gen.rs

1. Simplify `generate_rewrite_clauses()` to only handle base rewrites
2. Remove the complex heuristics in `requires_indexed_projection()`
3. Keep indexed projection ONLY for multi-element collection rewrites (like communication)

### validator.rs

1. Add validation: base rewrites (no premise) cannot have collection patterns with single element
2. Allow: `(PPar {A, B})` (multi-element, collection-level rewrite)
3. Disallow: `(PPar {(PDrop ...), ...rest})` (should use congruence instead)

## Benefits

1. **Correctness**: No more order-dependent bugs
2. **Clarity**: Clear separation between base rewrites and congruence
3. **Efficiency**: Projection relations computed once, reused many times
4. **Simplicity**: Less code, fewer heuristics
5. **Extensibility**: Easy to add new collection types

## Migration Path

### For RhoCalc

**Before**:
```rust
rewrites {
    (PPar {(PInput chan x P), (POutput chan Q), ...rest})
        => (PPar {(subst P x (NQuote Q)), ...rest});
    
    (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});

    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

**After**:
```rust
rewrites {
    // Communication: multi-element collection rewrite (KEEP)
    (PPar {(PInput chan x P), (POutput chan Q), ...rest})
        => (PPar {(subst P x (NQuote Q)), ...rest});
    
    // Drop-quote: base rewrite (SIMPLIFY)
    (PDrop (NQuote P)) => P;

    // Generic congruence (KEEP)
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

The congruence rule will automatically lift the `(PDrop (NQuote P)) => P` rewrite into any PPar collection.

## Open Questions

1. **Multiple Collection Fields**: What if a constructor has multiple collection fields?
   - Generate separate projection relations for each
   - Name them: `constructor_contains_field0`, `constructor_contains_field1`

2. **Nested Collections**: What about `PPar(HashBag<PPar(HashBag<Proc>)>)`?
   - Projection relations handle this naturally - they're recursive
   - Inner projections create more facts, outer projections relate to them

3. **Performance**: Will this create too many facts?
   - Projection relations are linear in collection size
   - Much better than the current eager deconstruction (which is exponential)
   - Can be optimized with lazy evaluation if needed

4. **Backward Compatibility**: How to migrate existing theories?
   - Add a warning for deprecated patterns
   - Provide automatic migration in a future version
   - Document the new best practices clearly

## Next Steps

1. Create a minimal test case to validate the approach
2. Implement Phase 1 (projection relations)
3. Test with RhoCalc communication (should still work)
4. Implement Phase 2 (collection congruence)
5. Test with RhoCalc drop-quote (should now work)
6. Add validation (Phase 3)
7. Update documentation and examples

## References

- Current bug: `{*@(a!(0)), b!(0)}` doesn't rewrite because `*@(a!(0))` isn't first
- Generated code: Lines 476-492 in terminal output (uses `.iter().next()`)
- Related docs: `COLLECTION-PATTERN-FIX.md`, `INDEXED-PROJECTION-AUTOMATION-PLAN.md`

