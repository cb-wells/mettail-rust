## 🎓 Research Questions & Open Problems

### Architecture & Design Principles

1. **Structural vs. Equational Properties**
   - *Lesson learned*: Some properties (like collection flattening) are better implemented structurally during construction rather than as equations
   - *Question*: What systematic criteria determine when a property should be structural vs. equational?
   - *Impact*: Affects user burden, correctness guarantees, and performance

2. **Compile-Time vs. Runtime**
   - *Success*: Automatic flattening via generated helpers demonstrates power of compile-time code generation
   - *Question*: What other runtime properties can be moved to compile-time for zero overhead?
   - *Examples*: Type checking, freshness, some congruence rules?

3. **Code Generation Strategies**
   - *Current*: Generate straightforward, unoptimized code
   - *Question*: When to optimize generated code vs. rely on LLVM/rustc?
   - *Trade-off*: Compilation time vs. runtime performance vs. code maintainability

### Pattern Matching & Rewriting

4. **Order-Independent Matching at Scale**
   - *Solved*: Indexed projection for flat shared variables
   - *Open*: Deep projection for nested shared variables (Ambient calculus)
   - *Question*: Can we automatically detect and optimize common nesting patterns?
   - *Challenge*: Balance generality with performance

5. **Collection Pattern Complexity**
   - *Observation*: N=2 patterns are common and fast, N>2 rare but expensive
   - *Question*: Should we provide multiple strategies (fast path for N=2, slow path for N>2)?
   - *Trade-off*: Code complexity vs. performance vs. expressiveness

6. **Equational Theories & Modularity**
   - *Current*: Equations are global within a theory
   - *Question*: How to compose theories with different equational theories?
   - *Example*: Combine theory with AC operators + theory with just associativity
   - *Challenge*: Ensure soundness across theory boundaries

### Performance & Scalability

7. **Ascent Materialization**
   - *Current*: All relations fully materialized in memory
   - *Limitation*: Large rewrite graphs exceed memory
   - *Question*: Can we use incremental computation or lazy evaluation?
   - *Alternative*: Switch to on-demand evaluation for large graphs?

8. **Parallel Execution**
   - *Blocker*: `ascent_run_par!` has type incompatibilities with generated code
   - *Question*: Is parallelism essential, or is single-threaded "fast enough"?
   - *Observation*: Most examples complete in < 1 second after optimizations

9. **Term Generation Completeness**
   - *Gap*: Currently skips collection types
   - *Question*: What's the right generation strategy for collections?
   - *Challenge*: Combinatorial explosion for multiset enumeration

### Poly-Lingual Computation

10. **Theory Composition**
    - *Vision*: Build complex theories from simpler components
    - *Questions*:
      - How to handle name collisions?
      - How to ensure type safety across composition?
      - Can we infer common abstractions automatically?
    - *Goal*: Zero-cost abstractions for theory reuse

11. **Cross-Theory Translation**
    - *Unsolved*: How to translate terms between theories?
    - *Example*: Lambda calculus ⟷ Combinatory logic
    - *Questions*:
      - Specify translations declaratively or algorithmically?
      - How to prove translation correctness?
      - Can translations be bidirectional?

12. **Federated Execution**
    - *Vision*: Execute theories across distributed systems
    - *Questions*:
      - How to partition rewrite computation?
      - How to handle cross-system term communication?
      - Can we prove distributed confluence?

### Developer Experience

13. **Debugging Rewrite Systems**
    - *Current*: Limited visibility into Ascent execution
    - *Need*: Trace rule applications, explain equivalences, visualize rewrite graphs
    - *Question*: What's the minimal set of debugging primitives needed?
    - *Solution*: Term Explorer REPL (planned Q1 2026)

14. **Error Messages**
    - *Current*: Proc-macro errors can be cryptic
    - *Question*: How to provide actionable error messages for:
      - Grammar mistakes
      - Type errors in rewrites
      - Unbound variables
      - Freshness violations
    - *Challenge*: Error location in macro-generated code

15. **IDE Integration**
    - *Gap*: No IDE support for `theory!` macro DSL
    - *Desired*:
      - Syntax highlighting
      - Auto-completion
      - Jump-to-definition
      - Inline type information
    - *Question*: Extend rust-analyzer or custom language server?

### Formal Properties

16. **Confluence**
    - *Current*: Assumed, not checked
    - *Question*: Can we automatically detect non-confluence?
    - *Approach*: Critical pair analysis? Counter-example search?
    - *Trade-off*: Checking cost vs. theorem proving burden

17. **Termination**
    - *Current*: User responsibility to ensure termination
    - *Question*: Can we bound computation or detect cycles?
    - *Approach*: Size-decreasing metrics? Fixpoint detection?

18. **Soundness of Equational Matching**
    - *Current*: `eq_cat()` relations computed via reflexivity + congruence
    - *Question*: How to prove equational matching is sound and complete?
    - *Challenge*: Interaction between equations and rewrites

### Open Research Directions

19. **E-Graph Integration**
    - *Question*: Would equality saturation improve performance/completeness?
    - *Trade-off*: Ascent (explicit relations) vs. e-graphs (implicit equality)?
    - *Observation*: Ascent already handles equality well for our use cases

20. **Optimization Passes**
    - *Gap*: No optimization of rewrite rules themselves
    - *Potential*:
      - Common subexpression elimination
      - Rule fusion (combine multiple rewrites)
      - Dead code elimination (unreachable rules)
    - *Question*: Is hand-written code optimization worth complexity?

---
