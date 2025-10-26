# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Phase 2 (Current) - Execution
Target: 3-4 months
- Parser generation with LALRPOP
- Pattern matching for rewrite rules
- Rewrite application engine
- Multi-step reduction strategies
- Simple interpreter CLI

### Phase 3 (Future) - Theory Composition
- Parameterized theories
- Theory instantiation
- Cross-theory operations

### Phase 4 (Future) - Optimization
- E-graph integration
- Equality saturation
- Advanced optimizations

---

[Unreleased]: https://github.com/your-username/mettail-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/your-username/mettail-rust/releases/tag/v0.1.0

