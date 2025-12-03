// MeTTaIL Theory Definitions Library
//
// This crate contains the core theory definitions used across examples and the REPL.
// Each theory is defined in its own module using the theory! macro.

pub mod rhocalc;
pub mod ambient;

// Note: Both rhocalc and ambient export Proc and Name types
// Users should import from specific modules to avoid ambiguity:
//   use mettail_theories::rhocalc::*;
//   use mettail_theories::ambient::*;
