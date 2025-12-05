//! Congruence rule generation for Ascent
//! 
//! Handles:
//! - Collection congruences (if S => T then {S, ...} => {T, ...})
//! - Regular congruences (if S => T then Constructor(...S...) => Constructor(...T...))
//! - Binding congruences (if S => T then (new x.S) => (new x.T))
//! - Projection-based approaches for efficient matching

use crate::ast::{TheoryDef, RewriteRule, Expr};
use proc_macro2::TokenStream;

mod analysis;
mod collection;
mod regular;
mod binding;
mod projections;

// Re-export key types and functions from analysis
pub use analysis::{
    parse_congruence_lhs,
    contains_collection_pattern,
    extract_collection_congruence_info,
    extract_category,
    find_collection_congruence_element_categories,
    get_constructor_collection_element_type,
};

// Re-export from collection
pub use collection::generate_new_collection_congruence_clauses;

// Re-export from regular

// Re-export from binding
pub use binding::generate_projection_based_binding_congruence;

// Re-export from projections
pub use projections::generate_congruence_projections;

/// Dispatch function: Route to appropriate congruence handler
pub fn generate_congruence_rewrite(
    idx: usize,
    rewrite: &RewriteRule,
    theory: &TheoryDef
) -> Option<TokenStream> {
    // Only process rules with a congruence premise
    let (source_var, target_var) = rewrite.premise.as_ref()?;
    
    // Extract category from LHS
    let category = extract_category(&rewrite.left, theory)?;
    
    // Check if this is a collection congruence
    if let Expr::Apply { constructor, args } = &rewrite.left {
        for arg in args {
            if let Expr::CollectionPattern { elements, .. } = arg {
                // Check if source_var appears in elements
                for elem in elements {
                    if let Expr::Var(v) = elem {
                        if v == source_var {
                            // Collection congruence - use simple approach for non-projection cases
                            let cat_lower = quote::format_ident!("{}", category.to_string().to_lowercase());
                            let rw_rel = quote::format_ident!("rw_{}", category.to_string().to_lowercase());
                            let rest_var = if let Expr::CollectionPattern { rest, .. } = arg {
                                rest.as_ref()
                            } else {
                                None
                            };
                            
                            return collection::generate_collection_congruence(
                                &category,
                                &cat_lower,
                                &rw_rel,
                                constructor,
                                source_var,
                                target_var,
                                rest_var,
                                theory,
                            );
                        }
                    }
                }
            }
        }
    }
    
    // Regular (non-collection) congruence
    let (constructor, field_idx, bindings) = parse_congruence_lhs(&rewrite.left, source_var, theory)?;
    
    // Check if this is a binding constructor
    let rule = theory.terms.iter().find(|r| r.label == constructor)?;
    let is_binding = !rule.bindings.is_empty();
    
    if is_binding {
        // Use projection-based binding congruence
        eprintln!("DEBUG: Generating projection-based binding congruence for {} (idx={})", constructor, idx);
        let result = generate_projection_based_binding_congruence(idx, &category, constructor.clone(), field_idx, rule, theory);
        if result.is_none() {
            eprintln!("DEBUG: generate_projection_based_binding_congruence returned None for {}", constructor);
        }
        result
    } else {
        // Regular non-binding congruence
        let cat_lower = quote::format_ident!("{}", category.to_string().to_lowercase());
        let rw_rel = quote::format_ident!("rw_{}", category.to_string().to_lowercase());
        regular::generate_regular_congruence(&category, &cat_lower, &rw_rel, constructor, field_idx, &bindings)
    }
}
