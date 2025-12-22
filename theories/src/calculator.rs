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

// Type alias for backward compatibility
pub use CalculatorIntEnv as CalculatorEnv;

/// Parse and evaluate a statement (assignment or expression) with environment.
/// Returns the computed value.
/// This is a manual helper function that replaces the previously auto-generated version.
pub fn parse_and_eval_with_env(
    input: &str,
    env: &mut CalculatorIntEnv,
) -> Result<i32, String> {
    use ascent::*;

    mettail_runtime::clear_var_cache();

    let trimmed = input.trim();

    // Parse the input (handles both assignments and expressions)
    let parser = calculator::IntParser::new();
    let term = parser
        .parse(trimmed)
        .map_err(|e| format!("parse error: {:?}", e))?;

    // Get environment facts - convert Int enum to i32 for Ascent
    let env_facts: Vec<(String, i32)> = env
        .env_to_facts()
        .into_iter()
        .map(|(name, val)| {
            // Extract i32 from Int enum (NumLit variant)
            match val {
                Int::NumLit(v) => Ok((name, v)),
                _ => Err("Environment value must be a NumLit".to_string()),
            }
        })
        .collect::<Result<Vec<_>, String>>()?;

    // Run Ascent to rewrite to normal form
    let prog = ascent_run! {
        include_source!(calculator_source);

        int(term.clone());

        // Seed environment facts
        env_var(n.clone(), v) <-- for (n, v) in env_facts.clone();
    };

    // Find normal form (term with no outgoing rewrites)
    let rewrites: Vec<(Int, Int)> = prog
        .rw_int
        .iter()
        .map(|(from, to)| (from.clone(), to.clone()))
        .collect();

    let mut current = term;
    loop {
        // Find rewrite from current term
        if let Some((_, next)) = rewrites.iter().find(|(from, _)| from == &current) {
            current = next.clone();
        } else {
            // No more rewrites - this is the normal form
            break;
        }
    }

    // Handle assignments: extract value and update environment
    if let Int::Assign(var, rhs) = &current {
        // Extract value from RHS (should be a NumLit after rewriting)
        let val = match rhs.as_ref() {
            Int::NumLit(v) => *v,
            _ => {
                // Try to evaluate if not a NumLit
                // Check for undefined variables first by looking for VarRef
                if has_var_ref(rhs) {
                    return Err("undefined variable in expression".to_string());
                }
                // Try to evaluate
                rhs.eval()
            }
        };

        // Update environment if we have a variable name
        if let Some(var_name) = match var {
            mettail_runtime::OrdVar(mettail_runtime::Var::Free(ref fv)) => {
                fv.pretty_name.clone()
            }
            _ => None,
        } {
            env.set(var_name, Int::NumLit(val));
        }

        Ok(val)
    } else {
        // Not an assignment - extract value from normal form
        match &current {
            Int::NumLit(v) => Ok(*v),
            _ => {
                // Check for undefined variables
                if has_var_ref(&current) {
                    return Err("undefined variable in expression".to_string());
                }
                // Try to evaluate
                Ok(current.eval())
            }
        }
    }
}

/// Helper function to check if a term contains a VarRef (undefined variable)
fn has_var_ref(term: &Int) -> bool {
    match term {
        Int::VarRef(_) => true,
        Int::NumLit(_) => false,
        Int::Add(l, r) => has_var_ref(l) || has_var_ref(r),
        Int::Sub(l, r) => has_var_ref(l) || has_var_ref(r),
        Int::Assign(_, rhs) => has_var_ref(rhs),
    }
}
