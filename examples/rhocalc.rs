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
        (PPar {}) == PZero;
        // Flattening equations are no longer needed - automatic!
    },
        
    rewrites {

        (PPar {(PInput chan x P) , (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
        
        (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});

    }
} 

fn main() {

    let start_time = Instant::now();
    
    // let vars = vec!["a".to_string(), "b".to_string()];
    // let redex = Proc::generate_random_at_depth(&vars, 6);
    // println!("Term: {}", redex);
    
    let rdx_str = "{
        a!(0) ,
        for(a->x0){
            { x0!(0) , for(b->y1){ y1!(*a) } }
        } ,
        b!(0) ,
        for(b->x1){
            a!(*b)
        } ,
        c!(0) ,
        for(c->x2){
            x2!(0)
        } ,
        for(@(0)->y0){
            *y0
        }
    }";
    mettail_runtime::clear_var_cache();
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();

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

    println!("Terms: {}", prog.proc.len());
    println!("Equations: {}", prog.eq_proc.len());
    println!("Rewrites: {}", prog.rw_proc.len());

    let outputs = prog.poutput_proj_r0_p1;
    println!("outputs:");
    for (_,_,_,out) in outputs {
        println!("{}", out);
    }
    let inputs = prog.pinput_proj_r0_p0;
    println!("inputs:");
    for (_,_,_,_,inp) in inputs {
        println!("{}", inp);
    }
    
    let mut path_full = prog.path_full.clone();
    path_full.sort_by(|a,b| a.0.cmp(&b.0));
    
    println!("\n=== Paths to normal forms: {} ===", path_full.len());
    println!("{}\n ~>", redex.clone());
    for (_, t) in path_full {
        println!("{}", t);
    }   

    let elapsed = Instant::now().duration_since(start_time);
    println!("Time: {:?}", elapsed);
}