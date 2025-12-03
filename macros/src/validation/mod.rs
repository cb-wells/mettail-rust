//! Validation and type checking for theory definitions
//! 
//! This module provides semantic validation, type checking, and error reporting
//! for theory definitions parsed by the theory! macro.

mod error;
mod validator;
mod typechecker;

pub use error::*;
pub use validator::*;
pub use typechecker::*;

