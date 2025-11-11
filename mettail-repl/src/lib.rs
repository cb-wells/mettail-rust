pub mod theory;
pub mod registry;
pub mod state;
pub mod repl;
pub mod rhocalc_theory;
pub mod examples;
pub mod pretty;

// Re-export eqrel for the generated Ascent code
pub use ascent_byods_rels::eqrel;

pub use theory::{Theory, Term, AscentResults, TermInfo, Rewrite, EquivClass};
pub use registry::{TheoryRegistry, build_registry};
pub use state::{ReplState, HistoryEntry};
pub use repl::Repl;
pub use rhocalc_theory::RhoCalculusTheory;
pub use examples::Example;
pub use pretty::format_term_pretty;
