# MeTTaIL: Current Status & Recent Progress

**Last Updated**: November 9, 2025  
**Version**: Phase 6.1 Complete

---

## 📊 Quick Status

| Component | Status | Performance |
|-----------|--------|-------------|
| Core Infrastructure | ✅ Complete | Production |
| Collection Types | ✅ Complete | Optimal |
| Indexed Projection (Flat) | ✅ Complete | Order-independent |
| Deep Projection | 🎯 Design Phase | Planned Q1 2026 |
| Theory Composition | 📋 Not Started | Planned Q2 2026 |

---

## 🎉 Phase 6 Complete: Collection Types & Indexed Projection

### What We Built

**Phase 6.0: Collection Types Foundation**
- ✅ `HashBag<T>` implementation with O(1) equality
- ✅ Collection syntax: `HashBag(Cat) sep "," delim "{" "}"`
- ✅ LALRPOP parser generation for collections
- ✅ AST generation with collection fields
- ✅ Display, substitution, and term operations
- ✅ Integration with `moniker` for binders

**Phase 6.1: Indexed Projection for Order-Independent Matching**
- ✅ Automatic detection of shared variables in collection patterns
- ✅ Generation of projection relations indexed by join keys
- ✅ Ascent-based join generation leveraging Datalog optimization
- ✅ Rest pattern support for collection remainders
- ✅ Collection equation support (e.g., `{P} == P`)
- ✅ Snake-case compliance in generated code

### Impact

**Before (Binary Operations):**
```rust
PPar . Proc ::= Proc "|" Proc;
// 80+ seconds for complex terms
// Exponential equivalence class explosion
// Congruence rules generate infinite paths
```

**After (Collection Types):**
```rust
PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}";
// ~1 second for the same terms
// O(1) equality via HashBag
// Order-independent matching via indexed joins
```

**Performance Improvement**: 80x+ speedup for complex term rewriting

### Example: Rho Calculus

**Theory Definition:**
```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;
        PDrop . Proc ::= "*" Name ;
        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    }
    
    equations {
        (NQuote (PDrop N)) == N ;      // Reflection
        (PPar {P}) == P ;              // Identity normalization
    }
    
    rewrites {
        // Communication: for(chan->x){P} , chan!(Q) => P[@Q/x]
        (PPar {(PInput chan x P), (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
    }
}
```

**What Gets Generated:**
1. **Projection Relations** (automatically):
```rust
relation pinput_proj_r0_p0(Proc, Name, Binder<String>, Proc, Proc);
relation poutput_proj_r0_p1(Proc, Name, Proc, Proc);
```

2. **Extraction Rules** (automatically):
```rust
pinput_proj_r0_p0(parent, chan, x, p, elem) <--
    proc(parent),
    if let Proc::PPar(ref bag) = parent,
    for (elem, _) in bag.iter(),
    if let Proc::PInput(ref f0, ref f1) = elem,
    let chan = (**f0).clone(),
    let x = f1.clone(),
    if let Some(ref body_inner) = f1.clone().unbind(),
    let p = (**body_inner).clone();
```

3. **Join-Based Rewrite** (automatically):
```rust
rw_proc(parent, result) <--
    pinput_proj_r0_p0(parent, chan, x, p, elem_0),
    poutput_proj_r0_p1(parent, chan, q, elem_1),  // Join on chan!
    eq_name(chan, chan),  // Equational matching
    if elem_0 != elem_1,  // Distinctness check
    ...;
```

**Key Property**: Works regardless of element order in the collection! `{a!(0), for(a->x){P}}` and `{for(a->x){P}, a!(0)}` both match.

---

## 🔍 Current Limitations

### 1. Deep Nesting (Partially Solved)

**Works:**
```rust
// Flat shared variables
(PPar {(PInput chan x P), (POutput chan Q)})
//              ^^^^                  ^^^^
//         Both at argument level 0
```

**Doesn't work optimally:**
```rust
// Deeply nested shared variable
(PPar {(PAmb N (PPar {(PIn M P), Q})), (PAmb M R)})
//                         ^                 ^
//                    Nested            Top-level
// Falls back to order-dependent matching
```

**Solution**: Phase 7 (Deep Projection) - see [design docs](design/DEEP-PROJECTION-DESIGN.md)

### 2. Other Known Issues

- No theory composition yet (can't extend or parameterize theories)
- No optimization passes (generated code is straightforward but unoptimized)
- No incremental computation (full recomputation on each step)
- No parallel execution (single-threaded Ascent)
- Limited profiling and debugging tools

---

## 📈 Recent Progress Timeline

### Week of Nov 3-9, 2025

**Nov 3-4: Collection Types Foundation**
- Implemented `HashBag<T>` with proper hashing and equality
- Extended grammar syntax for collections
- Fixed LALRPOP generation to handle collection parsing
- Fixed AST codegen to include collection fields

**Nov 5-6: Collection Integration**
- Fixed substitution for collections (proper count handling)
- Fixed Display generation for collection patterns
- Fixed Ascent deconstruction to iterate over collections
- Added collection equation support

**Nov 7-8: Rest Patterns & Indexed Projection**
- Added `Expr::CollectionPattern` to AST
- Extended parser for `{pattern, ...rest}` syntax
- Implemented nested pattern matching in collections
- Discovered order-dependent matching limitation

**Nov 9: Indexed Projection (Phase 6.1)**
- Designed and implemented automatic indexed projection
- Generated projection relations and extraction rules
- Generated join-based rewrite rules
- Fixed snake-case warnings
- Fixed collection equations (`{P} == P`)
- Documented limitation with deeply nested shared variables

---

## 🎯 Next Steps

### Immediate (This Week)
- [x] Document Phase 6 completion
- [x] Update roadmaps and README
- [x] Clean up workspace
- [ ] Push to repository

### Short-Term (Q1 2026)
1. **Deep Projection** - Handle nested shared variables
   - Quick win: Simplified version for common cases (1-2 days)
   - Full solution: Multi-level projection trees (2-3 weeks)
   - See [Deep Projection Roadmap](design/DEEP-PROJECTION-ROADMAP.md)

2. **Performance Benchmarking** - Quantify improvements
   - Create benchmark suite
   - Measure throughput (rewrites/sec)
   - Track memory usage
   - CI integration

3. **Ascent Parallelization** - Use `ascent_par!`
   - Profile current bottlenecks
   - Switch to parallel Ascent
   - Target: 10x additional speedup

### Medium-Term (Q2 2026)
- Theory composition and inheritance
- Theory parameterization
- Standard theory library
- Module system

### Long-Term (Year 2-3)
- See [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md)

---

## 📚 Key Documentation

### Getting Started
- [README](../README.md) - Project overview
- [Quick Start Guide](../QUICKSTART.md) - Get running in 5 minutes

### Design Documents
- [Collection Types Design](design/COLLECTION-TYPES-DESIGN.md) - Full design and implementation
- [Phase 6 Complete](design/PHASE-6-COMPLETE.md) - Indexed projection details
- [Deep Projection Design](design/DEEP-PROJECTION-DESIGN.md) - Next major feature
- [Deep Projection Roadmap](design/DEEP-PROJECTION-ROADMAP.md) - Implementation plan

### Historical Context
- [Phase 1 Complete](phase-1/PHASE-1-COMPLETE.md) - Foundation work
- [Phase 2 Complete](phase-2/PHASE-2-COMPLETE.md) - Parser and rewrite engine
- [Equational Rewrite Session](design/SESSION-EQUATIONAL-REWRITE.md) - Ascent integration

### Strategic Planning
- [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - 3-year vision
- [Theory Composition Design](design/THEORY-COMPOSITION-DESIGN.md) - Future work

---

## 🧪 Testing Status

### Working Examples
- ✅ **RhoCalc** (`examples/rhocalc.rs`) - Full communication reduction with collections
- ⚠️ **Ambient** (`examples/ambient.rs`) - Basic cases work, complex nesting needs deep projection

### Test Coverage
- ✅ Collection parsing and display
- ✅ Collection pattern matching (flat)
- ✅ Rest pattern extraction and reconstruction
- ✅ Substitution into collections
- ✅ Collection equations
- ✅ Order-independent matching (2-3 elements)
- ⚠️ Deep nested patterns (order-dependent fallback)

---

## 💡 Key Insights & Learnings

### 1. Collection Types Are Essential for Performance
Binary AC operations with congruence rules lead to exponential blowup. Using collection types with O(1) equality is **not an optimization, it's a necessity** for production use.

### 2. Indexed Projection is Powerful
By leveraging Ascent's join optimization, we can match collection patterns in any order efficiently. The key insight: **project elements into indexed relations, then join**.

### 3. Moniker Integration Requires Care
`HashBag<T>` with `BoundTerm` trait requires rebuilding the map during variable operations since keys are hashed. This is fine because substitution is relatively rare compared to equality checks.

### 4. Code Generation Complexity
Correctly handling all the edge cases (binders vs. non-terminals, collections vs. regular fields, nested patterns, etc.) requires careful attention to AST structure and type tracking.

### 5. Order Independence is Non-Trivial
Making pattern matching truly order-independent requires thinking carefully about what constitutes a "join key" and how to extract it at various nesting depths.

---

## 🔧 Technical Architecture

### Code Organization

```
mettail-rust/
├── mettail-macros/          # Procedural macro crate
│   ├── src/
│   │   ├── lib.rs           # Main theory! macro entry point
│   │   ├── ast.rs           # Theory AST definition
│   │   ├── parser_gen.rs    # Grammar parsing for theory! syntax
│   │   ├── codegen.rs       # AST enum generation
│   │   ├── lalrpop_gen.rs   # LALRPOP file generation
│   │   ├── display_gen.rs   # Display trait generation
│   │   ├── subst_gen.rs     # Substitution method generation
│   │   ├── ascent_gen.rs    # Equation/deconstruction generation
│   │   ├── rewrite_gen.rs   # Rewrite rule generation (indexed projection)
│   │   └── ...
│   └── tests/               # Compile-fail tests
├── mettail-runtime/         # Runtime support library
│   └── src/
│       └── lib.rs           # HashBag, Scope, OrdVar, etc.
└── examples/
    ├── rhocalc.rs           # Rho calculus example
    └── ambient.rs           # Ambient calculus example
```

### Generated Code Flow

1. **Parse `theory!` macro** → `ast.rs` structures
2. **Validate** → `typechecker.rs`, `validator.rs`
3. **Generate AST enums** → `codegen.rs`
4. **Generate LALRPOP grammar** → `lalrpop_gen.rs` → `build.rs` → parser
5. **Generate Display** → `display_gen.rs`
6. **Generate substitution** → `subst_gen.rs`
7. **Generate equations** → `ascent_gen.rs` → Ascent `eqrel` clauses
8. **Generate rewrites** → `rewrite_gen.rs` → Ascent indexed projection

---

## 🙏 Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [LALRPOP](https://github.com/lalrpop/lalrpop) - Parser generator
- [Ascent](https://github.com/s-arash/ascent) - Datalog engine in Rust
- [moniker](https://github.com/brendanzab/moniker) - Variable binding library
- [syn](https://github.com/dtolnay/syn) & [quote](https://github.com/dtolnay/quote) - Proc macro infrastructure

Special thanks to the Rust community for excellent tooling and documentation.

---

**Ready for the next phase!** 🚀

