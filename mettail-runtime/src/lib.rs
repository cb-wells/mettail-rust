//! Runtime support for MeTTaIL-generated code
//! 
//! Re-exports moniker for binder support in generated theories.

// Re-export moniker types for generated code
pub use moniker::{
    Var, FreeVar, Binder, Scope, BoundTerm, BoundPattern,
};

use std::fmt;

/// Base trait for all generated AST nodes
pub trait Term: Clone + fmt::Debug + PartialEq {
    /// Get the category/type of this term
    fn category(&self) -> &'static str;
    
    /// Pretty-print the term using the theory's syntax
    fn display(&self) -> String;
}

/// Trait for theories with equations
pub trait HasEquations<T: Term> {
    /// Check if two terms are equal according to the theory's equations
    fn equal(&self, left: &T, right: &T) -> bool;
}

/// Error type for parsing
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Trait for parsing terms from strings
pub trait Parser<T: Term> {
    fn parse(&self, input: &str) -> Result<T, ParseError>;
}

/// Helper trait for substitution (will use moniker's approach)
pub trait Substitutable: Term {
    /// Substitute all occurrences of a free variable with a value
    fn substitute_var(&self, var: &FreeVar<String>, value: &Self) -> Self;
    
    /// Get all free variables in this term
    fn free_variables(&self) -> Vec<FreeVar<String>>;
}
