# MeTTaIL Rust Implementation - Project Understanding

## Executive Summary

This document summarizes the current state of the f1r3node/f1r3fly ecosystem and the proposed migration of MeTTaIL (Meta Type Talk Intermediate Language) from Scala to Rust using procedural macros.

---

## Current Architecture

### 1. MeTTaIL (Scala + BNFC)
**Location:** `/MeTTaIL/`

**What it is:**
- A meta-language framework for defining programming languages as composable theories
- Currently implemented in Scala with BNFC (BNF Converter) for grammar generation
- Defines Rholang through theory composition

**Key Components:**
- **Module System:** Import/export theories and compose them
- **Theory Algebra:** Conjunction (`/\`), Disjunction (`\/`), Subtraction (`\`)
- **Language Features:**
  - Exports (categories/types)
  - Terms (grammar rules)
  - Equations (equational axioms)
  - Rewrites (rewrite rules with contexts)
  - Replacements (rename/reinterpret rules)

**Example (Rholang.module):**
```scala
Theory ParMonoid(cm: u.CommutativeMonoid) {
  cm
    Exports {
      Elem => Proc;
    }
    Replacements {
      [] Zero.Proc => PZero.Proc ::= "0";
      [0, 1] Plus.Proc => PPar.Proc ::= "(" Proc "|" Proc ")";
    }
    Rewrites {
      RPar1 : let Src ~> Tgt in
              ( PPar Src Q ) ~> ( PPar Tgt Q ) ;
    }
}
```

**Current Implementation:**
- **Parser:** BNFC generates parser from `.cf` grammar file
- **Interpreter:** Scala (`InstInterpreter.scala`) interprets theory compositions
- **Validation:** Runtime coherence checking (category consistency, variable scope, etc.)
- **Output:** Generates BNFC grammar for the composed language

### 2. Rholang (Rust Implementation)
**Location:** `/rholang/`

**What it is:**
- Concurrent programming language based on ρ-calculus (reflective π-calculus)
- Message-passing with channel-based communication
- Quote/drop for reflection (processes as names, names as processes)

**Current State:**
- **Parser:** Uses external `rholang-parser` crate (from F1R3FLY-io/rholang-rs)
- **Compiler:** Normalizes Rholang to internal representation
- **Interpreter:** Reduces processes using pattern matching and spatial matching
- **Storage:** Integrates with rspace++ for tuple space operations

**Key Features Implemented:**
- ✅ Parallel composition (`|`)
- ✅ Send/receive (`x!(P)`, `for(y <- x) { P }`)
- ✅ Quote/drop (`@P`, `*x`)
- ✅ New channels (`new x in P`)
- ✅ Pattern matching
- ✅ Persistent sends/receives (`x!!(P)`, `for(y <= x) { P }`)
- ⚠️ Guarded patterns (not working)
- ⚠️ 0-arity send/receive (broken)

### 3. RSpace++ (Rust Tuple Space)
**Location:** `/rspace++/`

**What it is:**
- Direct port of Scala rspace to Rust
- Tuple space implementation for concurrent process coordination
- Backs Rholang's channel operations

**Integration:**
- Used by Rholang interpreter for message storage and retrieval
- Handles pattern matching on received messages
- Provides persistence and indexing

### 4. F1R3Node Infrastructure
**Other Components:**
- `/casper/` - Consensus protocol
- `/comm/` - Network communication
- `/crypto/` - Cryptographic primitives
- `/models/` - Data models and protobuf definitions
- `/node/` - Node orchestration

---

## The Vision: MeTTaIL in Rust

### Core Idea
Transform MeTTaIL from a runtime-interpreted meta-language into a **compile-time Rust macro system** that:
1. Defines languages as Rust procedural macros
2. Validates theory composition at compile time
3. Generates type-safe parsers and interpreters
4. Enables network transmission of language definitions (for f1r3fly)

### Example of Proposed Syntax
```rust
use mettail::theory;

theory! {
    name: ParMonoid,
    params: (cm: CommutativeMonoid),

    exports {
        Elem => Proc;
    }

    replacements {
        Zero.Proc => PZero.Proc ::= "0";
        Plus.Proc => PPar.Proc ::= "(" Proc "|" Proc ")";
    }

    rewrites {
        RPar1: let Src ~> Tgt in
               (PPar Src Q) ~> (PPar Tgt Q);
    }
}
```

### Key Advantages Over Current Scala Implementation

| Feature | Current (Scala+BNFC) | Proposed (Rust Macros) |
|---------|---------------------|------------------------|
| **Validation** | Runtime | Compile-time |
| **Type Safety** | Limited | Full Rust type system |
| **Performance** | Interpreter overhead | Zero-cost abstractions |
| **Integration** | FFI boundary | Native Rust |
| **IDE Support** | Basic | rust-analyzer aware |
| **Network Transmission** | Manual | Auto-serializable |
| **Memory Safety** | GC overhead | Rust guarantees |

---

## Critical Requirements for f1r3fly

### 1. Language-as-Data Transmission
**Need:** Send complete language definitions over the network and execute them remotely

**Approaches:**
- **Serialized AST + Interpreter** - Send theory descriptor, reconstruct on remote side
- **JIT Compilation (Cranelift)** - Compile theories to native code on-the-fly
- **WASM Sandboxing** - Compile to WebAssembly for secure execution

### 2. Rewrite Rule Execution
**Need:** Efficiently apply contextual rewrite rules (e.g., communication reduction)

**Approaches:**
- **Compiled Pattern Matching** - Generate Rust match statements from patterns
- **RETE Algorithm** - Build discrimination network for multi-pattern matching
- **E-graphs (egg crate)** - Equality saturation for optimal rewriting

### 3. Freshness Conditions
**Need:** Support "if x # Q" (x is fresh in Q) in equations

**Approaches:**
- **Free Variable Caching** - Precompute and cache free variables
- **Scope Graphs** - Build precise binding analysis structure

### 4. Theory Composition
**Need:** Compose theories via conjunction, disjunction, subtraction

**Approaches:**
- **Type-level Computation** - Theories as Rust types with trait bounds
- **Procedural Macro Expansion** - Expand compositions during macro processing

---

## Current Gaps & Challenges

### Gap 1: Parser Generation
**Problem:** BNFC generates Haskell/Java parsers; need Rust equivalent

**Options:**
1. **LALRPOP** - LR parser generator, similar to YACC
2. **Pest** - PEG parser, easier but less powerful
3. **Tree-sitter** - Incremental parser, great for editors
4. **Nom** - Parser combinators, hand-written

**Recommendation:** LALRPOP for production, Tree-sitter for IDE support

### Gap 2: Macro Complexity
**Problem:** Procedural macros are sophisticated to implement

**Mitigation:**
- Start with simple prototype
- Incremental feature addition
- Comprehensive testing
- Clear error messages

### Gap 3: Dynamic Loading
**Problem:** Network-received theories need dynamic dispatch

**Solution:** Hybrid approach
- Compile-time macros for known theories
- Runtime interpreter for received theories
- Optional JIT compilation for hot paths

### Gap 4: Backward Compatibility
**Problem:** Existing MeTTaIL modules need migration path

**Solution:**
- Parser for `.module` files
- Translator to Rust macro syntax
- Dual support during transition

---

## Integration Points with f1r3node

### With Rholang
```
MeTTaIL Rust Macros → Generate Rholang Parser/Interpreter
                    ↓
            rholang/ (current implementation)
                    ↓
            rspace++/ (tuple space)
```

**Benefits:**
- Remove external `rholang-parser` dependency
- Generate optimized interpreter from theory
- Unify language definition and implementation

### With Network Layer
```
theory! { ... } → Serialize TheoryDescriptor
                ↓
        f1r3fly Network
                ↓
    Receive & Validate → Instantiate Theory
                       ↓
                Execute Remotely
```

**Benefits:**
- Send complete languages as data
- Execute programs in transmitted languages
- Network-programmable protocol semantics

---

## Dependencies & Ecosystem

### Existing Rust Crates in f1r3node
```toml
# Already in use
rholang-parser = { git = "https://github.com/F1R3FLY-io/rholang-rs", ... }
tree-sitter = "0.25.8"
serde = "1.0"
bincode = "1.3.3"

# Shared across workspace
crypto/
models/
shared/
```

### New Dependencies Needed
```toml
# For procedural macros
proc-macro2 = "1.0"
quote = "1.0"
syn = "2.0"

# For parser generation
lalrpop = "0.20"
lalrpop-util = "0.20"

# For rewrite optimization (optional)
egg = "0.9"  # e-graphs

# For JIT compilation (optional)
cranelift = "0.100"
wasmtime = "14.0"
```

---

## Success Criteria

A successful Rust implementation of MeTTaIL should:

1. ✅ **Compile-time validation** - Invalid theories fail to compile
2. ✅ **Type safety** - Categories are Rust types
3. ✅ **Performance** - Zero overhead vs hand-written code
4. ✅ **Network-ready** - Theories serializable/transmissible
5. ✅ **IDE integration** - rust-analyzer support
6. ✅ **Backward compatible** - Can parse existing `.module` files
7. ✅ **Extensible** - Easy to add new theory operations
8. ✅ **Well-documented** - Clear examples and tutorials

---

## Next Steps

See `01-MINIMAL-POC.md` for the proposed proof-of-concept plan.

