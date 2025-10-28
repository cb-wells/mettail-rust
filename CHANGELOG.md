# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Phase 2 Complete ✅ (October 28, 2025)

#### Rewrite Engine (NEW!)
- **Automatic rewrite engine generation** from declarative rewrite rules
- **Nested pattern matching** with arbitrary nesting depth
- **Binder extraction** (automatic `unbind()` for `Scope` structures)
- **Freshness checking** (generates `x # Q` conditions using moniker)
- **Capture-avoiding substitution** integration with generated methods
- **Type-safe execution** with proper boxing/unboxing throughout

#### Parser Generation (LALRPOP)
- **Precedence-aware grammars** with automatic infix operator handling
- **Binder parsing** directly into `Scope` structures
- **Variable binding fix** - FreeVars from body are reused as binders
- **Parentheses support** for precedence override
- **Left-associativity** for binary operators
- **Display trait generation** for pretty-printing terms

#### Examples & Demos
- Working Rho Calculus communication reduction: `for(a<-x){*x}|a!(0)` → `*@(0)`
- Multi-step execution demo with visualization
- Round-trip parse/display tests

#### Documentation
- [REWRITE-ENGINE-COMPLETE.md](docs/phase-2/REWRITE-ENGINE-COMPLETE.md) - Technical deep dive
- [DEMO.md](DEMO.md) - Demo guide for presentations
- Updated README and QUICKSTART with rewrite engine info
- [K-FRAMEWORK-COMPARISON.md](docs/design/K-FRAMEWORK-COMPARISON.md) - Feature comparison

#### Technical Improvements
- Variable identity preservation in parser (critical for moniker binding)
- Nested pattern binding order fix (bindings before freshness checks)
- Unique field names for nested patterns (prevents shadowing)
- Proper dereferencing for boxed fields (`**field` for `&Box<T>`)

### Stats
- **~4000 LOC** - Core implementation
- **~3400 LOC** - Generated for Rho Calculus (AST + parser + substitution + rewrite engine)
- **0.7s** - Compile time for theory
- **~600 LOC** - Rewrite engine generator

### Fixed
- Variable binding in LALRPOP parser (FreeVar identity mismatch)
- Nested pattern variable scoping issues
- Binding order for freshness checks
- Dereferencing for boxed pattern fields

### Added
- Initial repository setup with comprehensive documentation
- GitHub Actions CI/CD pipeline
- Issue and PR templates
- Code of Conduct and Contributing guidelines

## [0.1.0] - 2025-10-26

### Added - Phase 1 Complete ✅

#### Core Features
- Theory definition syntax with `theory!` macro
- Type-safe AST generation from theory definitions
- Category inference and type-checking
- Binder support with correct scoping via `moniker`
- Cross-category substitution (e.g., `Proc.substitute_name(var, Name)`)
- Rewrite rule syntax parsing and validation
- Comprehensive compile-fail tests

#### Code Generation
- Type-safe AST enums with proper binder handling
- Capture-avoiding substitution methods
- Standard trait derivations (Debug, Clone, PartialEq, Eq, BoundTerm)
- Parser stubs (to be completed in Phase 2)

#### Examples
- Rho Calculus implementation with communication
- Lambda Calculus example
- Multiple test theories

#### Documentation
- Complete Phase 1 documentation
- Architecture diagrams and design documents
- Comprehensive README with examples
- Detailed roadmap through Phase 4

### Known Issues
- Parser generation incomplete (Phase 2 priority)
- Pattern matching not yet implemented
- Rewrite application not yet implemented
- No reduction engine yet

## Phase Roadmap

### Phase 2 (COMPLETE ✅) - Execution
- ✅ Parser generation with LALRPOP
- ✅ Pattern matching for rewrite rules
- ✅ Rewrite application engine
- ✅ Multi-step reduction with visualization
- ✅ Capture-avoiding substitution integration

### Phase 3 (Next) - Theory Composition
Target: 2-3 months
- Theory imports and reuse
- Parameterized theories (`List<T>`)
- Theory instantiation
- Extension syntax
- Module system with namespacing

### Phase 4 (Future) - Optimization
- E-graph integration for equations
- Equality saturation
- Congruence rule generation
- Advanced optimization passes

---

[Unreleased]: https://github.com/your-username/mettail-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-username/mettail-rust/releases/tag/v0.1.0

