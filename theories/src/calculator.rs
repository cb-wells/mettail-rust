#![allow(
    non_local_definitions,
    clippy::crate_in_macro_def,
    clippy::empty_line_after_outer_attr
)]

use mettail_macros::theory;
use std::collections::HashMap;

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

//=============================================================================
// ENVIRONMENT
//=============================================================================

/// Environment for storing variable bindings
#[derive(Debug, Clone)]
pub struct CalculatorEnv {
    vars: HashMap<String, i32>,
}

impl CalculatorEnv {
    /// Create a new empty environment
    pub fn new() -> Self {
        CalculatorEnv {
            vars: HashMap::new(),
        }
    }

    /// Store a variable binding
    pub fn set(&mut self, name: String, value: i32) {
        self.vars.insert(name, value);
    }

    /// Look up a variable value
    pub fn get(&self, name: &str) -> Option<i32> {
        self.vars.get(name).copied()
    }

    /// Clear all bindings
    pub fn clear(&mut self) {
        self.vars.clear();
    }
}

impl Default for CalculatorEnv {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// EVALUATION
//=============================================================================

impl Int {
    /// Evaluate the expression as `i32`.
    /// With native types, `NumLit` contains the integer value directly,
    /// so evaluation is straightforward.
    pub fn eval(&self) -> i32 {
        match self {
            Int::NumLit(n) => *n,
            Int::Add(a, b) => a.eval() + b.eval(),
            Int::Sub(a, b) => a.eval() - b.eval(),
        }
    }
}

/// Substitute variable names with their values in an expression string.
/// Handles cases like "x + 2" by replacing "x" with its value from the environment.
fn substitute_vars(expr: &str, env: &CalculatorEnv) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() || ch == '_' {
            // Start of an identifier
            let mut ident = String::from(ch);
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    ident.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }

            // Look up the variable
            if let Some(val) = env.get(&ident) {
                result.push_str(&val.to_string());
            } else {
                return Err(format!("undefined variable: {}", ident));
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}

/// Parse and evaluate a statement (assignment or expression) with environment.
/// Returns the computed value.
pub fn parse_and_eval_with_env(
    input: &str,
    env: &mut CalculatorEnv,
) -> Result<i32, String> {
    mettail_runtime::clear_var_cache();

    let trimmed = input.trim();

    // Check if it's an assignment
    if let Some(eq_pos) = trimmed.find('=') {
        let var_part = trimmed[..eq_pos].trim();
        let expr_part = trimmed[eq_pos + 1..].trim();

        // Validate variable name (no operators in LHS)
        if !var_part.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid variable name: {}", var_part));
        }

        // Substitute variable references in the RHS expression
        let substituted = substitute_vars(expr_part, env)?;

        // Parse and evaluate the substituted expression
        let parser = calculator::IntParser::new();
        let expr = parser
            .parse(&substituted)
            .map_err(|e| format!("parse error: {:?}", e))?;

        let val = expr.eval();
        env.set(var_part.to_string(), val);
        Ok(val)
    } else {
        // Check if it's a variable reference (bare identifier)
        if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') && !trimmed.chars().next().unwrap_or('0').is_numeric() {
            return env.get(trimmed)
                .ok_or_else(|| format!("undefined variable: {}", trimmed));
        }

        // Substitute variables in the expression
        let substituted = substitute_vars(trimmed, env)?;

        // Parse and evaluate the substituted expression
        let parser = calculator::IntParser::new();
        let expr = parser
            .parse(&substituted)
            .map_err(|e| format!("parse error: {:?}", e))?;

        Ok(expr.eval())
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
