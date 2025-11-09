# Term Generation: Simplifying Assumptions and Future Work

This document catalogs all simplifying assumptions made in the current term generation implementation (both exhaustive and random), particularly around binding operations. These should be addressed in future work to achieve full correctness.

## Critical Assumption Summary

The current implementation makes **significant simplifications** in three main areas:
1. **Single-binder constructors only**
2. **Fixed depth strategies for multi-argument constructors**
3. **Simplified binding inference**

---

## 1. Binding Structure Assumptions

### Current State

**Assumption 1.1: Only first binding is considered**
```rust
// In termgen_gen.rs and random_generation.rs:
let (binder_idx, body_indices) = &rule.bindings[0];  // Only uses first!
```

- **Limitation**: If a constructor has multiple binders, only the first is handled
- **Impact**: Multi-binder constructors like `let x = e1 in let y = e2 in body` would only process the first `let`

**Assumption 1.2: Only first body is considered**
```rust
let body_idx = body_indices[0];  // Only uses first body!
```

- **Limitation**: If a binder binds in multiple positions, only the first is handled
- **Impact**: Constructors where a binder affects multiple sub-terms won't work correctly

### Grammar Example (Rho Calculus)

```rust
// From rhocalc.rs:
PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
//                                   ^binder^      ^body (bound position)^
```

**Current binding detection** (from `ast.rs:294-321`):
```rust
fn infer_bindings(items: &[GrammarItem]) -> Vec<(usize, Vec<usize>)> {
    // Each Binder at position i binds in the next NonTerminal/Binder at position j > i
    for (j, next_item) in items.iter().enumerate().skip(i + 1) {
        match next_item {
            GrammarItem::NonTerminal(_) | GrammarItem::Binder { .. } => {
                bound_indices.push(j);
                break; // ⚠️ ONLY BINDS IN IMMEDIATELY FOLLOWING ITEM
            }
            GrammarItem::Terminal(_) => continue,
        }
    }
}
```

### What's Missing

1. **Multiple binders in sequence**:
   ```
   LetPair . Expr ::= "let" <Var> "," <Var> "=" Expr "in" Expr ;
   //                       ^x^        ^y^                ^body where both x,y are bound^
   ```

2. **Nested binder scopes**:
   ```
   LetRec . Expr ::= "letrec" <Var> "=" Expr "in" Expr ;
   //                         ^x^       ^body1^  ^body2^
   //                                   where x is bound in BOTH body1 and body2
   ```

3. **Pattern binders**:
   ```
   Match . Expr ::= "match" Expr "with" Pattern "->" Expr ;
   //                                    ^binds multiple vars^ ^in here^
   ```

---

## 2. Depth Distribution Assumptions

### Exhaustive Generation

**Assumption 2.1: Multi-arg constructors use depth-1 for all args**
```rust
// In termgen_gen.rs, generate_nary_constructor_case:
// Simplified: just use depth-1 for all args
quote! {
    if depth > 0 {
        let d = depth - 1;
        #(#field_names)* {
            #(#arg_iters)* {
                terms.push(#cat_name::#label(#(#constructor_args),*));
            }
        }
    }
}
```

- **Limitation**: All arguments are generated at `depth-1`
- **Impact**: Misses valid terms where arguments have different depths that still sum correctly
- **Example**: For `PPar(p1, p2)` at depth 3, we only generate combinations where both `p1` and `p2` are at depth 2, missing valid cases like `p1` at depth 1 and `p2` at depth 2

**Assumption 2.2: Binary constructors use partial depth distribution**
```rust
// In termgen_gen.rs, generate_binary_constructor_case:
for d1 in 0..depth {
    for d2 in 0..depth {
        if d1.max(d2) != depth - 1 {
            continue; // ⚠️ REQUIRES MAX DEPTH = DEPTH-1
        }
        // ...
    }
}
```

- **Limitation**: Requires at least one argument at depth-1
- **Impact**: Conservative but sound approach, may miss some valid depth combinations
- **Rationale**: Ensures generated term is actually at target depth

**Assumption 2.3: Binders with args use simplified depths**
```rust
// In generate_binder_with_multiple_args:
let arg_generations: Vec<TokenStream> = other_args.iter().map(|(_, cat)| {
    quote! {
        Box::new(#cat::generate_random_at_depth_internal(vars, depth - 1, rng, binding_depth))
        // ⚠️ All non-body args at depth-1
    }
}).collect();
```

- **Limitation**: Non-body arguments in binder constructors all use `depth-1`
- **Impact**: Less diversity in generated terms
- **Example**: For `PInput(channel, scope)`, channel is always at depth-1

### Random Generation

**Assumption 2.4: Random depth choices for binary ops**
```rust
// In random_generation.rs, generate_random_binary:
let d1 = rng.gen_range(0..depth);
let d2 = if d1 == depth - 1 {
    rng.gen_range(0..depth)
} else {
    depth - 1  // ⚠️ FORCE d2 = depth-1 if d1 < depth-1
};
```

- **Limitation**: Ensures at least one arg is at depth-1 (for soundness)
- **Impact**: Random distribution is biased toward depth-1
- **Alternative**: Could allow more freedom but needs depth validation

---

## 3. Variable Pool Assumptions

### Current State

**Assumption 3.1: Fixed variable pool**
```rust
// User provides initial pool:
let vars = vec!["a".to_string(), "b".to_string()];
Proc::generate_terms(&vars, depth);
```

- **Limitation**: Variable pool doesn't grow with term complexity
- **Impact**: Large terms have limited variable diversity
- **Rationale**: Simple and sufficient for most use cases

**Assumption 3.2: Binder names are deterministic**
```rust
let binder_name = format!("x{}", binding_depth);
```

- **Limitation**: All binders at depth N use name `xN`
- **Impact**: Less diversity in binder names
- **Rationale**: Ensures uniqueness and consistency

---

## 4. Constructor Classification Assumptions

### Current State

**Assumption 4.1: Simple arity-based classification**

Constructors are classified by counting non-terminal, non-binder arguments:

```rust
// In random_generation.rs:
fn generate_random_non_binder_case(...) {
    match arg_cats.len() {
        0 => generate_random_nullary(...),
        1 => generate_random_unary(...),
        2 => generate_random_binary(...),
        _ => generate_random_nary(...),  // ⚠️ Falls back to simplified approach
    }
}
```

- **Limitation**: All 3+ argument constructors use simplified generation
- **Impact**: Less optimal depth distribution for complex constructors
- **Example**: A 5-argument constructor uses depth-1 for all args

**Assumption 4.2: Binder presence is binary**

```rust
if !rule.bindings.is_empty() {
    // It's a binder constructor
    generate_binder_constructor_case(...)
} else {
    // It's a regular constructor
    generate_non_binder_case(...)
}
```

- **Limitation**: Either all arguments are binders or none are (in terms of handling)
- **Impact**: Mixed binder/non-binder constructors may not be handled optimally

---

## 5. Depth Semantics Assumptions

### Current Definition

**Operator depth** counts nesting levels of constructors:

```rust
// Depth 0: nullary constructors and variables
0, *a, *b

// Depth 1: one level of operators
a!(0), for(a->x0){0}, 0|0

// Depth 2: nested operators
a!(b!(0)), for(a->x0){for(b->x1){0}}, (a!(0))|(b!(0))
```

**Assumption 5.1: Variables have depth 0**
- **Limitation**: Doesn't distinguish between free and bound variables
- **Impact**: Both `*a` and `*x0` (in `for(a->x0){*x0}`) are depth 0
- **Rationale**: Consistent with treating variables as atomic

**Assumption 5.2: Binders count as one depth level**
- **Limitation**: `for(x->body)` has depth = 1 + depth(body)
- **Impact**: Binder overhead adds to depth count
- **Rationale**: Binders are operators, should count toward complexity

---

## 6. Category Export Assumptions

### Current State

**Assumption 6.1: Only exported categories are generated**
```rust
fn is_exported(cat: &Ident, theory: &TheoryDef) -> bool {
    theory.exports.iter().any(|e| &e.name == cat)
}

// Used everywhere:
if !is_exported(cat, theory) {
    return quote! {};  // Skip non-exported categories
}
```

- **Limitation**: Non-exported categories are silently ignored
- **Impact**: If a constructor uses a non-exported category, it can't be generated
- **Rationale**: Only public API types should be generated

**Assumption 6.2: Cross-category dependencies are correct**
- **Limitation**: Assumes all category dependencies form a DAG
- **Impact**: Circular dependencies would cause infinite recursion (or stack overflow)
- **Mitigation**: Not currently checked or enforced

---

## 7. Moniker Integration Assumptions

### Current State

**Assumption 7.1: Scope::new handles closing**
```rust
let scope = mettail_runtime::Scope::new(binder, Box::new(body));
// Assumes Scope::new automatically closes free occurrences of binder in body
```

- **Limitation**: Relies on moniker's behavior being correct
- **Impact**: If body doesn't contain the binder variable, no error is raised
- **Rationale**: Moniker is battle-tested

**Assumption 7.2: Variable cache is cleared when needed**
```rust
mettail_runtime::clear_var_cache();
// User must manually clear before parsing or generating fresh terms
```

- **Limitation**: Easy to forget, can cause variable collisions
- **Impact**: Generated and parsed terms may share FreeVar instances incorrectly
- **Mitigation**: Documented in user-facing API

**Assumption 7.3: `get_or_create_var` is deterministic**
```rust
let binder_var = mettail_runtime::get_or_create_var(&binder_name);
// Same name always returns same FreeVar instance (within a cache lifetime)
```

- **Limitation**: Variable identity is global, not scoped
- **Impact**: Terms generated in separate contexts may share variable instances
- **Rationale**: Required for moniker's variable tracking

---

## 8. Performance and Scalability Assumptions

### Current State

**Assumption 8.1: Exhaustive generation is practical for low depths**
```rust
/// # Warning
/// Number of terms grows exponentially with depth!
/// Recommend max_depth <= 3 for most use cases.
pub fn generate_terms(vars: &[String], max_depth: usize) -> Vec<#cat_name>
```

- **Limitation**: No bounds checking or early termination
- **Impact**: `generate_terms(&vars, 10)` may run out of memory
- **Mitigation**: Documented warning in API

**Assumption 8.2: Cloning terms is acceptable**
```rust
terms.push(#cat_name::#label(Box::new(arg1.clone()), scope));
// Clones terms liberally throughout generation
```

- **Limitation**: May be inefficient for large terms
- **Impact**: Memory overhead for deep terms
- **Rationale**: Simplifies code, correctness over performance

**Assumption 8.3: Deduplication via sort is sufficient**
```rust
terms.sort();
terms.dedup();
```

- **Limitation**: O(n log n) deduplication
- **Impact**: Large term sets are slow to deduplicate
- **Alternative**: Could use HashSet, but requires Hash trait (already implemented)

---

## Future Work Roadmap

### Priority 1: Multiple Binders
**Goal**: Support constructors with multiple binders or multi-body binders

**Changes needed**:
1. Update `generate_binder_constructor_case` to iterate over all bindings
2. Handle multiple bodies per binder
3. Extend variable pool with multiple unique names per depth level
4. Test with examples like:
   ```
   LetPair . Expr ::= "let" <Var> "," <Var> "=" Expr "in" Expr ;
   ```

**Complexity**: Medium - requires careful handling of variable scopes

---

### Priority 2: Improved Depth Distribution
**Goal**: More complete coverage of valid depth combinations

**Changes needed**:
1. For n-ary constructors (n > 2), enumerate all depth partitions that sum to target depth
2. Implement depth partition generation:
   ```rust
   fn depth_partitions(n: usize, total: usize) -> Vec<Vec<usize>> {
       // Generate all ways to distribute total depth across n arguments
       // where max(depths) == total - 1
   }
   ```
3. Update exhaustive generation to use partitions
4. Optionally: add heuristics to random generation for better depth distribution

**Complexity**: Medium - combinatorial enumeration

---

### Priority 3: Advanced Binding Inference
**Goal**: Detect and handle complex binding patterns

**Changes needed**:
1. Extend `infer_bindings` to handle:
   - Multiple binders binding in same body
   - Binders binding in multiple bodies
   - Recursive bindings (binder in its own scope)
2. Add explicit binding annotations to grammar syntax:
   ```
   LetRec . Expr ::= "letrec" <Var> "=" Expr(bound) "in" Expr(bound) ;
   //                         ^x binds in both^
   ```
3. Update codegen to generate correct scopes for each pattern

**Complexity**: High - requires grammar syntax extension

---

### Priority 4: Depth Semantics Refinement
**Goal**: More principled depth accounting

**Changes needed**:
1. Define depth more formally (e.g., AST height, operator count)
2. Consider different depth metrics:
   - **Height**: max nesting level
   - **Size**: total operator count
   - **Weighted**: different constructors have different costs
3. Allow user to choose metric via API
4. Adjust generation algorithms accordingly

**Complexity**: Medium - mostly refactoring

---

### Priority 5: Dynamic Variable Pool
**Goal**: Automatically grow variable pool based on term complexity

**Changes needed**:
1. Add heuristic to determine needed pool size (e.g., `depth * 2`)
2. Auto-generate additional variables: `"v0"`, `"v1"`, `"v2"`, ...
3. Make initial pool optional:
   ```rust
   Proc::generate_terms_with_vars(&vars, depth);
   Proc::generate_terms_auto(depth);  // Auto-gen vars
   ```

**Complexity**: Low - simple extension

---

### Priority 6: Category Dependency Analysis
**Goal**: Validate and optimize cross-category generation

**Changes needed**:
1. Build category dependency graph at macro expansion time
2. Detect cycles and report compile-time error
3. Topologically sort categories for optimal generation order
4. Cache results across categories during exhaustive generation

**Complexity**: Medium - graph algorithms

---

### Priority 7: Performance Optimization
**Goal**: Handle larger depths and term sets efficiently

**Changes needed**:
1. Replace `Vec` with `HashSet` for term storage (dedup as we go)
2. Add configurable term count limits:
   ```rust
   Proc::generate_terms_bounded(&vars, depth, max_terms: 10000);
   ```
3. Implement streaming generation (iterator-based):
   ```rust
   for term in Proc::generate_terms_iter(&vars, depth) {
       // Process one at a time without storing all
   }
   ```
4. Add depth early-termination heuristics

**Complexity**: Medium-High - requires API changes

---

## Testing Recommendations

### Test Cases Needed

1. **Multiple binders**:
   ```rust
   theory! {
       terms {
           LetPair . Expr ::= "let" <Var> "," <Var> "=" Expr "in" Expr ;
       }
   }
   // Test: verify both vars are bound in body
   ```

2. **Binder in multiple bodies**:
   ```rust
   LetRec . Expr ::= "letrec" <Var> "=" Expr "in" Expr ;
   // Test: verify var is bound in BOTH Expr positions
   ```

3. **Complex depth partitions**:
   ```rust
   // Test PPar(p1, p2, p3) at depth 5
   // Verify all valid partitions: (4,0,0), (0,4,0), (0,0,4), (4,1,0), etc.
   ```

4. **Large variable pools**:
   ```rust
   let vars = (0..100).map(|i| format!("v{}", i)).collect();
   Proc::generate_terms(&vars, 3);
   // Test: verify no variable collisions
   ```

5. **Deep nesting**:
   ```rust
   Proc::generate_random_at_depth(&vars, 50);
   // Test: verify all x0..x49 can appear
   // Test: no stack overflow
   ```

---

## Documentation Checklist

- [x] Document binder name scheme (`x0`, `x1`, ...)
- [x] Document depth semantics (operator nesting)
- [x] Warn about exponential growth
- [ ] Document binding inference algorithm
- [ ] Document depth distribution strategies
- [ ] Document variable pool management
- [ ] Add examples of unsupported constructs
- [ ] Add migration guide for future API changes

---

## Summary Table

| Assumption | Current State | Limitation | Priority | Complexity |
|------------|---------------|------------|----------|------------|
| Single binder only | Uses `bindings[0]` | Can't handle multiple binders | P1 | Medium |
| Single body only | Uses `body_indices[0]` | Can't bind in multiple positions | P1 | Medium |
| Depth-1 for multi-arg | All args at `depth-1` | Misses valid combinations | P2 | Medium |
| Simple binder inference | Only next NonTerminal | Can't handle complex patterns | P3 | High |
| Fixed variable pool | User-provided only | Limited diversity | P5 | Low |
| No cycle detection | Assumes DAG | Could infinite loop | P6 | Medium |
| No term count limits | Generate all terms | Can OOM | P7 | Medium |

---

## Related Documents

- `docs/design/BINDER-SHADOWING-FIX.md` - How unique binder names work
- `docs/design/TERM-GENERATION-DESIGN.md` - Original design
- `docs/design/TERM-GENERATION-COMPLETE.md` - Initial implementation
- `docs/design/SORTING-DESIGN.md` - Term ordering foundation

---

*This document should be updated as assumptions are addressed or new ones are discovered.*

