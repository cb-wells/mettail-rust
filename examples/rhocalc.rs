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

        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;

        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    },
    
    equations {
        (NQuote (PDrop N)) == N ;
    },
        
    rewrites {
        (PPar {(PInput chan x P), (POutput chan Q)})
            => (PPar {(subst P x (NQuote Q))});
        
        (PDrop (NQuote P)) => P;

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
    }
} 

fn main() {

    // let start_time = Instant::now();
    
    // ===== TERM GENERATION DEMONSTRATION =====
    
    // 1. Exhaustive generation
    println!("=== Exhaustive Term Generation ===");
    let vars = vec!["a".to_string(), "b".to_string()];
    let max_depth = 2;
    let max_collection_width = 3;
    
    let all_procs = Proc::generate_terms(&vars, max_depth, max_collection_width);
    println!("Generated {} Proc terms at depth {} with collection width {}:", 
             all_procs.len(), max_depth, max_collection_width);
    for (i, term) in all_procs.iter().enumerate() {
        println!("  {}: {}", i, term);
    }
    println!();
    
    // 2. Random generation
    println!("=== Random Term Generation ===");
    let random_depth = 3;
    let random_width = 4;
    println!("Generating 10 random terms at depth {} with collection width {}:", 
             random_depth, random_width);
    for i in 0..10 {
        let term = Proc::generate_random_at_depth(&vars, random_depth, random_width);
        println!("  {}: {}", i, term);
    }
    println!();
    
    // 3. Deterministic random generation with seed
    println!("=== Deterministic Random Generation (with seed) ===");
    let seed = 42;
    println!("Generating 3 terms with seed {}:", seed);
    for i in 0..3 {
        let term = Proc::generate_random_at_depth_with_seed(&vars, random_depth, random_width, seed);
        println!("  {}: {}", i, term);
    }
    println!();
    
    // ===== REWRITE DEMONSTRATION =====
    
    // println!("=== Rewrite System Demo ===");
    // let vars = vec!["a".to_string(), "b".to_string()];
    // let redex = Proc::generate_random_at_depth(&vars, 6, 3);
    // println!("Term: {}", redex);
    
    // let rdx_str = "{
    //     a!({}) ,
    //     for(a->x0){
    //         { x0!({}) , for(b->y1){ y1!(*a) } }
    //     } ,
    //     b!({}) ,
    //     for(b->x1){
    //         a!(*b)
    //     } ,
    //     c!({}) ,
    //     for(c->x2){
    //         x2!({})
    //     } ,
    //     for(@({})->y0){
    //         *y0
    //     }
    // }";
    // mettail_runtime::clear_var_cache();
    // let parser = rhocalc::ProcParser::new();
    // let redex = parser.parse(rdx_str).unwrap();

    // let prog = ascent_run! {
    //     include_source!(rhocalc_source);
        
    //     // Seed the initial term
    //     proc(p) <-- for p in [redex.clone()];
        
    //     relation redex_eq(Proc);
    //     redex_eq(q.clone()) <-- eq_proc(redex.clone(), q);
    //     proc(q) <-- redex_eq(q);
        
    //     // relation path(Proc, Proc);
    //     // path(redex.clone(), redex.clone()) <-- for _ in [()];
    //     // path(redex.clone(), q.clone()) <-- redex_eq(q);
    //     // path(p.clone(),q.clone()) <-- rw_proc(p,q);
    //     // path(p.clone(),r.clone()) <-- rw_proc(p,q), path(q.clone(),r);
        
    //     // relation is_normal_form(Proc);

    //     // is_normal_form(t.clone()) <-- proc(t), !rw_proc(t.clone(),_);
        
    //     // relation path_full(Proc,Proc);
    //     // path_full(redex.clone(),z.clone()) <-- is_normal_form(z), path(redex.clone(), z);
    // };

    // println!("Terms: {}", prog.proc.len());
    // println!("Proc equations: {}", prog.__eq_proc_ind_common.count_exact());
    // println!("Name equations: {}", prog.__eq_name_ind_common.count_exact());
    // // for (lhs, rhs) in prog.__eq_proc_ind_common.iter_all_added() {
    // //     println!("  {} = {}", lhs, rhs);
    // // }
    
    // // let mut path_full = prog.path_full.clone();
    // // path_full.sort_by(|a,b| a.0.cmp(&b.0));

    // // println!("\n=== Paths to normal forms: {} ===", path_full.len());
    // // println!("{}\n ~>", redex.clone());
    // // for (_, t) in path_full {
    // //     println!("{}", t);
    // // }   

    // let mut rewrites = prog.rw_proc.clone();
    // rewrites.sort_by(|a,b| a.0.cmp(&b.0));
    // println!("Rewrites: {}", rewrites.len());
    // for (from, to) in rewrites.iter() {
    //     println!("  {} ~> {}", from, to);
    // }
    // println!();


    // let elapsed = Instant::now().duration_since(start_time);
    // println!("Time: {:?}", elapsed);
}
