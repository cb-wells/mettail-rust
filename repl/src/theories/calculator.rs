use crate::examples::TheoryName;
use crate::theory::{AscentResults, Term, TermInfo, Theory};
use anyhow::Result;
use std::fmt;
use std::cell::RefCell;

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
        0 // No rewrite rules defined
    }

    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>> {
        mettail_runtime::clear_var_cache();
        CALC_ENV.with(|env| {
            let mut env_ref = env.borrow_mut();
            let result = parse_and_eval_with_env(input, &mut env_ref)
                .map_err(|e| anyhow::anyhow!("Parse/eval error: {}", e))?;
            Ok(Box::new(CalcTerm(result)) as Box<dyn Term>)
        })
    }

    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults> {
        // Calculator has no rewrite rules or equation derivation
        // So we just return empty results for now
        let calc_term = term
            .as_any()
            .downcast_ref::<CalcTerm>()
            .ok_or_else(|| anyhow::anyhow!("Expected CalcTerm"))?;

        let term_info = TermInfo {
            term_id: compute_term_id(calc_term.0),
            display: format!("{}", calc_term.0),
            is_normal_form: true, // Evaluation result is already normal
        };

        Ok(AscentResults {
            all_terms: vec![term_info],
            rewrites: Vec::new(),
            equivalences: Vec::new(),
        })
    }

    fn format_term(&self, term: &dyn Term) -> String {
        format!("{}", term)
    }
}

/// Wrapper for i32 evaluation result (native type)
#[derive(Clone)]
struct CalcTerm(i32);

impl Term for CalcTerm {
    fn clone_box(&self) -> Box<dyn Term> {
        Box::new(self.clone())
    }

    fn term_id(&self) -> u64 {
        compute_term_id(self.0)
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

/// Compute a unique ID for a term
fn compute_term_id(val: i32) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}
