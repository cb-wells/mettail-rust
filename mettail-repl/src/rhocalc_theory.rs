use crate::theory::{AscentResults, Rewrite, Term, TermInfo, Theory};
use anyhow::Result;
use std::fmt;

use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;

// Define the RhoCalc theory directly here so we have access to rhocalc_source!
theory! {
    name: RhoCalc,
    
    exports {
        Proc
        Name
    },
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;
        PDrop . Proc ::= "*" Name ;

        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    },
    
    equations {
        (NQuote (PDrop N)) == N ;
        (PPar {P}) == P;
        (PPar {}) == PZero ;
    },
        
    rewrites {
        (PPar {(PInput chan x P) , (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
        
        (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
    }
}

/// RhoCalc theory implementation
pub struct RhoCalculusTheory;

impl Theory for RhoCalculusTheory {
    fn name(&self) -> &str {
        "rhocalc"
    }

    fn categories(&self) -> Vec<String> {
        vec!["Proc".to_string(), "Name".to_string()]
    }

    fn constructor_count(&self) -> usize {
        8 // PZero, PInput, POutput, PPar, PDrop, NQuote, NVar, + Var
    }

    fn equation_count(&self) -> usize {
        3 // (NQuote (PDrop N)) == N, (PPar {P}) == P, (PPar {}) == PZero
    }

    fn rewrite_count(&self) -> usize {
        3 // Communication, Drop, Congruence
    }

    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>> {
        mettail_runtime::clear_var_cache();
        let parser = rhocalc::ProcParser::new();
        let proc = parser
            .parse(input)
            .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;
        Ok(Box::new(RhoTerm(proc)))
    }

    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults> {
        // Downcast to RhoTerm
        let rho_term = term
            .as_any()
            .downcast_ref::<RhoTerm>()
            .ok_or_else(|| anyhow::anyhow!("Expected RhoTerm"))?;

        let initial_proc = rho_term.0.clone();

        // Run Ascent with the generated source (rhocalc_source! is defined by the theory! macro above)
        let prog = ascent::ascent_run! {
            include_source!(rhocalc_source);
            
            // Seed the initial term
            proc(initial_proc.clone());
        };

        // Extract results
        let all_procs: Vec<Proc> = prog.proc.iter().map(|(p,)| p.clone()).collect();
        let rewrites: Vec<(Proc, Proc)> = prog.rw_proc.iter().map(|(from, to)| (from.clone(), to.clone())).collect();

        // Build term info
        let mut term_infos = Vec::new();

        for proc in &all_procs {
            let term_id = compute_term_id(proc);

            // Check if it's a normal form (no outgoing rewrites)
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
        let equivalences = Vec::new(); // TODO: Extract from prog.eq_proc

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
struct RhoTerm(Proc);

impl Term for RhoTerm {
    fn clone_box(&self) -> Box<dyn Term> {
        Box::new(self.clone())
    }

    fn term_id(&self) -> u64 {
        compute_term_id(&self.0)
    }

    fn term_eq(&self, other: &dyn Term) -> bool {
        if let Some(other_rho) = other.as_any().downcast_ref::<RhoTerm>() {
            self.0 == other_rho.0
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for RhoTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for RhoTerm {
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
