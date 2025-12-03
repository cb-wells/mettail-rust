//! Runtime support for MeTTaIL-generated code
//! 
//! This crate provides:
//! - Variable binding support (via moniker wrappers)
//! - Collection types (HashBag for associative-commutative operations)
//! - Utility functions for parsing and variable management

// Variable binding support
mod binding;
pub use binding::*;

// Collection types
mod hashbag;
pub use hashbag::HashBag;

// Re-export LALRPOP utilities for generated parsers
pub use lalrpop_util::ParseError as LalrpopParseError;
