# MeTTaIL Project Roadmap

**Vision:** A meta-language framework for defining, composing, and executing formal languages with rigorous semantics, starting with process calculi like Rholang.

---

## 🎯 Core Philosophy

MeTTaIL aims to be:
1. **Compositional** - Theories built from other theories as building blocks
2. **Rigorous** - Type-safe with formal semantics
3. **Practical** - Executable with multiple backends (interpreter, JIT, WASM)
4. **Extensible** - Easy to add new languages and features

---

## 📍 Current Status (End of Phase 1)

### What Works ✅
- **Theory Definition** - Declarative `theory! {}` macro syntax
- **Type-Checking** - Sound category inference and validation
- **Binder Handling** - Correct variable scoping via `moniker`
- **Cross-Category Substitution** - Full support for heterogeneous substitution
- **Rewrite Rules** - Parsing and type-checking (not execution yet)
- **Test Case** - Rho Calculus with communication rule

### What's Missing ❌
- **Parser Generation** - Current approach broken, needs LALRPOP
- **Runtime Execution** - Can't actually reduce/rewrite terms yet
- **Theory Composition** - Design exists but not implemented
- **Equation Semantics** - Unclear how equations interact with rewrites
- **Congruence Rules** - No automatic generation (e.g., `s => t` ⊢ `P|s => P|t`)

---

## 🗺️ Roadmap Overview

```
Phase 1: Foundation ✅ COMPLETE
├─ AST generation
├─ Type-checking
├─ Binders & substitution
└─ Rewrite rule syntax

Phase 2: Execution 🎯 NEXT (3-4 months)
├─ Parser generation (LALRPOP)
├─ Basic reduction engine  
├─ Pattern matching
├─ Rewrite application
└─ Simple interpreter

Phase 3: Composition (2-3 months)
├─ Theory parameterization
├─ Module system
├─ Export/import
└─ Namespace management

Phase 4: Advanced Semantics (3-4 months)
├─ E-graph integration (egg/egglog)
├─ Equation handling
├─ Congruence rule generation
├─ Confluence checking
└─ Termination analysis

Phase 5: Production Features (3-4 months)
├─ JIT compilation (Cranelift)
├─ WASM backend
├─ Optimization passes
├─ Incremental computation
└─ Parallel reduction

Phase 6: Tooling & Ecosystem (2-3 months)
├─ Language server (LSP)
├─ REPL
├─ Debugger
├─ Pretty printer
└─ Documentation generator

Phase 7: Advanced Types (ongoing)
├─ Dependent types
├─ Refinement types
├─ Session types
├─ Linear types
└─ Effect systems
```

---

## 📋 Phase 2: Execution Engine (CURRENT)

**Goal:** Make theories executable - parse input, apply rewrites, produce output.

**Duration:** 3-4 months

### 2.1 Parser Generation (Weeks 1-2) 🔴 CRITICAL

**Problem:** Current parser combinator approach is fundamentally broken.

**Solution:** Integrate LALRPOP for proper LR(1) parsing.

**Tasks:**
- [ ] Add LALRPOP dependency and build integration
- [ ] Generate `.lalrpop` grammar files from `theory!` definitions
- [ ] Handle operator precedence and associativity
- [ ] Support binder syntax in grammar
- [ ] Generate parser modules with proper error handling
- [ ] Test with Rho Calculus examples

**Deliverable:** `RhoCalc::parse("0 | *@0")` returns correct AST.

---

### 2.2 Pattern Matching (Weeks 3-4)

**Goal:** Match term patterns against AST for rewrite rule application.

**Tasks:**
- [ ] Design pattern ADT (variables, constructors, wildcards)
- [ ] Implement pattern matching algorithm
- [ ] Handle binders in patterns (α-equivalence)
- [ ] Support conditional patterns (freshness)
- [ ] Generate match code for rewrite LHS
- [ ] Test with various Rho Calculus patterns

**Example:**
```rust
// Given pattern: (PPar (PInput x P) (POutput y Q))
// Match against: 0 | *@0 | for(ch z){*z} | ch!(0)
//result: Finds PInput/POutput pair, binds x=ch, z=z, y=ch, P=*z, Q=0
```

---

### 2.3 Rewrite Application (Weeks 5-6)

**Goal:** Apply rewrite rules to transform terms.

**Tasks:**
- [ ] Implement rewrite rule application
- [ ] Check freshness conditions at runtime
- [ ] Apply substitutions from RHS
- [ ] Handle multiple matches (strategy: first, all, choice?)
- [ ] Test communication rule: `for(ch x){P} | ch!(Q) => P[@Q/x]`
- [ ] Measure performance and optimize

**Deliverable:** `term.apply_rewrite(rule)` produces reduced term.

---

### 2.4 Reduction Engine (Weeks 7-8)

**Goal:** Repeatedly apply rewrites until normal form (or timeout).

**Design Decisions:**
1. **Strategy:** Which rewrite to apply when multiple match?
   - **Innermost** - Apply to subterms first
   - **Outermost** - Apply to whole term first
   - **Leftmost** - Textual order  
   - **Random** - Non-deterministic (for exploration)

2. **Termination:** How to prevent infinite loops?
   - **Step limit** - Max N rewrites
   - **Term size** - Stop if term grows too large
   - **Cycle detection** - Track seen terms

3. **Congruence:** How to handle context rules?
   - **Manual** - User writes: `s => t` and `P|s => P|t` separately
   - **Auto-generate** - System derives congruence rules
   - **E-graph** - Deferred to Phase 4

**Tasks:**
- [ ] Implement reduction strategies
- [ ] Add termination checking
- [ ] Support trace/debug output
- [ ] Benchmark various strategies
- [ ] Test with Rho Calculus reduction sequences

**Example:**
```rust
let term = parse("for(ch x){*x | *x} | ch!(0)");
let result = reduce(term, &rhocalc_rewrites, Strategy::Innermost);
// Result: *@0 | *@0
```

---

### 2.5 Simple Interpreter (Weeks 9-10)

**Goal:** End-to-end execution of Rho Calculus programs.

**Tasks:**
- [ ] Command-line interface
- [ ] Read theory definitions from files
- [ ] Parse input terms
- [ ] Reduce to normal form
- [ ] Pretty-print results
- [ ] Add verbose/debug modes
- [ ] Write example programs

**Deliverable:** `mettail run rhocalc.theory "for(ch x){*x} | ch!(0)"` outputs `*@0`.

---

### 2.6 Phase 2 Milestones

**Milestone 1 (Week 4):** Parser working for Rho Calculus  
**Milestone 2 (Week 6):** Pattern matching and single rewrite application working  
**Milestone 3 (Week 8):** Full reduction engine with strategies  
**Milestone 4 (Week 10):** Complete interpreter with CLI  

**Success Criteria:**
- ✅ Parse complex Rho Calculus terms
- ✅ Apply communication rule correctly
- ✅ Reduce multi-step programs to normal form
- ✅ Handle edge cases (shadowing, freshness, etc.)
- ✅ Performance: 1000+ rewrites/second

---

## 📋 Phase 3: Theory Composition (FUTURE)

**Goal:** Build complex theories from simpler ones.

### 3.1 Parameterization (Weeks 1-2)

**Syntax:**
```rust
theory! {
    name: List(T),  // T is a parameter
    exports { List }
    terms {
        Nil . List ::= "[]" ;
        Cons . List ::= T "::" List ;
    }
}

// Instantiation:
type IntList = List(Int);
type ProcList = List(Proc);
```

**Tasks:**
- [ ] Parse theory parameters
- [ ] Type-check instantiations
- [ ] Generate monomorphized code
- [ ] Support higher-order theories (theory params)

---

### 3.2 Import/Export (Weeks 3-4)

**Syntax:**
```rust
theory! {
    name: RhoCalc,
    imports { BasicTypes(Int, Bool) }
    exports { Proc, Name }
    // ...
}
```

**Tasks:**
- [ ] Module system design
- [ ] Namespace management
- [ ] Qualified imports
- [ ] Re-exports
- [ ] Circular dependency detection

---

### 3.3 Theory Libraries (Weeks 5-6)

**Goal:** Standard library of reusable theories.

**Examples:**
- `Monoid(T)` - Abstract monoid
- `Lattice(T)` - Lattice operations
- `Process` - Basic process calculus
- `Communication` - Message passing primitives
- `Reflection` - Quote/drop operators

---

## 📋 Phase 4: Advanced Semantics (FUTURE)

**Goal:** Rigorous handling of equations and equivalences.

### 4.1 E-graph Integration (Weeks 1-3)

**Technology:** `egg` or `egglog` for equality saturation.

**Purpose:** Represent all equivalent terms compactly.

**Example:**
```
Given: P|Q == Q|P  and  P|0 == P
E-graph contains: {0 | x, x | 0, x} in same equivalence class
```

**Tasks:**
- [ ] Integrate `egg` or `egglog`
- [ ] Convert AST to e-graph
- [ ] Run equality saturation
- [ ] Extract best term from e-class
- [ ] Benchmark performance

---

### 4.2 Equation Semantics (Weeks 4-5)

**Decision:** How do equations interact with rewrites?

**Option A:** Equations = Bidirectional Rewrites
- `P|Q == Q|P` generates: `P|Q => Q|P` and `Q|P => P|Q`
- Simple but may not terminate

**Option B:** Equations = E-graph Axioms
- Equations define equivalence classes
- Rewrites work modulo equivalence
- More powerful but more complex

**Option C:** Equations = Proof Obligations
- Type system proves equations
- Dependent types ensure correctness
- Most rigorous but hardest to implement

**Tasks:**
- [ ] Choose semantics (likely Option B)
- [ ] Implement chosen approach
- [ ] Test with complex equational theories
- [ ] Document semantics formally

---

### 4.3 Congruence Rules (Weeks 6-7)

**Problem:** Given `s => t`, we want `P|s => P|t` automatically.

**Solution:** Generate congruence rules for each constructor.

**Algorithm:**
```
For each rewrite rule R: s => t
For each constructor C with field of type T (where s, t : T)
  Generate: C(..., s, ...) => C(..., t, ...)
```

**Example:**
```rust
// User writes:
P => Q

// System generates:
P | R => Q | R
R | P => R | Q
for(ch x){P} => for(ch x){Q}  // if x not free in P, Q
*@P => *@Q
// etc.
```

**Tasks:**
- [ ] Identify which constructors apply
- [ ] Generate congruence rules
- [ ] Check side conditions (freshness)
- [ ] Integrate with rewrite engine
- [ ] Test with Rho Calculus

---

### 4.4 Confluence & Termination (Weeks 8-10)

**Goal:** Analyze rewrite systems for desirable properties.

**Confluence:** Do all reduction paths lead to the same normal form?

**Termination:** Does reduction always terminate?

**Tasks:**
- [ ] Implement confluence checking (critical pairs)
- [ ] Implement termination checking (orderings)
- [ ] Generate warnings for non-confluent systems
- [ ] Support user-provided termination proofs
- [ ] Create test suite of known systems

---

## 📋 Phase 5: Production Features (FUTURE)

### 5.1 JIT Compilation (Cranelift)

**Goal:** Fast execution via native code generation.

**Tasks:**
- [ ] Design IR for terms
- [ ] Lower rewrites to Cranelift IR
- [ ] Generate efficient match code
- [ ] Implement GC integration
- [ ] Benchmark vs. interpreter (target: 100x speedup)

---

### 5.2 WASM Backend

**Goal:** Run MeTTaIL theories in browsers and WASM environments.

**Tasks:**
- [ ] Compile to WASM
- [ ] JS bindings
- [ ] Browser demo
- [ ] NPM package

---

### 5.3 Optimization

**Strategies:**
- **Memoization** - Cache rewrite results
- **Partial evaluation** - Specialize rules
- **Deforestation** - Eliminate intermediate terms
- **Parallel reduction** - Multi-core execution

---

## 📋 Phase 6: Tooling (FUTURE)

### 6.1 Language Server (LSP)

**Features:**
- Syntax highlighting
- Go-to-definition
- Type on hover
- Error diagnostics
- Auto-completion
- Refactoring

---

### 6.2 REPL

**Features:**
- Interactive theory exploration
- Step-by-step reduction
- Term inspection
- Theory hot-reloading

---

### 6.3 Debugger

**Features:**
- Breakpoints on rewrites
- Term inspection
- Reduction trace
- Time-travel debugging

---

## 📋 Phase 7: Advanced Types (ONGOING)

### 7.1 Dependent Types

**Goal:** Types can depend on values.

**Example:**
```rust
Vec(T, n: Nat) // Vector of length n
```

---

### 7.2 Session Types

**Goal:** Protocol specification for communication.

**Example:**
```rust
Chan(?Int.!Bool.End) // Receive Int, send Bool, close
```

---

### 7.3 Linear Types

**Goal:** Resource management (channels used exactly once).

---

## 🎯 Success Metrics

### Phase 2 Success Criteria
- ✅ Parse and reduce 100+ Rho Calculus examples
- ✅ Performance: 1000+ rewrites/second
- ✅ All Rho Calculus properties (comm, assoc, identity) verified
- ✅ Zero runtime errors on valid programs
- ✅ Clear error messages for invalid programs

### Phase 3 Success Criteria
- ✅ Define 10+ reusable theory libraries
- ✅ Build complex theories from simple ones
- ✅ No code duplication via composition

### Phase 4 Success Criteria
- ✅ Prove confluence for Rho Calculus subset
- ✅ Auto-generate 100+ congruence rules
- ✅ E-graph speeds up matching by 10x+

### Long-term Vision
- 📚 Standard library of 50+ theories
- 🏢 Production use in real systems
- 📖 Published papers on MeTTaIL semantics
- 👥 Active community of theory developers
- 🔬 Integration with proof assistants (Coq, Lean, Agda)

---

## 🤔 Open Research Questions

1. **What's the right level of abstraction?**
   - Too abstract: Hard to execute efficiently
   - Too concrete: Not reusable enough

2. **How to handle non-determinism?**
   - Process calculi are inherently non-deterministic
   - Multiple reduction paths
   - How to explore the space?

3. **How to integrate with existing languages?**
   - FFI to/from Rust?
   - Embed MeTTaIL in other languages?
   - Target existing VMs (JVM, CLR)?

4. **What's the killer app?**
   - Process calculi (Rholang)?
   - Proof assistants?
   - Domain-specific languages?
   - Formal verification?

5. **How to make it practical?**
   - Good error messages
   - Fast compilation
   - Reasonable performance
   - Easy integration

---

## 📚 Related Work & Inspiration

- **BNFC** - Grammar-driven language definition
- **K Framework** - Rewriting-based semantics
- **Spoofax** - Language workbench
- **Rascal** - Meta-programming language
- **PLT Redex** - Semantic modeling in Racket
- **egg/egglog** - E-graph libraries
- **Cranelift** - JIT compilation
- **Moniker** - Variable binding library
- **LALRPOP** - Rust parser generator

---

## 🚀 Next Immediate Steps

1. **Document Phase 1 completion** ✅ (this document)
2. **Update all existing docs** ✅ (in progress)
3. **Choose Phase 2 starting point:** LALRPOP integration
4. **Set up LALRPOP build** (Week 1)
5. **Generate first working parser** (Week 2)
6. **Celebrate small wins!** 🎉

---

**Last Updated:** After Phase 1 completion and substitution fix

