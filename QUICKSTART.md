# Quick Start Guide

Welcome to MeTTaIL! This guide will get you up and running in under 5 minutes.

## Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/mettail-rust.git
cd mettail-rust

# Build the project
cargo build

# Run tests to verify everything works
cargo test
```

## Your First Theory

Create a simple theory to see MeTTaIL in action:

```rust
use mettail_macros::theory;

theory! {
    name: SimpleLambda,
    exports { Term }
    
    terms {
        Var . Term ::= Var ;
        Lam . Term ::= "\\" "(" Term <Term> ")" ;
        App . Term ::= Term Term ;
    }
}

fn main() {
    // Create terms using the generated AST
    let x = SimpleLambda::Term::Var(Box::new(/* ... */));
    println!("Created a lambda term!");
}
```

## Explore the Examples

MeTTaIL includes working examples with full execution:

```bash
# Run the Rho Calculus rewrite demo
cargo run --bin rhocalc

# Output:
# === Rho Calculus Rewrite Demo ===
# 
# Input:  for(a<-x){*x}|a!(0)
# 
# Step 1: *@(0)
# 
# → Normal form reached after 1 step(s)

# See generated code (AST + parser + substitution + rewrite engine)
cargo expand --bin rhocalc > output.rs
```

## Project Structure

```
mettail-rust/
├── mettail-macros/     # Procedural macros (theory! macro)
├── mettail-runtime/    # Runtime support (BoundTerm trait, etc.)
├── examples/           # Example theories
├── theories/           # Theory definitions (Rho Calculus, etc.)
└── docs/              # Comprehensive documentation
```

## Next Steps

1. **Read the [README.md](README.md)** - High-level overview and features
2. **Check out [examples/lambda_calc.rs](examples/lambda_calc.rs)** - Simple example
3. **Review [theories/rhocalc.rs](theories/rhocalc.rs)** - Complex example with rewrites
4. **Read the [ROADMAP.md](docs/ROADMAP.md)** - Understand the project vision

## Common Commands

```bash
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Check code without building
cargo check --workspace

# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features

# Generate documentation
cargo doc --open

# Expand macros to see generated code
cargo expand -p mettail-examples --bin rhocalc
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/your-username/mettail-rust/issues)
- **Discussions**: For questions and ideas
- **Documentation**: See the `docs/` directory

## Contributing

Ready to contribute? Check out:
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [docs/design/REMAINING-ISSUES.md](docs/design/REMAINING-ISSUES.md) - Known issues
- [docs/ROADMAP.md](docs/ROADMAP.md) - Phase 2 priorities

## Current Status

**Phase 1**: ✅ Complete - Theory definition, type-checking, substitution  
**Phase 2**: ✅ Complete - Parser generation, rewrite engine, execution  
**Phase 3**: 📋 Planned - Theory composition  
**Phase 4**: 💡 Future - E-graph integration  

**Latest:** Fully functional rewrite engine with pattern matching, freshness checking, and capture-avoiding substitution! 🎉

---

Welcome aboard! 🚀

