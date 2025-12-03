# Congruence-Driven Projection Generation

**Date**: November 13, 2025  
**Status**: Design Phase  
**Priority**: Critical (Architectural Refactoring)

## Executive Summary

The current approach to collection rewriting is backwards: base rewrites decide whether they need projections, leading to heuristic-based code generation that fails for single-element patterns. Instead, **congruence rules** should drive projection generation by analyzing the LHS of base rewrites and creating targeted projection relations.

**Key simplification discovered:** Base rewrites never need `...rest` patterns. The congruence rule's `...rest` automatically handles embedding base rewrites in larger collections. This makes the user-facing syntax much cleaner and the implementation more elegant.

### Before (Current - Broken)
```rust
rewrites {
    // User forced to write workarounds
    (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});
    (PPar {(PInput chan x P), (POutput chan Q), ...rest}) 
        => (PPar {(subst P x (NQuote Q)), ...rest});
}
// Problem: Uses .iter().next() for single patterns → order-dependent bugs
```

### After (Proposed - Clean)
```rust
rewrites {
    // Clean base rewrites (no ...rest!)
    (PDrop (NQuote P)) => P;
    (PPar {(PInput chan x P), (POutput chan Q)}) 
        => (PPar {(subst P x (NQuote Q))});
    
    // One congruence rule lifts ALL rewrites
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
// Solution: Congruence analyzes base rewrites → generates projections → correct everywhere
```

**Impact:** Correctness fix + simpler syntax + cleaner implementation.

## Problem Statement

### Current Architecture (Incorrect)

```
Base Rewrite → Heuristic Analysis → Maybe Generate Projection?
                 ↓
        (requires_indexed_projection)
                 ↓
   Checks: shared variables? → YES → indexed projection
                              → NO  → naive extraction (.iter().next())
```

### Issues

1. **Heuristic Failure**: Single-element patterns without shared variables use naive extraction
2. **Wrong Responsibility**: Base rewrites shouldn't know about their embedding context
3. **Code Duplication**: Both the explicit base rewrite AND the congruence generate rules
4. **Ordering Bugs**: Naive extraction only checks first element (arbitrary iteration order)

### Example: RhoCalc Drop-Quote

**User writes:**
```rust
rewrites {
    (PDrop (NQuote P)) => P;                                    // Base rewrite
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest}); // Congruence
}
```

**Current behavior:**
- Base rewrite generates: `rw_proc(s, t) <-- proc(s), if let Proc::PDrop(...) ...`
- User also writes: `(PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest})` as workaround
- This generates projection, but since no shared variables, uses naive `.iter().next()` (BUG!)
- Congruence rule also tries to generate: `rw_proc(parent, result) <-- ppar_contains(parent, elem), rw_proc(*elem, ...)`

**Result**: Two different code paths, one broken, redundant user code.

## Proposed Architecture

### Congruence-Driven Generation

```
Congruence Rule → Analyze Element Type Base Rewrites → Generate Projections
     ↓                          ↓                              ↓
  if S => T              Find all rewrites               Create targeted
  (PPar {S, ...})        where S appears                 projection relations
                         as root constructor              for those patterns
```

### Key Insight

A congruence rule like:
```rust
if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})
```

Means: "Apply **any** rewrite on Proc to elements inside PPar."

So we should:
1. Find all base rewrites on Proc: `(PDrop (NQuote P)) => P`, `(PPar {(PInput ...), (POutput ...)})`
2. Extract the LHS element patterns that could appear in collections
3. Generate projection relations for each element pattern
4. Base rewrites without collections don't generate standalone clauses (congruence handles them)

## Detailed Design

### Phase 1: Allow Collections in Base Rewrites

Base rewrites can contain collection patterns (like the communication rule):
```rust
rewrites {
    // Base rewrite with collection - matches specific elements
    (PPar {(PInput chan x P), (POutput chan Q)})
        => (PPar {(subst P x (NQuote Q))});
    
    // Base rewrite without collection - operates on single terms
    (PDrop (NQuote P)) => P;
    
    // Congruence - lifts any base rewrite on Proc into PPar
    // The ...rest pattern ONLY appears in the congruence rule
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

**Key insights:**
- Base rewrites match exact structures (no `...rest` needed)
- The congruence rule handles embedding in larger collections
- The congruence analyzes ALL base rewrites (with or without collections) to generate projections

### Phase 2: Congruence-Driven Projection Generation

#### New Function in `ascent_gen.rs`

```rust
/// Generate projection relations driven by congruence rules
/// For each congruence `if S => T then (Constructor {S, ...rest}) => ...`:
/// 1. Find all base rewrites that touch category S
/// 2. Find all regular congruences on category S (non-collection)
/// 3. Extract element patterns from both
/// 4. Generate projection relations for all patterns
fn generate_congruence_projections(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut projections = Vec::new();
    
    for (cong_idx, rule) in theory.rewrites.iter().enumerate() {
        // Only process congruence rules
        if let Some((source_var, _target_var)) = &rule.premise {
            // Check if this is a collection congruence
            if let Some(cong_info) = extract_collection_congruence_info(&rule.left, source_var) {
                // cong_info contains:
                // - constructor: PPar
                // - element_category: Proc
                // - rest_var: Option<Ident>
                
                // Find all base rewrites that involve this element category
                let base_rewrites = find_base_rewrites_for_category(&cong_info.element_category, theory);
                
                // Find all REGULAR congruences on this element category
                // These are congruence rules that aren't collection-based
                let regular_congruences = find_regular_congruences_for_category(&cong_info.element_category, theory);
                
                // Generate projections for base rewrites
                for (base_idx, base_rule) in base_rewrites.iter().enumerate() {
                    // Extract element patterns from the base rewrite's LHS
                    let element_patterns = extract_element_patterns(
                        &base_rule.left, 
                        &cong_info,
                        theory
                    );
                    
                    // Generate projection for each element pattern
                    for (pat_idx, pattern) in element_patterns.iter().enumerate() {
                        let proj = generate_projection_for_pattern(
                            cong_idx,
                            base_idx,
                            pat_idx,
                            &cong_info,
                            pattern,
                            theory,
                            ProjectionKind::BaseRewrite,
                        );
                        projections.push(proj);
                    }
                }
                
                // Generate projections for regular congruences
                // e.g., if S => T then (PNew x S) => (PNew x T)
                // needs projection for PNew patterns in collections
                for (reg_idx, reg_cong) in regular_congruences.iter().enumerate() {
                    let congruence_pattern = extract_regular_congruence_pattern(reg_cong, theory);
                    
                    let proj = generate_projection_for_regular_congruence(
                        cong_idx,
                        reg_idx,
                        &cong_info,
                        &congruence_pattern,
                        theory,
                    );
                    projections.push(proj);
                }
            }
        }
    }
    
    projections
}

#[derive(Debug)]
enum ProjectionKind {
    BaseRewrite,        // From a base rewrite pattern
    RegularCongruence,  // From a regular (non-collection) congruence
}

#[derive(Debug)]
struct CollectionCongruenceInfo {
    constructor: Ident,           // PPar
    parent_category: Ident,       // Proc
    element_category: Ident,      // Proc (for PPar)
    source_var: Ident,            // S in "if S => T"
    target_var: Ident,            // T in "if S => T"
    rest_var: Option<Ident>,      // rest in "{S, ...rest}"
}

#[derive(Debug)]
struct RegularCongruencePattern {
    constructor: Ident,           // PNew
    category: Ident,              // Proc
    rewrite_field_idx: usize,     // Which field rewrites (the one matching source_var)
    binder_var: Option<Ident>,    // x in "PNew x S" (if it's a binding constructor)
    other_fields: Vec<FieldInfo>, // Non-rewriting fields
}

fn extract_collection_congruence_info(lhs: &Expr, source_var: &Ident) -> Option<CollectionCongruenceInfo> {
    // Parse: (Constructor {source_var, ...rest})
    if let Expr::Apply { constructor, args } = lhs {
        for arg in args {
            if let Expr::CollectionPattern { elements, rest, .. } = arg {
                // Check if elements contains just source_var
                if elements.len() == 1 {
                    if let Expr::Var(v) = &elements[0] {
                        if v == source_var {
                            // Found it! Get category info
                            let parent_category = /* ... from theory ... */;
                            let element_category = /* ... from theory ... */;
                            
                            return Some(CollectionCongruenceInfo {
                                constructor: constructor.clone(),
                                parent_category,
                                element_category,
                                source_var: source_var.clone(),
                                target_var: /* from premise */,
                                rest_var: rest.as_ref().map(|r| r.clone()),
                            });
                        }
                    }
                }
            }
        }
    }
    None
}

fn find_base_rewrites_for_category<'a>(
    category: &Ident,
    theory: &'a TheoryDef
) -> Vec<&'a RewriteRule> {
    theory.rewrites.iter()
        .filter(|rule| {
            // Base rewrites only (no premise)
            rule.premise.is_none() &&
            // Rule involves the target category (either as root or in collection)
            rule_involves_category(&rule.left, category, theory)
        })
        .collect()
}

/// Find regular (non-collection) congruence rules for a category
/// e.g., if S => T then (PNew x S) => (PNew x T)
fn find_regular_congruences_for_category<'a>(
    category: &Ident,
    theory: &'a TheoryDef
) -> Vec<&'a RewriteRule> {
    theory.rewrites.iter()
        .filter(|rule| {
            // Has premise (is a congruence)
            if rule.premise.is_none() {
                return false;
            }
            
            // Is NOT a collection congruence
            if is_collection_congruence(rule) {
                return false;
            }
            
            // Operates on the target category
            extract_category(&rule.left, theory)
                .map(|cat| cat == *category)
                .unwrap_or(false)
        })
        .collect()
}

fn is_collection_congruence(rule: &RewriteRule) -> bool {
    // Check if LHS contains CollectionPattern
    contains_collection_pattern(&rule.left)
}

/// Check if a rewrite LHS involves a category
/// This includes:
/// - Root constructor of that category: (PDrop ...)
/// - Collection containing that category: (PPar {(PInput ...), ...})
fn rule_involves_category(expr: &Expr, category: &Ident, theory: &TheoryDef) -> bool {
    // Check root category
    if let Some(root_cat) = extract_category_from_expr(expr, theory) {
        if root_cat == *category {
            return true;
        }
    }
    
    // Check collection element categories
    if let Expr::Apply { args, .. } = expr {
        for arg in args {
            if let Expr::CollectionPattern { .. } = arg {
                // Get collection element type from theory
                if let Some(elem_cat) = get_collection_element_category(expr, theory) {
                    if elem_cat == *category {
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

/// Extract element patterns from a rewrite LHS
/// For `(PDrop (NQuote P)) => P`: returns [(PDrop, ...)]
/// For `(PPar {(PInput ...), (POutput ...)}) => ...`: returns [(PInput, ...), (POutput, ...)]
fn extract_element_patterns(
    lhs: &Expr,
    cong_info: &CollectionCongruenceInfo,
    theory: &TheoryDef,
) -> Vec<ElementPatternInfo> {
    let mut patterns = Vec::new();
    
    // Case 1: LHS is a direct constructor of the element category
    // e.g., (PDrop (NQuote P))
    if let Some(root_cat) = extract_category_from_expr(lhs, theory) {
        if root_cat == cong_info.element_category {
            // This is a direct pattern on the element category
            if let Some(pattern_info) = analyze_constructor_pattern(lhs, theory) {
                patterns.push(pattern_info);
            }
        }
    }
    
    // Case 2: LHS contains a collection with element patterns
    // e.g., (PPar {(PInput ...), (POutput ...)})
    if let Expr::Apply { constructor, args } = lhs {
        if *constructor == cong_info.constructor {
            // This is the collection constructor we're interested in
            for arg in args {
                if let Expr::CollectionPattern { elements, .. } = arg {
                    // Extract each element pattern
                    for elem in elements {
                        if let Some(pattern_info) = analyze_constructor_pattern(elem, theory) {
                            patterns.push(pattern_info);
                        }
                    }
                }
            }
        }
    }
    
    patterns
}

#[derive(Debug)]
struct ElementPatternInfo {
    constructor: Ident,      // PInput, POutput, PDrop
    category: Ident,          // Proc, Name
    captures: Vec<CaptureInfo>,
    field_patterns: Vec<FieldPattern>,
}
```

#### Projection Generation for Element Patterns

```rust
/// Generate projection for a regular congruence constructor
/// e.g., for "if S => T then (PNew x S) => (PNew x T)", generate:
/// pnew_proj_c0_r1(parent, x, body, elem) to extract PNew patterns from collections
fn generate_projection_for_regular_congruence(
    cong_idx: usize,
    reg_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    pattern: &RegularCongruencePattern,
    theory: &TheoryDef,
) -> TokenStream {
    let rel_name = format_ident!(
        "{}_proj_c{}_r{}",
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        reg_idx
    );
    
    let parent_cat = &cong_info.parent_category;
    let parent_cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
    let collection_constructor = &cong_info.constructor;
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;
    
    // For PNew, we need to extract the binder and the body
    // Signature: (parent, binder_var, rewrite_field, elem)
    let mut field_types = vec![quote! { #parent_cat }];
    
    if let Some(_binder) = &pattern.binder_var {
        field_types.push(quote! { mettail_runtime::Binder<String> });
    }
    
    // The field that will be rewritten
    field_types.push(quote! { #elem_cat });
    
    // The original element
    field_types.push(quote! { #elem_cat });
    
    let rel_decl = quote! {
        relation #rel_name(#(#field_types),*);
    };
    
    // Generate pattern extraction
    // For PNew: extract scope, unbind to get (x, body)
    let extraction = if pattern.binder_var.is_some() {
        quote! {
            if let #elem_cat::#elem_constructor(ref scope) = elem,
            let (binder_var, body_box) = scope.clone().unbind(),
            let rewrite_field = (*body_box).clone()
        }
    } else {
        // Non-binding constructor: directly extract field
        quote! {
            if let #elem_cat::#elem_constructor(ref field) = elem,
            let rewrite_field = (**field).clone()
        }
    };
    
    let tuple = if pattern.binder_var.is_some() {
        quote! { (parent.clone(), binder_var.clone(), rewrite_field.clone(), elem.clone()) }
    } else {
        quote! { (parent.clone(), rewrite_field.clone(), elem.clone()) }
    };
    
    let population_rule = quote! {
        #rel_name(#tuple) <--
            #parent_cat_lower(parent),
            if let #parent_cat::#collection_constructor(ref bag_field) = parent,
            for (elem, _count) in bag_field.iter(),
            #extraction;
    };
    
    quote! {
        #rel_decl
        #population_rule
    }
}

fn generate_projection_for_pattern(
    cong_idx: usize,
    base_idx: usize,
    pat_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    pattern: &ElementPatternInfo,
    theory: &TheoryDef,
    kind: ProjectionKind,
) -> TokenStream {
    // Generate projection relation name
    let rel_name = format_ident!(
        "{}_proj_c{}_b{}_p{}", 
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        base_idx,
        pat_idx
    );
    
    // Build relation signature: (Parent, Capture1, Capture2, ..., Element)
    let mut field_types = vec![quote! { #(cong_info.parent_category) }];
    for capture in &pattern.captures {
        let cat = &capture.category;
        if capture.is_binder {
            field_types.push(quote! { mettail_runtime::Binder<String> });
        } else {
            field_types.push(quote! { #cat });
        }
    }
    field_types.push(quote! { #(cong_info.element_category) });
    
    // Generate relation declaration
    let rel_decl = quote! {
        relation #rel_name(#(#field_types),*);
    };
    
    // Generate population rule
    let parent_cat_lower = format_ident!("{}", cong_info.parent_category.to_string().to_lowercase());
    let constructor = &cong_info.constructor;
    let parent_cat = &cong_info.parent_category;
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;
    
    // Generate pattern matching for element
    let elem_pattern_match = generate_pattern_match(pattern);
    
    // Generate captures
    let capture_bindings = generate_capture_bindings(pattern);
    
    // Generate tuple for relation
    let rel_tuple = generate_relation_tuple(pattern);
    
    let population_rule = quote! {
        #rel_name(#rel_tuple) <--
            #parent_cat_lower(parent),
            if let #parent_cat::#constructor(ref bag_field) = parent,
            for (elem, _count) in bag_field.iter(),
            if let #elem_cat::#elem_constructor(#elem_pattern_match) = elem,
            #capture_bindings;
    };
    
    quote! {
        #rel_decl
        #population_rule
    }
}

#[derive(Debug)]
struct PatternInfo {
    root_constructor: Ident,    // PDrop
    category: Ident,             // Proc
    captures: Vec<CaptureInfo>,  // [(P, Proc, field_idx=0, is_binder=false)]
    field_patterns: Vec<FieldPattern>,
}

#[derive(Debug)]
struct FieldPattern {
    field_idx: usize,
    constructor: Option<Ident>,  // NQuote
    category: Ident,              // Name
    nested: Option<Box<FieldPattern>>,
}
```

### Phase 3: Modify Base Rewrite Generation

Base rewrites generate different code depending on their structure:

**Case 1: Base rewrite with collection** 
```rust
(PPar {(PInput chan x P), (POutput chan Q)}) => (PPar {(subst P x (NQuote Q))})
```
- Operates on the collection structure itself
- Generates a join-based rewrite using projections from congruence
- Does NOT generate standalone rewrite clause
- Works at any depth because congruence lifts it
  
**Case 2: Base rewrite without collection**
```rust
(PDrop (NQuote P)) => P
```
- Does NOT generate standalone rewrite clause if covered by congruence
- The congruence handles all rewriting (both at top level and inside collections)

**Case 3: Base rewrite on non-collection category**
```rust
(NQuote (PDrop N)) => N  // on Name, which has no collection congruence
```
- Generates normal standalone rewrite clause
- No collection context to worry about

#### Modified Logic

```rust
fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut clauses = Vec::new();
    
    // Find which categories are subjects of collection congruences
    let collection_congruence_categories = find_collection_congruence_element_categories(theory);
    
    for (rule_idx, rule) in theory.rewrites.iter().enumerate() {
        // Skip congruence rules (handled separately)
        if rule.premise.is_some() {
            continue;
        }
        
        let category = extract_category(&rule.left);
        
        // Check if this rewrite has a collection pattern
        if contains_collection_pattern(&rule.left) {
            // Collection-based rewrite (like communication)
            // The congruence rule will have generated projections for this
            // We just need to generate the join-based rewrite
            let join_rewrite = generate_collection_join_rewrite(rule_idx, rule, theory);
            clauses.push(join_rewrite);
        } else if collection_congruence_categories.contains(&category) {
            // This category has collection congruence
            // DON'T generate a standalone base rewrite - congruence handles it
            continue;
        } else {
            // Normal base rewrite on non-collection category
            clauses.push(generate_rewrite_clause(rule, theory));
        }
    }
    
    clauses
}

/// Generate join-based rewrite for collection patterns
/// Uses projections generated by congruence analysis
fn generate_collection_join_rewrite(
    rule_idx: usize,
    rule: &RewriteRule,
    theory: &TheoryDef,
) -> TokenStream {
    // This is for multi-element patterns like communication
    // Find the congruence that covers this collection
    let cong_idx = find_covering_congruence(rule, theory);
    
    // Extract element patterns
    let element_patterns = extract_all_element_patterns(&rule.left, theory);
    
    // Generate joins for each projection
    let projection_joins = generate_projection_joins(cong_idx, rule_idx, &element_patterns);
    
    // Generate RHS reconstruction
    let rhs_reconstruction = generate_rhs_from_projections(&rule.right, &element_patterns);
    
    // Build the full rewrite clause
    quote! {
        rw_proc(parent, result) <--
            #projection_joins,
            #rhs_reconstruction;
    }
}

fn find_collection_congruence_element_categories(theory: &TheoryDef) -> HashSet<Ident> {
    let mut categories = HashSet::new();
    
    for rule in &theory.rewrites {
        if let Some((source_var, _)) = &rule.premise {
            if let Some(info) = extract_collection_congruence_info(&rule.left, source_var) {
                categories.insert(info.element_category);
            }
        }
    }
    
    categories
}
```

### Phase 4: Integrating Regular and Collection Congruences

**Key insight:** Collection congruences need to account for BOTH base rewrites AND regular congruences.

#### Example: Composing Congruences

```rust
rewrites {
    // Base rewrite
    (PDrop (NQuote P)) => P;
    
    // Regular congruence (non-collection)
    if S => T then (PNew x S) => (PNew x T);
    
    // Collection congruence
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

The collection congruence must handle:
1. `(PPar {(PDrop (NQuote P))})` - base rewrite pattern
2. `(PPar {(PNew x (PDrop (NQuote P)))})` - regular congruence pattern

**Solution:** Collection congruence generates TWO types of projections:

**Type 1: Base Rewrite Projections**
```rust
relation pdrop_proj_c2_b0_p0(Proc, Proc, Proc);  // (parent, P, elem)
// Extracts PDrop patterns from collections
```

**Type 2: Regular Congruence Projections**
```rust
relation pnew_proj_c2_r0(Proc, Binder<String>, Proc, Proc);  // (parent, x, body, elem)
// Extracts PNew patterns from collections
```

Then generates TWO congruence clauses:

**Clause 1: For base rewrite (applies RHS transformation)**
```rust
rw_proc(parent, result) <--
    pdrop_proj_c2_b0_p0(parent, p, elem),
    // Apply the RHS of the base rewrite directly
    let rewritten = p,
    // Reconstruct collection
    ...;
```

**Clause 2: For regular congruence (recursive rewriting)**
```rust
rw_proc(parent, result) <--
    pnew_proj_c2_r0(parent, x, body, elem),
    // Recursively rewrite the body using rw_proc
    rw_proc(body, body_rewritten),
    let rewritten = Proc::PNew(Scope::new(x, Box::new(body_rewritten))),
    // Reconstruct collection
    ...;
```

**Key difference:**
- Base rewrite projections: Apply transformation directly
- Regular congruence projections: Call `rw_proc` recursively to allow nested rewriting

This allows `(PPar {(PNew x (PDrop (NQuote P)))})` to rewrite:
1. PNew projection extracts body `(PDrop (NQuote P))`
2. Calls `rw_proc(body, body_rewritten)`
3. PDrop base rewrite matches via its own projection
4. Body rewrites to `P`
5. PNew reconstruction creates `(PNew x P)`
6. Collection reconstruction creates `(PPar {(PNew x P)})`

### Phase 5: Congruence Rule Generation

The congruence rule itself becomes simpler - it just references the projections:

```rust
fn generate_collection_congruence(
    cong_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    theory: &TheoryDef,
) -> TokenStream {
    let rw_rel = format_ident!("rw_{}", cong_info.parent_category.to_string().to_lowercase());
    
    // For each base rewrite, generate a congruence clause using its projection
    let base_rewrites = find_base_rewrites_for_category(&cong_info.element_category, theory);
    
    let clauses: Vec<_> = base_rewrites.iter().enumerate().map(|(base_idx, base_rule)| {
        let pattern_info = analyze_lhs_pattern(&base_rule.left, theory);
        let proj_rel = format_ident!(
            "{}_proj_c{}_b{}",
            pattern_info.root_constructor.to_string().to_lowercase(),
            cong_idx,
            base_idx
        );
        
        // Generate RHS reconstruction
        let rhs_term = reconstruct_term(&base_rule.right, &pattern_info);
        
        let constructor = &cong_info.constructor;
        let parent_cat = &cong_info.parent_category;
        let constructor_lower = format_ident!("{}", constructor.to_string().to_lowercase());
        let insert_helper = format_ident!("insert_into_{}", constructor_lower);
        
        // Generate captures tuple
        let captures = generate_captures_tuple(&pattern_info);
        
        quote! {
            #rw_rel(parent, result) <--
                #proj_rel(parent, #captures, elem),
                if let #parent_cat::#constructor(ref bag) = parent,
                let rest = {
                    let mut b = bag.clone();
                    b.remove(elem);
                    b
                },
                let rewritten = #rhs_term,
                let result = #parent_cat::#constructor({
                    let mut bag = rest;
                    #parent_cat::#insert_helper(&mut bag, rewritten);
                    bag
                });
        }
    }).collect();
    
    quote! {
        #(#clauses)*
    }
}
```

## Concrete Example: RhoCalc with Ambient Features

### User Input
```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name },
    terms {
        PDrop . Proc ::= "*" Name ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PNew . Proc ::= "new" <Name> "in" "{" Proc "}" ;  // Binding constructor
        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;
        NQuote . Name ::= "@" "(" Proc ")" ;
        // ... other constructors
    },
    rewrites {
        // Base rewrite 0: Communication (collection pattern)
        (PPar {(PInput chan x P), (POutput chan Q)})
            => (PPar {(subst P x (NQuote Q))});
        
        // Base rewrite 1: Drop-quote (simple pattern)
        (PDrop (NQuote P)) => P;
        
        // Regular congruence 0: PNew
        if S => T then (PNew x S) => (PNew x T);
        
        // Collection congruence: PPar
        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
    }
}
```

### Generated Code

#### Step 1: Congruence Analysis
- Congruence rule #2 (collection): `if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})`
- Element category: Proc
- Find base rewrites involving Proc:
  - Rewrite 0: `(PPar {(PInput chan x P), (POutput chan Q)}) => (PPar {(subst P x (NQuote Q))})`
  - Rewrite 1: `(PDrop (NQuote P)) => P`
- Find regular congruences on Proc:
  - Congruence 0: `if S => T then (PNew x S) => (PNew x T)`

#### Step 2: Extract Patterns
- From Rewrite 0 collection: `(PInput chan x P)` and `(POutput chan Q)`
- From Rewrite 1 root: `(PDrop (NQuote P))`
- From Congruence 0: `(PNew x S)` where S can be rewritten
- Result: 4 projections to generate (2 + 1 + 1)

#### Step 3: Generate Projections (from collection congruence)
```rust
// Projection for PInput pattern (from rewrite 0, pattern 0)
relation pinput_proj_c2_b0_p0(Proc, Name, mettail_runtime::Binder<String>, Proc, Proc);
// (parent, chan, x, P, elem)

pinput_proj_c2_b0_p0(parent.clone(), cap_chan.clone(), binder_x.clone(), cap_p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),  // ✅ ALL elements
    if let Proc::PInput(ref f0, ref f1) = elem,
    let (binder_x, body_x) = (*f1).clone().unbind(),
    let cap_p = (*body_x).clone(),
    let cap_chan = (**f0).clone();

// Projection for POutput pattern (from rewrite 0, pattern 1)
relation poutput_proj_c2_b0_p1(Proc, Name, Proc, Proc);
// (parent, chan, Q, elem)

poutput_proj_c2_b0_p1(parent.clone(), cap_chan.clone(), cap_q.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc::POutput(ref f0, ref f1) = elem,
    let cap_chan = (**f0).clone(),
    let cap_q = (**f1).clone();

// Projection for PDrop pattern (from rewrite 1, pattern 0)
relation pdrop_proj_c2_b1_p0(Proc, Proc, Proc);
// (parent, P, elem)

pdrop_proj_c2_b1_p0(parent.clone(), cap_p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc::PDrop(ref f0) = elem,
    if let Name::NQuote(ref f0_f0) = **f0,
    let cap_p = (**f0_f0).clone();

// Projection for PNew pattern (from regular congruence 0)
relation pnew_proj_c2_r0(Proc, mettail_runtime::Binder<String>, Proc, Proc);
// (parent, x, body, elem)

pnew_proj_c2_r0(parent.clone(), binder_x.clone(), body.clone(), elem.clone()) <--
    proc(parent),
    if let Proc::PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc::PNew(ref scope) = elem,
    let (binder_x, body_box) = scope.clone().unbind(),
    let body = (*body_box).clone();
```

#### Step 4: Generate Collection Join Rewrite (for base rewrite 0)
```rust
// Communication rewrite using projections
// Matches: (PPar {(PInput chan x P), (POutput chan Q)})
// Also works on: (PPar {(PInput chan x P), (POutput chan Q), other_elem1, other_elem2, ...})
// Because projections iterate all elements and congruence reconstruction adds them back
rw_proc(parent, result) <--
    pinput_proj_c0_b0_p0(parent, chan, x, p, elem_0),
    poutput_proj_c0_b0_p1(parent, chan, q, elem_1),  // Join on 'chan' and 'parent'
    if let Proc::PPar(ref bag) = parent,
    let remaining = {
        let mut b = bag.clone();
        b.remove(&elem_0);
        b.remove(&elem_1);
        b
    },
    let result = Proc::PPar({
        let mut bag = remaining;
        Proc::insert_into_ppar(&mut bag, p.substitute_name(&x.0, &Name::NQuote(Box::new(q))));
        bag
    });
```

#### Step 5: Generate Congruence Rule (for base rewrite 1)
```rust
// Drop-quote lifted into collections via congruence
// Matches: (PPar {(PDrop (NQuote P))}) 
// Also: (PPar {(PDrop (NQuote P)), other_elem1, other_elem2, ...})
// The other elements are preserved via the reconstruction
rw_proc(parent, result) <--
    pdrop_proj_c0_b1_p0(parent, p, elem),
    if let Proc::PPar(ref bag) = parent,
    let remaining = {
        let mut b = bag.clone();
        b.remove(elem);
        b
    },
    let result = Proc::PPar({
        let mut bag = remaining;
        Proc::insert_into_ppar(&mut bag, p);
        bag
    });
```

#### Step 8: NO Standalone Rewrite for PDrop
```rust
// PNew congruence lifted into collections
// Matches: (PPar {(PNew x S)}) where S can be rewritten
// Recursively applies rw_proc to the body
rw_proc(parent, result) <--
    pnew_proj_c2_r0(parent, x, body, elem),
    rw_proc(body, body_rewritten),  // ✅ Recursive rewriting!
    if let Proc::PPar(ref bag) = parent,
    let remaining = {
        let mut b = bag.clone();
        b.remove(elem);
        b
    },
    let result = Proc::PPar({
        let mut bag = remaining;
        Proc::insert_into_ppar(&mut bag, Proc::PNew(Scope::new(x, Box::new(body_rewritten))));
        bag
    });
```

#### Step 7: Regular Congruence (PNew) Direct Rule
The regular congruence `if S => T then (PNew x S) => (PNew x T)` ALSO generates a direct rule (not via collection):

```rust
// Direct PNew congruence (for non-collection contexts)
rw_proc(s, t) <--
    proc(s),
    if let Proc::PNew(ref scope) = s,
    let (x, body_box) = scope.clone().unbind(),
    let body = (*body_box).clone(),
    rw_proc(body, body_rewritten),
    let t = Proc::PNew(Scope::new(x, Box::new(body_rewritten)));
```

This allows `(PNew x (PDrop (NQuote P)))` to rewrite even when NOT inside a collection.
Because `Proc` is subject to collection congruence, we DON'T generate:
```rust
// ❌ SKIP THIS for categories with congruence:
// rw_proc(s, t) <-- proc(s), if let Proc::PDrop(n) = s, ...
```

**Why?** The congruence rule handles ALL rewriting:
- At top level: `(PDrop (NQuote P))` → projection matches → rewrites to `P`
- Inside collection: `(PPar {(PDrop (NQuote P))})` → projection matches → rewrites to `(PPar {P})`
- In larger context: `(PPar {(PDrop (NQuote P)), a!(0), b!(0)})` → projection matches → rewrites to `(PPar {P, a!(0), b!(0)})`

The single projection-based congruence rule handles all cases.

#### Composition Example

Now we can handle nested structures like `(PPar {(PNew x (PDrop (NQuote P)))})`:

1. **Collection congruence** has PNew projection → extracts `(PNew x (PDrop (NQuote P)))`
2. **PNew projection clause** calls `rw_proc(body, body_rewritten)` where `body = (PDrop (NQuote P))`
3. **Collection congruence** has PDrop projection (or regular congruence direct rule) → matches `(PDrop (NQuote P))`
4. **PDrop rewrites** to `P`
5. **PNew reconstruction** creates `(PNew x P)`
6. **Collection reconstruction** creates `(PPar {(PNew x P)})`

**Key insight:** Regular congruences and collection congruences compose naturally because:
- Collection congruences generate projections for regular congruence constructors
- Those projections call `rw_proc` recursively
- The recursive calls can match other projections or direct rules
- This creates a transitive closure of rewriting

## Benefits

### 1. Correctness
- ✅ No more naive `.iter().next()` bugs
- ✅ All elements checked via projection iteration
- ✅ Order-independent matching
- ✅ Single source of truth: congruence drives projection generation

### 2. Clarity
- ✅ Clear separation: base rewrites match structures, congruence lifts them
- ✅ User intent explicit: congruence rule means "lift all rewrites on this type"
- ✅ No heuristics: congruence analyzes LHS patterns directly
- ✅ **No `...rest` in base rewrites**: cleaner syntax, congruence handles it

### 3. Efficiency
- ✅ Projections computed once, reused by all rewrites
- ✅ Communication rule and drop-quote rule share projection infrastructure
- ✅ Ascent indexes optimize joins on shared variables (like `chan`)
- ✅ No redundant generation

### 4. Simplicity
- ✅ Remove `requires_indexed_projection` heuristic entirely
- ✅ Single code path: congruence generates all projections
- ✅ Base rewrites don't need to know about embedding context
- ✅ **User writes minimal code**: `(PDrop (NQuote P)) => P` works everywhere

### 5. Elegance of the Design

The key insight that makes this elegant:

**Separation of Concerns:**
- **Base rewrites** describe transformations on structures
  - `(PDrop (NQuote P)) => P` says "drop-of-quote is identity"
  - `(PPar {(PInput chan x P), (POutput chan Q)}) => (PPar {(subst ...)})` says "matching input/output communicate"
  - No mention of context or embedding

- **Congruence rules** describe how transformations lift into contexts
  - `if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})` says "any rewrite works inside PPar"
  - The `...rest` is ONLY in the congruence, not base rewrites
  - Applies to ALL base rewrites on that category

**Result:** User writes minimal, intuitive code. Implementation is driven by a single analysis pass (congruence → inspect base rewrites → generate projections).

## Implementation Plan

### Week 1: Core Refactoring

#### Day 1-2: AST Analysis Functions
- `extract_collection_congruence_info`
- `find_base_rewrites_for_category`
- `analyze_lhs_pattern` (enhanced)
- Unit tests for pattern analysis

#### Day 3-4: Projection Generation
- `generate_congruence_projections`
- `generate_projection_from_base_rewrite`
- Test projection generation for simple cases

#### Day 5: Congruence Generation
- Modify `generate_collection_congruence`
- Use projections instead of generic contains relation
- Test with RhoCalc example

### Week 2: Integration & Cleanup

#### Day 1-2: Modify Base Rewrite Generation
- `find_collection_congruence_element_categories`
- Skip base rewrite for congruence-covered categories
- Test that redundant rules aren't generated

#### Day 3: Validation
- Add checks for disallowed patterns
- Error messages for single-element collection in base rewrites
- Allow multi-element (communication) patterns

#### Day 4: Testing
- RhoCalc: drop-quote, communication
- Ambient calculus (if applicable)
- Performance benchmarks

#### Day 5: Documentation
- Update REPL-GUIDE.md
- Update examples
- Migration guide

### Week 3: Polish & Edge Cases

#### Day 1-2: Multiple Congruences
- What if multiple congruences on same constructor?
- Deduplicate projections if possible

#### Day 3: Nested Collections
- Test: `(PPar {(PPar {P})})`
- Ensure projections compose correctly

#### Day 4: Error Handling
- Better error messages
- Validation of congruence premise matching

#### Day 5: Final Testing
- All examples
- Stress tests
- Documentation review

## Migration Guide

### For Existing Theories

**Before (incorrect/redundant):**
```rust
rewrites {
    // Workaround: explicitly write collection variant
    (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});
}
```

**After (correct and simple):**
```rust
rewrites {
    // Just write the base rewrite (no ...rest needed!)
    (PDrop (NQuote P)) => P;
    
    // Congruence automatically lifts it into collections
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

**Key point**: The base rewrite doesn't need `...rest` because the congruence rule handles embedding in larger collections.

### For Multi-Element Patterns (Syntax Simplified)

Multi-element collection rewrites also don't need `...rest`:

**Before:**
```rust
rewrites {
    // Workaround with ...rest
    (PPar {(PInput chan x P), (POutput chan Q), ...rest})
        => (PPar {(subst P x (NQuote Q)), ...rest});
}
```

**After:**
```rust
rewrites {
    // Clean: no ...rest needed!
    (PPar {(PInput chan x P), (POutput chan Q)})
        => (PPar {(subst P x (NQuote Q))});
    
    // Congruence handles embedding in larger collections
    if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
}
```

**How it works:**
1. Base rewrite matches exact structure: 2 specific elements
2. Congruence generates projections for PInput and POutput patterns
3. Join-based rewrite finds matching pairs at any depth
4. Reconstruction preserves other elements in the collection

**Result:** `(PPar {(PInput a x P), (POutput a Q), b!(0), c!(0)})` correctly rewrites to `(PPar {(subst P x (NQuote Q)), b!(0), c!(0)})`

## Open Questions

### Q1: What if no congruence is specified?
If user writes `(PDrop (NQuote P)) => P` but no congruence, behavior depends on category:
- **With congruence on that category**: Works at all levels (including inside collections)
- **Without congruence**: Only works at top level (not inside collections)

This is correct behavior - user must explicitly opt into lifting via congruence.

### Q2: Multiple collection types?
If we have `PPar` and `PSeq`, each gets its own congruence:
```rust
if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
if S => T then (PSeq {S, ...rest}) => (PSeq {T, ...rest});
```
Each generates its own set of projections (e.g., `pdrop_proj_c0_b1_p0` for PPar, `pdrop_proj_c1_b1_p0` for PSeq). ✅ Works fine.

### Q3: Shared projections across congruences?
If two congruences need the same pattern projection, we could deduplicate by:
1. Generating projections in a separate pass
2. Hashing by (constructor, field_structure, captures)
3. Reusing relation names where patterns match

This is an optimization for later. Start with simple per-congruence generation.

### Q4: Performance impact?
Expected changes:
- **Reduced**: No more redundant base rewrite clauses for congruence-covered categories
- **Increased**: More projection relations (but they're shared across rewrites)
- **Net**: Likely neutral or positive (projections are more efficient than eager deconstruction)

Benchmark before/after to confirm.

### Q5: Empty collections?
What about `(PPar {}) => PZero` (or similar)?
- Not covered by this design (it's an equation, not a rewrite)
- Remains in the equations section
- No projections needed

### Q6: How do regular and collection congruences interact?
**Elegantly!** Collection congruences generate TWO kinds of projections:

1. **Base rewrite projections** - apply RHS transformation directly
2. **Regular congruence projections** - call `rw_proc` recursively

This allows composition:
```
(PPar {(PNew x (PDrop (NQuote P)))})
  → PNew projection extracts body
  → Recursive rw_proc call
  → PDrop projection matches
  → Rewrites through nested structure
```

The key: Regular congruences generate both:
- Direct rules (for non-collection contexts)
- Projections via collection congruences (for collection contexts)

## Success Criteria

### Must Have
- ✅ `{*@(a!(0)), b!(0)}` reduces correctly to `{a!(0), b!(0)}`
- ✅ Communication rule still works
- ✅ No `.iter().next()` in generated code for single-element patterns
- ✅ Order-independent matching verified

### Should Have
- ✅ Performance within 10% of current (correct) implementation
- ✅ Clear error messages for invalid patterns
- ✅ All examples updated and working

### Nice to Have
- ✅ Projection deduplication
- ✅ Better compile-time error messages
- ✅ Documentation with examples

## Conclusion

This refactoring inverts control: instead of each base rewrite deciding how it should be generated, **congruence rules** analyze base rewrites and create the necessary infrastructure. This is more correct, more explicit, and eliminates heuristics.

The key insight: **congruence rules express intent to lift rewrites**, so they should drive the generation of lifting infrastructure (projections).

