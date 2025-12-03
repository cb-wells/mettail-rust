// Theory implementations for the REPL
//
// Each theory wrapper implements the Theory trait, providing:
// - Parsing (using the theories crate)
// - Ascent execution
// - Result extraction

pub mod rhocalc;
pub mod ambient;

pub use rhocalc::RhoCalculusTheory;
pub use ambient::AmbCalculusTheory;

