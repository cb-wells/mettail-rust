use mettail_macros::theory;
use mettail_runtime;
use lalrpop_util::lalrpop_mod;

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
        (PPar P PZero) == P ;
        
        (PDrop (NQuote P)) == P ;
    },
    
    rewrites {
        if x # Q then (PPar (PInput chan x P) (POutput chan Q))
            => (subst P x (NQuote Q));
        // if S => T then (PPar P S) => (PPar P T)
    }
}

#[cfg(test)]
mod tests {}

ascent_source! {
    theory_source:

    relation proc(Proc);
    relation eq(Proc, Proc);
    relation rw(Proc, Proc);
    relation path(Proc, Vec<Proc>);
    relation path_terminal(Proc, Vec<Proc>);

    proc(p1) <-- 
        proc(p0), rw(p0,p1);
    proc(p1) <--
        proc(p0), eq(p0,p1);
    proc(*p.clone()), proc(*q.clone()) <-- 
        proc(p0), if let Proc::PPar(p,q) = p0;

    // Commutativity
    eq(p0,p1) <--
        proc(p0),
        if let Proc::PPar(p,q) = p0,
        let p1 = Proc::PPar(q.clone(),p.clone());
    // Associativity
    eq(p0,p1) <--
        proc(p0),
        if let Proc::PPar(t,r) = p0,
        if let Proc::PPar(p,q) = &**t,
        let p1 = Proc::PPar(p.clone(),Box::new(Proc::PPar(q.clone(),r.clone())));
    
    eq(p,p) <-- proc(p);
    eq(q,p) <-- eq(p,q);
    eq(p,r) <-- eq(p,q), eq(q,r);

    rw(s,*p.clone()) <--
        proc(s),
        if let Proc::PDrop(n) = s,
        if let Name::NQuote(p) = &**n;

    rw(s, t.clone()) <-- 
        proc(s),
        if let Some(t) = try_rewrite_rule_0(&s);
    rw(s,t) <-- 
        proc(s),
        if let Proc::PPar(s0,p) = s,
        rw(**s0,t0),
        let t = Proc::PPar(Box::new(t0.clone()),p.clone());
    rw(s1,t) <-- rw(s0,t), eq(s0,s1);
    
    path(p1, vec![p2.clone()]) <--
        rw(p1,p2);
    path(p1, ps) <--
        rw(p1,p2),
        path(p2,qs),
        let ps = [vec![p2.clone()], qs.clone()].concat();

    path_terminal(p,ps) <--
        path(p,ps),
        let z = ps.last().unwrap(),
        !rw(z,_);
}

fn main() {
    // println!("=== Rho Calculus Demo ===\n");

    // // === Term Generation Demo ===
    // println!("--- Term Generation ---");
    // println!("Generating Proc terms up to depth 2 with vars [a, b]...\n");
    
    // let vars = vec!["a".to_string(), "b".to_string()];
    // let terms = Proc::generate_terms(&vars, 2);
    
    // println!("Generated {} terms total", terms.len());
    // for term in terms {
    //     println!("  {}", term);
    // }
    
    // println!("\n--- Rewrite Engine Demo ---");
    let rdx_str = "for(a->x){*x}|a!(b!(*n))|for(b->y){*y}|b!(0)";
    mettail_runtime::clear_var_cache();
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();

    let prog = ascent_run! {
        // ascent modularity
        include_source!(theory_source);
        
        proc(p) <-- for p in [redex.clone()];
        relation full_path(Proc, Vec<Proc>);
        full_path(s,ps) <-- path_terminal(s,ps), eq(s,redex.clone());
    };

    let mut paths = prog.full_path.clone();
    paths.sort_by(|a,b| a.0.cmp(&b.0));

    println!("Paths found: {}", paths.len());
    for (s, ps) in paths.iter().take(3) {
        println!("{} ~> {}", s, ps.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ~> "));
        println!();
    }
}

