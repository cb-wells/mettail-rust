# MeTTaIL: Metalanguage for language implementation

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
    }
    
    rewrites {
        // Communication: for(chan->x){P} , chan!(Q) => P[@Q/x]
        (PPar {(PInput chan x P), (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
    }
}
```

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

## 💡 Key Insights from Development

### What We've Learned

1. **Collections over Binary Ops**: HashBag + indexed projection is 100x+ faster than binary Par with AC equations
2. **Structural Properties**: Auto-flatten demonstrates compile-time code generation eliminates entire class of user burden
3. **Ascent Power**: Datalog is excellent for rewriting - declarative, compositional, and performant
4. **Type Safety Matters**: Category tracking prevents entire classes of bugs
5. **Simplicity Wins**: Straightforward generated code is easier to debug than clever optimizations

### Design Principles Emerging

1. **Default to Structural**: Implement properties structurally when possible, equations when necessary
2. **Generate Simple Code**: Let Rust compiler optimize, focus on correctness
3. **Pay for What You Use**: Fast paths for common cases (flat collections), slow paths for rare cases (deep nesting)
4. **Progressive Enhancement**: Start with working system, add optimizations where needed
5. **User-Centered**: Minimize theory author burden, maximize generated code quality

---
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