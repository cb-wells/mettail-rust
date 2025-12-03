# ascent_gen.rs Function Organization Map

## File: macros/src/ascent/ascent_gen.rs (2,044 lines)

### 📍 **KEEP in ascent_gen.rs (Orchestrator)**

**Lines 10-56: `generate_ascent_source()`**
- ✅ **KEEP HERE** - Main entry point, orchestrates all generation
- Calls: `generate_relations()`, `generate_category_rules()`, `generate_equation_rules()`, `generate_rewrite_rules()`
- Role: Public API and debug printing

---

### 📦 **MOVE to ascent/categories.rs**

**Lines 62-79: `generate_deconstruction_rules()`**
- 🔵 **categories.rs** - Category-specific deconstruction logic
- Already has similar logic in categories.rs

**Lines 81-126: `generate_deconstruction_for_constructor()`**
- 🔵 **categories.rs** - Per-constructor deconstruction
- Helper for category exploration

**Lines 128-146: `generate_collection_deconstruction()`**
- 🔵 **categories.rs** - Collection deconstruction (disabled but belongs here)

**Lines 148-199: `generate_collection_projection_population()`**
- 🔵 **categories.rs** - Populating collection projections
- Already has similar logic in categories.rs

**Lines 201-249: `generate_projection_seeding_rules()`**
- 🔵 **categories.rs** - Seeding from projections
- Part of category exploration

**Lines 251-294: `generate_regular_deconstruction()`**
- 🔵 **categories.rs** - Regular constructor deconstruction

**Lines 296-376: `generate_binding_deconstruction()`**
- 🔵 **categories.rs** - Binding constructor deconstruction

**Lines 378-475: `generate_congruence_rules()`**
- 🔵 **categories.rs** - Equality congruence for constructors
- NOTE: This is for **equality** (eq_*), not rewrites (rw_*)

---

### 📦 **MOVE to ascent/equations.rs (NEW FILE)**

**Lines 477-514: `generate_equation_rules()`**
- 🟢 **equations.rs** - Main equation generation entry point

**Lines 1067-1155: `generate_equation_pattern_via_rewrite_logic()`**
- 🟢 **equations.rs** - Equation pattern matching adapter

**Lines 1157-1279: `generate_equation_rhs_simple()`**
- 🟢 **equations.rs** - RHS generation for equations

**Lines 1281-1325: `generate_equation_clause()`**
- 🟢 **equations.rs** - Single equation clause generation

**Lines 1327-1346: `normalize_collection_apply()`**
- 🟢 **equations.rs** - Helper for equation normalization

**Lines 1399-1436: `to_snake_case()`**
- 🟢 **equations.rs** - Utility for variable naming

**Lines 1438-1468: `generate_collection_equation_rhs()`**
- 🟢 **equations.rs** - Collection RHS for equations

**Lines 1470-1595: `generate_equation_rhs()`**
- 🟢 **equations.rs** - Full equation RHS generation

**Lines 1597-1622: `generate_equation_freshness()`**
- 🟢 **equations.rs** - Freshness condition generation

---

### 📦 **MOVE to ascent/congruence/collection.rs**

**Lines 577-637: `generate_congruence_rewrite()`**
- 🟡 **congruence/collection.rs** OR **congruence/mod.rs** - Dispatcher for congruence types
- Routes to collection vs regular congruence

**Lines 639-687: `extract_congruence_info()`**
- 🟡 **congruence/analysis.rs** - Already exists there, or move this variant

**Lines 689-752: `generate_collection_congruence()`**
- 🟡 **congruence/collection.rs** - Old-style collection congruence

**Lines 1624-1703: `generate_new_collection_congruence_clauses()`**
- 🟡 **congruence/collection.rs** - New projection-based collection congruence

**Lines 1705-1883: `generate_joined_base_rewrite_clause()`**
- 🟡 **congruence/collection.rs** - Multi-element projection joins

**Lines 1885-1951: `generate_rhs_reconstruction()`**
- 🟡 **congruence/collection.rs** - RHS for collection congruence

---

### 📦 **MOVE to ascent/congruence/regular.rs**

**Lines 754-801: `generate_regular_congruence()`**
- 🟠 **congruence/regular.rs** - Regular (non-collection) congruence

**Lines 1953-2042: `generate_regular_congruence_clause()`**
- 🟠 **congruence/regular.rs** - Clause generation for regular congruence

---

### 📦 **MOVE to ascent/congruence/binding.rs (or projections.rs)**

**Lines 803-869: `generate_binding_congruence()`**
- 🔴 **congruence/binding.rs** - Old-style binding congruence (may be obsolete?)

**Lines 871-985: `generate_projection_based_binding_congruence()`**
- 🔴 **congruence/projections.rs** - New projection-based binding congruence

**Lines 987-997: `generate_binding_proj_declaration()`**
- 🔴 **congruence/projections.rs** - Projection relation declaration

**Lines 999-1040: `generate_binding_proj_population()`**
- 🔴 **congruence/projections.rs** - Projection population rule

**Lines 1042-1065: `generate_binding_congruence_clause()`**
- 🔴 **congruence/projections.rs** - Congruence clause using projection

---

### 📦 **KEEP in ascent_gen.rs (Orchestrator)**

**Lines 516-575: `generate_rewrite_rules()`**
- ✅ **KEEP HERE** - Orchestrates base rewrites + congruences
- Could alternatively move to `rewrites/mod.rs` as public entry point

---

### 🔧 **Utility Functions (Keep in ascent_gen.rs or move to utils)**

**Lines 1348-1374: `extract_category_from_expr()`**
- Could stay in ascent_gen.rs or move to `utils.rs` (shared utility)

**Lines 1376-1379: `is_constructor()`**
- Could stay in ascent_gen.rs or move to `utils.rs`

**Lines 1381-1389: `category_has_collections()`**
- Could stay in ascent_gen.rs or move to `utils.rs`

**Lines 1391-1397: `is_nullary_constructor()`**
- Could stay in ascent_gen.rs or move to `utils.rs`

---

## 📊 Summary by Destination

### ✅ KEEP in `ascent_gen.rs` (2 functions, ~100 lines)
- `generate_ascent_source()` - Main orchestrator
- `generate_rewrite_rules()` - Rewrite orchestrator (or move to rewrites/mod.rs)
- Utility functions (or extract to utils)

### 🔵 MOVE to `categories.rs` (9 functions, ~450 lines)
- `generate_deconstruction_rules()`
- `generate_deconstruction_for_constructor()`
- `generate_collection_deconstruction()`
- `generate_collection_projection_population()`
- `generate_projection_seeding_rules()`
- `generate_regular_deconstruction()`
- `generate_binding_deconstruction()`
- `generate_congruence_rules()` (equality congruence)

### 🟢 MOVE to `equations.rs` (NEW, 9 functions, ~550 lines)
- `generate_equation_rules()`
- `generate_equation_pattern_via_rewrite_logic()`
- `generate_equation_rhs_simple()`
- `generate_equation_clause()`
- `normalize_collection_apply()`
- `to_snake_case()`
- `generate_collection_equation_rhs()`
- `generate_equation_rhs()`
- `generate_equation_freshness()`

### 🟡 MOVE to `congruence/collection.rs` (5 functions, ~550 lines)
- `generate_congruence_rewrite()` (dispatcher - could go in congruence/mod.rs)
- `generate_collection_congruence()` (old style)
- `generate_new_collection_congruence_clauses()`
- `generate_joined_base_rewrite_clause()`
- `generate_rhs_reconstruction()`

### 🟠 MOVE to `congruence/regular.rs` (2 functions, ~100 lines)
- `generate_regular_congruence()`
- `generate_regular_congruence_clause()`

### 🔴 MOVE to `congruence/projections.rs` (5 functions, ~200 lines)
- `generate_projection_based_binding_congruence()`
- `generate_binding_congruence_clause()`
- `generate_binding_proj_declaration()`
- `generate_binding_proj_population()`
- `generate_binding_congruence()` (old style - may be obsolete)

### 🔧 Utilities (4 functions, ~40 lines)
- `extract_category_from_expr()` - Could be in utils or ascent_gen
- `is_constructor()` - Could be in utils
- `category_has_collections()` - Could be in utils
- `is_nullary_constructor()` - Could be in utils

---

## 🎯 Recommended Extraction Order

1. **First: Create `equations.rs`** (~550 lines, clean extraction)
   - All equation-related functions are well-isolated
   
2. **Second: Move deconstruction to `categories.rs`** (~450 lines)
   - Extends existing categories.rs
   
3. **Third: Split `congruence.rs` into subdirectory**
   - Rename current `congruence.rs` to `congruence/analysis.rs`
   - Create `congruence/collection.rs` with collection congruence from ascent_gen
   - Create `congruence/regular.rs` with regular congruence
   - Create `congruence/projections.rs` with projection-based congruence
   - Create `congruence/mod.rs` to orchestrate and re-export

4. **Fourth (Optional): Split `rewrites.rs` into subdirectory**
   - If it continues to grow or has sub-concerns
   
5. **Fifth: Minimal `ascent_gen.rs`**
   - Keep only `generate_ascent_source()` and orchestration
   - ~100 lines total

---

## 📏 Final Size Estimates

After reorganization:

```
ascent/
├── ascent_gen.rs              ~100 lines ✅ Minimal orchestrator
├── relations.rs               ~94 lines ✅
├── categories.rs              ~800 lines ✅ (363 + 450 new)
├── equations.rs               ~550 lines ✅ NEW
├── mod.rs                     ~50 lines
├── rewrites/
│   └── (current rewrites.rs split if needed)
└── congruence/
    ├── mod.rs                 ~50 lines
    ├── analysis.rs            ~1125 lines (current congruence.rs)
    ├── collection.rs          ~550 lines (from ascent_gen)
    ├── regular.rs             ~100 lines (from ascent_gen)
    └── projections.rs         ~200 lines (from ascent_gen)
```

**Result:**
- ✅ No file over 1,125 lines
- ✅ Clear organization by concern
- ✅ Subdirectories for complex multi-file modules
- ✅ Single files for focused concerns
- ✅ Minimal orchestrator

---

## 🎓 Design Principles Applied

1. **Single files for < 800 lines, single concern**
   - relations.rs, equations.rs
   
2. **Subdirectories when:**
   - Total > 1000 lines AND
   - Multiple sub-concerns AND
   - Likely to grow
   
3. **Keep orchestrators minimal**
   - ascent_gen.rs should just wire things together
   
4. **Group by feature, not by type**
   - ✅ `congruence/collection.rs` (feature: collection congruence)
   - ❌ `generators/congruence.rs` (type: generator)

This structure balances maintainability with organization depth!

