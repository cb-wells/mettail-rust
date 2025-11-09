# Documentation Guide

This document provides an overview of all MeTTaIL documentation and where to find specific information.

---

## 🎯 Start Here

### For New Users
1. **[README](../README.md)** - Overview, quick start, and examples
2. **[Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md)** - Strategic vision and 3-year plan
3. **[Session Summary](SESSION-SUMMARY.md)** - Recent accomplishments and current state

### For Contributors
1. **[Technical Roadmap](ROADMAP.md)** - Detailed implementation phases
2. **[Collection Types Design](design/COLLECTION-TYPES-DESIGN.md)** - Current work in progress
3. **[Phase 2 Complete](phase-2/PHASE-2-COMPLETE.md)** - Recent technical achievements

---

## 📚 Documentation Map

### Strategic Planning
- **[Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md)** - 3-year strategic plan for poly-lingual computation
  - Year 1: Performance & Foundations (Q4 2025 - Q4 2026)
  - Year 2: Poly-Lingual Features (Q1 2027 - Q4 2027)
  - Year 3: Advanced Features & Ecosystem (Q1 2028 - Q4 2028)
  - Use cases, success metrics, research agenda

- **[Technical Roadmap](ROADMAP.md)** - Phase-by-phase implementation plan
  - Phase 1: Foundation ✅
  - Phase 2: Parser & Rewrite Engine ✅
  - Phase 3: Theory Composition (Next)
  - Phase 4-8: Execution, Semantics, Production, Tooling, Types

### Current Status
- **[Session Summary](SESSION-SUMMARY.md)** - November 2025 progress summary
  - Ascent-based rewrite engine
  - Equational matching
  - Performance analysis and bottlenecks
  - Next steps

### Design Documents

#### Active Designs (In Progress)
- **[Collection Types](design/COLLECTION-TYPES-DESIGN.md)** - Multisets/bags for AC operations
  - Motivation: 100x speedup target
  - Implementation plan (5 phases)
  - Ascent support analysis
  - Pattern matching over collections

#### Completed Designs
- **[Rewrite Equational Matching](design/REWRITE-EQUATIONAL-MATCHING.md)** - Ascent-based rewrites
- **[Variable Equality Implementation](VARIABLE-EQUALITY-IMPLEMENTATION.md)** - Type-aware variable tracking
- **[Binder Shadowing Fix](design/BINDER-SHADOWING-FIX.md)** - Unique binder names
- **[Sorting Design](design/SORTING-DESIGN.md)** - Term ordering implementation
- **[Term Generation Assumptions](design/TERM-GENERATION-ASSUMPTIONS.md)** - Simplifications and future work
- **[Substitution Cross-Category Fix](design/SUBSTITUTION-CROSS-CATEGORY-FIX.md)** - Global substitution methods
- **[Rewrite Nesting Fixed](design/REWRITE-NESTING-FIXED.md)** - Arbitrary-depth pattern matching

#### Pending/Future Designs
- **[Theory Composition](design/THEORY-COMPOSITION-DESIGN.md)** - Imports, parameters, modules
- **[Data Structures](design/DATA-STRUCTURES-DESIGN.md)** - Collection types and indexing
- **[K Framework Comparison](design/K-FRAMEWORK-COMPARISON.md)** - Comparison with K semantics
- **[Congruence Rules](design/CONGRUENCE-RULES-DESIGN.md)** - Automatic generation
- **[Binder Design](design/BINDER-DESIGN.md)** - Variable binding semantics
- **[Variable Typing Analysis](design/VARIABLE-TYPING-ANALYSIS.md)** - Type inference for variables

### Phase Documentation

#### Phase 1: Foundation ✅
- **[Phase 1 Plan](phase-1/PHASE-1-PLAN.md)** - Original implementation plan
- **[Phase 1 Complete](phase-1/PHASE-1-COMPLETE.md)** - Achievement summary
- **[Phase 1 Critical Analysis](phase-1/PHASE-1-CRITICAL-ANALYSIS.md)** - Post-mortem
- **[Progress](phase-1/PROGRESS.md)** - Detailed progress tracking
- **[Foundation Review](phase-1/FOUNDATION-REVIEW.md)** - Mid-phase review
- **[Rewrite Rules Complete](phase-1/REWRITE-RULES-COMPLETE.md)** - Syntax and validation
- **[Week 3 Summary](phase-1/WEEK-3-SUMMARY.md)** - Weekly update

##### Substitution Work
- **[Substitution POC](phase-1/substitution/SUBSTITUTION-POC.md)** - Initial proof of concept
- **[Substitution Foundation](phase-1/substitution/SUBSTITUTION-FOUNDATION.md)** - Core implementation
- **[Substitution Complete](phase-1/substitution/SUBSTITUTION-COMPLETE.md)** - Final implementation
- **[Substitution in Rewrites](phase-1/substitution/SUBSTITUTION-IN-REWRITES-COMPLETE.md)** - Integration

##### Initial Work
- **[Initial Theory Design](phase-1/initial/THEORY-DESIGN.md)** - First design iteration
- **[AST Generation](phase-1/initial/AST-GENERATION.md)** - AST codegen
- And more...

#### Phase 2: Parser & Rewrite Engine ✅
- **[Phase 2 Complete](phase-2/PHASE-2-COMPLETE.md)** - Achievement summary
- **[LALRPOP Design](phase-2/LALRPOP-DESIGN.md)** - Parser generation approach
- **[Parsing Success](phase-2/PARSING-SUCCESS.md)** - Initial parsing working
- **[Parsing Test Guide](phase-2/PARSING-TEST-GUIDE.md)** - Testing strategy
- **[Precedence Success](phase-2/PRECEDENCE-SUCCESS.md)** - Operator precedence
- **[Rewrite Engine Complete](phase-2/REWRITE-ENGINE-COMPLETE.md)** - Execution engine
- **[Quick Start](phase-2/QUICK-START.md)** - Getting started guide
- **[Session Summary](phase-2/SESSION-SUMMARY.md)** - Phase 2 wrap-up
- **[Week 1 Progress](phase-2/WEEK-1-PROGRESS.md)** - Weekly update

### Cleanup & Maintenance
- **[Cleanup Complete](CLEANUP-COMPLETE.md)** - Code cleanup summary
- **[Implementation Summary](IMPLEMENTATION-SUMMARY.md)** - Architecture overview
- **[Workspace Cleanup Assessment](WORKSPACE-CLEANUP-ASSESSMENT.md)** - Repository organization

### Historical Documents
- **[POC Results](phase-1/POC-RESULTS.md)** - Early proof-of-concept results
- **[Phase 1 to 2 Transition](design/PHASE-1-TO-2-TRANSITION.md)** - Transition planning

---

## 🔍 Finding Information

### By Topic

#### Performance
- [Collection Types Design](design/COLLECTION-TYPES-DESIGN.md)
- [Session Summary](SESSION-SUMMARY.md) - Current bottlenecks
- [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Q4 2025 performance focus

#### Type System
- [Variable Typing Analysis](design/VARIABLE-TYPING-ANALYSIS.md)
- [Variable Equality Implementation](VARIABLE-EQUALITY-IMPLEMENTATION.md)
- [Binder Design](design/BINDER-DESIGN.md)

#### Execution Engine
- [Rewrite Engine Complete](phase-2/REWRITE-ENGINE-COMPLETE.md)
- [Rewrite Equational Matching](design/REWRITE-EQUATIONAL-MATCHING.md)
- [Rewrite Nesting Fixed](design/REWRITE-NESTING-FIXED.md)

#### Theory Composition
- [Theory Composition Design](design/THEORY-COMPOSITION-DESIGN.md)
- [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Q1 2026 focus
- [Technical Roadmap](ROADMAP.md) - Phase 3

#### Code Generation
- [AST Generation](phase-1/initial/AST-GENERATION.md)
- [LALRPOP Design](phase-2/LALRPOP-DESIGN.md)
- [Implementation Summary](IMPLEMENTATION-SUMMARY.md)

### By Phase

#### Active Work (Current)
1. [Collection Types Design](design/COLLECTION-TYPES-DESIGN.md) - IN PROGRESS
2. [Session Summary](SESSION-SUMMARY.md) - Recent work
3. [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Strategic direction

#### Recently Completed (Phase 2)
1. [Phase 2 Complete](phase-2/PHASE-2-COMPLETE.md)
2. [Rewrite Engine Complete](phase-2/REWRITE-ENGINE-COMPLETE.md)
3. [Rewrite Equational Matching](design/REWRITE-EQUATIONAL-MATCHING.md)

#### Next Up (Phase 3)
1. [Theory Composition Design](design/THEORY-COMPOSITION-DESIGN.md)
2. [Technical Roadmap](ROADMAP.md) - Phase 3 details
3. [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Q1 2026 plan

---

## 📖 Reading Order

### For Understanding MeTTaIL

**New to MeTTaIL?**
1. [README](../README.md) - What is MeTTaIL?
2. [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Why poly-lingual computation?
3. [Session Summary](SESSION-SUMMARY.md) - What works today?
4. [Phase 1 Complete](phase-1/PHASE-1-COMPLETE.md) - Foundation
5. [Phase 2 Complete](phase-2/PHASE-2-COMPLETE.md) - Execution

**Want to Understand the Code?**
1. [Implementation Summary](IMPLEMENTATION-SUMMARY.md)
2. [AST Generation](phase-1/initial/AST-GENERATION.md)
3. [LALRPOP Design](phase-2/LALRPOP-DESIGN.md)
4. [Rewrite Engine Complete](phase-2/REWRITE-ENGINE-COMPLETE.md)

**Planning to Contribute?**
1. [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - Strategic direction
2. [Collection Types Design](design/COLLECTION-TYPES-DESIGN.md) - Current work
3. [Technical Roadmap](ROADMAP.md) - Upcoming phases
4. [Phase 3 plan](ROADMAP.md#phase-3) - Next priorities

### For Research

**Theoretical Foundations**
1. [Binder Design](design/BINDER-DESIGN.md)
2. [Variable Typing Analysis](design/VARIABLE-TYPING-ANALYSIS.md)
3. [Congruence Rules Design](design/CONGRUENCE-RULES-DESIGN.md)
4. [K Framework Comparison](design/K-FRAMEWORK-COMPARISON.md)

**Practical Implementation**
1. [Collection Types Design](design/COLLECTION-TYPES-DESIGN.md)
2. [Rewrite Equational Matching](design/REWRITE-EQUATIONAL-MATCHING.md)
3. [Substitution Implementation](phase-1/substitution/SUBSTITUTION-COMPLETE.md)

**Future Directions**
1. [Poly-Lingual Roadmap](POLY-LINGUAL-ROADMAP.md) - 3-year research agenda
2. [Theory Composition Design](design/THEORY-COMPOSITION-DESIGN.md)
3. [Technical Roadmap](ROADMAP.md) - Phase 4-8

---

## 📝 Documentation Standards

### Active Documents
Updated regularly, reflect current state:
- README.md
- Session Summary
- Poly-Lingual Roadmap
- Collection Types Design

### Stable Documents
Completed work, historical record:
- Phase 1 Complete
- Phase 2 Complete
- Design documents for implemented features

### Historical Documents
No longer current, kept for reference:
- POC Results
- Early design iterations
- Phase transition documents

---

## 🔄 Keeping Documentation Updated

### After Major Milestones
- [ ] Update Session Summary
- [ ] Update README with new features
- [ ] Mark phase as complete
- [ ] Update roadmap with actual dates

### During Development
- [ ] Update design docs with decisions
- [ ] Document open questions
- [ ] Track progress in phase docs
- [ ] Link related documents

### Before Release
- [ ] Update all roadmap dates
- [ ] Ensure README is current
- [ ] Verify all links work
- [ ] Update metrics and stats

---

## 📬 Documentation Feedback

Missing information? Unclear explanations? Found a broken link?

Please open an issue or PR to improve our documentation!

**Goal:** Make MeTTaIL accessible to everyone from beginners to experts.

---

**Last Updated:** November 2025

