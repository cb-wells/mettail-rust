#![allow(
    clippy::cmp_owned,
    clippy::too_many_arguments,
    clippy::needless_borrow,
    clippy::for_kv_map,
    clippy::let_and_return,
    clippy::unused_enumerate_index,
    clippy::expect_fun_call,
    clippy::collapsible_match,
    clippy::unwrap_or_default,
    clippy::unnecessary_filter_map
)]

use super::analysis::{
    contains_collection_pattern, extract_category, is_collection_congruence,
    CollectionCongruenceInfo,
};
use crate::ast::{Expr, RewriteRule, TheoryDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

// Re-export from analysis for convenience
pub use super::analysis::RegularCongruencePattern;

/// Generate congruence for regular (non-binding) constructors
/// Example: if S => T then (PPar P S) => (PPar P T)
/// Generates:
/// rw_proc(s, t) <--
///     proc(s),
///     if let Proc::PPar(p, s0) = s,
///     rw_proc(**s0, t0),
///     let t = Proc::PPar(p.clone(), Box::new(t0.clone()));
pub fn generate_regular_congruence(
    category: &Ident,
    cat_lower: &Ident,
    rw_rel: &Ident,
    constructor: Ident,
    field_idx: usize,
    bindings: &[Ident],
) -> Option<TokenStream> {
    // Generate field patterns
    let field_patterns: Vec<_> = bindings
        .iter()
        .enumerate()
        .map(|(i, var)| {
            let var_lower = format_ident!("{}", var.to_string().to_lowercase());
            if i == field_idx {
                // This is the field being rewritten - name it s0
                format_ident!("s0")
            } else {
                var_lower
            }
        })
        .collect();

    // Generate the recursive rewrite clause
    let rewritten_field = format_ident!("t0");

    // Generate reconstruction arguments
    let recon_args: Vec<_> = bindings
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if i == field_idx {
                quote! { Box::new(#rewritten_field.clone()) }
            } else {
                let field_name = &field_patterns[i];
                quote! { #field_name.clone() }
            }
        })
        .collect();

    Some(quote! {
        #rw_rel(s, t) <--
            #cat_lower(s),
            if let #category::#constructor(#(#field_patterns),*) = s,
            #rw_rel(**s0, #rewritten_field),
            let t = #category::#constructor(#(#recon_args),*);
    })
}

/// Generate congruence clause for a regular congruence
/// Uses the projection and recursively calls rw_rel on the body
pub fn generate_regular_congruence_clause(
    cong_idx: usize,
    reg_idx: usize,
    _cong_info: &CollectionCongruenceInfo,
    pattern: &RegularCongruencePattern,
    rw_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    insert_helper: &Ident,
    theory: &TheoryDef,
) -> TokenStream {
    let rel_name = format_ident!(
        "{}_proj_c{}_r{}",
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        reg_idx
    );

    let elem_rw_rel = format_ident!("rw_{}", pattern.category.to_string().to_lowercase());
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;

    // Generate reconstruction based on whether it's a binding constructor
    let reconstruction = if pattern.is_binding {
        // IMPORTANT: Use from_parts_unsafe to avoid rebinding (which would change variable IDs)
        // The body still has Bound variables, so we preserve them
        quote! {
            let scope_tmp = mettail_runtime::Scope::from_parts_unsafe(binder_var.clone(), Box::new(body_rewritten.clone())),
            let rewritten = #elem_cat::#elem_constructor(scope_tmp)
        }
    } else {
        // For non-binding constructors, we need to extract all fields from elem,
        // then reconstruct with the rewritten field replacing the original
        let grammar_rule = theory
            .terms
            .iter()
            .find(|r| r.label == *elem_constructor)
            .expect("Constructor not found");

        let field_count = grammar_rule
            .items
            .iter()
            .filter(|item| !matches!(item, crate::ast::GrammarItem::Terminal(_)))
            .count();

        // Generate field patterns for destructuring
        let mut field_pats = Vec::new();
        for i in 0..field_count {
            let field_name = format_ident!("elem_field_{}", i);
            field_pats.push(field_name);
        }

        // Generate reconstruction arguments, replacing the rewritten field
        let recon_args: Vec<_> = (0..field_count)
            .map(|i| {
                if i == pattern.rewrite_field_idx {
                    quote! { Box::new(body_rewritten.clone()) }
                } else {
                    let field_name = &field_pats[i];
                    quote! { #field_name.clone() }
                }
            })
            .collect();

        quote! {
            if let #elem_cat::#elem_constructor(#(#field_pats),*) = elem,
            let rewritten = #elem_cat::#elem_constructor(#(#recon_args),*)
        }
    };

    let proj_args = if pattern.is_binding {
        quote! { (parent, binder_var, body, elem) }
    } else {
        quote! { (parent, body, elem) }
    };

    quote! {
        #rw_rel(parent, result) <--
            #rel_name #proj_args,
            #elem_rw_rel(body, body_rewritten),
            if let #parent_cat::#constructor(ref bag) = parent,
            let remaining = {
                let mut b = bag.clone();
                b.remove(elem);
                b
            },
            #reconstruction,
            let result = #parent_cat::#constructor({
                let mut bag = remaining;
                #parent_cat::#insert_helper(&mut bag, rewritten);
                bag
            }).normalize();
    }
}

/// Extract regular congruence pattern from a congruence rule
/// Returns Some if this is a regular (non-collection) congruence
pub fn extract_regular_congruence_pattern(
    rule: &RewriteRule,
    theory: &TheoryDef,
) -> Option<RegularCongruencePattern> {
    // Check that this has a premise but no collection pattern
    let (source_var, target_var) = rule.premise.as_ref()?;

    // LHS must not contain collection pattern
    if contains_collection_pattern(&rule.left) {
        return None;
    }

    // LHS must be Apply
    if let Expr::Apply { constructor, args } = &rule.left {
        let category = theory
            .terms
            .iter()
            .find(|r| r.label == *constructor)
            .map(|r| r.category.clone())?;

        // Find which field contains source_var
        let mut rewrite_field_idx = None;
        for (idx, arg) in args.iter().enumerate() {
            if let Expr::Var(v) = arg {
                if v == source_var {
                    rewrite_field_idx = Some(idx);
                    break;
                }
            }
        }

        let rewrite_field_idx = rewrite_field_idx?;

        // Check if this is a binding constructor
        let grammar_rule = theory.terms.iter().find(|r| r.label == *constructor)?;
        let is_binding = !grammar_rule.bindings.is_empty();

        return Some(RegularCongruencePattern {
            constructor: constructor.clone(),
            category,
            rewrite_field_idx,
            is_binding,
            source_var: source_var.clone(),
            target_var: target_var.clone(),
        });
    }

    None
}

/// Find all regular (non-collection) congruence rules for a category
pub fn find_regular_congruences_for_category<'a>(
    category: &Ident,
    theory: &'a TheoryDef,
) -> Vec<&'a RewriteRule> {
    theory
        .rewrites
        .iter()
        .filter(|rule| {
            // Has premise (is a congruence)
            if rule.premise.is_none() {
                return false;
            }

            // Is NOT a collection congruence
            if is_collection_congruence(rule, theory) {
                return false;
            }

            // Operates on the target category
            extract_category(&rule.left, theory)
                .map(|cat| cat == *category)
                .unwrap_or(false)
        })
        .collect()
}
