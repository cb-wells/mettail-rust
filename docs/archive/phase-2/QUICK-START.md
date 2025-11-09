# Quick Start: Continuing Phase 2 Work

## Current Status
✅ **Parsing tests passing:** 6/6 Rho Calculus tests  
⏳ **Next:** Precedence handling & pretty-printing

## Run Tests
```bash
# Parsing tests
cargo test --test rhocalc_parsing_tests -- --nocapture

# Grammar generation tests
cargo test --package mettail-macros --lib lalrpop_gen -- --nocapture

# All mettail-runtime tests
cargo test --package mettail-runtime
```

## Key Files to Edit

### For Precedence Fixes
- `mettail-macros/src/lalrpop_gen.rs` - Grammar generation
  - Need to detect infix operators
  - Generate tiered rules (Expr → Factor → Term)
  - Handle associativity

### For Pretty-Printing
- `mettail-macros/src/codegen.rs` - Add `Display` impl generation
  - Walk through AST constructors
  - Generate match arms for each variant
  - Handle binders specially

### For Testing
- `mettail-runtime/tests/rhocalc_parsing_tests.rs` - Add more tests
- `theories/rhocalc.rs` - Full theory definition (currently has precedence issues)

## Example: Adding Precedence

```rust
// In lalrpop_gen.rs
fn generate_precedence_tiers(rules: &[GrammarRule]) -> Vec<PrecedenceTier> {
    // Group rules by precedence
    // Infix operators → lower tier
    // Prefix operators → higher tier
    // Atoms → highest tier
}

// Generated LALRPOP:
pub Proc: Proc = {
    <ParProc>,
};

ParProc: Proc = {
    <ParProc> "|" <AtomProc> => Proc::PPar(...),
    <AtomProc>,
};

AtomProc: Proc = {
    "0" => Proc::PZero,
    "*" <Name> => Proc::PDrop(...),
    "(" <Proc> ")",
};
```

## Example: Pretty-Printing

```rust
// In codegen.rs - generate_ast()
impl std::fmt::Display for Proc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Proc::PZero => write!(f, "0"),
            Proc::PDrop(n) => write!(f, "*{}", n),
            Proc::POutput(n, p) => write!(f, "{}!({})", n, p),
            Proc::PPar(p, q) => write!(f, "{} | {}", p, q),
            Proc::PInput(chan, scope) => {
                let (binder, body) = scope.clone().unbind();
                write!(f, "for({} {}){{{}}}", chan, binder.0, body)
            }
        }
    }
}
```

## Debugging Tips

### LALRPOP Errors
```bash
# See full error output
cargo build --package mettail-runtime 2>&1 | less

# Check generated parsers
ls target/debug/build/mettail-runtime-*/out/

# View generated parser
cat target/debug/build/mettail-runtime-*/out/rhocalc_simple.rs | less
```

### Grammar Files
```bash
# See test grammars
ls mettail-macros/target/test_grammars/

# View generated grammar
cat mettail-macros/target/test_grammars/lambda.lalrpop
```

### Force Rebuild
```bash
# Clean and rebuild
cargo clean -p mettail-runtime
cargo build --package mettail-runtime

# Or touch build script
touch mettail-runtime/build.rs
cargo build --package mettail-runtime
```

## TODOs

- [ ] **Precedence:** Implement tiered grammar rules
- [ ] **Parentheses:** Add grouping support
- [ ] **Full Rho:** Test `for(x y){P}` and `P|Q`
- [ ] **Display:** Generate pretty-printing
- [ ] **Round-trip:** `parse(display(ast)) == ast`

## References

- **LALRPOP Book:** https://lalrpop.github.io/lalrpop/
- **Precedence:** https://lalrpop.github.io/lalrpop/tutorial/005_building_ASTs.html
- **Binders:** See `mettail-macros/src/lalrpop_gen.rs:generate_binder_alternative()`

