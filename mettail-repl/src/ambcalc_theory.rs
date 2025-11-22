use crate::theory::{AscentResults, Rewrite, Term, TermInfo, Theory};
use crate::examples::TheoryName;
use anyhow::Result;
use std::fmt;

use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;

theory! {
    name: Ambient,
    exports {
        Proc
        Name
    },
    terms {
        PZero . Proc ::= "0" ;
        
        PIn . Proc ::= "in(" Name "," Proc ")";
        POut . Proc ::= "out(" Name "," Proc ")";
        POpen . Proc ::= "open(" Name "," Proc ")";
        
        PAmb . Proc ::= Name "[" Proc "]";
        PNew . Proc ::= "new(" <Name> "," Proc ")";

        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;

        PVar . Proc ::= Var;
        NVar . Name ::= Var ;
    },
    equations {
        // (PPar {P}) == P;
        // (PPar {PZero, ...rest}) == (PPar {...rest});
        if x # C then (PPar {P, (PNew x C)}) == (PNew x (PPar {P, C}));
        if x # N then (PNew x (PPar {P, (PIn N Q)})) == (PPar {P, (PIn N (PNew x Q))});
        if x # N then (PNew x (PPar {P, (POut N Q)})) == (PPar {P, (POut N (PNew x Q))});
        if x # N then (PNew x (PPar {P, (POpen N Q)})) == (PPar {P, (POpen N (PNew x Q))});
        if x # N then (PNew x (PPar {P, (PAmb N Q)})) == (PPar {P, (PAmb N (PNew x Q))});
        // (PNew x (PNew y P)) == (PNew y (PNew x P));
    },
    rewrites {

        // {n[{in(m,p), ...q}], m[r]} => {m[{n[{p, ...q}], r}]}
        (PPar {(PAmb N (PPar {(PIn M P) , ...rest})) , (PAmb M R)}) 
            => (PPar {(PAmb M (PPar {(PAmb N (PPar {P , ...rest})), R}))});
        
        // m[{n[{out(m,p), ...q}], r}] => {n[{p, ...q}], m[r]}
        (PAmb M (PPar {(PAmb N (PPar {(POut M P), ...rest})), R}))
            => (PPar {(PAmb N (PPar {P, ...rest})), (PAmb M R)});

        // {open(n,p), n[q]} => {p, q}
        (PPar {(POpen N P), (PAmb N Q)})
            => (PPar {P,Q});

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});

        if S => T then (PNew x S) => (PNew x T);
        if S => T then (PAmb N S) => (PAmb N T);
    }
}

/// RhoCalc theory implementation
pub struct AmbCalculusTheory;

impl Theory for AmbCalculusTheory {
    fn name(&self) -> TheoryName {
        TheoryName::AmbientCalculus
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
        let parser = ambient::ProcParser::new();
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
            include_source!(ambient_source);
            
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
