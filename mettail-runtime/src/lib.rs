//! Runtime support for MeTTaIL-generated code
//! 
//! Re-exports moniker for binder support in generated theories.

use std::collections::HashMap;
use std::sync::Mutex;
use std::hash::{Hash, Hasher};

// Re-export moniker types (official crate from crates.io)
pub use moniker::{
    Var, FreeVar, Binder, BoundTerm, BoundPattern, BoundVar,
};

// Wrapper for Scope with Hash implementation
pub use scope_wrapper::Scope;

// Re-export LALRPOP utilities for generated parsers
pub use lalrpop_util::ParseError as LalrpopParseError;

use std::fmt;

// Variable cache for consistent variable identity within a parsing session
lazy_static::lazy_static! {
    static ref VAR_CACHE: Mutex<HashMap<String, FreeVar<String>>> = 
        Mutex::new(HashMap::new());
}

/// Wrapper module for Scope with Hash implementation
mod scope_wrapper {
    use super::*;
    use moniker::Scope as MonikerScope;

    /// Wrapper around moniker::Scope that adds Hash implementation.
    /// 
    /// This is needed because the official moniker crate doesn't implement Hash for Scope,
    /// but we need it for using Scopes in HashMap-based data structures (like Ascent relations).
    /// 
    /// The Hash implementation hashes both the pattern and body, which is safe because
    /// Scope's PartialEq already compares these fields alpha-equivalently.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Scope<P, T> {
        inner: MonikerScope<P, T>,
    }

    impl<P: Hash, T: Hash> Hash for Scope<P, T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Hash the pattern and body directly
            // This is safe because Scope's PartialEq implementation already ensures
            // that equal scopes have equal patterns and bodies (alpha-equivalently)
            self.inner.unsafe_pattern.hash(state);
            self.inner.unsafe_body.hash(state);
        }
    }

    impl<P, T> Scope<P, T> {
        /// Create a new scope by binding a term with the given pattern
        pub fn new<N>(pattern: P, body: T) -> Scope<P, T>
        where
            N: Clone + PartialEq,
            P: moniker::BoundPattern<N>,
            T: moniker::BoundTerm<N>,
        {
            Scope {
                inner: MonikerScope::new(pattern, body),
            }
        }

        /// Unbind a term, returning the freshened pattern and body
        pub fn unbind<N>(self) -> (P, T)
        where
            N: Clone + Eq + std::hash::Hash,
            P: moniker::BoundPattern<N>,
            T: moniker::BoundTerm<N>,
        {
            self.inner.unbind()
        }

        /// Simultaneously unbind two terms
        pub fn unbind2<N, P2, T2>(self, other: Scope<P2, T2>) -> (P, T, P2, T2)
        where
            N: Clone + Eq + std::hash::Hash,
            P: moniker::BoundPattern<N>,
            T: moniker::BoundTerm<N>,
            P2: moniker::BoundPattern<N>,
            T2: moniker::BoundTerm<N>,
        {
            self.inner.unbind2(other.inner)
        }

        /// Access to the underlying moniker Scope (for advanced use)
        pub fn inner(&self) -> &MonikerScope<P, T> {
            &self.inner
        }
    }

    // Implement BoundTerm by delegating to inner Scope
    impl<N, P, T> moniker::BoundTerm<N> for Scope<P, T>
    where
        N: Clone + PartialEq,
        P: moniker::BoundPattern<N>,
        T: moniker::BoundTerm<N>,
    {
        fn term_eq(&self, other: &Scope<P, T>) -> bool {
            self.inner.term_eq(&other.inner)
        }

        fn close_term(&mut self, state: moniker::ScopeState, on_free: &impl moniker::OnFreeFn<N>) {
            self.inner.close_term(state, on_free)
        }

        fn open_term(&mut self, state: moniker::ScopeState, on_bound: &impl moniker::OnBoundFn<N>) {
            self.inner.open_term(state, on_bound)
        }

        fn visit_vars(&self, on_var: &mut impl FnMut(&moniker::Var<N>)) {
            self.inner.visit_vars(on_var)
        }

        fn visit_mut_vars(&mut self, on_var: &mut impl FnMut(&mut moniker::Var<N>)) {
            self.inner.visit_mut_vars(on_var)
        }
    }

    // Allow cloning from moniker::Scope
    impl<P: Clone, T: Clone> From<MonikerScope<P, T>> for Scope<P, T> {
        fn from(scope: MonikerScope<P, T>) -> Self {
            Scope { inner: scope }
        }
    }
}

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

pub fn get_or_create_var(name: impl Into<String>) -> FreeVar<String> {
    let name = name.into();
    let mut cache = VAR_CACHE.lock().unwrap();
    
    cache.entry(name.clone())
        .or_insert_with(|| FreeVar::fresh_named(name))
        .clone()
}

pub fn clear_var_cache() {
    VAR_CACHE.lock().unwrap().clear();
}

pub fn var_cache_size() -> usize {
    VAR_CACHE.lock().unwrap().len()
}
