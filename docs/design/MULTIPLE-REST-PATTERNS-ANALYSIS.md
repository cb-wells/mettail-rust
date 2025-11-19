# Multiple Rest Patterns Analysis

## Goal

Assess feasibility of supporting multiple explicit rest variables (`...rest1`, `...rest2`, etc.) in rewrite rules with arbitrary shapes.

## Current State

### What Works Today
- **Single rest in collection congruence**: `if S => T then {S, ...rest} => {T, ...rest}`
- **Explicit rest in base rewrites**: Limited - not fully implemented for complex patterns

### What Doesn't Work
- Multiple rest variables in a single rule
- Rest patterns in nested collection contexts
- Rest in multi-pattern rules (e.g., communication with rest in both sides)

## Use Cases

### Use Case 1: Communication with Context
**Scenario**: Process calculus communication rule with surrounding processes

```rust
// Input has context rest1, output has context rest2
{input(c, x, p), ...rest1, output(c, q), ...rest2} 
    => {p[q/x], ...rest1, ...rest2}
```

**Semantic Clarity**: ✅ **Well-defined**
- rest1 and rest2 are in the **same collection**, but represent disjoint subsets
- After matching input(...) and output(...), everything else goes into rest1 ∪ rest2
- **Problem**: How to split the remaining elements between rest1 and rest2?
  - Ambiguous for unordered collections!
  - Would need additional constraints or greedy matching strategy

### Use Case 2: Nested Collections with Multiple Rest
**Scenario**: Mobile ambients with context at multiple levels

```rust
// Outer context (rest1) and inner context (rest2)
{n[{in(m,p), ...rest1}], m[r], ...rest2}
    => {m[{n[{p, ...rest1}], r}], ...rest2}
```

**Semantic Clarity**: ✅ **Well-defined**
- rest1: elements in n's collection (besides in(m,p))
- rest2: elements in outer collection (besides n[...] and m[r])
- Each rest is **scoped to its collection context**
- No ambiguity!

### Use Case 3: Multiple Independent Collections
**Scenario**: Parallel rules affecting different collections

```rust
// Two independent ambient rewrites
{n[{in(m,p), ...rest_n}], m[{s, ...rest_m}]}
    => {m[{n[{p, ...rest_n}], s, ...rest_m}]}
```

**Semantic Clarity**: ✅ **Well-defined**
- rest_n scoped to n's inner collection
- rest_m scoped to m's inner collection
- No interaction between rest variables
- Clean semantics!

## Theoretical Analysis

### Well-Defined Cases

**Criterion**: Multiple rest variables are well-defined when each rest is **scoped to a distinct collection instance**.

Examples:
1. ✅ `{outer[{inner, ...rest_inner}], ...rest_outer}`
2. ✅ `{a[{...rest_a}], b[{...rest_b}]}`
3. ✅ Nested: `{x[{y[{...rest_y}], ...rest_x}]}`

### Ambiguous Cases

**Criterion**: Multiple rest variables in the **same collection** are ambiguous for unordered collections.

Examples:
1. ❌ `{A, ...rest1, B, ...rest2}` - which elements go where?
2. ❌ `{...rest1, A, ...rest2, B, ...rest3}` - even more ambiguous

**Why ambiguous?**
- Collections are **unordered multisets** (HashBag)
- After matching required elements A and B, remaining elements could be distributed arbitrarily
- No canonical choice without additional semantics (e.g., "greedy left-to-right")

### Potential Disambiguation Strategies

#### Strategy 1: Prohibit Multiple Rest in Same Collection
**Rule**: Each collection can have at most one rest variable

✅ Pros:
- Clear semantics
- Easy to implement
- Covers most practical use cases (nested collections)

❌ Cons:
- Doesn't support `{A, ...rest1, B, ...rest2}` patterns
- But these are ambiguous anyway!

#### Strategy 2: Sequential Matching (Not Suitable)
**Rule**: Match rest variables left-to-right greedily

❌ Problem:
- Violates semantics of unordered collections
- `{A, ...rest, B}` would be different from `{B, ...rest, A}`
- Breaks associativity/commutativity

#### Strategy 3: Union All Rest
**Rule**: Multiple rest in same collection merge into one

Example: `{A, ...rest1, B, ...rest2}` → `{A, B, ...rest}` where `rest = rest1 ∪ rest2`

✅ Pros:
- Preserves unordered semantics
- Unambiguous

❌ Cons:
- Loses information (can't distinguish rest1 from rest2 in RHS)
- Not useful if you need separate rest variables

## Implementation Feasibility

### Architecture Changes Needed

#### 1. AST Representation (Already Supported!)

```rust
// ast.rs - ALREADY EXISTS
CollectionPattern {
    constructor: Option<Ident>,
    elements: Vec<Expr>,
    rest: Option<Ident>,  // ✅ Single rest per collection
}
```

**Assessment**: ✅ Current AST can represent nested collections with multiple rest variables across different scopes.

#### 2. Validation Layer

**New validator**: `validate_rest_patterns`

```rust
fn validate_rest_patterns(expr: &Expr) -> Result<(), String> {
    match expr {
        Expr::CollectionPattern { elements, rest, .. } => {
            // Check 1: At most one rest per collection
            if rest.is_some() {
                // Already enforced by AST structure ✅
            }
            
            // Check 2: No rest in element patterns that would create multiple rests
            for elem in elements {
                validate_rest_patterns(elem)?;
            }
            Ok(())
        }
        Expr::Apply { args, .. } => {
            for arg in args {
                validate_rest_patterns(arg)?;
            }
            Ok(())
        }
        _ => Ok(())
    }
}
```

**Assessment**: ✅ Straightforward validation - already structurally enforced by AST.

#### 3. Projection Generation

**Changes needed** in `congruence_analysis.rs::generate_base_rewrite_projection`:

```rust
// For pattern: {A, B, ...rest}
// Generate projection that:
// 1. Extracts required elements (A, B)
// 2. Computes rest = bag \ {A, B}
// 3. Returns rest in projection tuple

relation projection(Parent, CaptureA, CaptureB, ElemA, ElemB, HashBag<Elem>);

projection(parent, cap_a, cap_b, elem_a, elem_b, rest) <--
    parent_cat(parent),
    if let Cat::Constructor(ref bag) = parent,
    for (elem_a, _) in bag.iter(),
    // ... match A ...
    for (elem_b, _) in bag.iter(),
    if &elem_b != &elem_a,
    // ... match B ...
    let rest = {
        let mut b = bag.clone();
        b.remove(elem_a);
        b.remove(elem_b);
        b
    };
```

**Assessment**: ✅ Can be implemented - similar to current single-element projection logic, just need to track rest bag.

#### 4. RHS Reconstruction with Multiple Rest

**Changes needed** in `ascent_gen.rs::generate_rhs_reconstruction`:

For RHS like `{C, ...rest1, ...rest2}`:

```rust
let result_bag = {
    let mut bag = HashBag::new();
    Category::insert_into_constructor(&mut bag, cap_c);
    // Merge rest1
    for (elem, count) in rest1.iter() {
        for _ in 0..count {
            bag.insert(elem.clone());
        }
    }
    // Merge rest2
    for (elem, count) in rest2.iter() {
        for _ in 0..count {
            bag.insert(elem.clone());
        }
    }
    bag
};
```

**Assessment**: ✅ Straightforward - just merge all rest bags sequentially.

#### 5. Complex Multi-Pattern Rules

**Example**: Communication with nested rest

```rust
// LHS: {n[{in(m,p), ...rest_n}], m[r], ...rest_outer}
// RHS: {m[{n[{p, ...rest_n}], r}], ...rest_outer}
```

**Projection Generation**:
1. Generate outer projection for `{n[...], m[...], ...rest_outer}`
2. Generate inner projection for `{in(m,p), ...rest_n}` inside n
3. Join projections in clause generation

**Assessment**: ✅ Composable - each collection's rest is independent.

### Complexity Assessment

#### Implementation Complexity: **Medium**

**Easy parts**:
- AST already supports it (one rest per collection)
- Validation is trivial (already structurally enforced)
- RHS reconstruction is straightforward (merge bags)

**Moderate parts**:
- Projection generation needs to track rest bags
- Clause generation needs to handle multiple rest variables
- Need to update signature of projections to include rest

**Hard parts**:
- None! The scoped nature makes this clean.

#### Runtime Complexity: **Same as Current**

- Rest computation: O(N) bag operations
- No additional join complexity
- Rest variables don't increase search space (they're computed, not matched)

### Testing Requirements

1. **Unit tests**: Rest extraction in projections
2. **Integration tests**: 
   - Nested collections with multiple rest
   - Communication rules with rest
   - Ambient calculus examples
3. **Edge cases**:
   - Empty rest bags
   - Rest with no required elements: `{...rest}` (matches any collection)
   - Multiple nested levels: `{a[{b[{c, ...r3}], ...r2}], ...r1}`

## Comparison with Implicit Rest Heuristic

| Aspect | Implicit Rest | Multiple Explicit Rest |
|--------|---------------|------------------------|
| **Syntax** | No change needed | Requires `...` syntax |
| **Semantics** | Heuristic-based | Explicit and clear |
| **Flexibility** | Limited to last variable | Arbitrary nesting |
| **Implementation** | Simpler (fewer cases) | More complete |
| **User Intent** | Implicit/magical | Explicit/clear |
| **Backward Compat** | ✅ No syntax change | ⚠️ Need to support `...rest` |

## Recommendation

### Primary Recommendation: **Implement Multiple Explicit Rest**

**Rationale**:
1. ✅ **Semantically clean**: No ambiguity when scoped to distinct collections
2. ✅ **More general**: Covers implicit rest + nested cases
3. ✅ **Explicit intent**: Users clearly indicate where rest applies
4. ✅ **Feasible**: Medium implementation complexity, no theoretical blockers
5. ✅ **Future-proof**: Supports complex patterns in process calculi

**Restriction**: Enforce at most one rest per collection (already guaranteed by AST structure).

### Secondary Enhancement: Add Implicit Rest on Top

After implementing explicit rest, **optionally** add implicit rest heuristic:
- Patterns like `{A, B}` where B is a variable → desugar to `{A, ...B}`
- This becomes syntax sugar for the explicit form
- Best of both worlds!

## Implementation Roadmap

### Phase 1: Explicit Rest in Base Rewrites ✅ (Partially done)
- Update projection generation to compute rest bag
- Include rest in projection signature
- Test with simple cases

### Phase 2: Rest in RHS Reconstruction
- Generate merge code for multiple rest bags
- Handle rest in `generate_rhs_reconstruction`
- Test with ambient calculus rules

### Phase 3: Multi-Pattern with Rest
- Support multiple projections with different rest variables
- Join projections in clause generation
- Test with complex nested patterns

### Phase 4: Implicit Rest Syntax Sugar (Optional)
- Add heuristic to detect implicit rest
- Desugar to explicit rest during parsing/validation
- Update documentation

## Open Questions

### Q1: Should rest variables be allowed in equations?
**Answer**: Yes, same semantics apply
- Equations are symmetric, so rest should work on both sides
- Example: `{P, ...rest} == {Q, ...rest}` (structural equivalence with context)

### Q2: How to handle rest in substitution?
**Example**: `(subst {P, ...rest} x Q)`
**Answer**: Substitute into rest bag elementwise
- Apply substitution to each element in rest
- Requires `rest.map(|elem| elem.substitute(x, Q))`

### Q3: Should we support rest in projection targets?
**Example**: Matching `{P, ...rest}` where rest itself is matched against a pattern
**Answer**: Not in Phase 1 - rest is atomic
- Can add later if needed (unlikely use case)

### Q4: Interaction with congruence rules?
**Example**: `if S => T then {S, ...rest} => {T, ...rest}`
**Answer**: Already supported! This is the current collection congruence
- Just need to ensure rest propagates correctly in RHS

## Conclusion

**Verdict**: ✅ **Multiple explicit rest variables are feasible and recommended**

**Key Insights**:
1. Well-defined when scoped to distinct collections
2. Ambiguous only if multiple rest in same collection (prohibit this)
3. Current AST structure already supports it
4. Implementation is straightforward extension of current projection logic
5. More general and cleaner than implicit heuristic

**Next Steps**:
1. Implement explicit rest extraction in projections (Phase 1)
2. Add rest merging in RHS reconstruction (Phase 2)
3. Test with ambient calculus examples
4. Consider implicit rest as syntax sugar (Phase 4)

