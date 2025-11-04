# MeTTaIL: Metalanguage for language implementation

**Status:** Phase 1 Complete ✅ | Phase 2 Complete ✅ | **Rewrite Engine Complete** ✅ | Phase 3 Next 🎯

---

## 📖 Quick Links

- **[Roadmap](docs/ROADMAP.md)** - Long-term vision and phases
- **[Phase 1 Complete](docs/phase-1/PHASE-1-PLAN.md)** - Foundation (AST, types, binders) ✅
- **[Phase 2 Complete](docs/phase-2/PHASE-2-COMPLETE.md)** - Parser generation (LALRPOP) ✅
- **[Phase 3 Design](docs/design/THEORY-COMPOSITION-DESIGN.md)** - Theory composition (NEXT)
- **[Progress](docs/phase-1/PROGRESS.md)** - Detailed progress and metrics

---

## 🎯 What is MeTTaIL?

MeTTaIL is a **meta-language framework** for defining formal languages through:
1. **Operations** - BNF-like syntax with binders
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
        NQuote . Name ::= "@" "(" Proc ")" ;
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

**Generated:** Type-safe AST, parser, substitution, **rewrite engine**, and more!

---

## ✅ What Works Now

### Core Features
- ✅ **Theory Definition** - Declarative syntax with macros
- ✅ **Type-Checking** - Sound category inference
- ✅ **Binders & Variables** - Correct scoping via `moniker`
- ✅ **Cross-Category Substitution** - Full support for heterogeneous substitution
- ✅ **Rewrite Rule Syntax** - Parsing and validation
- ✅ **Rewrite Engine** - Pattern matching, freshness checking, and execution
- ✅ **Test Case** - Rho Calculus with full communication reduction

### Code Generation
From a theory definition, MeTTaIL generates:
- **AST enums** - Clean, type-safe data structures
- **LALRPOP grammars** - Full parser generation with precedence handling
- **Substitution methods** - Capture-avoiding, cross-category
- **Rewrite engine** - Pattern matching with nested binders and freshness checks
- **Type derivations** - `Debug`, `Clone`, `PartialEq`, `Eq`, `BoundTerm`, `Display`

---

## ✅ Phase 2: Parser & Rewrite Engine (COMPLETE)

### What We Built
- ✅ **Precedence-Aware Grammars** - Automatic handling of infix operators
- ✅ **Binder Parsing** - Direct parsing into `Scope` structures with proper variable binding
- ✅ **Parentheses Support** - Override precedence with grouping
- ✅ **Left-Associativity** - Correct parsing of `a | b | c` as `((a | b) | c)`
- ✅ **Rewrite Pattern Matching** - Nested patterns with binder extraction
- ✅ **Freshness Checking** - Automatic generation of `x # Q` checks
- ✅ **Capture-Avoiding Substitution** - Full integration with generated rewrite engine

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
```rust
pub fn try_rewrite_rule_0(term: &Proc) -> Option<Proc> {
    if let Proc::PPar(field_0, field_1) = term {
        let field_0_inner = &(**field_0);
        if let Proc::PInput(field_0, scope_field) = field_0_inner {
            let (binder, body) = scope_field.clone().unbind();
            let field_1_inner = &(**field_1);
            if let Proc::POutput(field_1_inner_0, field_1_inner_1) = field_1_inner {
                // Freshness check: x # Q
                if !is_fresh(&binder.clone(), &(**field_1_inner_1).clone()) {
                    return None;
                }
                // Apply substitution: P[@Q/x]
                return Some(
                    (*body).clone().substitute_name(
                        &(binder.clone()).0,
                        &Name::NQuote(Box::new((**field_1_inner_1).clone()))
                    )
                );
            }
        }
    }
    None
}
```

---

## 🎯 Phase 3: What's Next (Theory Composition)

### Immediate Priorities
1. **Theory Imports** - Import and reuse other theories
2. **Theory Parameters** - Generic theories (e.g., `List<T>`)
3. **Extension Syntax** - Extend existing theories with new rules
4. **Module System** - Proper namespacing and visibility

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

- **~4000 LOC** - Core implementation
- **15+ tests** - All passing ✅
- **Working demos** - Rho Calculus with execution
- **0.7s compile** - For Rho Calculus theory
- **~3400 LOC** - Generated for Rho Calculus (AST + parser + substitution + rewrite engine)

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

### Run Rewrite Demo
```bash
cargo run --bin rhocalc

# Output:
# === Rho Calculus Rewrite Demo ===
# 
# Input:  for(a<-x){*x}|a!(0)
# 
# Step 1: *@(0)
# 
# → Normal form reached after 1 step(s)
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
- `docs/design/SORTING-DESIGN.md` - Term ordering and generation

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

**Current Focus:** Phase 3 - Theory composition and imports.

**Latest Achievement:** Generated rewrite engine with pattern matching, freshness checking, and capture-avoiding substitution! 🎉