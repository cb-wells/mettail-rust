# MeTTaIL: Main Goals

**Vision**: A production-ready meta-language framework for defining, executing, and translating formal languages with performance and precision.

**Status**: Foundation complete, building toward poly-lingual computation

---

## Core Objectives

### 1. Term Execution & Performance

**Goal**: Execute term rewrites at production speed for real-world applications.

**Current State**:
- ✅ Ascent-based Datalog execution engine
- ✅ Order-independent collection matching
- ✅ 42x speedup via lazy deconstruction
- ⏳ Single-threaded, interpreted execution

**Near-Term** (6 months):
- Native code generation via Cranelift
- 100x speedup target
- < 1 second compile time
- JIT for hot rewrites

**Long-Term** (2+ years):
- Distributed execution across nodes
- Incremental recomputation
- Parallel Ascent backend
- < 10ms latency for federated rewrites

---

### 2. Theory Features

**Goal**: Express any formal language naturally with rich abstractions.

#### Data Structures
**Current**: Collections (`HashBag<T>`) with pattern matching, automatic flattening
**Needed**: Lists (ordered), Sets, Maps, Trees, custom user-defined structures

#### Composition & Modularity
**Current**: Single standalone theories
**Needed**:
- Module system with namespacing
- Theory inheritance/extension
- Mixins and traits
- Dependency management

#### Parameterization
**Current**: Concrete theories only
**Needed**:
- Generic theories: `List<T>`, `Tree<T, Ord>`
- Metavariables in definitions
- Type-level computation
- Monomorphization strategy

#### Predicate Type System
**Current**: Category-based type checking
**Needed**:
- Predicates on terms: `x:phi` where `phi` is a logical formula
- Conditional rewrites: `for(x:phi <- n){q} | n!(p) => q[p/x]` only when `p` satisfies `phi`
- Type inference for predicates
- Decidable checking strategy

---

### 3. Term Communication

**Goal**: Integrate MeTTaIL theories with f1r3fly distributed network for real-world execution.

**Background**: f1r3fly is a network running RhoLang (Rho Calculus) for decentralized computation.

**Integration Points**:
- Network protocol for term exchange
- Serialization/deserialization
- Remote rewriting capabilities
- Consensus on reduction order
- Fault tolerance

**Architecture**:
```
MeTTaIL Theory ─→ Compiled Code ─→ f1r3fly Node
                                   ├─ Local Execution
                                   ├─ Remote Communication
                                   └─ Network Coordination
```

**Milestones**:
1. Define wire protocol for terms
2. Implement serialization for runtime types
3. Build f1r3fly adapter/shim
4. Deploy test theory on network
5. Full integration with production f1r3fly

---

### 4. Theory Exploration

**Goal**: Make theories debuggable, testable, and understandable through automated exploration.

#### Term Generation
**Current**: Basic exhaustive and random generation (not yet working for collections)
**Needed**:
- Full collection support in generators
- Property-based testing (QuickCheck-style)
- Counterexample shrinking
- Coverage metrics

#### Interactive Exploration (REPL)
**Current**: Basic REPL with rewrite navigation
**Needed**:
- History and backtracking
- Equivalence class visualization
- Path finding between terms
- Statistics dashboard
- Query language for graph exploration

#### Fuzzing & Testing
**Needed**:
- Automatic test case generation
- Confluence checking
- Termination analysis
- Property verification
- Regression test suites

---

### 5. Theory Translation

**Goal**: Automatically compile between theories with proven correctness.

**Current**: None - each theory is isolated

**Vision**: Define formal mappings between theories to enable:
- Lambda Calculus ↔ Rho Calculus
- High-level spec → Optimized implementation
- Cross-theory reasoning and proof reuse

#### Theory Morphisms
- Define translation functions between theories
- Prove correctness conditions (semantics preservation)
- Handle partial translations (not all constructs map)
- Generate bidirectional translators where possible

#### Use Cases
1. **Optimization**: Translate high-level theory to efficient low-level theory
2. **Interop**: Execute Lambda terms in Rho Calculus runtime
3. **Verification**: Prove in one theory, execute in another
4. **Legacy**: Formalize old language, translate to modern theory

#### Example
```rust
morphism! {
    name: LambdaToRho,
    from: LambdaCalc,
    to: RhoCalc,

    // λx.M ↦ new(x.P) where M ↦ P
    translate Lam(x, m) => PNew(x, translate(m)),

    // M N ↦ for(y <- M){N!(*y)}
    translate App(m, n) => ...,
}
```

---

## Development Priorities

### Immediate (Q1 2026)
1. **Fix collection term generation** - Enables testing
2. **Enhance REPL** - Developer experience
3. **Performance benchmarking** - Quantify current state

### Near-Term (Q2-Q4 2026)
1. **Native compilation** - 100x speedup
2. **Module system** - Code reuse
3. **VSCode extension** - Professional tooling
4. **Production deployment** - Real users

### Long-Term (2027+)
1. **Theory translation** - Poly-lingual computation
2. **f1r3fly integration** - Distributed execution
3. **Advanced types** - Dependent, session, linear
4. **Ecosystem** - Package manager, community

---

## Success Criteria

### Technical
- ✅ Correct by construction (type-safe macros)
- ✅ 100x performance (native compilation)
- ✅ Production-ready (error handling, monitoring)
- ✅ Distributed execution (f1r3fly integration)
- ✅ Poly-lingual translation (morphisms working)

### User Experience
- ✅ Time to first theory < 1 hour
- ✅ Interactive exploration (REPL)
- ✅ Professional IDE support (LSP)
- ✅ Excellent documentation
- ✅ Active community

### Impact
- ✅ Used in production systems
- ✅ Academic publications
- ✅ Industry partnerships
- ✅ Standard for formal language development

---

## Core Philosophy

1. **Precision First** - Formal semantics enable automation
2. **Performance Matters** - Must be production-grade, not toy
3. **Usability Counts** - Great UX drives adoption
4. **Composition is Key** - Build complex from simple
5. **Prove Correctness** - Translation must be verified
6. **Open Ecosystem** - Community drives innovation

---

**Next Milestone**: Term generation + REPL enhancement (Q1 2026)
**Long-Term Vision**: Standard platform for poly-lingual computation

See also:
- `getting_started.md` - How to use MeTTaIL
- `architecture.md` - System design and internals
- `design/` - Detailed design documents
- `contributing.md` - How to contribute

