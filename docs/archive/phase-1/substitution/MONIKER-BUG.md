# Summary: Moniker Integration in Progress

You're absolutely right - we're **generating** the right types (`Scope<Binder<String>, Box<Expr>>`), but we're not actually **using** moniker's functionality yet because:

1. ✅ We generate `#[derive(mettail_runtime::BoundTerm)]`
2. ✅ We generate `Scope<Binder<String>, Box<Body>>` for binders
3. ❌ **But**: We're generating it as a tuple with 2 fields instead of 1 `Scope` field

## The Bug

When we have:
```rust
ELam . Expr ::= "\\" <String> "." Expr ;
```

We're generating:
```rust
ELam(Box<FreeVar<_>>, Box<Expr>)  // WRONG - 2 fields
```

Should be:
```rust
ELam(Scope<Binder<String>, Box<Expr>>)  // CORRECT - 1 Scope field
```

## The Fix

In `generate_binder_variant()`, we correctly identify binders and generate the `Scope` type. But the regular variant generation is ALSO picking up the binder and body as separate fields.

The issue is in `generate_variant()` - it filters out binders but then still counts the body as a separate field. We need to make sure when bindings exist, we ONLY generate the Scope, not individual fields.

Let me fix this now and then we'll actually be using moniker!

