use crate::ast::{TheoryDef, RewriteRule, Expr};
use super::{generate_ascent_pattern};
use super::rhs::{generate_ascent_rhs, generate_rhs_construction};
use crate::ascent::congruence::{
    find_collection_congruence_element_categories,
    extract_collection_congruence_info,
    extract_category,
    contains_collection_pattern,
    CollectionCongruenceInfo,
};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;
use std::collections::{HashMap, HashSet};

/// Generate Ascent clauses for rewrite rules (for equational matching)
/// This is the new approach that allows duplicate variables to use eq_cat() relations
pub fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    use crate::ascent::congruence;
    
    let mut all_clauses = Vec::new();
    
    // Find which categories are covered by collection congruences
    let collection_cong_categories = congruence::find_collection_congruence_element_categories(theory);
    
    for (_rule_idx, rule) in theory.rewrites.iter().enumerate() {
        // Skip congruence rules (handled elsewhere)
        if rule.premise.is_some() {
            continue;
        }
        
        // Check if this base rewrite's LHS involves a collection pattern for a category
        // that's covered by a collection congruence
        let is_covered_by_congruence = if congruence::contains_collection_pattern(&rule.left) {
            // This rule has a collection pattern in its LHS
            // We only skip if the ROOT of the LHS is a collection constructor that's covered
            // by a collection congruence.
            //
            // For example:
            // - (PPar {(PIn ...), (POut ...)}) => ... should be SKIPPED (root is PPar collection)
            // - (PAmb M (PPar {...})) => ... should NOT be skipped (root is PAmb, not a collection)
            
            if let Expr::Apply { constructor, .. } = &rule.left {
                // Check if this constructor has a collection field
                if let Some(elem_cat) = congruence::get_constructor_collection_element_type(constructor, theory) {
                    // Root is a collection constructor
                    // Check if this element category is covered by a collection congruence
                    collection_cong_categories.contains(&elem_cat)
                } else {
                    // Root is not a collection constructor, but may contain nested collections
                    // Generate the rule directly (don't skip)
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        
        if is_covered_by_congruence {
            // Skip this rule - it will be handled by congruence-driven projection
            continue;
        }
        
        // OLD PATH: For non-collection patterns or uncovered categories
        all_clauses.push(generate_rewrite_clause(rule, theory));
    }
    
    all_clauses
}

/// Generate a single rewrite rule as an Ascent clause
/// Example output:
/// rw_proc(s, t) <--
///     proc(s),
///     if let Proc::PPar(p_in, p_out) = s,
///     if let Proc::PInput(chan1, scope) = &**p_in,
///     if let Proc::POutput(chan2, q) = &**p_out,
///     eq_name((**chan1).clone(), (**chan2).clone()),
///     let (x, p) = scope.clone().unbind(),
///     if !p.contains_free(&x),
///     let t = p.substitute(&x, &Name::NQuote((**q).clone()));
fn generate_rewrite_clause(rule: &RewriteRule, theory: &TheoryDef) -> TokenStream {
    let category = extract_category(&rule.left, theory).expect("Failed to extract category from rewrite rule LHS");
    let cat_lower = quote::format_ident!("{}", category.to_string().to_lowercase());
    let rw_rel = quote::format_ident!("rw_{}", category.to_string().to_lowercase());
    
    // Track variable occurrences for duplicate detection
    let mut var_occurrences: HashMap<String, Vec<usize>> = HashMap::new();
    let mut occurrence_idx = 0;
    collect_variable_occurrences(&rule.left, &mut var_occurrences, &mut occurrence_idx);
    
    // Identify which variables appear multiple times (need equational matching)
    let duplicate_vars: std::collections::HashSet<String> = var_occurrences
        .into_iter()
        .filter(|(_, occurrences)| occurrences.len() > 1)
        .map(|(var_name, _)| var_name)
        .collect();
    
    // Generate pattern matching clauses
    let mut bindings: HashMap<String, TokenStream> = HashMap::new();
    let mut variable_categories: HashMap<String, Ident> = HashMap::new();
    let mut equational_checks: Vec<TokenStream> = Vec::new();
    let mut clauses = Vec::new();
    
    // Start with proc(s) clause
    clauses.push(quote! { #cat_lower(s) });
    
    // Generate pattern matching with category tracking
    let lhs_category = extract_category(&rule.left, theory).expect("Failed to extract category from rewrite rule LHS");
    generate_ascent_pattern(
        &rule.left,
        &quote::format_ident!("s"),
        &lhs_category,
        theory,
        &mut bindings,
        &mut variable_categories,
        &mut clauses,
        &duplicate_vars,
        &mut equational_checks,
    );
    
    // Add equational checks for duplicate variables
    clauses.extend(equational_checks);
    
    // Add freshness checks
    for condition in &rule.conditions {
        let var_name = condition.var.to_string();
        let term_name = condition.term.to_string();
        
        let var_binding = bindings.get(&var_name)
            .unwrap_or_else(|| panic!("Freshness variable '{}' not bound. Available bindings: {:?}", var_name, bindings.keys().collect::<Vec<_>>()));
        let term_binding = bindings.get(&term_name)
            .unwrap_or_else(|| panic!("Freshness term '{}' not bound. Available bindings: {:?}", term_name, bindings.keys().collect::<Vec<_>>()));
        
        clauses.push(quote! {
            if is_fresh(&#var_binding, &#term_binding)
        });
    }
    
    // Generate RHS
    let rhs = generate_ascent_rhs(&rule.right, &bindings, theory);
    clauses.push(quote! { let t = (#rhs).normalize() });
    
    quote! {
        #rw_rel(s, t) <--
            #(#clauses),*;
    }
}

/// Collect all variable occurrences in an expression (for duplicate detection)
fn collect_variable_occurrences(
    expr: &Expr,
    occurrences: &mut HashMap<String, Vec<usize>>,
    idx: &mut usize,
) {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            occurrences.entry(var_name).or_insert_with(Vec::new).push(*idx);
            *idx += 1;
        }
        Expr::Apply { args, .. } => {
            for arg in args {
                collect_variable_occurrences(arg, occurrences, idx);
            }
        }
        Expr::Subst { term, .. } => {
            collect_variable_occurrences(term, occurrences, idx);
        }
        Expr::CollectionPattern { elements, rest, .. } => {
            for elem in elements {
                collect_variable_occurrences(elem, occurrences, idx);
            }
            if let Some(rest_var) = rest {
                let var_name = rest_var.to_string();
                occurrences.entry(var_name).or_insert_with(Vec::new).push(*idx);
                *idx += 1;
            }
        }
    }
}
