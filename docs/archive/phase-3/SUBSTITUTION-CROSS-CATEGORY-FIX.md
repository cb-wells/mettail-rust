# Cross-Category Substitution Generation Fix

## Problem

When generating substitution methods, the code generator was computing substitutable categories **per-category** rather than globally. This caused issues when:

1. Category A has fields of type B (e.g., `Proc` has `Name` fields)
2. Category A's `substitute` method tries to call `B::substitute_a` on those fields
3. Category B has no constructors that contain A (e.g., `Name` has no constructors with `Proc` fields)
4. The `B::substitute_a` method wasn't being generated, causing compilation errors

### Example

In the Ambient Calculus:
- `Proc` constructors like `PIn(Name, Proc)` have `Name` fields
- `Proc::substitute` calls `Name::substitute_proc` on those fields
- But `Name` only has `NVar(Var)` - no constructors containing `Proc`
- So `Name::substitute_proc` wasn't being generated

The user had to add a dummy `NQuote(Proc)` constructor just to make the code compile.

## Root Cause

In `subst_gen.rs`, the `find_substitutable_categories` function was called with only the rules for the **current category**:

```rust
let rules: Vec<&GrammarRule> = theory.terms
    .iter()
    .filter(|r| r.category == *category)  // Only current category!
    .collect();

let subst_cats = find_substitutable_categories(&rules);
```

This meant:
- When generating methods for `Name`, only `Name`'s constructors were examined
- Since `Name` has no constructors with `Proc` fields, `Proc` wasn't in `subst_cats`
- So `Name::substitute_proc` wasn't generated

## Solution

Changed to compute substitutable categories **globally** for the entire theory:

```rust
// Find all categories that appear anywhere in the theory
let all_subst_cats = find_all_substitutable_categories(&theory.terms);

let impls: Vec<TokenStream> = theory.exports.iter().map(|export| {
    // ...
    generate_category_substitution(category, &rules, &all_subst_cats)
}).collect();
```

The new `find_all_substitutable_categories` function:
- Takes **all** rules from the entire theory
- Collects all categories that appear anywhere (as fields, binders, or rule categories)
- Ensures every exported category gets `substitute_X` methods for all other categories

## Result

Now every exported category gets cross-category substitution methods for all other exported categories:

- `Name::substitute_proc` is generated (even though `Name` has no `Proc` fields)
- It's a simple no-op that returns `self.clone()` for all constructors
- But it exists, so `Proc::substitute` can call it without compilation errors

### Generated Code Example

```rust
impl Name {
    pub fn substitute_proc(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Proc,
    ) -> Self {
        match self {
            Name::NVar(_) => self.clone(),  // No Proc fields, just clone
        }
    }
}
```

This is semantically correct: since `Name` can't contain `Proc`, substituting `Proc` has no effect.

## Benefits

1. **No dummy constructors needed**: Users don't need workarounds like `NQuote`
2. **Consistent API**: All categories have `substitute_X` methods for all other categories
3. **Type safety**: The compiler ensures all cross-category calls are valid
4. **Simplicity**: The generated no-op methods are trivial and compile away efficiently

## Trade-offs

- Generates more methods (one for each category pair)
- Some methods are no-ops that could theoretically be avoided
- But the cost is minimal: simple match statements that clone `self`
- The benefit of API consistency and avoiding compilation errors outweighs this

