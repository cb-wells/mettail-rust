# MeTTaIL: Meta-Language for Theory-Based Language Implementation

**Status:** Phase 1 Complete ✅ | Phase 2 Starting 🎯

---

## 📖 Quick Links

- **[Roadmap](docs/ROADMAP.md)** - Long-term vision and phases
- **[Phase 1 Plan](docs/phase-1/PHASE-1-PLAN.md)** - Foundation implementation (COMPLETE)
- **[Progress](docs/phase-1/PROGRESS.md)** - Detailed progress and metrics
- **[Remaining Issues](docs/design/REMAINING-ISSUES.md)** - Known problems and priorities
- **[Theory Composition Design](docs/design/THEORY-COMPOSITION-DESIGN.md)** - Phase 3 design

---

## 🎯 What is MeTTaIL?

MeTTaIL is a **meta-language framework** for defining formal languages through:
1. **Grammars** - BNF-like syntax with binders
2. **Equations** - Structural equivalences
3. **Rewrites** - Computational rules with substitution

### Example: Rho Calculus in MeTTaIL

```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= Proc "|" Proc ;
        PDrop . Proc ::= "*" Name ;
        NQuote . Name ::= "@" Proc ;
        NVar . Name ::= Var ;
    }
    
    equations {
        (PPar P Q) == (PPar Q P) ;              // Commutativity
        (PPar P (PPar Q R)) == (PPar (PPar P Q) R) ;  // Associativity
        (PPar P PZero) == P ;                   // Identity
        (PDrop (NQuote P)) == P ;               // Reflection
    }
    
    rewrites {
        // Communication: for(chan x){P} | chan!(Q) => P[@Q/x]
        if x # Q then (PPar (PInput chan x P) (POutput chan Q))
            => (subst P x (NQuote Q))
    }
}
```

**Generated:** Type-safe AST, parser, substitution, and more!

---

## ✅ Phase 1: What Works Now

### Core Features
- ✅ **Theory Definition** - Declarative syntax with macros
- ✅ **Type-Checking** - Sound category inference
- ✅ **Binders & Variables** - Correct scoping via `moniker`
- ✅ **Cross-Category Substitution** - Full support for heterogeneous substitution
- ✅ **Rewrite Rule Syntax** - Parsing and validation
- ✅ **Test Case** - Rho Calculus with communication

### Code Generation
From a theory definition, MeTTaIL generates:
- **AST enums** - Clean, type-safe data structures
- **Substitution methods** - Capture-avoiding, cross-category
- **Type derivations** - `Debug`, `Clone`, `PartialEq`, `Eq`, `BoundTerm`
- **Parser stubs** - Currently broken, Phase 2 will fix with LALRPOP

### Example Generated Code
```rust
pub enum Proc {
    PZero,
    PInput(Box<Name>, Scope<Binder<String>, Box<Proc>>),
    POutput(Box<Name>, Box<Proc>),
    PPar(Box<Proc>, Box<Proc>),
    PDrop(Box<Name>),
}

impl Proc {
    pub fn substitute_name(
        &self, 
        var: &FreeVar<String>, 
        replacement: &Name
    ) -> Self {
        // Automatic capture-avoiding substitution
        // Recurses into all fields correctly ✅
    }
}
```

---

## 🎯 Phase 2: What's Next (3-4 months)

### Immediate Priorities
1. **Parser Generation** (Weeks 1-2) - LALRPOP integration
2. **Pattern Matching** (Weeks 3-4) - Match rewrite LHS against terms
3. **Rewrite Application** (Weeks 5-6) - Apply rewrites to transform terms
4. **Reduction Engine** (Weeks 7-8) - Multi-step reduction strategies
5. **Simple Interpreter** (Weeks 9-10) - End-to-end CLI tool

### Deliverable
```bash
$ mettail run rhocalc.theory "for(ch x){*x} | ch!(0)"
*@0

$ mettail reduce rhocalc.theory "0 | for(ch x){*x | *x} | ch!(0)" --trace
Step 1: 0 | for(ch x){*x | *x} | ch!(0)
Step 2: 0 | (*@0 | *@0)
Step 3: *@0 | *@0
```

---

## 🌟 Key Innovations

### 1. Cross-Category Substitution
**Problem:** Process calculi bind variables of one type (Name) in terms of another type (Proc).

**Solution:** Generate multiple substitution methods:
- `Proc.substitute(var, Proc)` - Same-category
- `Proc.substitute_name(var, Name)` - Cross-category
- Always recurse into all fields

### 2. Theory Composition (Phase 3)
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

### 3. E-graph Integration (Phase 4)
**Vision:** Use equality saturation for equation handling.

```
Given: P|Q == Q|P, P|0 == P
E-graph: {0|x, x|0, x} in same equivalence class
Match rewrites modulo equivalence
```

---

## 📊 Current Stats

- **~3500 LOC** - Core implementation
- **15+ tests** - All passing ✅
- **7 examples** - Including Rho Calculus
- **0.7s compile** - For Rho Calculus theory
- **~700 LOC** - Generated for Rho Calculus

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

### Type Safety
```rust
// Compile-time error if types don't match:
theory! {
    terms {
        Wrong . Proc ::= Name "|" Name ;  // ❌ PPar expects Proc, not Name
    }
}
```

---

## 🎓 Research Questions

### Semantics
1. **Equations vs. Rewrites** - How do they interact?
2. **Congruence** - Auto-generate `s => t` ⊢ `P|s => P|t`?
3. **Confluence** - Check or assume?
4. **Termination** - Prove or bound?

### Performance
1. **E-graphs** - Always beneficial or situational?
2. **JIT compilation** - Worth the complexity?
3. **Parallel reduction** - Challenges in non-determinism?

### Usability
1. **Error messages** - How to make them great?
2. **IDE support** - What features matter most?
3. **Debuggability** - How to step through rewrites?

---

## 🚀 Getting Started

### Prerequisites
```bash
# Rust 1.70+
rustup update

# Clone the repo
cd f1r3node/mettail-rust-exploration
```

### Run Tests
```bash
cargo test --bin rhocalc
```

### See Generated Code
```bash
cargo expand -p mettail-examples --bin rhocalc > output.rs
```

### Run Example
```bash
cargo run --bin rhocalc
```

---

## 📚 Documentation Index

### Getting Started
- `README.md` (this file)
- `docs/ROADMAP.md` - Vision and phases

### Phase 1 (Complete)
- `docs/phase-1/PHASE-1-PLAN.md` - Implementation plan
- `docs/phase-1/PHASE-1-COMPLETE.md` - Achievement summary
- `docs/phase-1/PROGRESS.md` - Detailed progress

### Reference
- `docs/design/REMAINING-ISSUES.md` - Known problems
- `docs/design/VARIABLE-TYPING-ANALYSIS.md` - Design decisions
- `docs/design/THEORY-COMPOSITION-DESIGN.md` - Phase 3 design

### Historical
- `docs/phase-1/FOUNDATION-REVIEW.md` - Mid-Phase 1 review
- `docs/phase-1/substitution/SUBSTITUTION-IN-REWRITES-COMPLETE.md` - Feature completion

---

## 🙏 Credits

**Core Technologies:**
- [syn](https://github.com/dtolnay/syn) - Rust parsing
- [quote](https://github.com/dtolnay/quote) - Code generation
- [moniker](https://github.com/brendanzab/moniker) - Variable binding
- [LALRPOP](https://github.com/lalrpop/lalrpop) - Parser generator (Phase 2)

**Inspiration:**
- [Rholang](https://rchain.coop/) - Motivating use case
- [K Framework](http://www.kframework.org/) - Rewriting semantics
- [BNFC](https://bnfc.digitalgrammars.com/) - Grammar-driven development
- [egg](https://egraphs-good.github.io/) - E-graph rewriting

---

## 📬 Contact & Collaboration

This is an active research project. We're working out the goals together as we go!

**Current Focus:** Phase 2 - Making theories executable.

---

**Last Updated:** After Phase 1 completion and substitution fix  
**Next Milestone:** LALRPOP parser integration (Week 2)
