# Deep Indexed Projection - Implementation Roadmap

**Date**: November 9, 2025  
**Goal**: Enable indexed projection for deeply nested shared variables  
**Target**: Make ambient calculus rule work in all orderings

---

## Quick Start: What We're Building

### Before (Current - Phase 6)
```rust
// Works: Top-level shared variables
(PPar {(PInput chan x P), (POutput chan Q), ...rest}) => ...
//              ^^^^                  ^^^^
//              Same depth, direct arguments
```

### After (Phase 7 - Deep Projection)
```rust
// Works: ANY depth shared variables  
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R), ...rest}) => ...
//                         ^                 ^
//                         Nested vs direct - still works!
```

---

## Implementation Phases

### Phase 7.1: Analysis - Extract Paths (3-4 days)

**Files to modify:**
- `mettail-macros/src/rewrite_gen.rs`

**What to build:**

1. **New data structures** (30 min):
```rust
#[derive(Debug, Clone)]
enum ExtractionStep {
    Field { field_idx: usize, category: Ident },
    CollectionMatch { field_idx: usize, constructor: Ident, element_category: Ident },
}

// Extend CaptureInfo
struct CaptureInfo {
    var_name: String,
    category: Ident,
    field_idx: usize,           // Keep for backward compat
    extraction_path: Vec<ExtractionStep>,  // NEW
    is_binder: bool,
}
```

2. **Recursive analyzer** (2-3 days):
```rust
fn analyze_element_pattern_deep(
    pattern: &Expr,
    theory: &TheoryDef,
    current_path: Vec<ExtractionStep>,
) -> Result<Vec<CaptureInfo>, AnalysisError>
```

**Test criteria**:
- [x] Extracts `M` from `PAmb(N, PPar({PIn(M, P), Q}))`
- [x] Path is: `[Field(1), CollectionMatch(PIn), Field(0)]`
- [x] Also extracts `N`, `P`, `Q` with correct paths
- [x] Handles binders in nested positions
- [x] Returns error for unsupported patterns

3. **Update `analyze_collection_pattern`** (1 day):
```rust
// Replace:
let captures = extract_captures_from_args(args, grammar_rule, theory);

// With:
let captures = analyze_element_pattern_deep(elem, theory, vec![]);
```

**Test criteria**:
- [x] Ambient rule analysis succeeds
- [x] Returns `M` in shared_variables
- [x] RhoCalc rules still work (regression test)

### Phase 7.2: Projection Tree Building (2 days)

**What to build:**

1. **Tree structure** (1 day):
```rust
struct ProjectionTree {
    root: ProjectionNode,
}

struct ProjectionNode {
    level: usize,
    relation_name: Ident,
    extraction_steps: Vec<ExtractionStep>,
    captures: Vec<CaptureInfo>,
    children: Vec<ProjectionNode>,
    join_keys: Vec<String>,  // Which captures are used for joining
}

fn build_projection_tree(
    spec: &ProjectionSpec,
    theory: &TheoryDef,
) -> ProjectionTree
```

**Algorithm**:
```
1. Group captures by extraction path prefix
2. For each unique prefix, create a node
3. Recursively build children for longer paths
4. Mark join keys at the appropriate level
```

2. **Path optimization** (1 day):
```rust
fn optimize_tree(tree: &mut ProjectionTree) {
    // Merge nodes with identical extraction paths
    // Remove unnecessary intermediate levels
    // Hoist joins to shallowest level possible
}
```

**Test criteria**:
- [x] Ambient rule produces 2-level tree
- [x] Common `PAmb` extraction shared
- [x] Join happens at correct level

### Phase 7.3: Code Generation (3-4 days)

**What to build:**

1. **Relation generation** (1 day):
```rust
fn generate_deep_projection_relations(
    tree: &ProjectionTree,
    rule_idx: usize,
) -> Vec<TokenStream>
```

For each node, generate:
```rust
relation node_name(Parent, ...PathVars..., ...Captures..., OriginalElem);
```

2. **Extraction generation** (2 days):
```rust
fn generate_deep_extraction_rules(
    tree: &ProjectionTree,
    rule_idx: usize,
    theory: &TheoryDef,
) -> Vec<TokenStream>
```

For each node:
- If leaf: extract directly from parent collection
- If internal: extract from parent node's results

**Key challenge**: Handle nested collection iteration properly

3. **Join generation** (1 day):
```rust
fn generate_deep_join_rewrite(
    tree: &ProjectionTree,
    rule: &RewriteRule,
    theory: &TheoryDef,
) -> TokenStream
```

Join at the level where all shared variables are available

**Test criteria**:
- [x] Ambient rule compiles
- [x] Generates correct Ascent clauses
- [x] Type-checks correctly

### Phase 7.4: Integration & Testing (2-3 days)

**What to do:**

1. **Wire into main pipeline** (1 day):
```rust
pub fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    for rule in &theory.rewrites {
        if requires_indexed_projection(rule, theory) {
            if let Some(spec) = analyze_collection_pattern(&rule.left, theory) {
                // Check depth of shared variables
                if has_deep_shared_variables(&spec) {
                    // NEW: Deep projection
                    generate_deep_projection(rule_idx, &spec, rule, theory)
                } else {
                    // EXISTING: Flat projection
                    generate_flat_projection(rule_idx, &spec, rule, theory)
                }
            }
        }
    }
}
```

2. **Regression testing** (1 day):
- [x] All existing tests pass
- [x] RhoCalc still uses flat projection
- [x] Performance hasn't regressed

3. **New test cases** (1 day):
- [x] Ambient rule works in all orderings
- [x] 3-level nesting works
- [x] Multiple shared variables at different depths
- [x] Mixed depth patterns

---

## Test Cases

### Level 1: Top-Level (Already Works ✅)
```rust
(PPar {(PInput chan x P), (POutput chan Q)})
// chan at depth 0 in both
```

### Level 2: One-Level Nesting (Target)
```rust
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R)})
// M at depth 2 in first, depth 0 in second
```

### Level 3: Multi-Level Nesting (Stretch Goal)
```rust
(PPar {(PAmb N1 (PPar {(PAmb N2 (PPar {(PIn M P), Q1})), Q2})), (PAmb M R)})
// M at depth 4 in first, depth 0 in second
```

### Level 4: Multiple Shared at Different Depths (Advanced)
```rust
(PPar {(PAmb N (PPar {(PIn M P), (POut N Q)})), (PAmb M R)})
// N at depth 0 and depth 2
// M at depth 2 and depth 0
```

---

## Code Organization

### New Files
- `mettail-macros/src/deep_projection.rs` - Deep projection logic
  - `ExtractionStep`, `ProjectionTree`, `ProjectionNode`
  - Analysis and tree building
  - Code generation

### Modified Files
- `mettail-macros/src/rewrite_gen.rs`
  - Add `use deep_projection::*;`
  - Update `generate_rewrite_clauses` router
  - Keep existing flat projection code

### New Tests
- `mettail-macros/tests/deep_projection_tests.rs`
  - Unit tests for path extraction
  - Integration tests for multi-level rules

---

## Decision Points

### 1. When to Use Deep Projection?

**Option A**: Always analyze depth, use deep if needed
- Pro: Fully automatic
- Con: Extra analysis cost for simple cases

**Option B**: Check max depth first, only use deep if depth > 1
- Pro: Faster for common cases
- Con: Two code paths to maintain

**Recommendation**: **Option A** - The analysis cost is negligible, and having one code path is cleaner

### 2. How Deep to Support?

**Option A**: Unlimited depth
- Pro: Fully general
- Con: Potential for code explosion

**Option B**: Limit to 3-5 levels
- Pro: Practical limit, better error messages
- Con: Artificial restriction

**Recommendation**: **Option B with 5 levels** - Sufficient for real-world patterns, prevents pathological cases

### 3. How to Handle Rest Patterns with Nesting?

**Option A**: Full support from day 1
- Pro: Complete feature
- Con: More complex implementation

**Option B**: Defer to later phase
- Pro: Faster initial delivery
- Con: Incomplete functionality

**Recommendation**: **Option B** - Rest patterns with deep nesting are rare, can add later

---

## Quick Win: Simplified Version

For **immediate progress** (1-2 days), implement a simplified version:

### Assumption
Only handle cases where:
1. Shared variable appears at **top-level in at least one pattern**
2. Nested in the other pattern(s)

### Example
```rust
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R)})
//                         ^                 ^
//                      nested            top-level
```

### Strategy
1. Detect if ANY shared variable is top-level in at least one pattern
2. Generate projection for the top-level occurrence
3. Generate extraction + search for the nested occurrence
4. Join on the shared variable

### Code
```rust
// Top-level M in PAmb
pamb_r0_p1(parent, m, r, elem) <-- ...;

// Nested M in PAmb -> PPar -> PIn  
pin_nested_r0_p0(parent, n, m, p, q, outer_elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for (outer_elem, _) in bag.iter(),
    if let Proc::PAmb(n_box, inner_proc) = outer_elem,
    let n = (**n_box).clone(),
    if let Proc::PPar(inner_bag) = (**inner_proc).as_ref(),
    for (inner_elem, _) in inner_bag.iter(),
    if let Proc::PIn(m_box, p_box) = inner_elem,
    let m = (**m_box).clone(),  // Extract M
    ...;

// Join on M
rw_proc(parent, result) <--
    pin_nested_r0_p0(parent, n, m, p, q, amb1_elem),
    pamb_r0_p1(parent, m, r, amb2_elem),  // Join on m!
    ...;
```

**Benefits**:
- Handles ambient rule immediately
- Simpler than full deep projection
- Can be extended later

**Timeline**: 1-2 days vs 2-3 weeks for full solution

---

## Recommendation

**For immediate progress**: Implement the **Simplified Version** (Quick Win)
- Handles ambient calculus today
- Provides immediate value
- Can be enhanced later

**For complete solution**: Implement **Full Deep Projection** (Phase 7.1-7.4)
- Handles all cases
- More robust and general
- Takes 2-3 weeks

**Suggested approach**: 
1. Start with simplified version (1-2 days)
2. Validate with ambient and other examples
3. Then enhance to full deep projection if needed

Which would you prefer to tackle first?

