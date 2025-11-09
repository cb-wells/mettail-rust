use mettail_macros::theory;
use mettail_runtime;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;
use std::time::Instant;

use ascent::*;

theory! {
    name: RhoCalc,
    
    exports {
        Proc
        Name
    },
    
    terms {
        PZero . Proc ::= "0" ;
        PDrop . Proc ::= "*" Name ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        PPar . Proc ::= Proc "|" Proc ;

        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    },
    
    equations {
        (PPar P Q) == (PPar Q P) ;
        (PPar P (PPar Q R)) == (PPar (PPar P Q) R) ;
        
        (NQuote (PDrop N)) == N ;
    },
    
    rewrites {
        if x # Q then (PPar (PInput chan x P) (POutput chan Q))
            => (subst P x (NQuote Q));
        
        (PDrop (NQuote P)) => P ;

        if S => T then (PPar P S) => (PPar P T);
    }
} 

fn main() {

    let start_time = Instant::now();
    
    // let vars = vec!["a".to_string(), "b".to_string()];
    // let terms = Proc::generate_terms(&vars, 2);
    // let redex = Proc::generate_random_at_depth(&vars, 6);
    // println!("Term: {}", term);
    
    let rdx_str = "a!(0)|for(@(*a)->x0){*x0}";
    mettail_runtime::clear_var_cache();
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();
    println!("Initial: {}", redex.clone());

    let prog = ascent_run! {
        include_source!(rhocalc_source);
        
        // Seed the initial term
        proc(p) <-- for p in [redex.clone()];
        
        relation redex_eq(Proc);
        redex_eq(q.clone()) <-- eq_proc(redex.clone(), q);
        proc(q) <-- redex_eq(q);
        
        relation path(Proc, Proc);
        path(redex.clone(), redex.clone()) <-- for _ in [()];
        path(redex.clone(), q.clone()) <-- redex_eq(q);
        path(p.clone(),q.clone()) <-- rw_proc(p,q);
        path(p.clone(),r.clone()) <-- rw_proc(p,q), path(q.clone(),r);
        
        relation is_normal_form(Proc);
        is_normal_form(t.clone()) <-- proc(t), !rw_proc(t.clone(),_);
        
        relation path_full(Proc,Proc);
        path_full(redex.clone(),z.clone()) <-- is_normal_form(z), path(redex.clone(), z);
    };

    let mut procs = prog.proc;
    procs.sort_by(|a,b| a.0.cmp(&b.0));

    println!("Terms: {}", procs.len());
    println!("Rewrites: {}", prog.rw_proc.len());
    println!("Normal forms: {}", prog.is_normal_form.len());
    
    // Debug: print proc facts
    println!("\n=== proc facts ===");
    println!("Count: {}", procs.len());
    for p in procs.iter().take(10) {
        println!("  {}", p.0);
    }
    
    // Debug: print name facts
    println!("\n=== name facts ===");
    let mut names: Vec<_> = prog.name.iter().collect();
    names.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    println!("Count: {}", names.len());
    for n in names.iter() {
        println!("  {}", n.0);
    }
    
    // Debug: print eq_name facts
    println!("\n=== eq_name facts ===");
    let mut eq_names: Vec<_> = prog.eq_name.iter().collect();
    eq_names.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    for (n1, n2) in eq_names.iter().take(20) {
        println!("  {} == {}", n1, n2);
    }
    
    let mut path_full = prog.path_full.clone();
    path_full.sort_by(|a,b| a.0.cmp(&b.0));
    
    println!("\n=== Paths to normal forms ===");
    println!("Count: {}", path_full.len());
    for (s, t) in path_full {
        println!("  {} ~> {}", s, t);
    }

    // get elapsed time 
    let elapsed = Instant::now().duration_since(start_time);
    println!("Time: {:?}", elapsed);    
}

