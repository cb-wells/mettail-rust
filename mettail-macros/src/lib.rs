mod ast;
mod validator;
mod codegen;
mod typechecker;
mod error;
mod parser_gen;
mod substitution;

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, abort};
use syn::parse_macro_input;

use ast::TheoryDef;
use validator::validate_theory;
use codegen::generate_ast;

#[proc_macro]
#[proc_macro_error]
pub fn theory(input: TokenStream) -> TokenStream {
    let theory_def = parse_macro_input!(input as TheoryDef);
    
    if let Err(e) = validate_theory(&theory_def) {
        let span = e.span();
        let msg = e.message();
        abort!(span, "{}", msg);
    }
    
    let generated = generate_ast(&theory_def);
    
    TokenStream::from(generated)
}

