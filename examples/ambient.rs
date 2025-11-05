use mettail_macros::theory;
use mettail_runtime::*;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;

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

        PPar . Proc ::= Proc "|" Proc ;

        PVar . Proc ::= Var;

        NVar . Name ::= Var ;
    },
    equations {
        (PPar P Q) == (PPar Q P) ;
        (PPar P (PPar Q R)) == (PPar (PPar P Q) R) ;
        (PPar P PZero) == P ;

        // if x # C then (PPar P (PNew x C)) == (PNew x (PPar P C));
        // if x # N then (PNew x (PIn N P)) == (PIn N (PNew x P));
        // if x # N then (PNew x (POut N P)) == (POut N (PNew x P));
        // if x # N then (PNew x (POpen N P)) == (POpen N (PNew x P));
        // if x # N then (PNew x (PAmb N P)) == (PAmb N (PNew x P));
        // (PNew x (PNew y P)) == (PNew y (PNew x P));
    },
    rewrites {
        (PPar (PAmb N (PPar (PIn M P) Q)) (PAmb M R)) 
            => (PAmb M (PPar (PAmb N (PPar P Q)) R));
        (PAmb M (PPar (PAmb N (PPar (POut M P) Q)) R))
            => (PPar (PAmb N (PPar P Q)) (PAmb M R));
        (PPar (POpen N P) (PAmb N Q))
            => (PPar P Q);

        // congruences
        if S => T then (PPar P S) => (PPar P T);
        // if S => T then (PNew x S) => (PNew x T);
        if S => T then (PAmb N S) => (PAmb N T);
    }
}

// ascent_source! {
//     theory_source:

//     relation proc(Proc);
//     relation eq(Proc, Proc);
//     relation rw(Proc, Proc);
//     relation path(Proc, Vec<Proc>);
//     relation path_terminal(Proc, Vec<Proc>);

//     proc(p1) <-- 
//         proc(p0), rw(p0,p1);
//     proc(p1) <--
//         proc(p0), eq(p0,p1);
//     proc(*p.clone()), proc(*q.clone()) <-- 
//         proc(p0), if let Proc::PPar(p,q) = p0;
//     proc(*p.clone()) <--
//         proc(p0), 
//         if let Proc::PNew(scope) = p0,
//         let (x,p) = scope.clone().unbind();

//     // Commutativity
//     eq(p0,p1) <--
//         proc(p0),
//         if let Proc::PPar(p,q) = p0,
//         let p1 = Proc::PPar(q.clone(),p.clone());
//     // Associativity
//     eq(p0,p1) <--
//         proc(p0),
//         if let Proc::PPar(t,r) = p0,
//         if let Proc::PPar(p,q) = &**t,
//         let p1 = Proc::PPar(p.clone(),Box::new(Proc::PPar(q.clone(),r.clone())));
    
//     eq(p,p) <-- proc(p);
//     eq(q,p) <-- eq(p,q);
//     eq(p,r) <-- eq(p,q), eq(q,r);

//     rw(s, t.clone()) <-- 
//         proc(s),
//         if let Some(t) = try_rewrite_rule_0(&s);
//     rw(s, t.clone()) <-- 
//         proc(s),
//         if let Some(t) = try_rewrite_rule_1(&s);
//     rw(s, t.clone()) <-- 
//         proc(s),
//         if let Some(t) = try_rewrite_rule_2(&s);
//     rw(s,t) <-- 
//         proc(s),
//         if let Proc::PPar(s0,p) = s,
//         rw(**s0,t0),
//         let t = Proc::PPar(Box::new(t0.clone()),p.clone());
//     rw(s,t) <-- 
//         proc(s),
//         if let Proc::PNew(scope) = s,
//         let (x, p) = scope.clone().unbind(),
//         rw(*p,t0),
//         let new_scope = mettail_runtime::Scope::new(x.clone(), Box::new(t0.clone())),
//         let t = Proc::PNew(new_scope);
//     rw(s1,t) <-- rw(s0,t), eq(s0,s1);
    
    // path(p1, vec![p2.clone()]) <--
    //     rw(p1,p2);
    // path(p1, ps) <--
    //     rw(p1,p2),
    //     path(p2,qs),
    //     let ps = [vec![p2.clone()], qs.clone()].concat();

//     path_terminal(p,ps) <--
//         path(p,ps),
//         let z = ps.last().unwrap(),
//         !rw(z,_);
// }

fn main() {
    println!("ambient calculus");

    let rdx_str = "open(m,0)|m[n[out(m,p)|q]|r]";
    mettail_runtime::clear_var_cache();
    let parser = ambient::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();

    // let vars = vec!["n".to_string()];
    // let term = Proc::generate_random_at_depth(&vars, 12);
    // println!("random term: {}", term);

    let prog = ascent_run! {
        include_source!(ambient_source);
        proc(p) <-- for p in [redex.clone()];

        relation path(Proc, Vec<Proc>);
        path(p1, vec![p2.clone()]) <-- rw_proc(p1,p2);
        path(p1, ps) <--
            rw_proc(p1,p2),
            path(p2,qs),
            let ps = [vec![p2.clone()], qs.clone()].concat();
    };

    let mut paths = prog.path.clone();
    paths.sort_by(|a,b| a.0.cmp(&b.0));

    println!("Paths: {}", paths.len());
    for (s, ps) in paths {
        println!("  {} ~> {}", s, ps.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ~> "));
    }
}