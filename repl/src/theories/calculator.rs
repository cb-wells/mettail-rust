use crate::examples::TheoryName;
use crate::theory::{AscentResults, Rewrite, Term, TermInfo, Theory};
use anyhow::Result;
use std::cell::RefCell;
use std::fmt;

// Import the theory definition from the theories crate
use mettail_theories::calculator::*;

thread_local! {
    static CALC_ENV: RefCell<CalculatorEnv> = RefCell::new(CalculatorEnv::new());
}

/// Calculator theory implementation for REPL
pub struct CalculatorTheory;

impl Theory for CalculatorTheory {
    fn name(&self) -> TheoryName {
        TheoryName::Calculator
    }

    fn categories(&self) -> Vec<String> {
        vec!["Int".to_string(), "Ident".to_string()]
    }

    fn constructor_count(&self) -> usize {
        5 // NumLit, Add, Sub, VarRef, Assign
    }

    fn equation_count(&self) -> usize {
        0 // No equations defined
    }

    fn rewrite_count(&self) -> usize {
        1 // Variable substitution rewrite rule
    }

    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>> {
        mettail_runtime::clear_var_cache();

        let trimmed = input.trim();

        // Parse to Int AST
        let parser = calculator::IntParser::new();
        let expr = parser
            .parse(trimmed)
            .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

        // Check if it's an assignment
        if let Int::Assign(var, rhs) = &expr {
            let expr_clone = expr.clone();
            // Handle assignment: evaluate RHS, update environment, return result
            CALC_ENV.with(|env| {
                let mut env_ref = env.borrow_mut();

                // Get environment facts
                let env_facts = env_to_facts(&env_ref);

                // Use Ascent to evaluate the RHS
                use ascent::*;
                let prog = ascent_run! {
                    include_source!(calculator_source);

                    int(rhs.as_ref().clone());

                    // Seed environment facts
                    env_var(n.clone(), v) <-- for (n, v) in env_facts.clone();
                };

                // Find the normal form of the RHS
                let rewrites: Vec<(Int, Int)> = prog
                    .rw_int
                    .iter()
                    .map(|(from, to)| (from.clone(), to.clone()))
                    .collect();

                let mut current = rhs.as_ref().clone();
                while let Some((_, next)) = rewrites.iter().find(|(from, _)| from == &current) {
                    current = next.clone();
                }

                // Try to evaluate the normal form
                // eval() panics if there are unevaluated terms, so we need to handle that
                let result = std::panic::catch_unwind(|| current.eval())
                    .map_err(|_| anyhow::anyhow!("Assignment RHS contains undefined variables"))?;

                // Update environment
                if let Some(var_name) = match var {
                    mettail_runtime::OrdVar(mettail_runtime::Var::Free(ref fv)) => {
                        fv.pretty_name.clone()
                    },
                    _ => None,
                } {
                    env_ref.set(var_name, result);
                }

                // Return the assignment term
                Ok(Box::new(CalcTerm(expr_clone)) as Box<dyn Term>)
            })
        } else {
            // Not an assignment - evaluate the expression using Ascent to get normal form
            CALC_ENV.with(|env| {
                let env_facts = env_to_facts(&env.borrow());

                use ascent::*;
                let prog = ascent_run! {
                    include_source!(calculator_source);

                    int(expr.clone());

                    // Seed environment facts
                    env_var(n.clone(), v) <-- for (n, v) in env_facts.clone();
                };

                // Find the normal form of the expression
                let rewrites: Vec<(Int, Int)> = prog
                    .rw_int
                    .iter()
                    .map(|(from, to)| (from.clone(), to.clone()))
                    .collect();

                let mut current = expr.clone();
                while let Some((_, next)) = rewrites.iter().find(|(from, _)| from == &current) {
                    current = next.clone();
                }

                Ok(Box::new(CalcTerm(current)) as Box<dyn Term>)
            })
        }
    }

    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults> {
        use ascent::*;

        let calc_term = term
            .as_any()
            .downcast_ref::<CalcTerm>()
            .ok_or_else(|| anyhow::anyhow!("Expected CalcTerm"))?;

        let initial_int = calc_term.0.clone();

        // Get environment facts from thread-local storage
        let env_facts: Vec<(String, i32)> = CALC_ENV.with(|env| env_to_facts(&env.borrow()));

        // Run Ascent with the generated source
        // Seed env_var facts using a rule that iterates over the collection
        let prog = ascent_run! {
            include_source!(calculator_source);

            int(initial_int.clone());

            // Seed environment facts from the vector
            env_var(n.clone(), v) <-- for (n, v) in env_facts.clone();
        };

        // Extract results from Ascent relations
        let all_ints: Vec<Int> = prog.int.iter().map(|(i,)| i.clone()).collect();
        let rewrites: Vec<(Int, Int)> = prog
            .rw_int
            .iter()
            .map(|(from, to)| (from.clone(), to.clone()))
            .collect();

        // Build term info (similar to rhocalc/ambient)
        let mut term_infos = Vec::new();
        for int_term in &all_ints {
            let term_id = compute_term_id(int_term);
            let has_rewrites = rewrites.iter().any(|(from, _)| from == int_term);

            term_infos.push(TermInfo {
                term_id,
                display: format!("{}", int_term),
                is_normal_form: !has_rewrites,
            });
        }

        // Build rewrite list
        let rewrite_list: Vec<Rewrite> = rewrites
            .iter()
            .map(|(from, to)| Rewrite {
                from_id: compute_term_id(from),
                to_id: compute_term_id(to),
                rule_name: Some("var_substitution".to_string()),
            })
            .collect();

        Ok(AscentResults {
            all_terms: term_infos,
            rewrites: rewrite_list,
            equivalences: Vec::new(), // Calculator has no equations
        })
    }

    fn format_term(&self, term: &dyn Term) -> String {
        if let Some(calc_term) = term.as_any().downcast_ref::<CalcTerm>() {
            // Try to evaluate the term
            match std::panic::catch_unwind(|| calc_term.0.eval()) {
                Ok(value) => format!("{}", value),
                Err(_) => format!("{}", calc_term.0),
            }
        } else {
            format!("{}", term)
        }
    }
}

/// Wrapper for Int AST that implements Term
#[derive(Clone)]
struct CalcTerm(Int);

impl Term for CalcTerm {
    fn clone_box(&self) -> Box<dyn Term> {
        Box::new(self.clone())
    }

    fn term_id(&self) -> u64 {
        compute_term_id(&self.0)
    }

    fn term_eq(&self, other: &dyn Term) -> bool {
        if let Some(other_calc) = other.as_any().downcast_ref::<CalcTerm>() {
            self.0 == other_calc.0
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for CalcTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CalcTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Compute a unique ID for an Int term
fn compute_term_id(term: &Int) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    term.hash(&mut hasher);
    hasher.finish()
}
