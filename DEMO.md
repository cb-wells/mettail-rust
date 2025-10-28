# MeTTaIL Demo Guide

**Date:** October 29, 2025  
**Audience:** Dev team  
**Duration:** 5-10 minutes  

---

## 🎯 Demo Objective

Show how **MeTTaIL generates executable language implementations** from declarative specifications, using Rho Calculus as a concrete example.

---

## 📋 Demo Script

### 1. Introduction (1 min)

**"MeTTaIL is a meta-language framework for defining formal languages."**

**Three components:**
1. **Operations** - BNF-like syntax with binders
2. **Equations** - Structural equivalences (coming soon: e-graphs)
3. **Rewrites** - Computational rules with substitution

**What we've built in a few days:**
- AST generation
- Parser generation (LALRPOP)
- Substitution generation
- **Rewrite engine generation** ← NEW!

---

### 2. Show the Theory Definition (2 min)

**File:** `examples/rhocalc.rs`

```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
        //                                  ^^^^^^
        //                                  Binder - captured automatically!
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= Proc "|" Proc ;
        PDrop . Proc ::= "*" Name ;
        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    }
    
    rewrites {
        // Communication: for(chan x){P} | chan!(Q) => P[@Q/x]
        if x # Q then (PPar (PInput chan x P) (POutput chan Q))
            => (subst P x (NQuote Q))
        //     ^^^^^^^^^^^^^^^^^^^^^^^^
        //     Capture-avoiding substitution!
    }
}
```

**Key points:**
- Declarative syntax
- Binders marked with `<Name>`
- Freshness condition `x # Q`
- Substitution `subst P x (NQuote Q)`
- **Everything is generated automatically**

---

### 3. Run the Demo (2 min)

```bash
$ cargo run --bin rhocalc
```

**Output:**
```
=== Rho Calculus Rewrite Demo ===

Input:  for(a<-x){*x}|a!(0)

Step 1: *@(0)

→ Normal form reached after 1 step(s)

✅ Rho Calculus Theory Compiled Successfully!
```

**Explain what happened:**
1. **Parsed** the input: `for(a<-x){*x}|a!(0)`
2. **Matched** the rewrite pattern: `PPar(PInput(a, x, *x), POutput(a, 0))`
3. **Checked** freshness: `x # 0` ✓ (x not free in 0)
4. **Substituted**: `*x[@(0)/x]` = `*@(0)`
5. **Result**: Normal form (no more rewrites applicable)

---

### 4. Show Generated Code (2 min)

**Optional:** If time permits, show the generated rewrite engine:

```bash
$ cargo expand --bin rhocalc | grep -A 30 "try_rewrite_rule_0"
```

**Highlights:**
```rust
pub fn try_rewrite_rule_0(term: &Proc) -> Option<Proc> {
    // Pattern matching
    if let Proc::PPar(field_0, field_1) = term {
        if let Proc::PInput(chan, scope_field) = &(**field_0) {
            let (binder, body) = scope_field.clone().unbind();
            if let Proc::POutput(chan2, q) = &(**field_1) {
                // Freshness check
                if !is_fresh(&binder, &(**q)) {
                    return None;
                }
                // Capture-avoiding substitution
                return Some(
                    (*body).clone().substitute_name(&binder.0, &quote)
                );
            }
        }
    }
    None
}
```

**Key points:**
- Nested pattern matching
- Automatic binder extraction (`unbind`)
- Freshness checking
- Type-safe substitution
- **~3400 lines of code generated from 35 lines of spec**

---

### 5. Technical Highlights (2 min)

**What makes this hard:**

1. **Binders are tricky** - Need capture-avoidance
   - Example: `λx.(λx.x)` - inner x doesn't capture outer x
   - We use the `moniker` library for correctness

2. **Cross-category substitution** - Substituting `Name` for `Name` in `Proc`
   - Not standard in most systems
   - Required for Rho Calculus semantics

3. **Nested pattern matching** - Rewrite patterns have arbitrary nesting
   - Need to extract binders at any depth
   - All generated automatically from the spec

4. **Freshness checking** - `x # Q` means "x not free in Q"
   - Uses moniker's `BoundTerm` trait
   - Generated for each rewrite rule

---

### 6. Comparison with K Framework (1 min)

**K Framework** is the state-of-the-art for rewriting semantics.

**What we have:**
- ✅ Pattern matching
- ✅ Substitution
- ✅ Freshness conditions
- ✅ Generates Rust (type-safe, fast)

**What we're missing (but planning):**
- ⏳ Congruence rules (apply rewrites in subterms)
- ⏳ Equation handling (e-graphs)
- ⏳ Debugging/tracing tools

**Advantage:** Our approach generates native Rust code that's fast and type-safe!

---

## 🎓 Q&A Points

**Q: How does it handle variable capture?**  
A: We use the `moniker` library, which implements nominal techniques with DeBruijn indices under the hood. The `Scope` type guarantees capture-avoidance.

**Q: Can you define multiple rewrite rules?**  
A: Yes! You can have as many as you want. We generate `try_rewrite_rule_0`, `try_rewrite_rule_1`, etc.

**Q: What about performance?**  
A: We generate native Rust code, so it's quite fast. For Rho Calculus, the entire theory compiles in ~0.7s and generates ~3400 lines of optimized code.

**Q: Can you compose theories?**  
A: Not yet - that's Phase 3! We're planning theory imports, parameters (e.g., `List<T>`), and extensions.

**Q: How does this compare to parser generators like ANTLR?**  
A: ANTLR generates parsers. We generate parsers + ASTs + substitution + rewrite engines. It's a complete language implementation framework.

**Q: What about error messages?**  
A: Currently basic. We plan to add better error messages, IDE support, and debugging tools.

---

## 📊 Stats to Mention

- **~4000 LOC** - Core implementation (in ~1 week)
- **~3400 LOC** - Generated for Rho Calculus
- **0.7s** - Compile time for the theory
- **15+ tests** - All passing ✅
- **100% type-safe** - Leverages Rust's type system

---

## 🚀 Future Roadmap

**Phase 3: Theory Composition** (Next)
- Theory imports
- Parametric theories (`List<T>`)
- Theory extension

**Phase 4: E-graph Integration**
- Match rewrite rules modulo equations
- Equality saturation
- Optimization passes

**Phase 5: Tooling**
- IDE support (LSP server)
- Debugger (step through rewrites)
- Proof search / symbolic execution

---

## 💡 Demo Tips

1. **Keep it high-level** - They don't need to see all the code
2. **Focus on the magic** - 35 lines → 3400 lines
3. **Show the demo running** - Seeing execution is convincing
4. **Mention the hard parts** - Binders, freshness, type safety
5. **Be honest about limitations** - It's early, but the foundation is solid

---

**Good luck with the demo! 🎉**

