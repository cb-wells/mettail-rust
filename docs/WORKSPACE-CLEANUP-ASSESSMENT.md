# Workspace Cleanup Assessment

## Executive Summary

The workspace needs several cleanup actions before pushing:

1. ✅ **Examples directory is fine** - workspace member is appropriate
2. ⚠️ **Generated files are massive** (6536 lines) - need better separation
3. ❌ **Large dependencies** - moniker (65MB) and ascent (17MB) should not be in repo
4. ⚠️ **Missing .gitignore entries** - these folders aren't ignored
5. ✅ **Hash trait issue** - can be solved without forking moniker

## Detailed Analysis & Recommendations

### 1. Examples Directory Structure ✅ GOOD

**Current State:**
```
examples/
├── Cargo.toml         # Workspace member
├── rhocalc.rs         # Example binary
├── test_var_identity.rs # Test binary
└── src/
    ├── rho_gen.rs     # Generated (6536 lines!)
    └── rhocalc.lalrpop # Source grammar
```

**Assessment:** ✅ This structure is **correct** for Rust workspaces.

**Reasoning:**
- Examples as a workspace member is standard practice (similar to ascent/, egglog/)
- Allows examples to have their own dependencies
- Can be run independently: `cargo run --bin rhocalc`
- Good for testing the macro system

**Recommendation:** **Keep as-is**. This is proper Rust workspace structure.

---

### 2. Generated File Length ⚠️ NEEDS FIX

**Current State:**
```
examples/src/rho_gen.rs - 6,536 lines!
```

**Problem:** LALRPOP generates a huge parser module that bloats the file.

**Root Cause:** Line 2 of `examples/rhocalc.rs`:
```rust
lalrpop_mod!(pub rhocalc);  // Inline LALRPOP module
```

This macro includes the entire ~6000 line LALRPOP parser module inline.

**Solution: Separate LALRPOP Output**

**Option A: Keep Generated Files Out of Git** (RECOMMENDED)
```rust
// examples/rhocalc.rs
use lalrpop_util::lalrpop_mod;

// Generate in target/ instead of src/
lalrpop_mod!(
    #[allow(clippy::all)]
    pub rhocalc,
    "/rhocalc.rs"  // Will be in OUT_DIR
);
```

Update `examples/build.rs`:
```rust
fn main() {
    // Generate grammar in OUT_DIR (not src/)
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();
}
```

Add to `.gitignore`:
```gitignore
# Generated LALRPOP parsers
examples/src/*_gen.rs
**/lalrpop_out/
```

**Benefits:**
- Generated files not in version control (standard practice)
- Smaller git diffs
- No merge conflicts on generated code
- Cleaner `cargo build` experience

**Option B: Separate Grammar File** (If you must commit generated code)
```
examples/
├── src/
│   ├── grammar/
│   │   ├── mod.rs
│   │   └── rhocalc.rs  # Generated parser
│   └── rhocalc.lalrpop
└── rhocalc.rs  # Main example
```

**Recommendation:** Use **Option A** - don't commit generated code.

---

### 3. Moniker Dependency ❌ CRITICAL

**Current State:**
- Local moniker fork: 65MB in repo
- Only for Hash trait implementations
- Found in `moniker/moniker/src/free_var.rs` and `bound_var.rs`

**Problem:** Forking an entire library for 2 trait impls is excessive.

**Solution: Newtype Pattern** (CLEAN & IDIOMATIC)

Instead of modifying moniker, wrap it:

```rust
// mettail-runtime/src/lib.rs

use moniker::FreeVar as MonikerFreeVar;
use std::hash::{Hash, Hasher};

/// Wrapper around moniker::FreeVar that adds Hash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeVar<N>(pub MonikerFreeVar<N>);

impl<N: Hash> Hash for FreeVar<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.unique_id.hash(state);
    }
}

// Deref for convenience
impl<N> std::ops::Deref for FreeVar<N> {
    type Target = MonikerFreeVar<N>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Similar for BoundVar
pub struct BoundVar<N>(pub moniker::BoundVar<N>);
impl<N: Hash> Hash for BoundVar<N> { /* ... */ }
```

**Benefits:**
- ✅ Use official moniker crate from crates.io
- ✅ No fork maintenance burden
- ✅ 65MB removed from repo
- ✅ Standard Rust pattern (newtype)
- ✅ Future moniker updates work seamlessly

**Update Dependencies:**
```toml
# mettail-runtime/Cargo.toml
[dependencies]
moniker = "0.5"  # Official crate, not path dependency
```

**Migration:**
- Replace `moniker::FreeVar` with `mettail_runtime::FreeVar` in generated code
- Add `.0` when passing to moniker functions

**Recommendation:** **Implement newtype pattern** and remove moniker/ directory.

---

### 4. Ascent Dependency ❌ CRITICAL

**Current State:**
- Local ascent fork: 17MB in repo
- Used only in `examples/rhocalc.rs` for Datalog queries

**Problem:** Another large dependency in the repo.

**Solution: Use Official Crate**

Check if you made any modifications:
```bash
cd ascent && git diff
```

**If no modifications:**
```toml
# workspace Cargo.toml
[workspace.dependencies]
ascent = "0.8"  # Use crates.io version

# examples/Cargo.toml
[dependencies]
ascent = { workspace = true }
```

Remove `ascent/` directory and add to `.gitignore`:
```gitignore
/ascent/
```

**If modifications exist:**
- Document what changes were made
- Consider upstreaming via PR to ascent
- Or: Use a published fork on crates.io

**Recommendation:** **Remove ascent/** and use official crate.

---

### 5. .gitignore Updates ⚠️ NEEDS FIX

**Add to `.gitignore`:**
```gitignore
# Large dependency directories (use crates.io instead)
/moniker/
/ascent/
/egglog/

# Generated parser files
examples/src/*_gen.rs
**/lalrpop_out/

# Build artifacts
**/target/
```

**Current .gitignore already has:**
```gitignore
/k/         # ✅
/ascent/    # ✅ Already there but commented out?
```

Wait, let me check the actual .gitignore:

Looking at line 7: `/ascent/` is already there! So why is it in the repo?

**Action:** Verify git tracking:
```bash
git rm -r --cached ascent/ moniker/
git commit -m "Remove large dependencies from repo"
```

---

### 6. Other Cleanup Items

#### A. Remove Obsolete Directories

Check if these are needed:
- `mettail-macros/wip/` - already in .gitignore ✅
- `mettail-macros/target/test_grammars/` - build artifact
- `examples/src/rhocalc.lalrpop` - source grammar (keep)

**Action:**
```gitignore
# Build and test artifacts
**/target/
```

#### B. Consolidate Documentation

**Current docs structure:**
```
docs/
├── phase-1/     # Historical
├── phase-2/     # Historical
├── design/      # Active
├── VAR-IDENTITY-SOLUTION.md
└── VARIABLE-EQUALITY-IMPLEMENTATION.md
```

**Recommendation:**
- Move recent docs to `docs/implementation/`
- Archive phase-1, phase-2 to `docs/archive/` or remove

#### C. Clean Up Root Directory

**Current:**
```
QUICKSTART.md
README.md
DEMO.md
CHANGELOG.md
```

**Recommendation:** Good as-is, but ensure they're up to date.

---

## Implementation Plan

### Priority 1: Remove Large Dependencies (1-2 hours)

1. **Implement newtype pattern for moniker:**
   ```bash
   # In mettail-runtime/src/lib.rs
   # Add FreeVar/BoundVar wrappers with Hash
   ```

2. **Update Cargo.toml:**
   ```toml
   moniker = "0.5"  # Use crates.io
   ascent = "0.8"   # Use crates.io
   ```

3. **Remove directories:**
   ```bash
   git rm -r moniker/ ascent/
   ```

4. **Update .gitignore:**
   ```gitignore
   /moniker/
   /ascent/
   ```

### Priority 2: Fix Generated Files (30 mins)

1. **Update build.rs** to put generated code in OUT_DIR
2. **Add to .gitignore:** `examples/src/*_gen.rs`
3. **Remove checked-in generated file:**
   ```bash
   git rm examples/src/rho_gen.rs
   ```

### Priority 3: Documentation Cleanup (15 mins)

1. Create `docs/implementation/` directory
2. Move recent docs there
3. Update README with new structure

---

## Expected Outcomes

**Before:**
- Repo size: ~85MB (including .git)
- Files tracked: 6500+ line generated files
- Dependencies: Forked libraries

**After:**
- Repo size: ~2MB (just source)
- Files tracked: Source only
- Dependencies: Official crates
- Build time: Same or faster (crates.io caching)

---

## Files to Modify

### High Priority
1. `mettail-runtime/src/lib.rs` - Add newtype wrappers
2. `mettail-runtime/Cargo.toml` - Use official moniker
3. `examples/Cargo.toml` - Use official ascent
4. `Cargo.toml` (workspace) - Update dependencies
5. `.gitignore` - Add exclusions
6. Remove: `moniker/`, `ascent/`, `examples/src/rho_gen.rs`

### Medium Priority
7. `examples/build.rs` - Generate to OUT_DIR
8. `examples/rhocalc.rs` - Update lalrpop_mod! call
9. Documentation structure

---

## Questions to Clarify

1. **Ascent modifications:** Did you modify ascent, or can we use the official version?
2. **Generated files:** Do you want them in version control? (Usually no)
3. **Documentation:** Keep phase-1/phase-2 history or archive?
4. **egglog/**: Is this directory supposed to exist? (mentioned in .gitignore line 6)

---

## Summary

**Critical Actions:**
1. ❌ **Remove moniker/ and ascent/** - Use official crates with newtype pattern
2. ⚠️ **Don't commit generated files** - Use OUT_DIR for LALRPOP output
3. ✅ **Examples structure is fine** - Keep as workspace member

**Estimated Time:** 2-3 hours for full cleanup

**Estimated Repo Size Reduction:** ~80MB

Would you like me to start implementing these changes?

