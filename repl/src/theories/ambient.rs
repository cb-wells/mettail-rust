use crate::theory::{AscentResults, EquivClass, Rewrite, Term, TermInfo, Theory};
use crate::examples::TheoryName;
use anyhow::Result;
use std::fmt;

// Import the theory definition from the theories crate
use mettail_theories::ambient::*;

/// Ambient Calculus theory implementation for REPL
pub struct AmbCalculusTheory;

impl Theory for AmbCalculusTheory {
    fn name(&self) -> TheoryName {
        TheoryName::AmbientCalculus
    }

    fn categories(&self) -> Vec<String> {
        vec!["Proc".to_string(), "Name".to_string()]
    }

    fn constructor_count(&self) -> usize {
        8 // PZero, PIn, POut, POpen, PAmb, PNew, PPar, PVar, NVar
    }

    fn equation_count(&self) -> usize {
        6 // Various scope extrusion equations
    }

    fn rewrite_count(&self) -> usize {
        5 // In, Out, Open, plus 2 congruences
    }

    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>> {
        mettail_runtime::clear_var_cache();
        let parser = ambient::ProcParser::new();
        let proc = parser
            .parse(input)
            .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;
        Ok(Box::new(AmbTerm(proc)))
    }

    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults> {
        use ascent::*;
        
        // Downcast to AmbTerm
        let amb_term = term
            .as_any()
            .downcast_ref::<AmbTerm>()
            .ok_or_else(|| anyhow::anyhow!("Expected AmbTerm"))?;

        let initial_proc = amb_term.0.clone();

        // Run Ascent with the generated source
        let prog = ascent_run! {
            include_source!(ambient_source);
            
            proc(initial_proc.clone());
        };

        // Extract results
        let all_procs: Vec<Proc> = prog.proc.iter().map(|(p,)| p.clone()).collect();
        let rewrites: Vec<(Proc, Proc)> = prog.rw_proc.iter().map(|(from, to)| (from.clone(), to.clone())).collect();

        // Build term info
        let mut term_infos = Vec::new();

        for proc in &all_procs {
            let term_id = compute_term_id(proc);
            let has_rewrites = rewrites.iter().any(|(from, _)| from == proc);

            term_infos.push(TermInfo {
                term_id,
                display: format!("{}", proc),
                is_normal_form: !has_rewrites,
            });
        }

        // Build rewrite list
        let rewrite_list: Vec<Rewrite> = rewrites
            .iter()
            .map(|(from, to)| Rewrite {
                from_id: compute_term_id(from),
                to_id: compute_term_id(to),
                rule_name: Some("rewrite".to_string()),
            })
            .collect();

        // Build equivalence classes (from eq_proc)
        let mut equivalences = Vec::new(); 
        for (lhs, rhs) in prog.__eq_proc_ind_common.iter_all_added() {
            if lhs.to_string() != rhs.to_string() {
                equivalences.push(EquivClass {
                    term_ids: vec![compute_term_id(lhs), compute_term_id(rhs)],
                });
            }
        }

        Ok(AscentResults {
            all_terms: term_infos,
            rewrites: rewrite_list,
            equivalences,
        })
    }

    fn format_term(&self, term: &dyn Term) -> String {
        format!("{}", term)
    }
}

/// Wrapper for Proc that implements Term
#[derive(Clone)]
struct AmbTerm(Proc);

impl Term for AmbTerm {
    fn clone_box(&self) -> Box<dyn Term> {
        Box::new(self.clone())
    }

    fn term_id(&self) -> u64 {
        compute_term_id(&self.0)
    }

    fn term_eq(&self, other: &dyn Term) -> bool {
        if let Some(other_amb) = other.as_any().downcast_ref::<AmbTerm>() {
            self.0 == other_amb.0
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for AmbTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for AmbTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Compute a unique ID for a term (simple hash for now)
fn compute_term_id(proc: &Proc) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    proc.hash(&mut hasher);
    hasher.finish()
}
