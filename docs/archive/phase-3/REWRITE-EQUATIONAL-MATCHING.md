# Rewrite Rules with Equational Matching

## The Problem

### Current Implementation

Currently, rewrite rules are implemented as **standalone functions** (`try_rewrite_rule_0`, etc.) that perform **syntactic pattern matching**. For example, the communication rule:

```
if x # Q then (PPar (PInput chan x P) (POutput chan Q))
    => (subst P x (NQuote Q));
```

Generates a function that checks if two `Name` values are **strictly equal** (same AST structure):

```rust
pub fn try_rewrite_rule_0(term: &Proc) -> Option<Proc> {
    if let Proc::PPar(field_0, field_1) = term {
        if let Proc::PInput(field_0_inner_0, field_0_inner_1) = &(**field_0) {
            if let Proc::POutput(field_1_inner_0, field_1_inner_1) = &(**field_1) {
                // PROBLEM: This checks (**field_0_inner_0).clone() == (**field_1_inner_0).clone()
                // which is SYNTACTIC equality, not EQUATIONAL equality
                if (**field_0_inner_0).clone() == (**field_1_inner_0).clone() {
                    // freshness check...
                    return Some(rhs);
                }
            }
        }
    }
    None
}
```

### The Issue

When a variable appears **twice** in a rewrite LHS (like `n` appearing in both `in(n,...)` and `out(n,...)`), we need **equational matching**, not just syntactic matching.

**Example:**
```
out(@(0), p) | in(@(0 | 0), \x.c)
```

Should match the communication rule because:
- `@(0)` and `@(0 | 0)` are **equationally equal** (by associativity/commutativity)
- Both represent the same channel name

But the current implementation would **reject** this because `@(0) != @(0 | 0)` syntactically.

### Why This Matters

1. **Correctness**: Rewrite systems with equations should respect those equations during matching
2. **Completeness**: We miss valid rewrites when names are equal but syntactically different
3. **Semantic accuracy**: The theory defines semantic equivalence through equations; rewrites should use it

## Design Options

### Option 1: Keep Functions, Add Equational Checks (Current + Patches)

**Approach:** Modify generated functions to call `eq_proc`, `eq_name`, etc. for duplicate variables.

**Pros:**
- Minimal changes to current architecture
- Functions remain self-contained
- Performance: direct function calls, no relation overhead

**Cons:**
- Functions need access to the `eq_*` relations (requires passing them as parameters)
- Two separate matching mechanisms (syntactic in functions, equational in Ascent)
- Inconsistent: some matching uses relations, some doesn't
- Complex generation: need to track which variables need equational vs syntactic matching
- Doesn't help with other matching scenarios (e.g., matching modulo AC)

### Option 2: Implement Rewrites as Ascent Rules (Unified Approach)

**Approach:** Generate rewrite rules **directly as Ascent clauses** instead of separate functions.

**Current:**
```rust
rw(s, t.clone()) <--
    proc(s),
    if let Some(t) = try_rewrite_rule_0(&s);
```

**Proposed:**
```rust
rw_proc(s, t) <--
    proc(s),
    if let Proc::PPar(p_in, p_out) = s,
    if let Proc::PInput(chan1, scope) = &**p_in,
    if let Proc::POutput(chan2, q) = &**p_out,
    eq_name(**chan1, **chan2),  // EQUATIONAL matching for duplicate variable
    let (x, p_body) = scope.clone().unbind(),
    if !p_body.contains_free(x),  // freshness check
    let t = p_body.substitute(x, &Name::NQuote(q.clone()));
```

**Pros:**
- **Unified matching**: All matching uses Ascent's relation semantics
- **Equational correctness**: `eq_name(chan1, chan2)` automatically uses equational theory
- **Consistency**: Rewrites, equations, and congruences all use same mechanism
- **Extensibility**: Easy to add more sophisticated matching (AC, modulo theories, etc.)
- **Declarative**: Rules are data, easier to analyze/optimize
- **Debugging**: Ascent provides relation inspection, can see intermediate matches

**Cons:**
- **Performance**: Relation lookups may be slower than direct pattern matching
- **Code generation complexity**: More complex to generate proper Ascent syntax
- **Ascent syntax limitations**: Some patterns may be harder to express
- **Startup cost**: Relations must be computed before rewrites can fire

### Option 3: Hybrid Approach

**Approach:** Use functions for simple syntactic matching, relations for equational matching.

- If no duplicate variables: generate function (fast path)
- If duplicate variables: generate Ascent rule (equational path)

**Pros:**
- Performance: most rewrites stay fast
- Correctness: equational matching where needed

**Cons:**
- Most complex: two code generation paths
- Inconsistent: hard to understand which rules are which
- Maintenance burden: need to keep both approaches working

## Recommended Approach: Option 2 (Unified Ascent Rules)

### Rationale

1. **Correctness First**: Equational matching is not a corner case—it's fundamental to rewrite systems with equations. We should default to correct semantics.

2. **Performance is Manageable**:
   - Ascent is highly optimized for Datalog evaluation
   - Relation lookups are indexed and efficient
   - The cost is likely negligible compared to the combinatorial explosion of term generation
   - If performance becomes an issue, Ascent supports incremental evaluation and other optimizations

3. **Simplicity**: One code generation path is easier to maintain and understand than two.

4. **Future-Proofing**:
   - Makes it easy to add matching modulo theories (AC, etc.)
   - Enables advanced features like conditional rewriting based on relations
   - Allows rewrite strategies to be expressed declaratively

5. **Consistency**: Everything in the theory (equations, rewrites, congruences) uses the same relation-based semantics.

## Implementation Plan

### Phase 1: Basic Ascent Rule Generation

1. **Modify `rewrite_gen.rs`:**
   - Instead of generating `try_rewrite_rule_N` functions
   - Generate Ascent rule clauses directly
   - Emit these as part of the theory source

2. **Pattern Matching:**
   - Use `if let` clauses for destructuring (as we already do in congruence rules)
   - Use `let` clauses for RHS construction

3. **Duplicate Variables:**
   - Track which variables appear multiple times in LHS
   - For first occurrence: bind to a variable
   - For subsequent occurrences: add `eq_cat(var1, var2)` clause

### Phase 2: Freshness Conditions

For `if x # P` conditions:
- Current: call `!term.contains_free(x)` in function
- New: Need to express as Ascent clause

**Option A:** Keep freshness as function call
```rust
rw_proc(s, t) <--
    proc(s),
    // ... pattern matching ...
    if !body.contains_free(&x),  // Still a function call
    let t = ...;
```

**Option B:** Freshness as relation
```rust
relation fresh(Binder<String>, Proc);
fresh(x, p) <-- proc(p), if !p.contains_free(&x);

rw_proc(s, t) <--
    proc(s),
    // ... pattern matching ...
    fresh(x, body),
    let t = ...;
```

Option A is simpler and likely sufficient for now.

### Phase 3: Substitution

Current RHS: `(subst P x (NQuote Q))`

This becomes:
```rust
let t = p.substitute(&x, &Name::NQuote(q.clone()));
```

This is straightforward—already supported in `let` clauses.

### Phase 4: Integration

1. **Remove function generation** from `rewrite_gen.rs`
2. **Generate Ascent clauses** instead, add to theory source
3. **Update `ascent_gen.rs`** to include rewrite clauses in theory source
4. **Remove calls** to `try_rewrite_rule_N` from default theory source

## Example Transformation

### Current (Function-Based)

Theory definition:
```rust
if x # Q then (PPar (PInput chan x P) (POutput chan Q))
    => (subst P x (NQuote Q));
```

Generated function:
```rust
pub fn try_rewrite_rule_0(term: &Proc) -> Option<Proc> {
    if let Proc::PPar(p_in, p_out) = term {
        if let Proc::PInput(chan1, scope1) = &(**p_in) {
            if let Proc::POutput(chan2, q) = &(**p_out) {
                if (**chan1).clone() == (**chan2).clone() {  // SYNTACTIC!
                    let (x, p) = scope1.clone().unbind();
                    if !p.contains_free(&x) {
                        return Some(p.substitute(&x, &Name::NQuote(q.clone())));
                    }
                }
            }
        }
    }
    None
}
```

Ascent invocation:
```rust
rw_proc(s, t.clone()) <--
    proc(s),
    if let Some(t) = try_rewrite_rule_0(&s);
```

### Proposed (Ascent-Based)

Generated Ascent clause:
```rust
rw_proc(s, t) <--
    proc(s),
    if let Proc::PPar(p_in, p_out) = s,
    if let Proc::PInput(chan1, scope1) = &**p_in,
    if let Proc::POutput(chan2, q) = &**p_out,
    eq_name((**chan1).clone(), (**chan2).clone()),  // EQUATIONAL!
    let (x, p) = scope1.clone().unbind(),
    if !p.contains_free(&x),
    let t = p.substitute(&x, &Name::NQuote((**q).clone()));
```

No function generated—just the clause in theory source.

## Challenges and Solutions

### Challenge 1: Ascent Syntax Limitations

**Issue:** Some complex patterns might be hard to express in Ascent's clause syntax.

**Solution:**
- For most cases, Ascent's `if let` and `let` are sufficient
- For truly complex cases, can still fall back to helper functions for RHS construction
- But keep matching (LHS) in Ascent for equational correctness

### Challenge 2: Performance Regression

**Issue:** Function calls might be faster than relation lookups.

**Solution:**
- Profile before optimizing
- Ascent is highly optimized; unlikely to be a bottleneck
- If needed, Ascent supports semi-naive evaluation and indexing
- Can use `#[derive(AscentProgram)]` optimizations

### Challenge 3: Debugging

**Issue:** Ascent rules might be harder to debug than functions.

**Solution:**
- Ascent allows inspecting relations (can see all matches)
- Actually easier to debug: can examine intermediate relations
- Can add helper relations for debugging (e.g., `relation rewrite_match(Proc, Name, Name)`)

### Challenge 4: Code Generation Complexity

**Issue:** Generating Ascent clauses is more complex than generating functions.

**Solution:**
- Reuse existing pattern generation from congruence rules
- The infrastructure for generating `if let` chains already exists
- Main new work: handling equational checks for duplicate variables

## Migration Path

1. **Implement new generation** alongside old (both work)
2. **Add feature flag** to choose between function-based and rule-based
3. **Test thoroughly** on rhocalc, ambient, other examples
4. **Compare performance** with benchmarks
5. **Switch default** to rule-based if tests pass
6. **Remove old code** after deprecation period

## Open Questions

1. **Should we keep functions as an optimization?**
   - Generate both: functions for syntactic match, rules for equational?
   - Use functions as fast path, fall back to rules?
   - Decision: No—keep it simple. One correct implementation.

2. **How to handle binders in equational matching?**
   - Should `eq_proc(PInput a x P, PInput b y Q)` hold if `a = b` and `P[x] = Q[y]`?
   - This requires alpha-equivalence
   - Current implementation: equations don't involve binders
   - Decision: Defer to future work; current equations don't need this

3. **Should congruence rules also be in theory source?**
   - Currently generated as part of theory source—correct approach
   - Keep as is

4. **Integration with normal forms?**
   - Normal form detection uses `!rw_proc(t, _)`
   - Works the same whether `rw_proc` comes from functions or rules
   - No change needed

## Conclusion

Moving rewrite rules from **generated functions** to **Ascent relation clauses** is the right design choice:

- **Correctness**: Enables equational matching for duplicate variables
- **Consistency**: All theory semantics (equations, rewrites, congruences) use the same mechanism
- **Extensibility**: Easier to add advanced matching features
- **Simplicity**: One code generation path, easier to maintain

The implementation is feasible and the performance impact should be minimal. This is a foundational improvement that makes the rewrite system semantically correct.

**Next Steps:**
1. Implement basic Ascent rule generation in `rewrite_gen.rs`
2. Handle duplicate variables with `eq_*` clauses
3. Test on rhocalc communication rule
4. Verify equational matching works correctly
5. Benchmark and optimize if needed

