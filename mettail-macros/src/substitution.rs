//! Substitution generation for MeTTaIL terms
//! 
//! Generates capture-avoiding substitution methods using moniker's BoundTerm trait.
//! For each exported category, we generate a `substitute` method that performs
//! capture-avoiding substitution of variables.

use crate::ast::{TheoryDef, GrammarRule, GrammarItem};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate substitution methods for a theory's AST types
/// 
/// For each exported category, generates:
/// - `substitute(var, replacement)` for same-category substitution
/// - `substitute_X(var, replacement)` for each category X that appears in this category
/// 
/// Uses moniker's BoundTerm operations for automatic capture avoidance.
pub fn generate_substitution(theory: &TheoryDef) -> TokenStream {
    let impls: Vec<TokenStream> = theory.exports.iter().map(|export| {
        let category = &export.name;
        
        // Find all rules for this category
        let rules: Vec<&GrammarRule> = theory.terms
            .iter()
            .filter(|r| r.category == *category)
            .collect();
        
        // Find all categories that we might need to substitute
        // This includes binder categories AND field categories
        let subst_cats = find_substitutable_categories(&rules);
        
        generate_category_substitution(category, &rules, &subst_cats)
    }).collect();
    
    quote! {
        #(#impls)*
    }
}

/// Find all categories that might be substituted in a set of rules
/// This includes:
/// - Binder categories (what Scopes bind)
/// - Field categories (what appears in constructors)
fn find_substitutable_categories(rules: &[&GrammarRule]) -> std::collections::HashSet<String> {
    let mut cats = std::collections::HashSet::new();
    
    for rule in rules {
        // Add binder categories
        if !rule.bindings.is_empty() {
            let (binder_idx, _) = &rule.bindings[0];
            if let GrammarItem::Binder { category } = &rule.items[*binder_idx] {
                cats.insert(category.to_string());
            }
        }
        
        // Add field categories
        for item in &rule.items {
            if let GrammarItem::NonTerminal(cat) = item {
                let cat_str = cat.to_string();
                // Don't include Var - it's handled specially
                if cat_str != "Var" {
                    cats.insert(cat_str);
                }
            }
        }
    }
    
    cats
}

/// Generate substitution implementation for a specific category
fn generate_category_substitution(
    category: &Ident,
    rules: &[&GrammarRule],
    subst_cats: &std::collections::HashSet<String>
) -> TokenStream {
    let category_str = category.to_string();
    
    // Generate the main substitute method (same-category)
    let main_method = generate_substitute_method(category, rules, category);
    
    // Generate cross-category substitute methods for OTHER categories
    let cross_methods: Vec<TokenStream> = subst_cats.iter()
        .filter(|cat| **cat != category_str)
        .map(|cat_str| {
            let cat = syn::Ident::new(cat_str, proc_macro2::Span::call_site());
            generate_cross_category_substitute_method(category, rules, &cat)
        })
        .collect();
    
    // Also generate a self-referential method (substitute_X where X == category)
    // This is just an alias for substitute, needed for cross-category recursion
    let self_method = generate_self_substitute_method(category);
    
    quote! {
        impl #category {
            #main_method
            #self_method
            #(#cross_methods)*
        }
    }
}

/// Generate the main substitute method (same-category substitution)
fn generate_substitute_method(
    category: &Ident,
    rules: &[&GrammarRule],
    _replacement_cat: &Ident
) -> TokenStream {
    let match_arms: Vec<TokenStream> = rules.iter().map(|rule| {
        generate_substitution_arm(category, rule, category)
    }).collect();
    
    quote! {
        /// Substitute `replacement` for free occurrences of `var` in this term
        /// 
        /// This performs capture-avoiding substitution using moniker's BoundTerm trait.
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

/// Generate a cross-category substitute method
fn generate_cross_category_substitute_method(
    category: &Ident,
    rules: &[&GrammarRule],
    binder_cat: &Ident
) -> TokenStream {
    let method_name = quote::format_ident!("substitute_{}", binder_cat.to_string().to_lowercase());
    
    let match_arms: Vec<TokenStream> = rules.iter().map(|rule| {
        generate_substitution_arm(category, rule, binder_cat)
    }).collect();
    
    quote! {
        /// Substitute `replacement` (of type #binder_cat) for free occurrences of `var` in this term
        /// 
        /// This is used for cross-category substitution where a binder binds variables
        /// of a different category than the term itself.
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

/// Generate a self-referential substitute method (substitute_X where X is the category itself)
/// This is just an alias for the main substitute method, needed for uniform cross-category recursion
fn generate_self_substitute_method(category: &Ident) -> TokenStream {
    let method_name = quote::format_ident!("substitute_{}", category.to_string().to_lowercase());
    
    quote! {
        /// Alias for substitute(), provided for uniform cross-category substitution
        pub fn #method_name(
            &self,
            var: &mettail_runtime::FreeVar<String>,
            replacement: &Self
        ) -> Self {
            self.substitute(var, replacement)
        }
    }
}

/// Generate a substitution match arm for a single constructor
fn generate_substitution_arm(category: &Ident, rule: &GrammarRule, replacement_cat: &Ident) -> TokenStream {
    let label = &rule.label;
    
    // Check if this is a variable constructor
    if is_var_constructor(rule) {
        // Variable constructors only substitute if replacement category matches
        if category == replacement_cat {
            // Special case: EVar(v) - check if v matches the variable to substitute
            return quote! {
                #category::#label(mettail_runtime::OrdVar(mettail_runtime::Var::Free(v))) if v == var => {
                    // This free variable matches - replace it
                    replacement.clone()
                }
                #category::#label(_) => {
                    // Different variable or bound variable - keep as is
                    self.clone()
                }
            };
        } else {
            // Different category - can't substitute
            return quote! {
                #category::#label(_) => self.clone()
            };
        }
    }
    
    // Check if this has bindings (uses Scope)
    if !rule.bindings.is_empty() {
        return generate_scope_substitution_arm(category, rule, replacement_cat);
    }
    
    // Regular constructor - substitute in all subterms
    generate_regular_substitution_arm(category, rule, replacement_cat)
}

/// Check if a rule is a variable constructor (Var category)
fn is_var_constructor(rule: &GrammarRule) -> bool {
    rule.items.len() == 1 && matches!(&rule.items[0], GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
}

/// Generate substitution for a constructor with Scope (binder)
fn generate_scope_substitution_arm(category: &Ident, rule: &GrammarRule, replacement_cat: &Ident) -> TokenStream {
    let label = &rule.label;
    
    let (binder_idx, body_indices) = &rule.bindings[0];
    let body_idx = body_indices[0];
    
    // Get the binder category to determine substitution type
    let binder_cat = match &rule.items[*binder_idx] {
        GrammarItem::Binder { category } => category,
        _ => panic!("Binding index doesn't point to a Binder"),
    };
    
    let body_cat = match &rule.items[body_idx] {
        GrammarItem::NonTerminal(cat) => cat,
        _ => panic!("Body index doesn't point to a NonTerminal"),
    };
    
    // Generate pattern bindings for all fields (in grammar order)
    // Track which position is the scope
    let mut field_bindings = Vec::new();
    let mut scope_field_idx = None;
    
    for (i, item) in rule.items.iter().enumerate() {
        if i == *binder_idx {
            // Skip binder - it's part of the Scope
            continue;
        }
        
        match item {
            GrammarItem::NonTerminal(_) | GrammarItem::Binder { .. } => {
                let field_name = if i == body_idx {
                    scope_field_idx = Some(field_bindings.len());
                    quote::format_ident!("scope")
                } else {
                    quote::format_ident!("field_{}", field_bindings.len())
                };
                field_bindings.push(field_name);
            }
            GrammarItem::Terminal(_) => {}
        }
    }
    
    let scope_idx = scope_field_idx.expect("Should have found scope field");
    
    // Check if we need to recurse into the body
    // We only recurse if the replacement category matches the binder category
    let binder_cat_str = binder_cat.to_string();
    let replacement_cat_str = replacement_cat.to_string();
    
    if binder_cat_str == replacement_cat_str {
        // The replacement type matches what this Scope binds
        // So we need to check for shadowing and potentially substitute in the body
        
        // Determine the method name to call on the body
        let body_cat_str = body_cat.to_string();
        let subst_method = if body_cat_str == replacement_cat_str {
            // Same category - use plain substitute
            quote! { substitute }
        } else {
            // Cross-category - use substitute_X
            let method_name = quote::format_ident!("substitute_{}", replacement_cat_str.to_lowercase());
            quote! { #method_name }
        };
        
        // Generate field reconstruction - substitute in scope, clone others
        let field_reconstructions: Vec<TokenStream> = field_bindings.iter().enumerate().map(|(i, field_name)| {
            if i == scope_idx {
                quote! { new_scope.clone() }
            } else {
                // For non-scope fields, we should also substitute!
                // Determine which method to call
                let field_cat = &rule.items.iter()
                    .enumerate()
                    .filter(|(idx, item)| {
                        *idx != *binder_idx && matches!(item, GrammarItem::NonTerminal(_))
                    })
                    .nth(i)
                    .map(|(_, item)| match item {
                        GrammarItem::NonTerminal(cat) => cat.to_string(),
                        _ => unreachable!(),
                    })
                    .unwrap_or_default();
                
                let subst_method = if *field_cat == replacement_cat_str {
                    quote! { substitute }
                } else {
                    let method_name = quote::format_ident!("substitute_{}", replacement_cat_str.to_lowercase());
                    quote! { #method_name }
                };
                
                // Recurse into non-scope fields too
                quote! { Box::new((**#field_name).#subst_method(var, replacement)) }
            }
        }).collect();
        
        quote! {
            #category::#label(#(#field_bindings),*) => {
                // Unbind to check if this scope shadows the variable
                let (binder, body) = scope.clone().unbind();
                
                // Check if the binder shadows our variable
                if binder.0 == *var {
                    // The scope binds the same variable we're substituting
                    // So the variable is not free in the body - no substitution needed
                    self.clone()
                } else {
                    // The scope doesn't shadow our variable - substitute in the body
                    let subst_body = body.#subst_method(var, replacement);
                    let new_scope = mettail_runtime::Scope::new(binder, Box::new(subst_body));
                    
                    // Reconstruct with updated scope and cloned other fields
                    #category::#label(#(#field_reconstructions),*)
                }
            }
        }
    } else {
        // The replacement type doesn't match what this Scope binds
        // So variables in the body won't be affected - just clone
        quote! {
            #category::#label(#(#field_bindings),*) => {
                // Cross-category mismatch: this Scope binds #binder_cat, but we're substituting #replacement_cat
                // Variables of type #binder_cat in the body won't match our substitution
                self.clone()
            }
        }
    }
}

/// Generate substitution for a regular constructor (no bindings)
fn generate_regular_substitution_arm(category: &Ident, rule: &GrammarRule, replacement_cat: &Ident) -> TokenStream {
    let label = &rule.label;
    
    // Check if this constructor has a Var field
    let has_var_field = rule.items.iter().any(|item| {
        matches!(item, GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
    });
    
    // For constructors with Var fields, we need special handling
    if has_var_field {
        // NVar case - substitute directly at the Var level
        let category_str = category.to_string();
        let replacement_cat_str = replacement_cat.to_string();
        
        if category_str == replacement_cat_str {
            // Same category - use moniker's built-in substitution
            return quote! {
                #category::#label(var_field) => {
                    use mettail_runtime::{Var, FreeVar};
                    match var_field {
                        Var::Bound(b) => #category::#label(Var::Bound(b.clone())),
                        Var::Free(ref fv) => {
                            if fv == var {
                                replacement.clone()
                            } else {
                                self.clone()
                            }
                        }
                    }
                }
            };
        } else {
            // Cross-category - no substitution possible in Var
            return quote! {
                #category::#label(_) => self.clone()
            };
        }
    }
    
    // Count total non-terminal fields for pattern matching (excluding Var)
    let total_fields: Vec<_> = rule.items
        .iter()
        .filter_map(|item| match item {
            GrammarItem::NonTerminal(ident) if ident.to_string() != "Var" => Some(ident),
            _ => None,
        })
        .collect();
    
    if total_fields.is_empty() {
        // Unit constructor - no fields at all
        return quote! {
            #category::#label => self.clone()
        };
    }
    
    // Check if ANY field is a category type (needs recursion)
    // We need to recurse into ALL category fields, not just matching ones!
    let has_category_fields = !total_fields.is_empty();
    
    if !has_category_fields {
        // No category fields at all - just clone
        return quote! {
            #category::#label(..) => self.clone()
        };
    }
    
    // Generate pattern bindings for all fields
    let field_bindings: Vec<TokenStream> = (0..total_fields.len())
        .map(|i| {
            let field = quote::format_ident!("field_{}", i);
            quote! { #field }
        })
        .collect();
    
    // Determine substitution method to call for each field
    // KEY INSIGHT: We need to recurse into ALL category fields!
    // The method name is determined by the replacement category.
    let category_str = category.to_string();
    let replacement_cat_str = replacement_cat.to_string();
    
    // Generate substitution calls - recurse into ALL category fields
    let field_substitutions: Vec<TokenStream> = (0..total_fields.len())
        .map(|i| {
            let field = quote::format_ident!("field_{}", i);
            let field_cat = &total_fields[i];
            let field_cat_str = field_cat.to_string();
            
            // Determine which method to call on this field
            // We're substituting replacement_cat, so call substitute_<replacement_cat>
            let subst_method = if field_cat_str == replacement_cat_str {
                // Field category matches replacement - use plain substitute
                quote! { substitute }
            } else {
                // Cross-category - use substitute_<replacement_cat>
                let method_name = quote::format_ident!("substitute_{}", replacement_cat_str.to_lowercase());
                quote! { #method_name }
            };
            
            // Always recurse into category fields
            quote! {
                Box::new((**#field).#subst_method(var, replacement))
            }
        })
        .collect();
    
    // Generate the match arm
    if total_fields.len() == 1 {
        // Single field (possibly boxed)
        quote! {
            #category::#label(#(#field_bindings),*) => {
                #category::#label(#(#field_substitutions),*)
            }
        }
    } else {
        // Multiple fields
        quote! {
            #category::#label(#(#field_bindings),*) => {
                #category::#label(#(#field_substitutions),*)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use syn::parse_quote;
    
    #[test]
    fn test_generate_simple_substitution() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Elem) }
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(Zero),
                    category: parse_quote!(Elem),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(Var),
                    category: parse_quote!(Elem),
                    items: vec![GrammarItem::NonTerminal(parse_quote!(Var))],
                    bindings: vec![],
                },
            ],
            equations: vec![],
            rewrites: vec![]
        };
        
        let output = generate_substitution(&theory);
        let output_str = output.to_string();
        
        println!("Generated substitution:\n{}", output_str);
        
        // Check that it generates substitute method
        assert!(output_str.contains("substitute"));
        assert!(output_str.contains("replacement"));
    }
}
