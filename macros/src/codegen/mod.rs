//! Code generation for theory definitions
//! 
//! This module orchestrates the generation of Rust AST types, Display implementations,
//! substitution logic, term generation, and parser integration.

mod ast_gen;
mod display;
mod subst;

pub mod termgen;
pub mod parser;

pub use ast_gen::*;

