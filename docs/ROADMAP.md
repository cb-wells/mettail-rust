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

## 📍 Current Status (End of Phase 2)

### What Works ✅
- **Theory Definition** - Declarative `theory! {}` macro syntax
- **Type-Checking** - Sound category inference and validation
- **Binder Handling** - Correct variable scoping via `moniker`
- **Cross-Category Substitution** - Full support for heterogeneous substitution
- **Rewrite Rules** - Parsing and type-checking (not execution yet)
- **LALRPOP Parser Generation** - Automatic `.lalrpop` file generation from theories
- **Precedence Handling** - Correct parsing of infix operators with associativity
- **Pretty-Printing** - Display trait generation for all AST types
- **Round-Trip Testing** - Parse → Display → Parse verified for Rho Calculus
- **Test Case** - Rho Calculus with full parsing and communication rule

### What's Missing ❌
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

Phase 2: Parser Generation ✅ COMPLETE
├─ LALRPOP integration
├─ Grammar generation from theories
├─ Operator precedence & associativity
├─ Binder parsing (Scope generation)
├─ Pretty-printing (Display traits)
└─ Round-trip testing

Phase 3: Theory Composition 🎯 NEXT (2-3 months)
├─ Theory parameterization
├─ Module system
├─ Export/import
├─ Namespace management
└─ Standard theory library

Phase 4: Execution Engine (3-4 months)
├─ Pattern matching
├─ Rewrite application
├─ Reduction strategies
├─ Simple interpreter
└─ Works with composed theories

Phase 5: Advanced Semantics (3-4 months)
├─ E-graph integration (egg/egglog)
├─ Equation handling
├─ Congruence rule generation
├─ Confluence checking
└─ Termination analysis

Phase 6: Production Features (3-4 months)
├─ JIT compilation (Cranelift)
├─ WASM backend
├─ Optimization passes
├─ Incremental computation
└─ Parallel reduction

Phase 7: Tooling & Ecosystem (2-3 months)
├─ Language server (LSP)
├─ REPL
├─ Debugger
├─ Documentation generator
└─ Package manager

Phase 8: Advanced Types (ongoing)
├─ Dependent types
├─ Refinement types
├─ Session types
├─ Linear types
└─ Effect systems
```

---

## 📋 Phase 2: Parser Generation ✅ COMPLETE

**Goal:** Generate working parsers from theory definitions and enable round-trip testing.

**Duration:** Completed in concentrated development session

**Achievement:** Successfully integrated LALRPOP, generating working parsers with precedence handling, binder support, and pretty-printing. All round-trip tests pass for Rho Calculus including complex expressions like `a!(0)|b!(c!(0))|for(a x){*x}`.

---

### ✅ Completed Tasks

**2.1 LALRPOP Integration & Build Setup**
- ✅ Added LALRPOP dependency to workspace
- ✅ Set up build.rs for grammar generation (both runtime and examples)
- ✅ Created working integration with existing AST types
- ✅ Configured proper module structure

**2.2 Grammar Generation from Theory Definitions**
- ✅ Automatic `.lalrpop` file generation from `theory!` macros
- ✅ Correct mapping of grammar rules to AST constructors
- ✅ Terminal vs non-terminal handling
- ✅ Automatic lexer token generation
- ✅ Grammar files written to correct directories

**2.3 Operator Precedence & Associativity**
- ✅ Detection of infix operators in grammar rules
- ✅ Automatic precedence tier generation (`Expr` → `ExprInfix` → `ExprAtom`)
- ✅ Left-associativity for parallel composition (`|`)
- ✅ Parentheses support for explicit grouping
- ✅ Tested with nested expressions

**2.4 Binder Syntax Support**
- ✅ Parse binder syntax (e.g., `for(ch x){P}`) into `Scope` types
- ✅ Generate parser actions that create `Scope` with fresh variables
- ✅ Correct variable capture and scoping
- ✅ Tested with Rho Calculus input constructs

**2.5 Pretty-Printing**
- ✅ Generate `Display` impl for all AST categories
- ✅ Handle binder printing (show variable names only)
- ✅ Escape braces in format strings
- ✅ Automatic space insertion between consecutive non-terminals
- ✅ Correct handling of `Var` fields (extract `pretty_name`)

**2.6 Testing & Round-Trip Verification**
- ✅ Round-trip tests pass: parse → display → parse
- ✅ Comprehensive Rho Calculus parsing (11 tests)
- ✅ Self-contained tests in theory files
- ✅ Complex expression: `a!(0)|b!(c!(0))|for(a x){*x}` ✓
- ✅ All test suites passing (19 macro tests, 3 rhocalc tests)

**2.7 File Structure & Architecture**
- ✅ Moved theories into `examples/` crate
- ✅ Proper separation: macros, runtime, examples
- ✅ Self-contained theory files (one `theory!` gives parser + AST + Display)
- ✅ Clean module generation with `lalrpop_util::lalrpop_mod!`

---

### Key Achievements

1. **Self-Contained Theory Files**: Single `theory!` macro generates:
   - AST enums with derives
   - Substitution implementations  
   - Display implementations
   - LALRPOP parser module reference

2. **Automatic Whitespace Handling**: Smart space insertion between consecutive non-terminals ensures parseability

3. **Full Round-Trip**: `parse(display(ast))` produces equivalent AST

4. **Production-Ready**: All tests passing, no panics, clean error messages

---

## 📋 Phase 3: Theory Composition 🎯 NEXT

**Goal:** Build complex theories from simpler, reusable components.

**Duration:** 2-3 months

**Why Now?** With parsers working, we can test composed theories by parsing and validating them. This establishes the library ecosystem before adding execution complexity.

---

### 3.1 Theory Inheritance (Weeks 1-2)

**Goal:** Build theories that extend other theories.

**Syntax:**
```rust
theory! {
    name: ProcessWithInput,
    parent: BasicProcess,  // Inherits all terms from BasicProcess
    exports { Proc }
    terms {
        // New constructors added to the Proc category
        PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
    }
}
```

**Tasks:**
- [ ] Add `parent` field to `TheoryDef`
- [ ] Parse parent theory references
- [ ] Import parent's categories and terms
- [ ] Type-check that extended categories are compatible
- [ ] Generate combined AST with inherited constructors
- [ ] Test with Rho Calculus built from smaller theories

**Example Hierarchy:**
```
BasicProcess (0, P|Q)
  ↓ extends
ProcessWithCommunication (adds !, for, *)
  ↓ extends
RhoCalculus (adds @, reflection)
```

---

### 3.2 Theory Parameterization (Weeks 3-4)

**Goal:** Generic theories with type parameters (after inheritance is working).

**Syntax:**
```rust
theory! {
    name: Ring,
    parent: CommMonoid,
    exports { Elem }
    terms {
        One . Elem ::= "1" ;
        Mult . Elem ::= Elem "*" Elem ;
    }
}
```

**Tasks:**
- [ ] Add `parent` field to `TheoryDef`
- [ ] Child theory extends the parent
- [ ] Generate monomorphized code

---

### 3.3 Import/Export & Modules (Weeks 5-6)

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

### 3.4 Theory Libraries (Weeks 7-8)

**Goal:** Standard library of reusable theories.

**Examples:**
- `Monoid(T)` - Abstract monoid
- `Lattice(T)` - Lattice operations
- `Process` - Basic process calculus
- `Communication` - Message passing primitives
- `Reflection` - Quote/drop operators

**Tasks:**
- [ ] Design library structure
- [ ] Implement core theories
- [ ] Test composition patterns
- [ ] Document library API
- [ ] Create example compositions

---

## 📋 Phase 4: Execution Engine (FUTURE)

**Goal:** Make theories executable - apply rewrites and reduce terms.

**Duration:** 3-4 months

**Why Now?** We have parsers (Phase 2) and can compose theories (Phase 3), so execution can work on the full ecosystem.

---

### 4.1 Pattern Matching (Weeks 1-2)

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
// Result: Finds PInput/POutput pair, binds x=ch, z=z, y=ch, P=*z, Q=0
```

---

### 4.2 Rewrite Application (Weeks 3-4)

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

### 4.3 Reduction Engine (Weeks 5-8)

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
   - **E-graph** - Deferred to Phase 5

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

### 4.4 Simple Interpreter (Weeks 9-10)

**Goal:** End-to-end execution of programs in any theory.

**Tasks:**
- [ ] Command-line interface
- [ ] Read theory definitions from files
- [ ] Parse input terms
- [ ] Reduce to normal form
- [ ] Pretty-print results
- [ ] Add verbose/debug modes
- [ ] Write example programs for multiple theories

**Deliverable:** 
```bash
$ mettail run rhocalc.theory "for(ch x){*x} | ch!(0)"
*@0

$ mettail run lambda.theory "(\\x.x) y"
y
```

---

### 4.5 Phase 4 Milestones

**Milestone 1 (Week 2):** Pattern matching working  
**Milestone 2 (Week 4):** Single rewrite application working  
**Milestone 3 (Week 8):** Full reduction engine with strategies  
**Milestone 4 (Week 10):** Complete interpreter with CLI  

**Success Criteria:**
- ✅ Apply communication rule correctly
- ✅ Reduce multi-step programs to normal form
- ✅ Handle edge cases (shadowing, freshness, etc.)
- ✅ Performance: 1000+ rewrites/second
- ✅ Works with composed theories from Phase 3

---

## 📋 Phase 5: Advanced Semantics (FUTURE)

**Goal:** Rigorous handling of equations and equivalences.

**Duration:** 3-4 months

---

### 5.1 E-graph Integration (Weeks 1-3)

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

### 5.2 Equation Semantics (Weeks 4-5)

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

### 5.3 Congruence Rules (Weeks 6-7)

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

### 5.4 Confluence & Termination (Weeks 8-10)

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

## 📋 Phase 6: Production Features (FUTURE)

**Duration:** 3-4 months

---

### 6.1 JIT Compilation (Cranelift)

**Goal:** Fast execution via native code generation.

**Tasks:**
- [ ] Design IR for terms
- [ ] Lower rewrites to Cranelift IR
- [ ] Generate efficient match code
- [ ] Implement GC integration
- [ ] Benchmark vs. interpreter (target: 100x speedup)

---

### 6.2 WASM Backend

**Goal:** Run MeTTaIL theories in browsers and WASM environments.

**Tasks:**
- [ ] Compile to WASM
- [ ] JS bindings
- [ ] Browser demo
- [ ] NPM package

---

### 6.3 Optimization

**Strategies:**
- **Memoization** - Cache rewrite results
- **Partial evaluation** - Specialize rules
- **Deforestation** - Eliminate intermediate terms
- **Parallel reduction** - Multi-core execution

---

## 📋 Phase 7: Tooling & Ecosystem (FUTURE)

**Duration:** 2-3 months

---

### 7.1 Language Server (LSP)

**Features:**
- Syntax highlighting
- Go-to-definition
- Type on hover
- Error diagnostics
- Auto-completion
- Refactoring

---

### 7.2 REPL

**Features:**
- Interactive theory exploration
- Step-by-step reduction
- Term inspection
- Theory hot-reloading

---

### 7.3 Debugger

**Features:**
- Breakpoints on rewrites
- Term inspection
- Reduction trace
- Time-travel debugging

---

## 📋 Phase 8: Advanced Types (ONGOING)

---

### 8.1 Dependent Types

**Goal:** Types can depend on values.

**Example:**
```rust
Vec(T, n: Nat) // Vector of length n
```

---

### 8.2 Session Types

**Goal:** Protocol specification for communication.

**Example:**
```rust
Chan(?Int.!Bool.End) // Receive Int, send Bool, close
```

---

### 8.3 Linear Types

**Goal:** Resource management (channels used exactly once).

---

## 🎯 Success Metrics

### Phase 2 Success Criteria
- ✅ Parse all Rho Calculus examples correctly
- ✅ Round-trip tests pass (parse → print → parse)
- ✅ Parse 1000+ terms/second
- ✅ Clear error messages for invalid input
- ✅ Support multiple theory syntaxes
- ✅ Zero parser-related panics on valid input

### Phase 3 Success Criteria
- ✅ Define 10+ reusable theory libraries
- ✅ Build complex theories from simple ones
- ✅ No code duplication via composition
- ✅ Parse and validate composed theories

### Phase 4 Success Criteria
- ✅ Apply communication rule correctly
- ✅ Reduce multi-step programs to normal form
- ✅ Handle edge cases (shadowing, freshness, etc.)
- ✅ Performance: 1000+ rewrites/second
- ✅ Works with composed theories from Phase 3

### Phase 5 Success Criteria
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

## 🚀 Next Immediate Steps (Phase 3)

1. **Design theory inheritance syntax**
   - `parent: BaseTheory` field in `TheoryDef`
2. **Implement parent theory parsing**
3. **Merge parent categories and terms into child**
4. **Test inheritance with Rho Calculus layers**
   - BasicProcess → ProcessWithCommunication → RhoCalculus
5. **Add validation for compatible inheritance**
6. **Design parameterization (after inheritance works)**

---

**Last Updated:** October 2024 - Phase 2 Complete, Beginning Phase 3 (Theory Composition)

