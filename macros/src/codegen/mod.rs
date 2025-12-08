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
