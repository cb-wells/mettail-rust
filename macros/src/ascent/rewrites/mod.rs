//! Rewrite rule generation for Ascent
//! 
//! Handles:
//! - Base rewrite rules (no premise)
//! - Pattern matching for LHS
//! - RHS construction
//! - Freshness functions

use crate::ast::TheoryDef;
use proc_macro2::TokenStream;
use quote::quote;

mod patterns;
mod clauses;
pub mod rhs;

// Re-export key functions
pub use patterns::generate_ascent_pattern;
pub use clauses::generate_rewrite_clauses;

/// Generate freshness checking functions
pub fn generate_freshness_functions(_theory: &TheoryDef) -> TokenStream {
    quote! {
        pub fn is_fresh<T>(binder: &mettail_runtime::Binder<String>, term: &T) -> bool
        where
            T: mettail_runtime::BoundTerm<String>
        {
            use mettail_runtime::BoundTerm;
            
            let mut is_fresh = true;
            term.visit_vars(&mut |v| {
                if let mettail_runtime::Var::Free(fv) = v {
                    if fv == &binder.0 {
                        is_fresh = false;
                    }
                }
            });
            
            is_fresh
        }
    }
}
