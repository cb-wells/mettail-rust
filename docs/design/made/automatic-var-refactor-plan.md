# Automatic Var Variant Refactor Plan

## Problem Statement

The codebase was refactored to automatically generate Var variants for each exported category when they're not explicitly defined in the theory. However, the substitution and display code generation modules (`subst.rs` and `display.rs`) were not updated to handle these automatically-generated variants, causing non-exhaustive pattern match errors.

## Current State

### What Works
- **AST Generation** (`ast_gen.rs`): Automatically generates Var variants when missing
  - Naming convention: First letter of category + "Var" (e.g., `Proc` → `PVar`, `Name` → `NVar`)
  - Variant type: `VarLabel(mettail_runtime::OrdVar)`
  - Detection: Uses `is_var_rule()` to check if Var rule already exists

### What's Broken
- **Substitution** (`subst.rs`): Only generates match arms for rules in `theory.terms`
  - Missing match arm for auto-generated Var variants
  - `is_var_constructor()` only detects explicit Var rules, not auto-generated ones
  - Non-exhaustive pattern match errors in generated code

- **Display** (`display.rs`): Only generates match arms for rules in `theory.terms`
  - Missing match arm for auto-generated Var variants
  - Non-exhaustive pattern match errors in generated code

## Solution Overview

Both `subst.rs` and `display.rs` need to:
1. Detect when a Var variant was auto-generated (same logic as `ast_gen.rs`)
2. Generate appropriate match arms for the auto-generated variant
3. Use the same naming convention as AST generation

## Detailed Changes

### 1. Substitution Module (`macros/src/codegen/subst.rs`)

#### Change 1.1: Update `generate_substitute_method()`
**Location**: Lines 109-130

**Current**: Only generates match arms from `rules` (explicit rules only)

**Change**: After generating match arms from rules, check if Var variant was auto-generated and add a match arm for it.

```rust
fn generate_substitute_method(
    category: &Ident,
    rules: &[&GrammarRule],
    replacement_cat: &Ident,
) -> TokenStream {
    let mut match_arms: Vec<TokenStream> = rules
        .iter()
        .map(|rule| generate_substitution_arm(category, rule, category))
        .collect();

    // Check if Var variant was auto-generated
    let has_var_rule = rules.iter().any(|rule| is_var_constructor(rule));
    if !has_var_rule {
        let var_arm = generate_auto_var_substitution_arm(category, category);
        match_arms.push(var_arm);
    }

    quote! {
        pub fn substitute(
            &self,
            var: &mettail_runtime::FreeVar<String>,
            replacement: &Self
        ) -> Self {
            match self {
                #(#match_arms),*
            }
        }
    }
}
```

#### Change 1.2: Update `generate_cross_category_substitute_method()`
**Location**: Lines 132-160

**Change**: Same pattern - add auto-generated Var variant match arm if needed.

```rust
fn generate_cross_category_substitute_method(
    category: &Ident,
    rules: &[&GrammarRule],
    binder_cat: &Ident,
) -> TokenStream {
    let method_name = quote::format_ident!("substitute_{}", binder_cat.to_string().to_lowercase());

    let mut match_arms: Vec<TokenStream> = rules
        .iter()
        .map(|rule| generate_substitution_arm(category, rule, binder_cat))
        .collect();

    // Check if Var variant was auto-generated
    let has_var_rule = rules.iter().any(|rule| is_var_constructor(rule));
    if !has_var_rule {
        let var_arm = generate_auto_var_substitution_arm(category, binder_cat);
        match_arms.push(var_arm);
    }

    quote! {
        // ... existing doc comment ...
        pub fn #method_name(
            &self,
            var: &mettail_runtime::FreeVar<String>,
            replacement: &#binder_cat
        ) -> Self {
            match self {
                #(#match_arms),*
            }
        }
    }
}
```

#### Change 1.3: Add `generate_auto_var_substitution_arm()`
**Location**: After `is_var_constructor()` function (around line 224)

**Purpose**: Generate substitution match arm for auto-generated Var variant

```rust
/// Generate substitution match arm for an auto-generated Var variant
fn generate_auto_var_substitution_arm(
    category: &Ident,
    replacement_cat: &Ident,
) -> TokenStream {
    // Generate Var label: first letter + "Var"
    let cat_str = category.to_string();
    let first_letter = cat_str.chars().next().unwrap_or('V').to_uppercase().collect::<String>();
    let var_label = quote::format_ident!("{}Var", first_letter);

    let category_str = category.to_string();
    let replacement_cat_str = replacement_cat.to_string();

    if category_str == replacement_cat_str {
        // Same category - can substitute
        quote! {
            #category::#var_label(mettail_runtime::OrdVar(mettail_runtime::Var::Free(v))) if v == var => {
                // This free variable matches - replace it
                replacement.clone()
            }
            #category::#var_label(_) => {
                // Different variable or bound variable - keep as is
                self.clone()
            }
        }
    } else {
        // Different category - can't substitute
        quote! {
            #category::#var_label(_) => self.clone()
        }
    }
}
```

#### Change 1.4: Import `is_var_constructor` or make it accessible
**Location**: Top of file

**Note**: `is_var_constructor()` is already defined in this file (line 220), so no import needed. However, we should verify it's accessible where needed.

### 2. Display Module (`macros/src/codegen/display.rs`)

#### Change 2.1: Update `generate_display_impl()`
**Location**: Lines 44-60

**Current**: Only generates match arms from `rules` (explicit rules only)

**Change**: After generating match arms from rules, check if Var variant was auto-generated and add a match arm for it.

```rust
fn generate_display_impl(category: &syn::Ident, rules: &[&GrammarRule]) -> TokenStream {
    let mut match_arms: Vec<TokenStream> = rules
        .iter()
        .map(|rule| generate_display_arm(rule))
        .collect();

    // Check if Var variant was auto-generated
    let has_var_rule = rules.iter().any(|rule| is_var_rule_for_display(rule));
    if !has_var_rule {
        let var_arm = generate_auto_var_display_arm(category);
        match_arms.push(var_arm);
    }

    quote! {
        impl std::fmt::Display for #category {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}
```

#### Change 2.2: Add `is_var_rule_for_display()`
**Location**: After `generate_display_impl()` (around line 60)

**Purpose**: Check if a rule is a Var rule (same logic as `is_var_rule` in `mod.rs`)

```rust
/// Check if a rule is a Var rule (for display generation)
fn is_var_rule_for_display(rule: &GrammarRule) -> bool {
    rule.items.len() == 1
        && matches!(&rule.items[0], GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
}
```

**Alternative**: Import `is_var_rule` from `crate::codegen::is_var_rule` if it's exported.

#### Change 2.3: Add `generate_auto_var_display_arm()`
**Location**: After `is_var_rule_for_display()` (around line 70)

**Purpose**: Generate display match arm for auto-generated Var variant

```rust
/// Generate display match arm for an auto-generated Var variant
fn generate_auto_var_display_arm(category: &syn::Ident) -> TokenStream {
    // Generate Var label: first letter + "Var"
    let cat_str = category.to_string();
    let first_letter = cat_str.chars().next().unwrap_or('V').to_uppercase().collect::<String>();
    let var_label = quote::format_ident!("{}Var", first_letter);

    quote! {
        #category::#var_label(var) => {
            match &var.0 {
                mettail_runtime::Var::Free(fv) => {
                    let name = fv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_");
                    write!(f, "{}", name)
                }
                mettail_runtime::Var::Bound(bv) => {
                    let name = bv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_");
                    write!(f, "{}", name)
                }
            }
        }
    }
}
```

### 3. Shared Utility (Optional but Recommended)

#### Change 3.1: Export `is_var_rule` from `codegen/mod.rs`
**Location**: `macros/src/codegen/mod.rs`

**Change**: Make `is_var_rule` public so both `subst.rs` and `display.rs` can use it.

```rust
// Already public, but verify it's accessible
pub fn is_var_rule(rule: &GrammarRule) -> bool {
    rule.items.len() == 1
        && matches!(&rule.items[0], GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
}
```

#### Change 3.2: Add helper function for Var label generation
**Location**: `macros/src/codegen/mod.rs` or `macros/src/utils.rs`

**Purpose**: Centralize the logic for generating Var variant labels to ensure consistency.

```rust
/// Generate the Var variant label for a category
/// 
/// Convention: First letter of category + "Var"
/// Examples: Proc -> PVar, Name -> NVar, Term -> TVar
pub fn generate_var_label(category: &syn::Ident) -> syn::Ident {
    let cat_str = category.to_string();
    let first_letter = cat_str.chars().next().unwrap_or('V').to_uppercase().collect::<String>();
    quote::format_ident!("{}Var", first_letter)
}
```

**Usage**: Use this in `ast_gen.rs`, `subst.rs`, and `display.rs` for consistency.

## Testing Strategy

### Unit Tests

1. **Substitution Tests**:
   - Test substitution with auto-generated Var variant
   - Test cross-category substitution with auto-generated Var variant
   - Verify match exhaustiveness

2. **Display Tests**:
   - Test display of auto-generated Var variant
   - Verify match exhaustiveness

3. **Integration Tests**:
   - Test full theory generation with auto-generated Var variants
   - Verify no compilation errors in generated code

### Manual Verification

1. Compile `theories/src/ambient.rs` (which removed explicit Var declarations)
2. Verify no non-exhaustive pattern match errors
3. Test substitution and display functionality

## Implementation Order

1. **Phase 1**: Add helper function for Var label generation (shared utility)
2. **Phase 2**: Update substitution module
   - Add `generate_auto_var_substitution_arm()`
   - Update `generate_substitute_method()`
   - Update `generate_cross_category_substitute_method()`
3. **Phase 3**: Update display module
   - Add `is_var_rule_for_display()` (or use shared `is_var_rule`)
   - Add `generate_auto_var_display_arm()`
   - Update `generate_display_impl()`
4. **Phase 4**: Testing and verification
   - Run existing tests
   - Add new tests for auto-generated variants
   - Verify ambient.rs compiles

## Edge Cases to Consider

1. **Category names starting with non-ASCII characters**: Current code uses `chars().next()` which should handle this, but verify.

2. **Empty category names**: Unlikely, but the code has a fallback to 'V'.

3. **Multiple exported categories**: Each needs its own auto-generated Var variant.

4. **Explicit Var rule exists**: Should not generate duplicate - already handled by `has_var_rule` check.

## Notes

- The naming convention must match exactly between `ast_gen.rs`, `subst.rs`, and `display.rs`
- Consider extracting the Var label generation logic to a shared utility function
- The auto-generated Var variant has type `VarLabel(mettail_runtime::OrdVar)`, not `VarLabel(Box<Var>)`
- Display logic should match the pattern used for explicit Var rules (extracting pretty_name from Var)

