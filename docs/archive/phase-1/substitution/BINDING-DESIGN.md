# Binding and Substitution Representation Design

**Date:** 2025-10-25
**Status:** Design Decision Needed

---

## The Problem

We need to represent variable binding and substitution in MeTTaIL theories. This affects:
- **Freshness conditions**: `if x # Q then ...`
- **Binders in grammar**: `(Bind x Name) "in" (x)Proc`
- **α-equivalence**: When are two terms "the same" modulo variable renaming?
- **Substitution**: How to replace variables safely

---

## Common Approaches

### 1. Named Representation (Naive)

**Representation:**
```rust
enum Proc {
    PZero,
    PNew { var: String, body: Box<Proc> },  // new x in P
    PDrop(Name),
}
```

**Pros:**
- Simple to understand
- Matches surface syntax directly
- Easy to pretty-print
- Natural for users

**Cons:**
- **Variable capture**: Substitution is complex and error-prone
- **α-equivalence**: Need to track renamings
- **Performance**: String comparisons are slow
- **Freshness**: Hard to generate fresh names correctly

**Example Problem:**
```rust
// Substitute P for x in: new x in *x
// Naive: new x in *P  -- WRONG! x is bound, shouldn't substitute
// Need to check if x is bound before substituting
```

---

### 2. De Bruijn Indices

**Representation:**
```rust
enum Proc {
    PZero,
    PVar(usize),  // 0 = innermost binder, 1 = next, etc.
    PNew(Box<Proc>),  // Binder is implicit, var is 0 in body
    PDrop(Name),
}
```

**Example:**
```
new x in new y in *(x @ y)
→ new (new (*(@(1) (0))))
  // 0 = y (innermost), 1 = x (outer)
```

**Pros:**
- **No variable capture**: Indices are unambiguous
- **α-equivalence is structural equality**: `new x in x == new y in y`
- **Substitution is well-defined**: Lift/shift operations
- **Efficient**: No string comparisons

**Cons:**
- **Hard to read**: Indices are unnatural for humans
- **Hard to pretty-print**: Need to reconstruct names
- **Complex lifting**: Substitution requires index shifting
- **Error-prone**: Off-by-one errors common

---

### 3. Locally Nameless

**Representation:**
```rust
enum Proc {
    PZero,
    PVarBound(usize),     // De Bruijn index for bound vars
    PVarFree(String),      // Name for free vars
    PNew(Box<Proc>),       // Binder is implicit
    PDrop(Name),
}
```

**Example:**
```
new x in (x @ y)  where y is free
→ new (*(@(0) (Free "y")))
  // 0 = bound x, "y" = free variable
```

**Pros:**
- **Best of both worlds**: Indices for bound, names for free
- **α-equivalence is easy**: Only compare indices for bound vars
- **Substitution is safe**: Only substitute free variables
- **Good pretty-printing**: Free vars already have names
- **Popular in proof assistants**: Coq, Agda use variants of this

**Cons:**
- **Two kinds of variables**: Slightly more complex
- **Need "opening" operation**: Convert bound to free for display
- **Still need shifting**: When going under binders

---

### 4. Higher-Order Abstract Syntax (HOAS)

**Representation:**
```rust
enum Proc {
    PZero,
    PVar(String),
    PNew(Box<dyn Fn(Proc) -> Proc>),  // Binder is a function!
    PDrop(Name),
}
```

**Example:**
```rust
// new x in *x
PNew(Box::new(|x| PDrop(NQuote(x))))
```

**Pros:**
- **Host language handles binding**: Rust's scoping rules
- **No variable capture**: Rust prevents it
- **Elegant for deeply embedded DSLs**

**Cons:**
- **Not serializable**: Can't send functions over network
- **Can't inspect structure**: Functions are opaque
- **Requires traits**: `Fn` trait is complex
- **Not suitable for MeTTaIL**: We need to inspect/transmit terms

---

### 5. Named with Explicit Scopes (Hybrid)

**Representation:**
```rust
enum Proc {
    PZero,
    PVar(String),
    PNew {
        var: String,
        var_id: usize,      // Unique ID for this binder
        body: Box<Proc>
    },
    PDrop(Name),
}
```

**Track scope separately:**
```rust
struct Scope {
    bindings: HashMap<String, Vec<usize>>,  // Stack of binder IDs
}
```

**Pros:**
- **Named variables for UX**: Easy to read and print
- **IDs prevent capture**: Substitution checks IDs
- **Flexible**: Can convert to other representations
- **Suitable for gradual typing**: Can add IDs incrementally

**Cons:**
- **Need to maintain IDs**: Extra bookkeeping
- **More complex data structure**: Scope must be threaded through
- **Still need freshness**: ID generation

---

## Recommendation for MeTTaIL

### Phase 1 (Current): **Named with Scope Tracking**

**Rationale:**
1. **User-facing**: MeTTaIL theories are written by humans
2. **Surface syntax**: Matches BNFC grammar directly
3. **Gradual complexity**: Start simple, refine later
4. **Good for macros**: Macro expansion works with names

**Implementation:**
```rust
// AST keeps names (user-friendly)
pub enum Expr {
    Var(Ident),
    Apply { constructor: Ident, args: Vec<Expr> },
}

// Scope tracks binding structure
pub struct Scope {
    bound_vars: HashMap<String, String>,  // var -> category
    free_vars: HashMap<String, String>,
}

// Type-checker uses scope to avoid capture
impl TypeChecker {
    fn infer_with_scope(
        &self,
        expr: &Expr,
        scope: &Scope
    ) -> Result<String, TypeError> {
        match expr {
            Expr::Var(v) => {
                if scope.is_bound(v) {
                    Ok(scope.get_category(v).unwrap())
                } else {
                    // Free variable
                    Ok("?".to_string())
                }
            }
            // ...
        }
    }
}
```

**Later phases can add:**
- IDs for binders (when we add runtime execution)
- Conversion to De Bruijn (when we need structural equality)
- Locally nameless (when we implement substitution)

---

### Phase 2-3: **Locally Nameless** (Recommended Future)

**When we implement:**
- Runtime term rewriting
- Pattern matching
- Network transmission

**Migration path:**
```rust
// Phase 1: Parse to named AST
theory! { ... } → NamedExpr

// Phase 2: Convert to locally nameless for execution
NamedExpr → LocallyNamelessExpr → execute() → LocallyNamelessExpr

// Display: Convert back to named
LocallyNamelessExpr → NamedExpr → pretty_print()
```

---

## Specific Design for Phase 1

### Grammar Binders

**Syntax:**
```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    terms {
        PNew . Proc ::= "new" (Bind x Name) "in" (x)Proc ;
        //                     ^^^^^^^^^^^^       ^^^
        //                     Binder declares    Uses bound var
    }
}
```

**AST Representation:**
```rust
pub struct GrammarRule {
    pub label: Ident,
    pub category: Ident,
    pub items: Vec<GrammarItem>,
    pub binders: Vec<BinderInfo>,  // NEW
}

pub struct BinderInfo {
    pub var: Ident,           // Variable name (e.g., "x")
    pub category: Ident,      // Category (e.g., "Name")
    pub scope: BinderScope,   // What it binds in
}

pub enum BinderScope {
    RestOfRule,               // Binds in remaining items
    Explicit(Vec<usize>),     // Binds in specific item indices
}
```

**Generated Code:**
```rust
// For: PNew . Proc ::= "new" (Bind x Name) "in" (x)Proc
#[derive(Clone, Debug, PartialEq)]
pub enum Proc {
    PNew {
        var_name: String,      // Keep name for display
        var_category: Name,     // The bound variable's value
        body: Box<Proc>,       // Body where var is bound
    },
    // ...
}
```

---

### Freshness Conditions

**Current validation (Phase 1):**
```rust
// if x # P then (PDrop (NQuote (PNew x P))) == P
fn validate_freshness(cond: &FreshnessCondition, eq: &Equation) -> Result<(), String> {
    let var = cond.var.to_string();
    let term = cond.term.to_string();

    // 1. Check both appear in equation ✓ (already done)
    // 2. Check x != term ✓ (already done)

    // 3. TODO: Check x doesn't appear free in term
    //    For now: accept any condition
    //    Later: use scope analysis to verify
    Ok(())
}
```

**Future (Phase 2):**
```rust
fn validate_freshness_semantic(
    cond: &FreshnessCondition,
    eq: &Equation,
    scope: &Scope
) -> Result<(), String> {
    // Check that x does not appear free in the binding of term
    let free_in_term = analyze_free_vars(&get_binding(term), scope);
    if free_in_term.contains(&var) {
        return Err("Freshness violated: x appears in term");
    }
    Ok(())
}
```

---

### Substitution (Future)

**Not needed in Phase 1**, but design for it:

```rust
// Phase 2: Add substitution support
impl Substitutable for Proc {
    fn substitute(&self, var: &str, value: &Self, scope: &Scope) -> Self {
        match self {
            Proc::PVar(v) if v == var && !scope.is_bound(v) => {
                // Only substitute if not bound
                value.clone()
            }
            Proc::PNew { var_name, var_category, body } => {
                // Don't substitute inside binder if shadowed
                if var_name == var {
                    self.clone()  // Shadowed
                } else {
                    let mut new_scope = scope.child();
                    new_scope.bind_var(var_name.clone(), "Name".to_string());
                    Proc::PNew {
                        var_name: var_name.clone(),
                        var_category: var_category.clone(),
                        body: Box::new(body.substitute(var, value, &new_scope)),
                    }
                }
            }
            // ...
        }
    }
}
```

---

## Decision Matrix

| Approach | Phase 1 Feasibility | User Friendliness | Correctness | Future-Proof |
|----------|-------------------|------------------|------------|--------------|
| Named (naive) | ✅ Easy | ✅ Best | ⚠️ Capture bugs | ❌ Need rewrite |
| De Bruijn | ⚠️ Complex | ❌ Unreadable | ✅ Correct | ✅ Good |
| **Locally Nameless** | ⚠️ Medium | ✅ Good | ✅ Correct | ✅✅ Best |
| HOAS | ❌ Hard | ✅ Good | ✅ Correct | ❌ Not suitable |
| **Named + Scope** | ✅✅ Easy | ✅✅ Best | ✅ Correct | ✅ Upgradeable |

---

## Final Recommendation

### For Phase 1: **Named Representation with Scope Tracking**

**Why:**
1. ✅ Matches user syntax directly
2. ✅ Simple to implement in macros
3. ✅ Easy to debug and test
4. ✅ Correct with proper scope tracking
5. ✅ Can upgrade to locally nameless later

**Limitations we accept:**
- No term rewriting yet (Phase 2)
- No runtime substitution yet (Phase 2)
- No network transmission yet (Phase 2)

### For Phase 2+: **Upgrade to Locally Nameless**

**When we need:**
- Term rewriting/evaluation
- Pattern matching
- Efficient α-equivalence
- Network transmission

**Migration:**
```rust
Named AST (Parse) → Locally Nameless (Execute) → Named AST (Display)
```

---

## Implementation Plan

### Week 1 ✅ (Done)
- [x] Freshness validation (basic)
- [x] Scope tracking infrastructure

### Week 2 (Current)
- [ ] Runtime AST with named vars ← **CURRENT**
- [ ] Binder syntax in grammar
- [ ] Scope-aware type checking

### Week 3
- [ ] Parser generation with binder support
- [ ] Pretty-printing with names

### Week 4
- [ ] Complete binder validation
- [ ] Test with RhoCalc subset

### Phase 2 (Future)
- [ ] Add locally nameless representation
- [ ] Implement substitution
- [ ] Add term rewriting

---

## Open Questions

### Q1: Should we add unique IDs to binders in Phase 1?

**Option A:** Just names (simpler)
```rust
PNew { var: String, body: Box<Proc> }
```

**Option B:** Names + IDs (safer)
```rust
PNew { var: String, var_id: usize, body: Box<Proc> }
```

**Recommendation:** **Option A** for Phase 1, add IDs in Phase 2 if needed.

---

### Q2: How to handle multiple binders?

```rust
// new x, y in P
PNew { vars: Vec<String>, body: Box<Proc> }

// vs

PNew { var: String, body: Box<Proc> }  // Nest for multiple
```

**Recommendation:** Single binder (nest for multiple) - simpler and more general.

---

### Q3: Freshness semantics?

**Option A:** Syntactic (Phase 1)
- `x # Q` means "x doesn't appear free in Q" (syntactic check)

**Option B:** Semantic (Phase 2)
- `x # Q` means "x is freshly generated" (runtime check)

**Recommendation:** **Option A** for Phase 1, add Option B for runtime in Phase 2.

---

## Conclusion

**Start with Named + Scope**, upgrade to **Locally Nameless** when we need execution.

This gives us:
- ✅ Simple Phase 1 implementation
- ✅ Good user experience
- ✅ Correct binding with scope tracking
- ✅ Clear migration path to efficient execution

**Next step:** Implement named binders in grammar parsing and code generation.

**Shall we proceed with this approach?**

