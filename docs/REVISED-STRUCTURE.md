# MeTTaIL Revised Directory Structure

**Date:** December 2, 2025  
**Based on:** User feedback and Rust best practices

---

## 🎯 Design Principles

1. **Simple naming** - No "mettail-" prefix bloat
2. **Clear separation** - Each crate has one purpose
3. **Minimal duplication** - Single source of truth
4. **Standard Rust patterns** - Follow community conventions
5. **Generated code isolation** - Keep generated files organized

---

## 📁 Proposed Structure

```
mettail-rust/                    # Root workspace
│
├── Cargo.toml                   # Workspace manifest
├── README.md
├── LICENSE
├── CHANGELOG.md
│
├── macros/                      # Proc-macro crate (was: mettail-macros)
│   ├── src/
│   │   ├── lib.rs              # theory! macro
│   │   ├── ast.rs
│   │   ├── codegen.rs
│   │   └── ...
│   ├── tests/
│   └── Cargo.toml              # [lib] proc-macro = true
│
├── runtime/                     # Runtime support (was: mettail-runtime)
│   ├── src/
│   │   ├── lib.rs              # HashBag, Scope, OrdVar
│   │   ├── hashbag.rs
│   │   └── ...
│   └── Cargo.toml              # Regular library
│
├── theories/                    # Theory definitions (was: examples)
│   ├── src/
│   │   ├── lib.rs              # Re-exports all theories
│   │   ├── rhocalc.rs          # RhoCalc theory + trait impl
│   │   ├── ambient.rs          # Ambient theory + trait impl
│   │   ├── spacecalc.rs        # Space theory + trait impl
│   │   └── generated/          # LALRPOP generated files
│   │       ├── rhocalc.lalrpop
│   │       ├── ambient.lalrpop
│   │       └── spacecalc.lalrpop
│   ├── build.rs                # LALRPOP build script
│   └── Cargo.toml
│
├── repl/                        # REPL application (was: mettail-repl)
│   ├── src/
│   │   ├── main.rs
│   │   ├── repl.rs
│   │   ├── theory.rs           # Theory trait
│   │   ├── registry.rs
│   │   ├── state.rs
│   │   ├── examples.rs
│   │   └── pretty.rs           # ← Stays here (formatting logic)
│   └── Cargo.toml              # Binary crate
│
├── examples/                    # Standalone example binaries
│   ├── rhocalc_simple.rs       # Basic RhoCalc demo
│   ├── ambient_demo.rs         # Basic Ambient demo
│   └── custom_theory.rs        # How to define your own
│
├── benches/                     # Future: workspace benchmarks
│   └── theory_performance.rs
│
└── docs/
    ├── guide/                   # User guides
    ├── design/                  # Design documents
    └── archive/                 # Development history
```

---

## 🔍 Key Changes Explained

### 1. Simplified Naming (No "mettail-" Prefix)

**Before:**
```
mettail-macros/
mettail-runtime/
mettail-examples/
mettail-repl/
```

**After:**
```
macros/         # Clear from context (in mettail-rust workspace)
runtime/        # Clear from context
theories/       # Descriptive name
repl/           # Clear from context
```

**Why:** 
- Workspace name provides context
- Internal crates don't need prefixing
- Shorter paths, cleaner structure
- Still use full names in Cargo.toml if publishing

**Publishing Note:**
```toml
# In macros/Cargo.toml - use full name for crates.io
[package]
name = "mettail-macros"  # Published name
```

---

### 2. Why `runtime/` is Separate from `macros/`

**Critical Rust Constraint:**

```rust
// macros/Cargo.toml
[lib]
proc-macro = true  // ← This means: ONLY export procedural macros
```

**Proc-macro crates cannot export:**
- ❌ Regular types (`struct`, `enum`)
- ❌ Regular functions
- ❌ Traits
- ❌ Constants
- ✅ Only: Procedural macros (`#[proc_macro]`, `#[derive]`, etc.)

**Why we need `runtime/` separate:**

```rust
// Generated code needs these types:
use mettail_runtime::HashBag;      // ← Must come from regular crate
use mettail_runtime::Scope;        // ← Cannot be in proc-macro crate
use mettail_runtime::OrdVar;       // ← Rust restriction

// The macro generates code that uses runtime types:
theory! {
    name: RhoCalc,
    terms {
        PPar . Proc ::= HashBag(Proc) sep "|" ;
        //              ^^^^^^^ Needs HashBag type from runtime
    }
}
```

**Solution:**
- `macros/` - Generates code (proc-macro crate)
- `runtime/` - Provides types used by generated code (regular library)
- Generated code imports from `runtime/`

---

### 3. `theories/` Organization

```
theories/
├── src/
│   ├── lib.rs                # Public API + re-exports
│   │   pub mod rhocalc;
│   │   pub mod ambient;
│   │   pub use rhocalc::*;
│   │   pub use ambient::*;
│   │
│   ├── rhocalc.rs            # Full theory implementation
│   │   use macros::theory;
│   │   
│   │   theory! { name: RhoCalc, ... }
│   │   
│   │   impl Theory for RhoCalc { ... }
│   │
│   ├── ambient.rs            # Full theory implementation
│   └── spacecalc.rs
│
└── generated/                # LALRPOP grammar files
    ├── rhocalc.lalrpop       # Moved from root src/
    ├── ambient.lalrpop
    └── spacecalc.lalrpop
```

**Key Points:**
- Each theory in its own module file
- Grammars in `generated/` subdirectory (clear purpose)
- Single source of truth per theory
- No duplication with REPL

---

### 4. `pretty.rs` Stays in `repl/`

**Why it was duplicated:**
```
examples/src/pretty.rs    # Original
repl/src/pretty.rs        # Copy due to circular dependency
```

**Issue:** 
- `theories/` (was `examples/`) had pretty formatting
- `repl/` needed formatting
- Importing from `theories` → `repl` → `theories` creates cycle

**Solution:**
```
repl/src/pretty.rs        # Formatting logic stays here
```

**Rationale:**
- Pretty printing is presentation logic
- REPL is the presentation layer
- Theories are data/behavior only
- No circular dependency
- Clear separation of concerns

**Usage:**
```rust
// theories/src/rhocalc.rs
theory! { name: RhoCalc, ... }
// No pretty printing here - just Display trait

// repl/src/pretty.rs
pub fn format_term_pretty(s: &str) -> String {
    // REPL-specific formatting
}

// examples/rhocalc_simple.rs
use repl::pretty::format_term_pretty;  // If needed
```

---

### 5. Generated Files Organization

**LALRPOP Grammar Files:**

```
theories/generated/           # Clear: these are generated
├── rhocalc.lalrpop          # Input to LALRPOP
├── ambient.lalrpop
└── ...
```

**Build Process:**
```toml
# theories/build.rs
lalrpop::process_root()  // Processes theories/generated/*.lalrpop
                         // Outputs to OUT_DIR (target/...)
```

**Why `generated/` subdirectory:**
- ✅ Clear these are inputs to code generation
- ✅ Separate from hand-written Rust code
- ✅ Easy to .gitignore if needed
- ✅ Organized and predictable

---

### 6. Examples Directory Usage

**Standalone Demos:**
```
examples/
├── rhocalc_simple.rs        # Minimal RhoCalc example
├── ambient_demo.rs          # Minimal Ambient example
└── custom_theory.rs         # Tutorial: define your own theory
```

**Purpose:**
- Show how to use the `theory!` macro
- Demonstrate basic usage patterns
- Keep simple (< 100 lines each)
- Not for complex tests or benchmarks

**Note:** May not be heavily used initially, but good for:
- Documentation examples
- Tutorial code
- Quick smoke tests

---

## 🔄 Migration Steps

### Step 1: Rename Crates

```bash
# Remove prefix from directory names
git mv mettail-macros macros
git mv mettail-runtime runtime
git mv examples theories
git mv mettail-repl repl

# Update workspace Cargo.toml
# members = ["macros", "runtime", "theories", "repl"]
```

### Step 2: Reorganize `theories/`

```bash
cd theories

# Create generated/ subdirectory
mkdir src/generated

# Move LALRPOP files
git mv src/*.lalrpop src/generated/

# Move theory implementations (if separate files exist)
# Currently they're inline in binary files, so will need refactoring
```

### Step 3: Update Build Scripts

```rust
// theories/build.rs
fn main() {
    // Process LALRPOP files from generated/ subdirectory
    lalrpop::Configuration::new()
        .set_in_dir("src/generated")
        .process()
        .unwrap();
}
```

### Step 4: Update Imports

```bash
# Find and replace across codebase
mettail_macros   → mettail::macros   (or just `macros` internally)
mettail_runtime  → mettail::runtime
mettail_examples → mettail::theories
mettail_repl     → mettail::repl
```

### Step 5: Consolidate Theories

**Before:**
```
examples/rhocalc.rs          # Inline theory definition
repl/src/rhocalc_theory.rs   # Duplicate theory + trait impl
```

**After:**
```rust
// theories/src/rhocalc.rs
use macros::theory;
use crate::Theory;  // Import trait from theories/src/lib.rs

theory! {
    name: RhoCalc,
    // ... full definition
}

impl Theory for RhoCalc {
    // ... trait implementation
}
```

```rust
// examples/rhocalc_simple.rs
use mettail::theories::rhocalc::*;

fn main() {
    let parser = rhocalc::ProcParser::new();
    // ...
}
```

### Step 6: Remove Duplication

```bash
# Delete duplicate files
rm repl/src/rhocalc_theory.rs
rm repl/src/ambcalc_theory.rs
rm repl/src/*.lalrpop

# Update repl to import from theories
```

---

## 📦 Cargo.toml Updates

### Root Workspace

```toml
# Cargo.toml (root)
[workspace]
members = ["macros", "runtime", "theories", "repl"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["MeTTaIL Contributors"]
license = "MIT"
repository = "https://github.com/cbwells/mettail-rust"

[workspace.dependencies]
# Core MeTTaIL (internal)
mettail-macros = { path = "./macros" }
mettail-runtime = { path = "./runtime" }
mettail-theories = { path = "./theories" }

# External dependencies
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full", "extra-traits"] }
proc-macro-error = "1.0"
lalrpop = "0.20"
lalrpop-util = "0.20"
ascent = { version = "0.8" }
ascent-byods-rels = { version = "0.8" }
moniker = { version = "0.5", features = ["moniker-derive"] }
# ... etc
```

### Individual Crates

```toml
# macros/Cargo.toml
[package]
name = "mettail-macros"  # Full name for potential publishing
version.workspace = true
edition.workspace = true

[lib]
proc-macro = true

[dependencies]
proc-macro2.workspace = true
quote.workspace = true
syn.workspace = true
# ...
```

```toml
# runtime/Cargo.toml
[package]
name = "mettail-runtime"
version.workspace = true
edition.workspace = true

[dependencies]
moniker.workspace = true
lalrpop-util.workspace = true
# ...
```

```toml
# theories/Cargo.toml
[package]
name = "mettail-theories"
version.workspace = true
edition.workspace = true

[dependencies]
mettail-macros.workspace = true
mettail-runtime.workspace = true
ascent.workspace = true
ascent-byods-rels.workspace = true
lalrpop-util.workspace = true

[build-dependencies]
lalrpop.workspace = true
```

```toml
# repl/Cargo.toml
[package]
name = "mettail-repl"
version.workspace = true
edition.workspace = true

[[bin]]
name = "mettail"
path = "src/main.rs"

[dependencies]
mettail-runtime.workspace = true
mettail-theories.workspace = true
mettail-macros.workspace = true
# CLI deps
rustyline = "14.0"
clap = { version = "4.5", features = ["derive"] }
colored = "2.1"
# ...
```

---

## 📊 Comparison: Before vs After

### Directory Structure

| Before                    | After              | Change               |
|---------------------------|--------------------|----------------------|
| `mettail-macros/`         | `macros/`          | Simpler name         |
| `mettail-runtime/`        | `runtime/`         | Simpler name         |
| `examples/`               | `theories/`        | Accurate name        |
| `mettail-repl/`           | `repl/`            | Simpler name         |
| `examples/src/*.lalrpop`  | `generated/*.lalrpop` | Clear organization |
| `examples/src/pretty.rs`  | Deleted            | Removed duplication  |
| REPL has duplicate theories | REPL imports from `theories/` | Single source |

### File Organization

**Before:**
```
examples/
├── rhocalc.rs              # Binary with inline theory
├── ambient.rs              # Binary with inline theory
├── src/
│   ├── lib.rs
│   ├── pretty.rs           # Duplicated in REPL
│   ├── rhocalc.lalrpop     # Grammar
│   └── ambient.lalrpop     # Grammar

repl/src/
├── rhocalc_theory.rs       # Duplicate theory
├── rhocalc.lalrpop         # Duplicate grammar
├── ambcalc_theory.rs       # Duplicate theory
└── ambcalc.lalrpop         # Duplicate grammar
```

**After:**
```
theories/
├── src/
│   ├── lib.rs              # Re-exports
│   ├── rhocalc.rs          # Single source
│   ├── ambient.rs          # Single source
│   └── generated/
│       ├── rhocalc.lalrpop
│       └── ambient.lalrpop

repl/src/
├── main.rs
├── repl.rs
├── theory.rs               # Trait definition
└── pretty.rs               # Presentation logic
```

---

## ✅ Benefits of New Structure

### 1. Clarity
- ✅ Crate names match their purpose
- ✅ No redundant prefixes
- ✅ Clear what goes where

### 2. No Duplication
- ✅ One theory definition per theory
- ✅ One grammar file per theory
- ✅ Single source of truth

### 3. Clean Dependencies
```
macros  ← runtime
  ↑
theories ← runtime, macros
  ↑
repl ← theories, runtime
```
- ✅ Linear dependency chain
- ✅ No circular dependencies
- ✅ Clear separation of concerns

### 4. Standard Patterns
- ✅ Follows Rust workspace conventions
- ✅ Generated files clearly marked
- ✅ Examples in standard location
- ✅ Intuitive for contributors

### 5. Scalability
- ✅ Easy to add new theories
- ✅ Easy to add new examples
- ✅ Clear where everything goes
- ✅ Minimal maintenance burden

---

## 🎓 Architecture Explanation

### Why This Structure?

```
┌─────────────────────────────────────────────────────┐
│  macros/              proc-macro crate              │
│  - Defines theory! macro                            │
│  - Cannot export types (Rust limitation)            │
└─────────────────────────────────────────────────────┘
                         │
                         │ uses types from
                         ▼
┌─────────────────────────────────────────────────────┐
│  runtime/             regular library               │
│  - HashBag, Scope, OrdVar types                     │
│  - Used by generated code                           │
└─────────────────────────────────────────────────────┘
                         │
                         │ imported by
                         ▼
┌─────────────────────────────────────────────────────┐
│  theories/            theory definitions            │
│  - Uses theory! macro                               │
│  - Generates code that uses runtime types           │
│  - One file per theory                              │
└─────────────────────────────────────────────────────┘
                         │
                         │ imported by
                         ▼
┌─────────────────────────────────────────────────────┐
│  repl/                REPL application              │
│  - Interactive interface                            │
│  - Uses theories for computation                    │
│  - Presentation logic (pretty printing)             │
└─────────────────────────────────────────────────────┘
```

### Key Insights

1. **Macro/Runtime Split is Mandatory**
   - Rust compiler requirement
   - Cannot be changed without language changes
   - Clean separation of code generation from runtime

2. **Theories as Library**
   - Reusable across applications
   - Can be used in REPL, examples, tests, benchmarks
   - Single place to define and maintain

3. **REPL is Consumer**
   - Depends on theories
   - Adds presentation layer
   - No business logic duplication

---

## 📝 Notes

### Publishing to crates.io

If publishing, keep full names:
```toml
[package]
name = "mettail-macros"      # Public name
path = "macros"              # Local path
```

### Backward Compatibility

Old imports will break:
```rust
// Old (breaks)
use mettail_examples::rhocalc;

// New
use mettail_theories::rhocalc;
```

**Solution:** Version bump (0.1 → 0.2) or re-export wrappers

### Documentation Updates Needed

- [ ] Update README with new structure
- [ ] Update REPL guide with new imports
- [ ] Update examples in docs
- [ ] Update architecture diagrams

---

## 🚀 Ready to Implement?

The structure is designed to be:
- ✅ Clean and intuitive
- ✅ Follow Rust best practices
- ✅ Eliminate all duplication
- ✅ Scale for future growth
- ✅ Respect Rust's technical constraints

Next step: Execute the migration! 🎯

