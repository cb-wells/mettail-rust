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
        // Variables parse as VarRef nodes
        VarRef . Int ::= Var ;
        // Integer literals - uses Integer keyword for native integer type
        NumLit . Int ::= Integer ;

        Add . Int ::= Int "+" Int ;
        Sub . Int ::= Int "-" Int ;
        
        // Assignment: x = expr evaluates expr and stores result
        Assign . Int ::= Var "=" Int ;
    },
    equations {
    },
    rewrites {
        // Variable substitution: if env_var(x, v) then VarRef(x) => NumLit(v)
        if env_var(x, v) then (VarRef x) => (NumLit v);
        
        // Congruence rules: propagate rewrites through Add, Sub, and Assign
        if S => T then (Add S R) => (Add T R);
        if S => T then (Add L S) => (Add L T);
        if S => T then (Sub S R) => (Sub T R);
        if S => T then (Sub L S) => (Sub L T);
        if S => T then (Assign x S) => (Assign x T);
    },
    semantics {
        Add: +,
        Sub: -,
    }
}

//=============================================================================
// EVALUATION
//=============================================================================
// Note: eval() method, CalculatorEnv, env_to_facts, and rewrite_to_normal_form
// are now generated automatically by the theory! macro

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

        // Parse the RHS expression (may contain variables)
        let parser = calculator::IntParser::new();
        let expr = parser
            .parse(expr_part)
            .map_err(|e| format!("parse error: {:?}", e))?;

        // Use Ascent to rewrite to normal form (generated function)
        let normal_form = rewrite_to_normal_form(expr, env)?;

        // Check for remaining variables (undefined variables)
        if has_var_ref(&normal_form) {
            return Err("undefined variable in expression".to_string());
        }

        let val = normal_form.eval();
        env.set(var_part.to_string(), val);
        Ok(val)
    } else {
        // Parse the expression (may be a variable reference or expression)
        let parser = calculator::IntParser::new();
        let expr = parser
            .parse(trimmed)
            .map_err(|e| format!("parse error: {:?}", e))?;

        // Use Ascent to rewrite to normal form (generated function)
        let normal_form = rewrite_to_normal_form(expr, env)?;

        // Check for remaining variables (undefined variables)
        if has_var_ref(&normal_form) {
            // Try to extract variable name for better error message
            if let Some(var_name) = extract_var_name(&normal_form) {
                return Err(format!("undefined variable: {}", var_name));
            }
            return Err("undefined variable".to_string());
        }

        Ok(normal_form.eval())
    }
}

/// Check if a term contains any VarRef nodes
fn has_var_ref(term: &Int) -> bool {
    match term {
        Int::VarRef(_) => true,
        Int::NumLit(_) => false,
        Int::Add(a, b) => has_var_ref(a) || has_var_ref(b),
        Int::Sub(a, b) => has_var_ref(a) || has_var_ref(b),
        Int::Assign(_, expr) => has_var_ref(expr),
    }
}

/// Extract variable name from a VarRef term (for error messages)
fn extract_var_name(term: &Int) -> Option<String> {
    match term {
        Int::VarRef(ord_var) => {
            match ord_var {
                mettail_runtime::OrdVar(mettail_runtime::Var::Free(ref fv)) => {
                    fv.pretty_name.clone()
                }
                _ => None,
            }
        }
        Int::NumLit(_) => None,
        Int::Add(a, b) => extract_var_name(a).or_else(|| extract_var_name(b)),
        Int::Sub(a, b) => extract_var_name(a).or_else(|| extract_var_name(b)),
        Int::Assign(_, expr) => extract_var_name(expr),
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
