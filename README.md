# MeTTaIL: Metalanguage for language implementation

**Status:** Collection Types ✅ | Automatic Flattening ✅ | Term Explorer REPL (Next)

---

## 📖 Quick Links

- **[Current Status](docs/CURRENT-STATUS.md)** - What works now and recent progress 📊
- **[Poly-Lingual Roadmap](docs/POLY-LINGUAL-ROADMAP.md)** - 3-year strategic vision 🎯
- **[Quick Start Guide](QUICKSTART.md)** - Get started in 5 minutes
- **[Known Limitations](docs/KNOWN-LIMITATIONS.md)** - Current gaps and TODOs
- **[Term Explorer REPL Design](docs/design/TERM-EXPLORER-REPL-DESIGN.md)** - Next: Interactive exploration tool 🎯

---

## 🎯 What is MeTTaIL?

MeTTaIL is a **meta-language framework** for **poly-lingual computation** - enabling seamless interoperability between formal languages with production-grade performance.

### Vision: Poly-Lingual Computation

**The Problem:** Modern systems need multiple programming paradigms, but language interoperability is hard:
- Process calculi for concurrency
- Lambda calculi for functional programming
- Logic programming for constraints
- Linear logic for resource management

**MeTTaIL's Solution:** Define languages formally with shared semantics, compose them, and execute efficiently.

### How MeTTaIL Works

MeTTaIL lets you define formal languages through:
1. **Operations** - BNF-like syntax with binders
2. **Equations** - Structural equivalences  
3. **Rewrites** - Computational rules with substitution

Then automatically generates:
- Type-safe AST with term sorting
- LALRPOP parsers with precedence
- Ascent-based rewrite engine with equational matching
- Display, substitution, and term generation

### Example: Rho Calculus in MeTTaIL

```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;  // AC operation
        PDrop . Proc ::= "*" Name ;
        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    }
    
    equations {
        (NQuote (PDrop N)) == N ;      // Reflection
        (PPar {P}) == P ;              // Identity normalization
        (PPar {}) == PZero ;           // Empty is zero
        // Note: Flattening is automatic! No equations needed for {a, {b, c}} == {a, b, c}
    }
    
    rewrites {
        // Communication: for(chan->x){P} , chan!(Q) => P[@Q/x]
        (PPar {(PInput chan x P), (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
    }
}
```

**Generated:** Type-safe AST, parser, substitution, **order-independent rewrite engine**, and more!

---

## ✅ What Works Now

### Core Features (Phase 6 Complete)
- ✅ **Theory Definition** - Declarative syntax with macros
- ✅ **Type-Checking** - Sound category inference
- ✅ **Binders & Variables** - Correct scoping via `moniker`
- ✅ **Cross-Category Substitution** - Full support for heterogeneous substitution
- ✅ **Collection Types** - HashBag for associative-commutative operations
- ✅ **Order-Independent Matching** - Indexed projection for optimal performance
- ✅ **Automatic Flattening** - Nested collections flatten structurally (no equations needed!)
- ✅ **Rest Patterns** - Extract and reconstruct collection remainders
- ✅ **Collection Equations** - Automatic normalization (e.g., `{P} == P`)
- ✅ **Performance Optimized** - 42x speedup via lazy deconstruction

### Key Innovation: Automatic Flattening

Collections automatically flatten during construction - no complex equations required:

```rust
// Automatic: {a, {b, c}} → {a, b, c}
// During substitution or rewrite construction
// Generated helper functions handle recursive flattening
// Zero user burden!
```

**Before:** Required complex equations with rest patterns  
**After:** Structural property, always correct by construction

### Code Generation
From a theory definition, MeTTaIL generates:
- **AST enums** - Clean, type-safe data structures with term sorting (`Ord`)
- **LALRPOP grammars** - Full parser generation with precedence handling
- **Substitution methods** - Capture-avoiding, cross-category
- **Ascent-based rewrite engine** - Order-independent pattern matching with indexed joins
- **Collection support** - `HashBag<T>` fields with efficient equality and hashing
- **Display implementations** - Pretty-printing with correct precedence
- **Type derivations** - `Debug`, `Clone`, `PartialEq`, `Eq`, `Ord`, `BoundTerm`, `Display`

---

## ✅ Phase 2: Parser & Rewrite Engine (COMPLETE)

### What We Built
- ✅ **Precedence-Aware Grammars** - Automatic handling of infix operators
- ✅ **Binder Parsing** - Direct parsing into `Scope` structures with proper variable binding
- ✅ **Parentheses Support** - Override precedence with grouping
- ✅ **Left-Associativity** - Correct parsing of `a | b | c` as `((a | b) | c)`
- ✅ **Equational Rewrite Matching** - Ascent-based rewrites with `eq_cat()` relations for duplicate variables
- ✅ **Nested Pattern Matching** - Arbitrary-depth pattern matching with binder extraction
- ✅ **Type-Aware Variable Tracking** - Category inference from constructor applications
- ✅ **Freshness Checking** - Automatic generation of `x # Q` checks
- ✅ **Capture-Avoiding Substitution** - Full integration with generated rewrite engine

### Current Performance Characteristics
- **Small terms (depth ≤3)**: ~100ms
- **Medium terms (depth 4-5)**: ~500ms  
- **Complex terms (depth 6+)**: ~2 seconds

**Recent Achievement**: 42x speedup via lazy deconstruction (18.5s → 435ms for complex examples)

**Remaining Bottleneck**: Deep nesting in collection patterns requires indexed projection, which is only implemented for flat shared variables. Deep projection (nested shared variables) planned for Q1 2026.

### Demo: Execution
```bash
$ cargo run --bin rhocalc

=== Rho Calculus Rewrite Demo ===

Input:  for(a<-x){*x}|a!(0)

Step 1: *@(0)

→ Normal form reached after 1 step(s)

✅ Rho Calculus Theory Compiled Successfully!
```

### Generated Rewrite Engine Example

**Ascent-based equational matching** (new approach):
```rust
// Generated Ascent clause for: 
// if x # Q then (PPar (PInput chan x P) (POutput chan Q)) => (subst P x (NQuote Q))

rw_proc(s, t) <--
    proc(s),
    if let Proc::PPar(s_f0, s_f1) = s,
    if let Proc::PInput(s_f0_f0_scope) = &**s_f0,
    let (s_f0_f0, s_f0_f1_scope) = s_f0_f0_scope.clone().unbind(),
    let (s_f0_f1, s_f0_f2) = s_f0_f1_scope.unbind(),
    if let Proc::POutput(s_f1_f0, s_f1_f1) = &**s_f1,
    
    // Equational matching: channels must be equal modulo equations
    eq_name((**s_f0_f0).clone(), (**s_f1_f0).clone()),
    
    // Freshness check
    if mettail_runtime::is_fresh(&s_f0_f1.0, &**s_f1_f1),
    
    // RHS construction with substitution
    let t = s_f0_f2.substitute_name(
        &s_f0_f1.0,
        &Name::NQuote((**s_f1_f1).clone())
    );
```

**Key features**:
- Equational matching via `eq_name()` instead of syntactic equality
- Integrates with Datalog semantics for transitive/symmetric closure
- Type-safe variable binding with category tracking

---

## 🎯 Current Focus: Q1 2026

### Next: Term Explorer REPL & Core Completeness

**Priority Order:**
1. **Term Generation for Collections** (2 weeks) - Unblock automated testing
2. **Deep Projection for Ambient Calculus** (3-4 weeks) - Fix nested pattern matching
3. **Term Explorer REPL** (4 weeks) - Interactive exploration and debugging
4. **Debugging & Diagnostics** (2-3 weeks) - Developer tooling

See [Poly-Lingual Roadmap](docs/POLY-LINGUAL-ROADMAP.md) for details.

### Term Explorer REPL Vision

```bash
$ mettail repl rhocalc
Theory: RhoCalc loaded

> term: {a!(0), for(a->x0){*x0}}
Running Ascent... Done. (45 terms, 62 rewrites, 11 normals)

[1] Show normal forms
[2] Show next rewrites (1 available)
[3] Explore rewrite graph

> 2
Next rewrites:
  [a] {*@(0)} via communication

> a
Stepped to: {*@(0)}  [Normal form ✓]
```

**Goal:** Make MeTTaIL accessible and debuggable through interactive exploration.

---

## 📊 Current Stats

- **~5500 LOC** - Core implementation
- **20+ tests** - All passing ✅
- **Working demos** - Rho Calculus and Ambient Calculus with execution
- **0.8s compile** - For Rho Calculus theory
- **~13000 LOC** - Generated for Rho Calculus (AST + parser + substitution + Ascent rules)

---

## 🔬 Technical Highlights

### Binder Handling
```rust
// Grammar:
PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
//                          ^^^^  ^^^^^^         ^^^^
//                        channel  binder       body

// Generated:
PInput(Box<Name>, Scope<Binder<String>, Box<Proc>>)
//     ^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     channel    automatically handles capture-avoidance
```

---

## 🎓 Research Questions

### Current Challenges
1. **Performance** - How to efficiently handle AC operations without exponential blowup?
   - *Solution in progress*: Collection-based representations (HashBag)
2. **Type System** - How to balance expressiveness with inferability?
3. **Equations vs. Rewrites** - When to use each, and how do they interact?

### Semantics
1. **Congruence** - Auto-generate `s => t` ⊢ `P|s => P|t`?
2. **Confluence** - Check or assume?
3. **Termination** - Prove or bound?

### Usability
1. **Error messages** - How to make them great?
2. **IDE support** - What features matter most?
3. **Debuggability** - How to step through rewrites?



## 🙏 Credits

**Core Technologies:**
- [ascent] - Datalog in Rust via macros
- [syn](https://github.com/dtolnay/syn) - Rust parsing
- [quote](https://github.com/dtolnay/quote) - Code generation
- [moniker](https://github.com/brendanzab/moniker) - Variable binding
- [LALRPOP](https://github.com/lalrpop/lalrpop) - Parser generator (Phase 2)

**Inspiration:**
- [Rholang](https://rchain.coop/) - Motivating use case
- [K Framework](http://www.kframework.org/) - Rewriting semantics
- [BNFC](https://bnfc.digitalgrammars.com/) - Grammar-driven development
- [egg](https://egraphs-good.github.io/) - E-graph rewriting