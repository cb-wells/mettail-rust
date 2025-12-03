//! Ascent Datalog code generation
//! 
//! This module orchestrates the generation of Ascent Datalog code for a theory.
//! 
//! ## Structure
//! 
//! - `relations` - Relation declarations (categories, equality, rewrites, projections)
//! - `categories` - Category exploration and term deconstruction rules
//! - `equations` - Equality/equation rules with congruence
//! - `rewrites/` - Base rewrite rules and pattern/RHS generation
//! - `congruence/` - Congruence rules for rewrites (collection, regular, binding)
//! 
//! ## Generated Code Components
//! 
//! 1. **Relations**: Declare all Ascent relations for terms, equality, and rewrites
//! 2. **Category Rules**: Explore term space via rewrites and deconstruct terms
//! 3. **Equation Rules**: Add reflexivity, congruence, and user-defined equalities
//! 4. **Rewrite Rules**: Base rewrites + congruence rules (propagate through constructors)

use crate::ast::TheoryDef;
use crate::utils::print_rule;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

mod relations;
mod categories;
mod equations;

pub mod rewrites;
pub mod congruence;

// Re-export key functions
pub use relations::generate_relations;
pub use categories::generate_category_rules;
pub use equations::generate_equation_rules;

// Re-export congruence types and functions used by lib.rs
pub use congruence::{
    extract_collection_congruence_info,
    generate_congruence_projections,
};

pub use rewrites::{
    generate_rewrite_clauses,
    generate_freshness_functions,
};

/// Main entry point: Generate complete Ascent source for a theory
pub fn generate_ascent_source(theory: &TheoryDef) -> TokenStream {
    let theory_name = theory.name.to_string().to_lowercase();
    let source_name = format_ident!("{}_source", theory_name);

    let relations = generate_relations(theory);
    let category_rules = generate_category_rules(theory);
    let equation_rules = generate_equation_rules(theory);
    let rewrite_rules = generate_rewrite_rules(theory);

    let result = quote! {
        ::ascent::ascent_source! {
            #source_name:

            #relations

            #category_rules

            #equation_rules

            #rewrite_rules
        }
    };
    
    eprintln!("\n========== FULL GENERATED ASCENT SOURCE ==========");
    eprintln!("ascent_source! {{");
    eprintln!("    {}:\n", source_name);
    eprintln!("    // Relations");
    for line in relations.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Category rules");
    for line in category_rules.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Equation rules");
    for line in equation_rules.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Rewrite rules");
    for line in rewrite_rules.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("}}");
    eprintln!("==================================================\n");
    
    result
}

/// Generate rewrite rules: base rewrites + congruence rules
/// 
/// - **Base rewrites**: Rules without premises (S => T)
/// - **Collection congruences**: If S => T, then {S, ...} => {T, ...}
/// - **Regular congruences**: If S => T, then Constructor(S) => Constructor(T)
/// - **Binding congruences**: If S => T, then (new x.S) => (new x.T)
pub fn generate_rewrite_rules(theory: &TheoryDef) -> TokenStream {
    let mut rules = Vec::new();
    
    // Generate base rewrite clauses (no premise)
    let base_rewrite_clauses = generate_rewrite_clauses(theory);
    rules.extend(base_rewrite_clauses);
    
    // Generate congruence rules (with premise: if S => T then ...)
    // For each collection congruence, generate projections and clauses
    for (cong_idx, rewrite) in theory.rewrites.iter().enumerate() {
        if let Some((source_var, target_var)) = &rewrite.premise {
            // Check if this is a collection congruence
            if let Some(cong_info) = extract_collection_congruence_info(
                &rewrite.left, source_var, target_var, theory
            ) {
                // Generate all projections for this congruence
                let (projections, base_patterns) = generate_congruence_projections(
                    cong_idx, &cong_info, theory
                );
                rules.extend(projections);
                
                // Generate congruence clauses using those projections
                let congruence_clauses = congruence::generate_new_collection_congruence_clauses(
                    cong_idx, &cong_info, &base_patterns, theory
                );
                rules.extend(congruence_clauses);
            } else {
                // Regular (non-collection) congruence - dispatch to appropriate handler
                if let Some(rule) = congruence::generate_congruence_rewrite(cong_idx, rewrite, theory) {
                    rules.push(rule);
                }
            }
        }
    }
    
    quote! {
        #(#rules)*
    }
}
