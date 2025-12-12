//! Code generation for theory definitions
//!
//! This module orchestrates the generation of Rust AST types, Display implementations,
//! substitution logic, term generation, and parser integration.

mod ast_gen;
mod display;
mod subst;
pub mod termgen;

pub mod parser;
pub mod blockly;

pub use ast_gen::*;

use crate::ast::{GrammarItem, GrammarRule};
use syn::Ident;

/// Checks if a rule is a Var rule (single item, NonTerminal "Var")
#[allow(clippy::cmp_owned)]
pub fn is_var_rule(rule: &GrammarRule) -> bool {
    rule.items.len() == 1
        && matches!(&rule.items[0], GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
}

/// Generate the Var variant label for a category
/// 
/// Convention: First letter of category + "Var"
/// Examples: Proc -> PVar, Name -> NVar, Term -> TVar
pub fn generate_var_label(category: &Ident) -> Ident {
    let cat_str = category.to_string();
    let first_letter = cat_str.chars().next().unwrap_or('V').to_uppercase().collect::<String>();
    quote::format_ident!("{}Var", first_letter)
}
