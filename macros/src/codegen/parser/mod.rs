//! Parser generation (LALRPOP integration)
//!
//! Generates LALRPOP grammar files and writes them to the filesystem.

mod actions;
mod lalrpop;
mod writer;

pub use lalrpop::*;
pub use writer::*;
