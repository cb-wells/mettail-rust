// Helper module for writing generated LALRPOP grammars to files
// This is used during macro expansion or build process

use std::fs;
use std::path::Path;
use crate::lalrpop_gen::generate_lalrpop_grammar;
use crate::ast::TheoryDef;

/// Write a LALRPOP grammar file for a theory
/// 
/// This writes the generated grammar to the directory of the crate where the macro is invoked.
/// For library crates, this is src/. For binary crates, this is the root.
pub fn write_grammar_file(theory_name: &str, grammar_content: &str) -> std::io::Result<()> {
    // Get the manifest directory of the crate that's using the macro
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    
    let base_dir = Path::new(&manifest_dir);
    
    // Try src/ first (for library crates), fall back to root (for binary crates)
    let src_dir = base_dir.join("src");
    let output_dir = if src_dir.exists() {
        src_dir
    } else {
        // For binary-only crates, put it in the root
        base_dir.to_path_buf()
    };
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&output_dir).ok();
    
    let theory_name_lower = theory_name.to_lowercase();
    let file_path = output_dir.join(format!("{}.lalrpop", theory_name_lower));
    
    fs::write(&file_path, grammar_content)?;
    
    eprintln!("Generated LALRPOP grammar: {}", file_path.display());
    Ok(())
}

/// Generate grammar file path for a theory
pub fn grammar_file_name(theory: &TheoryDef) -> String {
    format!("{}.lalrpop", theory.name.to_string().to_lowercase())
}