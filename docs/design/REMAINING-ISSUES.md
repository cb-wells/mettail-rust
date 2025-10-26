# Remaining Issues After Phase 1

**Last Updated:** After cross-category substitution fix (Phase 1 complete)

## Critical Issues (Blockers for Basic Usage)

### 1. Parser Generation is Completely Broken ⚠️

**Current State:** The generated parser combinator approach doesn't work.

**Problems:**
- No bounds checking (panics on empty input)
- Wrong token order (looks for `!` at start, but `POutput` is `Name!(Proc)`)
- Incorrect position tracking
- Doesn't handle `PInput` at all (skips binder rules)
- Can't handle recursive structures properly
- No operator precedence

**Example of broken generation:**
```rust
pub fn parse_proc(input: &str) -> Result<Proc, ParseError> {
    if input[0usize..].starts_with("!") && input[1usize..].starts_with("(")
    //         ^^^^^^^^ Will panic on empty string!
    //  ^^^^^ Wrong: should parse Name first, not look for "!"
```

**Impact:** Cannot parse any Rho Calculus terms from text.

**Solution:** Complete rewrite using LALRPOP or similar LR(1) parser generator.

---

## Important Issues (Phase 2 Priorities)

### 2. No Runtime Execution

**Current State:** We can:
- ✅ Define theories
- ✅ Type-check terms
- ✅ Generate AST types
- ✅ Generate substitution
- ❌ Actually execute/reduce terms

**Missing:**
- Rewrite rule application
- Equation checking (structural congruence)
- Reduction engine
- Pattern matching against rewrite sources
- Freshness condition checking at runtime

**Example:** We can define `out(n,p) | in(n, \x.f) => f[@p/x]` but can't **apply** it.

### 3. Theory Composition Not Implemented

**Current State:** Design exists (`docs/THEORY-COMPOSITION-DESIGN.md`) but not implemented.

**Missing:**
- Theory parameterization: `theory! { name: T(X) ... }`
- Theory instantiation: `T(Nat)` or `T(OtherTheory)`
- Export/import between theories
- Namespace management

**Impact:** Can't build theories compositionally (which is the whole point of MeTTaIL!).

---

## Moderate Issues (Quality of Life)

### 4. No Pretty Printing

**Current State:** Only debug output (`{:?}`).

**Needed:**
- Human-readable printing: `0 | *@0` instead of `PPar(PZero, PDrop(NQuote(PZero)))`
- Configurable precedence and associativity
- Smart parenthesization

### 5. Limited Error Messages

**Current State:** Compile-time errors have spans, but messages could be better.

**Improvements Needed:**
- More context in type errors
- Suggestions for common mistakes
- Better error recovery

### 6. No Variable Name Preservation

**Current State:** Variables become De Bruijn indices internally (via `moniker`).

**Issue:** Pretty printing shows indices, not original names.

**Solution:** Attach name hints to `FreeVar` and use them in display.

---

## Design Questions (Require Decisions)

### 7. Equation Semantics Undefined

**Current State:** Equations are parsed and type-checked, but their **meaning** is unclear.

**Questions:**
- Are equations **axioms** (assumed true) or **theorems** (to be proven)?
- Should they generate automatic rewrites in both directions?
- How do they interact with the rewrite engine?
- Do we need an E-graph (egg/egglog) for equality saturation?

**Example:** `(PPar P Q) == (PPar Q P)` could:
- **Option A:** Generate rewrites `P|Q => Q|P` and `Q|P => P|Q`
- **Option B:** Mean "these are equivalent in the E-graph"
- **Option C:** Be a theorem to prove via the type system

### 8. Rewrite Strategy Undefined

**Current State:** Rewrite rules are validated but not executed.

**Questions:**
- **Matching:** How to find where a rule applies? (Pattern matching? E-graph?)
- **Application:** Apply first match? All matches? Non-deterministic choice?
- **Confluence:** What if multiple rules apply? Priorities? Overlapping rules?
- **Termination:** How to prevent infinite reduction loops?
- **Congruence:** Should `P|Q => P|R` apply if `Q => R`? (Your example!)

**Example:** Given `s => t`, should we auto-generate congruence rules like:
- `P|s => P|t`
- `s|P => t|P`
- `for(x y){s} => for(x y){t}`
- etc.?

### 9. Freshness Condition Runtime Checking

**Current State:** `if x # Q` is validated at compile-time but not enforced at runtime.

**Questions:**
- How to check freshness at runtime when applying a rewrite?
- Should this be automatic or require explicit checks?
- Performance implications of checking free variables?

---

## Future Enhancements (Phase 3+)

### 10. Advanced Type Features

- Dependent types
- Refinement types
- Session types (for protocols)
- Linear types (for resources)

### 11. Optimization

- Lazy evaluation
- Memoization
- Incremental computation
- Parallel reduction

### 12. Tooling

- Language server (LSP) for IDE support
- Debugger
- Profiler
- Interactive REPL
- Proof assistant integration

### 13. Target Backends

- Interpreter (simple evaluation)
- JIT compilation (via Cranelift)
- WASM compilation
- Native compilation (via LLVM)
- Distributed execution

---

## Non-Issues (Working Correctly)

✅ **Type-checking** - Sound and complete for the grammar  
✅ **Binder handling** - Correct via `moniker`  
✅ **Substitution** - Full cross-category support, capture-avoiding  
✅ **Field ordering** - Preserves grammar order  
✅ **Freshness validation** - Compile-time checking works  
✅ **AST generation** - Clean, well-typed enums  
✅ **Derive integration** - Proper `Debug`, `Clone`, `PartialEq`, `Eq`, `BoundTerm`  

---

## Priority Order for Next Steps

1. **Parser Generation (LALRPOP)** - Blocking for usability
2. **Rewrite Engine Design** - Core functionality decision
3. **Basic Runtime Execution** - Apply rewrites manually
4. **Theory Composition** - Essential for scalability
5. **E-graph Integration** - For equation handling
6. **Pretty Printing** - Quality of life
7. **Advanced Features** - Dependent types, optimization, etc.

---

## Technical Debt

- Unused warnings in macro code (cleanup needed)
- Parser generation code should be removed or marked deprecated
- Some error types are defined but never used
- Test coverage could be more comprehensive
- Documentation strings could be more detailed

---

## Testing Gaps

- ✅ Basic substitution tested
- ✅ Cross-category substitution tested
- ❌ Parser not tested (because it's broken)
- ❌ Rewrite application not tested (not implemented)
- ❌ Equation checking not tested (not implemented)
- ❌ Theory composition not tested (not implemented)
- ❌ Edge cases (empty parallel, nested scopes, etc.)

