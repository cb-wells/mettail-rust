mod ast;
mod validator;
mod codegen;
mod typechecker;
mod error;
mod parser_gen;
mod substitution;
mod lalrpop_gen;
mod grammar_writer;
mod display_gen;
mod rewrite_gen;

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, abort};
use syn::parse_macro_input;

use ast::TheoryDef;
use validator::validate_theory;
use codegen::generate_ast;
use lalrpop_gen::generate_lalrpop_grammar;
use grammar_writer::write_grammar_file;
use rewrite_gen::generate_rewrite_engine;

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
    
    // Generate rewrite engine
    let rewrite_code = generate_rewrite_engine(&theory_def);
    
    // Generate LALRPOP grammar file with precedence handling
    let grammar = generate_lalrpop_grammar(&theory_def);
    if let Err(e) = write_grammar_file(&theory_def.name.to_string(), &grammar) {
        eprintln!("Warning: Failed to write LALRPOP grammar: {}", e);
    }
    
    let combined = quote::quote! {
        #ast_code
        #rewrite_code
    };
    
    TokenStream::from(combined)
}

