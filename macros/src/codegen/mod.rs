//! Code generation for theory definitions
//!
//! This module orchestrates the generation of Rust AST types, Display implementations,
//! substitution logic, term generation, and parser integration.

mod ast_gen;
mod display;
mod subst;

pub mod parser;
pub mod termgen;

pub use ast_gen::*;

use crate::ast::{GrammarItem, GrammarRule};

/// Checks if a rule is a Var rule (single item, NonTerminal "Var")
pub fn is_var_rule(rule: &GrammarRule) -> bool {
    rule.items.len() == 1
        && matches!(&rule.items[0], GrammarItem::NonTerminal(ident) if ident.to_string() == "Var")
}
