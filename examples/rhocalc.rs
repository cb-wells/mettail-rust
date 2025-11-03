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
        PInput . Proc ::= "for" "(" Name "<-" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= Proc "|" Proc ;
        PDrop . Proc ::= "*" Name ;
        
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

fn paths(redex: &Proc) -> Vec<Vec<Proc>> {
    let result = ascent_run! {
        relation proc(Proc);
        relation eq(Proc, Proc);
        relation rw(Proc, Proc);
        relation ev(Proc, Proc);
        relation path(Proc, Vec<Proc>);

        proc(p) <-- 
            for p in [redex];
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

        rw(s1, t.clone()) <-- 
            proc(s0),
            if let Some(t) = try_rewrite_rule_0(&s0),
            eq(s0,s1);
        rw(ss,t) <-- 
            proc(s),
            eq(s,ss),
            if let Proc::PPar(s0,p) = ss,
            rw(**s0,t0),
            let t = Proc::PPar(Box::new(t0.clone()),p.clone());
        
        // ev(s,t) <-- rw(s,t);
        // ev(s,u) <-- rw(s,t),ev(t,u);

        path(p1, vec![p1.clone(),p2.clone()]) <--
            rw(p1,p2);
        path(p1, ps) <--
            rw(p1,p2),
            path(p2,qs),
            let ps = [vec![p1.clone(), p2.clone()], qs.clone()].concat();
    };
    
    result.path.into_iter()
        .filter(|(s,_)| s == redex)
        .map(|(_,q)| q)
        .collect()
}

fn main() {
    // Test with matching channels - should fire the rewrite
    let rdx_str = "for(a<-x){*x}|a!(b!(0))|for(b<-y){*y}|b!(0)";
    
    // Clear variable cache before parsing to ensure fresh IDs for this term
    mettail_runtime::clear_var_cache();
    
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();

    // println!("Redex: {:?}", redex);
    
    let paths = paths(&redex);
    
    println!("Paths found:");
    for ps in &paths {
        for p in ps {
            print!("{}", p);
            print!(" ~> ");
        }
        println!(".");
    }
}

