pub mod theory;
pub mod theories;
pub mod registry;
pub mod state;
pub mod repl;
pub mod examples;
pub mod pretty;

// Re-export eqrel for the generated Ascent code
pub use ascent_byods_rels::eqrel;

pub use theory::{Theory, Term, AscentResults, TermInfo, Rewrite, EquivClass};
pub use theories::{RhoCalculusTheory, AmbCalculusTheory};
pub use registry::{TheoryRegistry, build_registry};
pub use state::{ReplState, HistoryEntry};
pub use repl::Repl;
pub use examples::Example;
pub use pretty::format_term_pretty;
