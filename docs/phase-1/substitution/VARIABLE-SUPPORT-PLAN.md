# Variable Support Implementation Plan

## Goal
Add `Var` as a special built-in category that generates `mettail_runtime::Var<String>` variants.

## Syntax

```rust
theory! {
    name: LambdaCalc,
    exports { Expr },
    terms {
        EVar . Expr ::= Var ;              // NEW: Var is a special keyword
        ELam . Expr ::= "\\" <Var> "." Expr ;  // <Var> indicates we're binding a Var
        EApp . Expr ::= Expr Expr ;
    }
}
```

## Generated Code

```rust
#[derive(Debug, Clone, mettail_runtime::BoundTerm)]
pub enum Expr {
    EVar(mettail_runtime::Var<String>),  // ← Variable variant
    ELam(mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Expr>>),
    EApp(Box<Expr>, Box<Expr>),
}
```

## Implementation Steps

### 1. Recognize `Var` as Special Keyword
In `ast.rs` parser, when we see `NonTerminal(ident)` where `ident == "Var"`, treat it specially.

### 2. Update Codegen
In `generate_variant()`, check if a field is `Var` and generate:
```rust
Label(mettail_runtime::Var<String>)
```
Instead of:
```rust
Label(Box<Var>)
```

### 3. No Need for Type Checking
`Var` is polymorphic - can be any category. The binder specifies what it binds.

### 4. Test
```rust
theory! {
    name: Test,
    exports { Expr },
    terms {
        EVar . Expr ::= Var ;
        ELam . Expr ::= "\\" <Var> "." Expr ;
    }
}

// Usage:
use mettail_runtime::{FreeVar, Binder, Scope, Var};

let x = FreeVar::fresh_named("x");
let var_expr = Expr::EVar(Var::Free(x.clone()));
let lambda = Expr::ELam(Scope::new(
    Binder(x.clone()),
    Box::new(var_expr)
));
```

## Why This Works

In moniker:
- `Var<String>` can be `Var::Free(FreeVar<String>)` or `Var::Bound(BoundVar<String>)`
- When we create `Scope::new(Binder(x), body)`, it automatically closes free vars in body
- `Var::Free(x)` in body becomes `Var::Bound(@0.0)` 
- When we `unbind()`, it opens back to fresh `Var::Free(x')`

This gives us automatic capture-avoidance!

## Start Implementation

Let's begin by making `Var` a recognized keyword in the grammar parser.

