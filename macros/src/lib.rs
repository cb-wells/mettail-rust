//! MeTTaIL procedural macro for defining formal language theories
//! 
//! This crate provides the `theory!` macro which defines a formal language with:
//! - AST types (Rust enums)
//! - Parser (LALRPOP-generated)
//! - Rewrite engine (Ascent-based)
//! - Term generation and manipulation

mod ast;
mod validation;
mod codegen;
mod utils;

// Ascent generation modules
mod ascent;           // Organized Ascent generation

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, abort};
use syn::parse_macro_input;

use ast::TheoryDef;
use validation::validate_theory;
use codegen::generate_ast;
use codegen::parser::{generate_lalrpop_grammar, write_grammar_file};
use ascent::generate_freshness_functions;
use ascent::generate_ascent_source;

#[proc_macro]
#[proc_macro_error]
pub fn theory(input: TokenStream) -> TokenStream {
    let theory_def = parse_macro_input!(input as TheoryDef);
    
    if let Err(e) = validate_theory(&theory_def) {
        let span = e.span();
        let msg = e.message();
        abort!(span, "{}", msg);
    }
    
    // Generate the Rust AST types
    let ast_code = generate_ast(&theory_def);
    
    // Generate freshness functions (needed by Ascent rewrite clauses)
    let freshness_fns = generate_freshness_functions(&theory_def);
    
    // Generate Ascent datalog source (includes rewrites as Ascent clauses)
    let ascent_code = generate_ascent_source(&theory_def);

    // Generate LALRPOP grammar file with precedence handling
    let grammar = generate_lalrpop_grammar(&theory_def);
    if let Err(e) = write_grammar_file(&theory_def.name.to_string(), &grammar) {
        eprintln!("Warning: Failed to write LALRPOP grammar: {}", e);
    }
    
    let combined = quote::quote! {
        #ast_code
        #freshness_fns
        #ascent_code
    };
    
    TokenStream::from(combined)
}
