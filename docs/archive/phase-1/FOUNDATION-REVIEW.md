# Phase 1 Foundation Review

**Date:** 2025-10-25
**Purpose:** Comprehensive review of what's needed for a proper foundation

---

## Executive Summary

After implementing basic type-checking, I've identified **7 critical gaps** that must be filled for a proper Phase 1 foundation. The current plan covers parser generation and binders, but misses several essential features needed for correctness.

---

## Current State ✅

### What Works
1. **Type-checking** with context tracking
   - Infers types from constructor usage
   - Tracks variable types across equation sides
   - Validates type consistency

2. **Equation parsing**
   - Handles `(LHS) == (RHS)` syntax
   - Parses freshness conditions `if x # Q then ...`
   - Supports nested expressions

3. **Basic validation**
   - Checks constructors are defined
   - Validates exported categories
   - Catches simple errors at compile-time

---

## Critical Gaps 🔴

### 1. Freshness Condition Validation (HIGHEST PRIORITY)

**Problem:** We parse `if x # Q then ...` but don't validate it

**Impact:** Can't properly define theories with scoping rules (e.g., `NewReplCalc`)

**What's Missing:**
```rust
// Need to validate:
equations {
    if x # Q then (NQuote (PNew x Q)) == Q
}
// Check:
// 1. x is a variable
// 2. Q is a valid term
// 3. x doesn't appear free in Q (freshness semantics)
// 4. The equation makes sense given the freshness constraint
```

**Where:** `mettail-macros/src/validator.rs` or new `freshness.rs`

**Estimated:** 1-2 days

---

### 2. Variable Scoping (HIGHEST PRIORITY)

**Problem:** No distinction between bound and free variables

**Impact:** Can't handle binders, can't validate freshness properly

**What's Missing:**
```rust
pub struct Scope {
    // Variables bound in this scope
    bound_vars: HashMap<String, String>, // var -> category
    // Free variables
    free_vars: HashMap<String, String>,
}

impl TypeChecker {
    pub fn infer_type_with_scope(
        &self,
        expr: &Expr,
        scope: &Scope,
        context: &mut HashMap<String, String>
    ) -> Result<String, TypeError> {
        // Handle bound vs free vars differently
    }
}
```

**Example:**
```rust
// In: (Bind x Name) "in" Proc
// x is bound in Proc
// Need to track this for freshness checking
```

**Where:** `mettail-macros/src/typechecker.rs` + `scope.rs`

**Estimated:** 2-3 days

---

### 3. Runtime AST Foundation (HIGH PRIORITY)

**Problem:** `mettail-runtime` is completely empty

**Impact:** Generated code has nowhere to go, can't instantiate terms

**What's Missing:**
```rust
// mettail-runtime/src/lib.rs

/// Base trait for all generated AST nodes
pub trait Term: Clone + Debug + PartialEq {
    /// Get the category/type of this term
    fn category(&self) -> &'static str;

    /// Pretty-print the term
    fn display(&self) -> String;
}

/// Trait for theories with equations
pub trait HasEquations<T: Term> {
    /// Check if two terms are equal according to theory equations
    fn equal(&self, left: &T, right: &T) -> bool;
}

/// Parser trait
pub trait Parser<T: Term> {
    type Error: std::error::Error;

    fn parse(&self, input: &str) -> Result<T, Self::Error>;
}
```

**Usage in generated code:**
```rust
theory! {
    name: MyTheory,
    exports { Proc }
    terms { ... }
}

// Should generate:
#[derive(Clone, Debug, PartialEq)]
pub enum Proc {
    PZero,
    PDrop(Name),
}

impl Term for Proc {
    fn category(&self) -> &'static str { "Proc" }
    // ...
}
```

**Where:** `mettail-runtime/src/lib.rs`, `term.rs`, `parser.rs`

**Estimated:** 2-3 days

---

### 4. Error Messages with Spans (HIGH PRIORITY)

**Problem:** Errors are just strings, no location information

**Impact:** Hard to debug macro usage, poor developer experience

**What's Missing:**
```rust
use syn::spanned::Spanned;

pub enum ValidationError {
    UnknownConstructor {
        name: String,
        span: proc_macro2::Span,
    },
    TypeMismatch {
        expected: String,
        found: String,
        context: String,
        span: proc_macro2::Span,
    },
    FreshnessViolation {
        var: String,
        term: String,
        span: proc_macro2::Span,
    },
    // etc.
}

impl ValidationError {
    pub fn to_compile_error(&self) -> proc_macro2::TokenStream {
        let span = self.span();
        let msg = self.message();
        quote_spanned!(span => compile_error!(#msg))
    }
}
```

**Example error output:**
```
error: Type mismatch in equation
  --> examples/quote_drop.rs:10:9
   |
10 |         (PZero) == (NQuote (PZero))
   |         ^^^^^^^ expected type 'Proc', found type 'Name'
```

**Where:** All files - `ast.rs`, `validator.rs`, `typechecker.rs`

**Estimated:** 1-2 days

---

### 5. Complete Category Validation (MEDIUM PRIORITY)

**Problem:** We check exports but not all category references in grammar

**Impact:** Could reference undefined categories, leading to runtime errors

**What's Missing:**
```rust
fn validate_grammar_rule(rule: &GrammarRule, theory: &TheoryDef)
    -> Result<(), ValidationError>
{
    // 1. Check that rule.category is exported
    if !theory.exports.iter().any(|e| e.name == rule.category) {
        return Err(ValidationError::CategoryNotExported {
            category: rule.category.to_string(),
            span: rule.category.span(),
        });
    }

    // 2. Check all NonTerminals reference exported categories
    for item in &rule.items {
        if let GrammarItem::NonTerminal(cat) = item {
            if !theory.exports.iter().any(|e| e.name == *cat) {
                return Err(ValidationError::UndefinedCategory {
                    category: cat.to_string(),
                    span: cat.span(),
                });
            }
        }
    }

    Ok(())
}
```

**Example catch:**
```rust
exports { Proc }
terms {
    Foo . Bar ::= (Baz) ;  // Error: Bar not exported, Baz not exported
}
```

**Where:** `mettail-macros/src/validator.rs`

**Estimated:** 0.5-1 day

---

### 6. Comprehensive Testing (MEDIUM PRIORITY)

**Problem:** Limited test coverage, no systematic testing

**Impact:** Bugs slip through, regressions not caught

**What's Missing:**

#### A. Compile-Fail Tests (using `trybuild`)
```rust
// tests/compile_fail/unknown_category.rs
use mettail_macros::theory;

theory! {
    name: Invalid,
    exports { Proc }
    terms {
        Foo . UndefinedCat ::= "foo" ;  // Should fail
    }
}

fn main() {}
```

```rust
// tests/compile_fail/unknown_category.stderr
error: Category 'UndefinedCat' is not exported
 --> tests/compile_fail/unknown_category.rs:6:15
  |
6 |         Foo . UndefinedCat ::= "foo" ;
  |               ^^^^^^^^^^^^
```

#### B. Integration Tests
```rust
// tests/integration/roundtrip.rs
#[test]
fn test_parse_and_roundtrip() {
    let input = "(Plus (Zero) (Succ (Zero)))";
    let parsed = Elem::parse(input).unwrap();
    let output = parsed.display();
    assert_eq!(input, output);
}
```

#### C. Property-Based Tests (using `proptest`)
```rust
// tests/property/typechecking.rs
proptest! {
    #[test]
    fn well_typed_expr_never_fails(expr in arb_well_typed_expr()) {
        let type_checker = TypeChecker::new(&test_theory());
        prop_assert!(type_checker.infer_type(&expr).is_ok());
    }
}
```

**Structure:**
```
mettail-macros/
├── tests/
│   ├── compile_fail/       # Tests that should fail compilation
│   │   ├── unknown_category.rs
│   │   ├── type_mismatch.rs
│   │   ├── undefined_constructor.rs
│   │   └── *.stderr        # Expected error messages
│   ├── compile_pass/       # Tests that should succeed
│   │   ├── simple_theory.rs
│   │   ├── equations.rs
│   │   └── composition.rs
│   ├── integration/        # End-to-end tests
│   │   ├── parser_roundtrip.rs
│   │   └── equation_checking.rs
│   └── property/           # Property-based tests
│       └── typechecking.rs
```

**Dependencies:**
```toml
[dev-dependencies]
trybuild = "1.0"
proptest = "1.0"
```

**Where:** `mettail-macros/tests/`

**Estimated:** 2-3 days (ongoing)

---

### 7. Documentation (MEDIUM PRIORITY)

**Problem:** Minimal documentation for users and contributors

**Impact:** Hard to use, hard to contribute to

**What's Missing:**

#### A. API Documentation
```rust
/// Type checker for MeTTaIL theories
///
/// Validates that:
/// - All expressions are well-typed
/// - Equations have matching types on both sides
/// - Variables are used consistently
///
/// # Example
/// ```
/// let theory = TheoryDef { /* ... */ };
/// let type_checker = TypeChecker::new(&theory);
/// type_checker.validate_equations(&theory.equations)?;
/// ```
pub struct TypeChecker { /* ... */ }
```

#### B. User Guide
```markdown
# Defining Theories in Rust

## Basic Structure
theory! {
    name: MyTheory,

    exports {
        Category1
        Category2
    }

    terms {
        Constructor1 . Category1 ::= "literal" ;
        Constructor2 . Category2 ::= (Category1) "+" (Category1) ;
    }

    equations {
        (Constructor2 x y) == (Constructor2 y x)  // Commutativity
    }
}

## Type Checking
...
```

#### C. Examples with Explanations
```rust
/// Example: Simple arithmetic expressions with equations
///
/// This demonstrates:
/// - Defining categories (Elem)
/// - Defining constructors (Zero, Succ, Plus)
/// - Defining equations (commutativity, associativity)
/// - Type-checking
mod arithmetic {
    use mettail_macros::theory;

    theory! {
        name: Arithmetic,
        exports { Elem }
        // ...
    }
}
```

**Where:**
- API docs: Throughout source files
- User guide: `docs/USER-GUIDE.md`
- Examples: `examples/` with comments

**Estimated:** 2-3 days

---

## Revised Phase 1 Timeline

### Original Plan (3 weeks)
- Week 1: Type-checking + Equations
- Week 2: Parser generation
- Week 3: Binders + composition

### Revised Plan (4 weeks)

#### Week 1: Type System + Validation ✅
- [x] Day 1-2: Basic type-checking ✅
- [x] Day 3-4: Equations + context tracking ✅
- [ ] **Day 5-6: Freshness validation** ⭐
- [ ] **Day 7: Variable scoping** ⭐

#### Week 2: Runtime + Error Handling
- [ ] **Day 1-2: Runtime AST types** ⭐
- [ ] **Day 3-4: Error spans** ⭐
- [ ] **Day 5: Category validation** ⭐
- [ ] Day 6-7: Testing infrastructure

#### Week 3: Parser Generation
- [ ] Day 1-2: Parser trait generation
- [ ] Day 3-4: Terminal/non-terminal handling
- [ ] Day 5: Recursive parsing
- [ ] Day 6-7: Error handling + tests

#### Week 4: Binders + Integration
- [ ] Day 1-2: Binder syntax parsing
- [ ] Day 3-4: Scope tracking + freshness integration
- [ ] Day 5-6: Testing + documentation
- [ ] Day 7: Polish + examples

---

## Priority Matrix

| Feature | Priority | Impact | Effort | When |
|---------|----------|--------|--------|------|
| Freshness validation | 🔴 Critical | High | 1-2d | Week 1 |
| Variable scoping | 🔴 Critical | High | 2-3d | Week 1 |
| Runtime AST | 🔴 Critical | High | 2-3d | Week 2 |
| Error spans | 🟡 High | Med | 1-2d | Week 2 |
| Category validation | 🟡 High | Low | 0.5d | Week 2 |
| Parser generation | 🟡 High | High | 5-6d | Week 3 |
| Binders | 🟡 High | High | 2-3d | Week 4 |
| Testing | 🟢 Med | Med | 2-3d | Ongoing |
| Documentation | 🟢 Med | Low | 2-3d | Ongoing |

---

## Open Questions

### 1. Freshness Semantics
**Question:** What exactly should `x # Q` validate?

**Options:**
- A: `x` does not appear free in `Q` (syntactic freshness)
- B: `x` is a fresh name, globally unique (semantic freshness)
- C: Both - syntactic check at compile-time, uniqueness at runtime

**Recommendation:** Start with **Option A** (simpler), add runtime checks later if needed

---

### 2. Runtime Execution Scope
**Question:** Should Phase 1 include term rewriting/evaluation?

**Current plan:** No, just AST types and parsing

**Reasoning:**
- Rewriting is complex (needs RETE, e-graphs, or similar)
- Can be deferred to Phase 2
- Phase 1 focuses on **definitions**, Phase 2 on **execution**

**Alternative:** Include basic structural equality checking

**Recommendation:** Defer to Phase 2, but include `PartialEq` for AST nodes

---

### 3. Parser Library Choice
**Question:** Which parser library for combinator generation?

**Options:**
- `nom`: Popular, well-tested, fast
- `pest`: PEG-based, simpler for some grammars
- `lalrpop`: LR(1), best performance, complex integration
- Hand-rolled: Maximum control, more work

**Recommendation:** **`nom`** for reliability and performance

**Reasoning:**
- Battle-tested
- Good error messages
- Flexible enough for our needs
- Can switch to LALRPOP in Phase 2 if needed

---

### 4. Binder Representation
**Question:** How to represent binders in the AST?

**Options:**
```rust
// A: Extend Expr with Binder variant
pub enum Expr {
    Var(Ident),
    Apply { constructor: Ident, args: Vec<Expr> },
    Binder { var: Ident, scope: Box<Expr> },  // NEW
}

// B: Separate type for binders
pub struct BinderExpr {
    var: Ident,
    var_category: Ident,
    body: Expr,
}

// C: Track in grammar rules
pub struct GrammarRule {
    // ...
    bound_vars: Vec<(Ident, Ident)>,  // (var, category)
}
```

**Recommendation:** **Option C** - track in grammar, generate special AST variants

**Reasoning:**
- Keeps parsing separate from semantics
- Each theory gets custom binder handling
- More flexible for different binding patterns

---

## Success Criteria (Updated)

### Phase 1 Complete When:

#### Core Features ✅
- [ ] **Freshness conditions** validated at compile-time
- [ ] **Variable scoping** distinguishes bound vs free
- [ ] **Runtime types** exist (`Term`, `Parser` traits)
- [ ] **Parsers** generated from grammar rules
- [ ] **Binders** work with proper scoping

#### Quality Features 🔶
- [ ] **Error messages** show spans and are helpful
- [ ] **All categories** validated in grammar
- [ ] **Test suite** comprehensive:
  - Unit tests for each validator
  - Compile-fail tests for bad theories
  - Integration tests for complete theories
  - Property tests for invariants

#### Examples 📚
- [ ] `simple_monoid` - Basic theory
- [ ] `quote_drop` - Equations without freshness
- [ ] `new_repl` - Equations with freshness
- [ ] `par_monoid` - Theory composition (if time)

#### Documentation 📖
- [ ] API docs on all public items
- [ ] User guide for defining theories
- [ ] Contributor guide for extending system

---

## Risk Assessment

### High Risk 🔴
1. **Freshness semantics** - Subtle, need to get right
   - Mitigation: Start simple, add complexity incrementally

2. **Parser generation** - Complex, might take longer
   - Mitigation: Use proven library (`nom`), start with simple cases

3. **Binders** - Scope handling is tricky
   - Mitigation: Study existing implementations (Bound library)

### Medium Risk 🟡
4. **Runtime design** - Need to get traits right for future
   - Mitigation: Start minimal, extend as needed

5. **Testing completeness** - Hard to test macros
   - Mitigation: Use `trybuild` for compile-time tests

### Low Risk 🟢
6. **Error spans** - Straightforward with `syn`
7. **Category validation** - Simple extension

---

## Recommendation

### Option A: Thorough Foundation (4 weeks)
**Includes:** All 7 critical features, comprehensive testing, good docs
**Pros:** Solid foundation, fewer bugs, easier to extend
**Cons:** Takes longer

### Option B: Minimal Viable (3 weeks)
**Includes:** Freshness, scoping, runtime, parser gen, binders
**Defer:** Error spans, complete testing, docs to Phase 1.5
**Pros:** Faster to Phase 2
**Cons:** More technical debt

### My Recommendation: **Option A** (4 weeks)

**Reasoning:**
- You emphasized "proper foundation" and "be thorough"
- The 7 critical features are all essential for correctness
- Better to take 1 extra week now than 3 weeks fixing bugs later
- Testing and error messages are part of the foundation, not polish

---

## Next Steps

### Immediate (This Week)
1. **Freshness validation** - Start with syntactic check
2. **Variable scoping** - Extend `TypeChecker` with `Scope`
3. **Runtime foundation** - Define `Term` trait

### Next Week
4. **Error spans** - Add span tracking to all errors
5. **Category validation** - Complete validation coverage
6. **Testing setup** - Add `trybuild`, start test suite

### Then
7. **Parser generation** (Week 3)
8. **Binders** (Week 4)
9. **Polish & docs** (Week 4)

---

## Go-to-Definition Fix ✅

The workspace is already properly set up! `Cargo.toml` at root defines workspace with all members. Rust-analyzer should now work.

To verify:
```bash
cd /Users/cbwells/Documents/GitHub/rholang/f1r3node/mettail-rust-exploration
cargo metadata --no-deps | jq '.workspace_members'
```

If go-to-def still doesn't work:
1. Reload VS Code / Cursor
2. Run "Rust Analyzer: Restart Server" from command palette
3. Check that `.vscode/settings.json` doesn't have conflicting settings

---

**Question for you:**

Do you agree with this analysis? Should we:
1. ✅ Proceed with 4-week plan (thorough foundation)
2. ⚡ Compress to 3 weeks (defer testing/docs)
3. 🔧 Adjust priorities (which features?)

