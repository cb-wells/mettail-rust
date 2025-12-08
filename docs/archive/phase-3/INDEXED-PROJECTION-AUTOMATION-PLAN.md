# Automated Indexed Projection: Implementation Plan

## Status: Ready to Implement
**Prototype**: ✅ Verified working in `rhocalc.rs`
**Performance**: ✅ 10.8ms, order-independent
**Correctness**: ✅ Handles N>2 elements correctly

---

## Overview

Replace the current order-dependent collection pattern matching with automatic generation of indexed projection relations that leverage Ascent's optimized hash joins.

### Current Problem
```rust
// Auto-generated (ORDER-DEPENDENT)
let elem_0 = bag.iter().next()      // Takes first element
let elem_1 = bag.iter().nth(1)      // Takes second element
if let PInput(...) = elem_0 { ... } // Hopes it matches
```

### Solution
```rust
// Auto-generated (ORDER-INDEPENDENT)
relation pinput_proj(Parent, Channel, ...captures);
relation poutput_proj(Parent, Channel, ...captures);

// Extract and index by channel - O(n)
pinput_proj(...) <-- proc(parent), for elem in bag, if let PInput(...) = elem;

// Join on shared variables - O(1) hash lookup
rw_proc(...) <-- pinput_proj(p, chan, ...), poutput_proj(p, chan, ...);
```

---

## Architecture Changes

### Phase 6.1: Detection & Analysis (File: `rewrite_gen.rs`)

**Goal**: Detect when collection patterns need indexed projections

#### Step 1.1: Identify Projection-Requiring Patterns

Add to `generate_rewrite_rules()`:
```rust
fn requires_indexed_projection(rule: &RewriteRule) -> bool {
    // Check if LHS has:
    // 1. Collection pattern with nested Apply patterns
    // 2. Shared variables across multiple nested patterns
    match &rule.lhs {
        Expr::Apply { constructor, args } => {
            for arg in args {
                if let Expr::CollectionPattern { elements, .. } = arg {
                    // Check if elements contain Apply with shared vars
                    let shared_vars = find_shared_variables(elements);
                    if !shared_vars.is_empty() && has_nested_apply(elements) {
                        return true;
                    }
                }
            }
        }
        _ => {}
    }
    false
}
```

#### Step 1.2: Extract Projection Metadata

New struct to capture projection info:
```rust
struct ProjectionSpec {
    /// The collection field being matched
    collection_field_idx: usize,

    /// Nested patterns within the collection
    element_patterns: Vec<ElementPattern>,

    /// Variables shared across patterns (join keys)
    shared_variables: Vec<String>,

    /// Rest variable binding, if present
    rest_variable: Option<String>,
}

struct ElementPattern {
    /// The nested constructor (e.g., PInput, POutput)
    constructor: Ident,

    /// Position in the collection pattern
    pattern_idx: usize,

    /// Variables to capture
    captures: Vec<(String, Ident)>,  // (var_name, category)

    /// Which captures are join keys
    join_key_indices: Vec<usize>,
}
```

#### Step 1.3: Build Projection Spec

```rust
fn analyze_collection_pattern(
    pattern: &Expr,
    theory: &TheoryDef,
) -> Option<ProjectionSpec> {
    match pattern {
        Expr::CollectionPattern { elements, rest, .. } => {
            let mut element_patterns = Vec::new();
            let mut all_vars = HashMap::new();

            // Analyze each element pattern
            for (idx, elem) in elements.iter().enumerate() {
                if let Expr::Apply { constructor, args } = elem {
                    let captures = extract_captures(args, theory);
                    element_patterns.push(ElementPattern {
                        constructor: constructor.clone(),
                        pattern_idx: idx,
                        captures: captures.clone(),
                        join_key_indices: vec![], // Filled later
                    });

                    // Track variable occurrences
                    for (var, _) in &captures {
                        all_vars.entry(var.clone())
                            .or_insert_with(Vec::new)
                            .push(idx);
                    }
                }
            }

            // Find shared variables (appear in multiple patterns)
            let shared_variables: Vec<String> = all_vars.iter()
                .filter(|(_, indices)| indices.len() > 1)
                .map(|(var, _)| var.clone())
                .collect();

            // Mark which captures are join keys
            for pattern in &mut element_patterns {
                for (i, (var, _)) in pattern.captures.iter().enumerate() {
                    if shared_variables.contains(var) {
                        pattern.join_key_indices.push(i);
                    }
                }
            }

            Some(ProjectionSpec {
                collection_field_idx: 0, // Computed by caller
                element_patterns,
                shared_variables,
                rest_variable: rest.clone().map(|r| r.to_string()),
            })
        }
        _ => None,
    }
}
```

---

### Phase 6.2: Code Generation (File: `rewrite_gen.rs`)

**Goal**: Generate Ascent projection relations and join-based rewrites

#### Step 2.1: Generate Projection Relations

```rust
fn generate_projection_relations(
    rule_idx: usize,
    spec: &ProjectionSpec,
    category: &Ident,
    grammar_rule: &GrammarRule,
    theory: &TheoryDef,
) -> Vec<TokenStream> {
    let mut relations = Vec::new();

    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_{}_{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );

        // Build relation signature: (Parent, JoinKey1, JoinKey2, ..., Capture1, Capture2, ..., OriginalElem)
        let mut field_types = vec![
            quote! { #category },  // Parent term
        ];

        // Add join key types (shared variables)
        for var in &spec.shared_variables {
            let var_category = get_variable_category(var, &elem_pattern.captures);
            field_types.push(quote! { #var_category });
        }

        // Add other capture types
        for (var, cat) in &elem_pattern.captures {
            if !spec.shared_variables.contains(var) {
                field_types.push(quote! { #cat });
            }
        }

        // Add original element (for reconstruction)
        let element_cat = &grammar_rule.items.iter()
            .find_map(|item| match item {
                crate::ast::GrammarItem::Collection { element_type, .. } => Some(element_type),
                _ => None
            })
            .expect("Collection field not found");
        field_types.push(quote! { #element_cat });

        relations.push(quote! {
            relation #rel_name(#(#field_types),*);
        });
    }

    relations
}
```

#### Step 2.2: Generate Extraction Rules

```rust
fn generate_extraction_rules(
    rule_idx: usize,
    spec: &ProjectionSpec,
    category: &Ident,
    term_name: &Ident,
    grammar_rule: &GrammarRule,
    theory: &TheoryDef,
) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let label = &grammar_rule.label;

    // Find the collection field
    let collection_field_name = format_ident!("bag_field");

    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_{}_{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );

        // Generate pattern matching for nested constructor
        let elem_var = format_ident!("elem");
        let mut capture_vars = Vec::new();
        let mut capture_exprs = Vec::new();

        for (var_name, _) in &elem_pattern.captures {
            let var_ident = format_ident!("{}", var_name);
            capture_vars.push(var_ident.clone());

            // Generate extraction expression (may need unbinding, dereferencing, etc.)
            let extraction = generate_capture_extraction(var_name, elem_pattern, theory);
            capture_exprs.push(extraction);
        }

        // Build relation fact with join keys first, then other captures
        let mut fact_args = vec![
            quote! { parent.clone() },
        ];

        // Add join keys in order
        for shared_var in &spec.shared_variables {
            if let Some((var_name, _)) = elem_pattern.captures.iter()
                .find(|(v, _)| v == shared_var)
            {
                let var_ident = format_ident!("{}", var_name);
                fact_args.push(quote! { #var_ident.clone() });
            }
        }

        // Add non-join captures
        for (var_name, _) in &elem_pattern.captures {
            if !spec.shared_variables.contains(var_name) {
                let var_ident = format_ident!("{}", var_name);
                fact_args.push(quote! { #var_ident.clone() });
            }
        }

        // Add original element
        fact_args.push(quote! { #elem_var.clone() });

        let elem_constructor = &elem_pattern.constructor;
        let element_category = get_element_category(grammar_rule);

        rules.push(quote! {
            #rel_name(#(#fact_args),*) <--
                #cat_lower(parent),
                if let #category::#label(ref #collection_field_name) = parent,
                for (#elem_var, _count) in #collection_field_name.iter(),
                if let #element_category::#elem_constructor(#(ref #capture_vars),*) = #elem_var,
                #(#capture_exprs),*;
        });
    }

    rules
}
```

#### Step 2.3: Generate Join-Based Rewrite

```rust
fn generate_join_rewrite(
    rule_idx: usize,
    spec: &ProjectionSpec,
    rule: &RewriteRule,
    category: &Ident,
    theory: &TheoryDef,
) -> TokenStream {
    let mut join_clauses = Vec::new();
    let parent_var = format_ident!("parent");

    // Create variables for shared join keys
    let join_key_vars: Vec<_> = spec.shared_variables.iter()
        .map(|v| format_ident!("{}", v))
        .collect();

    // Generate join clause for each projection
    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_{}_{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );

        let mut join_args = vec![parent_var.clone()];
        join_args.extend(join_key_vars.clone());

        // Add capture variables
        for (var_name, _) in &elem_pattern.captures {
            if !spec.shared_variables.contains(var_name) {
                join_args.push(format_ident!("{}", var_name));
            }
        }

        // Add element variable
        let elem_var = format_ident!("elem_{}", elem_pattern.pattern_idx);
        join_args.push(elem_var);

        join_clauses.push(quote! {
            #rel_name(#(#join_args),*)
        });
    }

    // Generate rest construction
    let elem_vars: Vec<_> = (0..spec.element_patterns.len())
        .map(|i| format_ident!("elem_{}", i))
        .collect();

    let rest_construction = if let Some(rest_var) = &spec.rest_variable {
        let rest_ident = format_ident!("{}", rest_var);
        quote! {
            if let #category::PPar(ref bag) = #parent_var,
            let #rest_ident = {
                let mut b = bag.clone();
                #(b.remove(&#elem_vars);)*
                b
            }
        }
    } else {
        quote! {}
    };

    // Generate RHS construction (delegate to existing code)
    let rhs_code = generate_ascent_rhs(&rule.rhs, &mut HashMap::new(), theory);

    quote! {
        rw_proc(#parent_var.clone(), result) <--
            #(#join_clauses),*,
            #rest_construction,
            let result = #rhs_code;
    }
}
```

---

### Phase 6.3: Integration (File: `rewrite_gen.rs`)

**Goal**: Wire everything together in the main generation pipeline

#### Step 3.1: Modify `generate_rewrite_rules()`

```rust
pub fn generate_rewrite_rules(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut all_rules = Vec::new();

    for (idx, rule) in theory.rewrites.iter().enumerate() {
        if requires_indexed_projection(rule) {
            // NEW PATH: Generate indexed projection approach
            let spec = analyze_collection_pattern(&rule.lhs, theory)
                .expect("Failed to analyze collection pattern");

            let (category, grammar_rule) = find_constructor_info(&rule.lhs, theory);

            // Generate projection relations
            let relations = generate_projection_relations(idx, &spec, category, grammar_rule, theory);
            all_rules.extend(relations);

            // Generate extraction rules
            let extractions = generate_extraction_rules(idx, &spec, category, &format_ident!("s"), grammar_rule, theory);
            all_rules.extend(extractions);

            // Generate join-based rewrite
            let rewrite = generate_join_rewrite(idx, &spec, rule, category, theory);
            all_rules.push(rewrite);

        } else {
            // EXISTING PATH: Use current generation logic
            let rewrite = generate_ascent_rewrite_rule(idx, rule, theory);
            all_rules.push(rewrite);
        }
    }

    all_rules
}
```

#### Step 3.2: Add Helper Functions

```rust
fn get_variable_category(var: &str, captures: &[(String, Ident)]) -> &Ident {
    captures.iter()
        .find(|(v, _)| v == var)
        .map(|(_, cat)| cat)
        .expect("Variable not found in captures")
}

fn get_element_category(grammar_rule: &GrammarRule) -> Ident {
    grammar_rule.items.iter()
        .find_map(|item| match item {
            crate::ast::GrammarItem::Collection { element_type, .. } => Some(element_type.clone()),
            _ => None
        })
        .expect("Collection field not found")
}

fn find_constructor_info<'a>(
    expr: &Expr,
    theory: &'a TheoryDef
) -> (&'a Ident, &'a GrammarRule) {
    match expr {
        Expr::Apply { constructor, .. } => {
            let rule = theory.terms.iter()
                .find(|r| r.label == *constructor)
                .expect("Constructor not found");
            (&rule.category, rule)
        }
        _ => panic!("Expected Apply expression")
    }
}
```

---

### Phase 6.4: Testing Strategy

#### Test 1: Existing Tests Must Pass
- `test_rest_patterns.rs` - Simple 2-element case
- Should work with both old and new approach

#### Test 2: Order Independence
```rust
// Test all permutations
{a!(0), for(a->x0){*x0}, b!(0)} -> {*@(0), b!(0)}
{b!(0), a!(0), for(a->x0){*x0}} -> {b!(0), *@(0)}
{for(a->x0){*x0}, b!(0), a!(0)} -> {*@(0), b!(0)}
```

#### Test 3: Multiple Matches
```rust
// Should find both pairs
{a!(0), for(a->x0){*x0}, b!(0), for(b->y0){*y0}}
  -> {*@(0), *@(0)}
```

#### Test 4: No Match
```rust
// Should remain unchanged
{a!(0), for(b->x0){*x0}} -> {a!(0), for(b->x0){*x0}}
```

#### Test 5: Complex Patterns
```rust
// Multiple join keys
pattern: (Foo {(Bar x y A), (Baz x y B), ...rest})
```

---

### Phase 6.5: Performance Validation

#### Metrics to Track
1. **Compilation time**: Should remain < 5 seconds
2. **Runtime performance**: Should match manual version (~10ms)
3. **Memory overhead**: 2n projection facts (acceptable)
4. **Scaling**: Linear with collection size

#### Benchmark Suite
```rust
// rhocalc_bench.rs
#[bench]
fn bench_2_elements(b: &mut Bencher) { ... }

#[bench]
fn bench_10_elements(b: &mut Bencher) { ... }

#[bench]
fn bench_100_elements(b: &mut Bencher) { ... }
```

---

## Implementation Checklist

### Phase 6.1: Detection ⏳
- [ ] Add `requires_indexed_projection()` function
- [ ] Define `ProjectionSpec` and `ElementPattern` structs
- [ ] Implement `analyze_collection_pattern()`
- [ ] Add helper: `find_shared_variables()`
- [ ] Add helper: `extract_captures()`
- [ ] Unit tests for detection logic

### Phase 6.2: Generation ⏳
- [ ] Implement `generate_projection_relations()`
- [ ] Implement `generate_extraction_rules()`
- [ ] Implement `generate_join_rewrite()`
- [ ] Implement `generate_capture_extraction()`
- [ ] Handle binder unbinding in captures
- [ ] Handle Box dereferencing in captures
- [ ] Unit tests for code generation

### Phase 6.3: Integration ⏳
- [ ] Modify `generate_rewrite_rules()` to use new path
- [ ] Add `find_constructor_info()` helper
- [ ] Add `get_variable_category()` helper
- [ ] Add `get_element_category()` helper
- [ ] Integration tests

### Phase 6.4: Testing ⏳
- [ ] Test: 2-element case (existing)
- [ ] Test: 3-element case with permutations
- [ ] Test: Multiple matches in one bag
- [ ] Test: No matches
- [ ] Test: Complex patterns with multiple join keys
- [ ] Verify all existing tests pass

### Phase 6.5: Documentation ⏳
- [ ] Update `COLLECTION-TYPES-DESIGN.md`
- [ ] Update `COLLECTION-MATCHING-SOLUTION.md` with "IMPLEMENTED" status
- [ ] Add user guide section on automatic projections
- [ ] Document performance characteristics
- [ ] Update `ROADMAP.md`

### Phase 6.6: Cleanup ⏳
- [ ] Remove old iteration-based code
- [ ] Remove `COLLECTION-MATCHING-LIMITATION.md` (no longer applicable)
- [ ] Update checklist with Phase 6 complete
- [ ] Create summary document

---

## Risk Analysis

### Risk 1: Complex Capture Extraction
**Issue**: Extracting captures from nested patterns (binders, boxes) is complex
**Mitigation**: Reuse existing `generate_ascent_pattern()` logic where possible
**Fallback**: Generate simplified extraction for MVP, optimize later

### Risk 2: Multiple Join Keys
**Issue**: Patterns with >1 shared variable need multi-column joins
**Mitigation**: Ascent supports multi-column joins naturally
**Test**: Add dedicated test case

### Risk 3: Performance Regression
**Issue**: Projection overhead might hurt small collections
**Mitigation**: Profile and optimize projection generation
**Fallback**: Add heuristic to use old approach for N≤2 elements

### Risk 4: Code Complexity
**Issue**: Code generation is already complex
**Mitigation**: Break into small, well-tested functions
**Fallback**: Keep manual approach as escape hatch

---

## Success Criteria

1. ✅ **Correctness**: All permutations of elements match correctly
2. ✅ **Performance**: Within 10% of manual version (~10ms)
3. ✅ **Completeness**: Handles all collection pattern variants
4. ✅ **Maintainability**: Clean, well-documented code
5. ✅ **Compatibility**: All existing tests pass

---

## Timeline Estimate

- **Phase 6.1** (Detection): 3-4 hours
- **Phase 6.2** (Generation): 6-8 hours
- **Phase 6.3** (Integration): 2-3 hours
- **Phase 6.4** (Testing): 3-4 hours
- **Phase 6.5** (Documentation): 2 hours
- **Phase 6.6** (Cleanup): 1 hour

**Total**: 17-22 hours (2-3 full work sessions)

---

## Next Steps

1. **Start with Phase 6.1**: Build detection and analysis infrastructure
2. **Validate with manual test**: Ensure spec extraction is correct
3. **Implement Phase 6.2**: Generate code matching manual version
4. **Test incrementally**: Verify each phase before moving on

Let's begin! 🚀

