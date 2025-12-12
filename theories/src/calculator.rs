#![allow(
    non_local_definitions,
    clippy::crate_in_macro_def,
    clippy::empty_line_after_outer_attr
)]

use mettail_macros::theory;

// Simple integer calculator theory: supports integer literals, + and -
// Uses native i32 type for direct integer support
theory! {
    name: Calculator,
    exports {
        ![i32] as Int
    },
    terms {
        // Numeric literals parse directly to i32 values
        NumLit . Int ::= Var ;

        Add . Int ::= Int "+" Int ;
        Sub . Int ::= Int "-" Int ;
    },
    equations {
    },
    rewrites {
        // TODO: Add native operation rewrites once AST supports them
        // For now, evaluation happens via the eval() method
    }
}

impl Int {
    /// Evaluate the expression as `i32`. With native types, `NumLit` contains
    /// the integer value directly, so evaluation is straightforward.
    pub fn eval(&self) -> i32 {
        match self {
            Int::NumLit(n) => *n,
            Int::Add(a, b) => a.eval() + b.eval(),
            Int::Sub(a, b) => a.eval() - b.eval(),
        }
    }
}

/// Parse an input string (simple integers and + / -) and evaluate it.
/// With native types, integers parse directly without preprocessing.
pub fn parse_and_eval(input: &str) -> Result<i32, String> {
    // Clear var cache so variable identities are fresh for this parse
    mettail_runtime::clear_var_cache();

    // Use the generated parser module (lalrpop will generate `calculator` module)
    // With native types, integers parse directly via the Integer token
    let parser = calculator::IntParser::new();
    // The parser returns our generated `Int` type
    let expr = parser
        .parse(input)
        .map_err(|e| format!("parse error: {:?}", e))?;

    Ok(expr.eval())
}
