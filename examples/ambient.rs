use mettail_macros::theory;
use mettail_runtime::*;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;
use std::time::Instant;


use ascent::*;

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
        // if x # C then (PPar P (PNew x C)) == (PNew x (PPar P C));
        // if x # N then (PNew x (PIn N P)) == (PIn N (PNew x P));
        // if x # N then (PNew x (POut N P)) == (POut N (PNew x P));
        // if x # N then (PNew x (POpen N P)) == (POpen N (PNew x P));
        // if x # N then (PNew x (PAmb N P)) == (PAmb N (PNew x P));
        // (PNew x (PNew y P)) == (PNew y (PNew x P));
    },
    rewrites {  
        (PPar {(PAmb N (PPar {(PIn M P) , Q})) , (PAmb M R), ...rest}) 
            => (PPar {(PAmb M (PPar {(PAmb N (PPar {P , Q})) , R})), ...rest});
            
        // (PAmb M (PPar (PAmb N (PPar (POut M P) Q)) R))
        //     => (PPar (PAmb N (PPar P Q)) (PAmb M R));
        // (PPar (POpen N P) (PAmb N Q))
        //     => (PPar P Q);

        // // if S => T then (PNew x S) => (PNew x T);
        // if S => T then (PAmb N S) => (PAmb N T);
    }
}

fn main() {
    let start_time = Instant::now();

    let rdx_str = "{m[r], n[{in(m,p),q}]}";
    mettail_runtime::clear_var_cache();
    let parser = ambient::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();
    println!("redex: {}", redex.clone());
    println!("ast: {:?}", redex.clone());

    // let vars = vec!["n".to_string()];
    // let term = Proc::generate_random_at_depth(&vars, 12);
    // println!("random term: {}", term);

    let prog = ascent_run! {
        include_source!(ambient_source);
        proc(p) <-- for p in [redex.clone()];

        relation path(Proc, Proc);
        path(p1, p2) <-- rw_proc(p1,p2);
        path(p1, p3) <-- path(p1,p2), path(p2,p3);

        relation is_normal_form(Proc);
        is_normal_form(t.clone()) <-- proc(t), !rw_proc(t.clone(),_);
        
        relation path_full(Proc, Proc);
        path_full(redex.clone(), z.clone()) <-- is_normal_form(z), path(redex.clone(), z.clone());
    };

    let mut paths = prog.rw_proc.clone();
    paths.sort_by(|a,b| a.0.cmp(&b.0));

    println!("Paths: {}", paths.len());
    for (s, t) in paths {
        println!("  {} ~> {}", s, t);
    }

    let elapsed = Instant::now().duration_since(start_time);
    println!("Time: {:?}", elapsed);  
}