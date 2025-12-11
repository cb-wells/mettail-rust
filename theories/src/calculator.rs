#![allow(
    non_local_definitions,
    clippy::crate_in_macro_def,
    clippy::empty_line_after_outer_attr
)]

use mettail_macros::theory;

// Simple integer calculator theory: supports integer literals, + and -
theory! {
    name: Calculator,
    exports {
        Expr
    },
    terms {
        // We represent numeric literals as Vars (identifier tokens) so that
        // the generated parser can capture them. We will pre-process input
        // to prefix numeric tokens with a letter (e.g. `3` -> `n3`) so they
        // match the generated `Ident` token. The `NumLit` variant carries
        // an `OrdVar` which we later read the pretty_name from.
        NumLit . Expr ::= Var ;

        Add . Expr ::= Expr "+" Expr ;
        Sub . Expr ::= Expr "-" Expr ;
    },
    equations {
    },
    rewrites {
    }
}

impl Expr {
    /// Evaluate the expression as `i64`. Numeric literals must be parsed
    /// as identifiers prefixed with a letter (e.g. `n42`).
    pub fn eval(&self) -> i64 {
        match self {
            Expr::NumLit(ordvar) => {
                // Extract pretty_name from the underlying Var (Free or Bound)
                match &ordvar.0 {
                    mettail_runtime::Var::Free(fv) => {
                        if let Some(name) = fv.pretty_name.as_ref() {
                            // strip optional leading non-digit prefix (e.g. 'n')
                            let digits: String = name
                                .chars()
                                .skip_while(|c| !c.is_ascii_digit())
                                .collect::<String>();
                            digits.parse::<i64>().unwrap_or(0)
                        } else {
                            0
                        }
                    },
                    mettail_runtime::Var::Bound(bv) => {
                        if let Some(name) = bv.pretty_name.as_ref() {
                            let digits: String = name
                                .chars()
                                .skip_while(|c| !c.is_ascii_digit())
                                .collect::<String>();
                            digits.parse::<i64>().unwrap_or(0)
                        } else {
                            0
                        }
                    },
                }
            },
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
        }
    }
}

/// Preprocess an input string by prefixing integer tokens with `n` so they
/// match the generated `Ident` token (which requires a leading letter or `_`).
fn preprocess_numbers(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            let mut num = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            out.push('n');
            out.push_str(&num);
        } else {
            out.push(c);
            chars.next();
        }
    }

    out
}

/// Parse an input string (simple integers and + / -) and evaluate it.
pub fn parse_and_eval(input: &str) -> Result<i64, String> {
    // Clear var cache so variable identities are fresh for this parse
    mettail_runtime::clear_var_cache();

    // Preprocess numeric tokens so they become valid identifiers
    let pre = preprocess_numbers(input);

    // Use the generated parser module (lalrpop will generate `calculator` module)
    let parser = calculator::ExprParser::new();
    // The parser returns our generated `Expr` type
    let expr = parser
        .parse(&pre)
        .map_err(|e| format!("parse error: {:?}", e))?;

    Ok(expr.eval())
}
