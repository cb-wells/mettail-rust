# MeTTaIL: Metalanguage for language implementation

**Status:** Collection Types ✅ | Indexed Projection ✅ | Deep Projection (Next)

---

## 📖 Quick Links

- **[Current Status](docs/CURRENT-STATUS.md)** - What works now and recent progress 📊
- **[Poly-Lingual Roadmap](docs/POLY-LINGUAL-ROADMAP.md)** - 3-year strategic vision 🎯
- **[Quick Start Guide](QUICKSTART.md)** - Get started in 5 minutes
- **[Phase 6 Complete](docs/design/PHASE-6-COMPLETE.md)** - Indexed Projection ✅
- **[Collection Types Design](docs/design/COLLECTION-TYPES-DESIGN.md)** - Implementation details
- **[Deep Projection Roadmap](docs/design/DEEP-PROJECTION-ROADMAP.md)** - Next steps

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

### Core Features (Phases 1-6 Complete)
- ✅ **Theory Definition** - Declarative syntax with macros
- ✅ **Type-Checking** - Sound category inference
- ✅ **Binders & Variables** - Correct scoping via `moniker`
- ✅ **Cross-Category Substitution** - Full support for heterogeneous substitution
- ✅ **Collection Types** - HashBag for associative-commutative operations
- ✅ **Order-Independent Matching** - Indexed projection for optimal performance
- ✅ **Rest Patterns** - Extract and reconstruct collection remainders
- ✅ **Collection Equations** - Automatic normalization (e.g., `{P} == P`)

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
- **Small terms (depth ≤3)**: ~1 second
- **Medium terms (depth 4-5)**: ~10 seconds  
- **Complex terms (depth 6+)**: 60-80 seconds

**Bottleneck**: Congruence rules for associative-commutative operations (e.g., `PPar`) generate exponentially many equality facts. Solution in progress: collection-based representations (see [Collection Types Design](docs/design/COLLECTION-TYPES-DESIGN.md)).

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

## 🎯 Current Focus: Performance & Type System

### Immediate Priorities
1. **Collection-Based Operations** - Replace binary `PPar` with `HashBag<Proc>` for 100x+ speedup
   - See [Collection Types Design](docs/design/COLLECTION-TYPES-DESIGN.md)
   - Target: < 1 second for complex term rewriting (currently 60-80s)
2. **Type System Refinement** - Category inference and type checking
3. **Execution Engine Optimization** - Reduce Ascent relation overhead

### Phase 3 (Next): Theory Composition
1. **Theory Imports** - Import and reuse other theories
2. **Theory Parameters** - Generic theories (e.g., `List<T>`)
3. **Extension Syntax** - Extend existing theories with new rules
4. **Module System** - Proper namespacing and visibility

**Vision:** Build complex theories from simpler ones.

```rust
theory! {
    name: List(T),
    exports { List }
    terms {
        Nil . List ::= "[]" ;
        Cons . List ::= T "::" List ;
    }
}

type ProcList = List(Proc);  // Instantiation
```

### Phase 4 (Future): Advanced Optimization
- **E-graph Integration** - Equality saturation for equation handling
- **JIT Compilation** - Compile rewrite rules to native code
- **Parallel Reduction** - Exploit non-determinism for parallelism

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