# MeTTaIL Phase Naming Clarification

**Date**: November 9, 2025

---

## Historical Phase Names

The project used numbered phases during development, but the numbering became inconsistent over time.

### Original Plan (from CHANGELOG.md)

- **Phase 1** (✅ Complete) - Foundation
  - AST generation, type-checking, binders, substitution
  - Completed: October 2025
  
- **Phase 2** (✅ Complete) - Parser & Rewrite Engine  
  - LALRPOP integration, pattern matching, execution
  - Completed: October 2025

- **Phase 3** (Planned) - Theory Composition
  - Theory imports, parameterization, module system
  - Target: Q2 2026

---

## What Happened with "Phase 6"?

During November 2025, we implemented **Collection Types** and **Indexed Projection**. These were documented as "Phase 6" in several places, but this numbering was **inconsistent** with the original plan.

**Why "Phase 6"?**
- Working sessions used internal phase numbers for collection types implementation
- "Phase 6" referred to the 6th sub-phase of the collection types work
- Got copied into documentation without renaming

**What it should have been:**
- Part of **performance improvements** (between Phases 2-3)
- Or labeled as a **sub-phase** of Phase 2 (execution)

---

## Current Recommendation: Drop Numbers

Instead of trying to retroactively fix the numbering, we recommend using **descriptive names** going forward:

### Completed Work

✅ **Foundation Phase** (Oct 2025)
- AST, types, binders, substitution
- Was: "Phase 1"
- Docs: `docs/phase-1/`

✅ **Execution Phase** (Oct 2025)  
- Parser, rewrite engine, Ascent integration
- Was: "Phase 2"
- Docs: `docs/phase-2/`

✅ **Collection Types** (Nov 2025)
- HashBag, rest patterns, order-independent matching
- Was: Inconsistently called "Phase 6"
- Docs: `docs/design/PHASE-6-COMPLETE.md` (keep name for historical continuity)

### Active/Planned Work

🎯 **Deep Projection** (Q1 2026)
- Nested shared variables in collection patterns
- Don't call it "Phase 7" - just "Deep Projection"
- Docs: `docs/design/DEEP-PROJECTION-*.md`

📋 **Theory Composition** (Q2 2026)
- Was: "Phase 3" in original plan
- Keep calling it "Theory Composition", not "Phase N"
- Docs: `docs/design/THEORY-COMPOSITION-DESIGN.md`

📋 **Performance & Benchmarking** (Q1 2026)
- Parallelization, profiling, optimization
- Part of Year 1 goals
- No phase number

---

## Documentation Consistency

### Keep As-Is (for historical accuracy)

These docs use phase numbers, but **don't rename them**:
- `docs/phase-1/` - Keep directory name
- `docs/phase-2/` - Keep directory name  
- `docs/design/PHASE-6-COMPLETE.md` - Keep filename
- `docs/design/PHASE-6.1-COMPLETE.md` - Keep filename
- `CHANGELOG.md` entries - Don't change history

### Use Descriptive Names Going Forward

New documentation should use descriptive names:
- ✅ "Deep Projection Roadmap" (not "Phase 7 Plan")
- ✅ "Theory Composition Design" (not "Phase 3 Design")
- ✅ "Performance Optimization" (not "Phase N")

### Update High-Level Docs

These should reference features, not phase numbers:
- ✅ `README.md` - Uses feature names ("Collection Types", "Indexed Projection")
- ✅ `CURRENT-STATUS.md` - Uses descriptive names
- ✅ `POLY-LINGUAL-ROADMAP.md` - Uses quarters and features, not phases

---

## Why This Approach?

**Pros:**
- No need to renumber existing documentation
- Historical docs remain accurate
- More descriptive for users ("Collection Types" > "Phase 6")
- Flexible ordering (can work on multiple features in parallel)
- Easier to reference ("see Deep Projection docs" vs "see Phase 7 docs")

**Cons:**
- Existing phase numbers remain inconsistent (but clearly marked as historical)

---

## Summary

- **Don't worry about phase numbering inconsistencies** - they're historical artifacts
- **Keep existing phase numbers in filenames** for continuity
- **Use descriptive names for new work** going forward
- **Reference features by name**, not phase number

The "Phase 6" naming is fine to keep - it's clear from context what it refers to (Collection Types + Indexed Projection).

