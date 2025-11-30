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
        PDrop . Proc ::= "*" "(" Name ")" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;

        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;

        NQuote . Name ::= "@" "(" Proc ")" ;

        PVar . Proc ::= Var;
        NVar . Name ::= Var;
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

    let start_time = Instant::now();
    
    println!("=== Rewrite System Demo ===");
    // let vars = vec!["a".to_string(), "b".to_string()];
    // let redex = Proc::generate_random_at_depth(&vars, 6, 3);
    // println!("Term: {}", redex);


    // let mut terms = Vec::new();
    // for i in 0..10 {
    //     terms.push(Proc::generate_random_at_depth(&vars, 6, 3));
    // }
    // println!("Terms: {}", terms.len());
    // for (i, term) in terms.iter().enumerate() {
    //     println!("  {}: {}", i, term);
    // }
    
    let rdx_str = "@(*(n))!({})";
    mettail_runtime::clear_var_cache();
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();
    println!("Input: {}", rdx_str);

    let terms = vec![redex.clone()];

    let prog = ascent_run! {
        include_source!(rhocalc_source);
        
        // Seed the initial term
        proc(p) <-- for p in terms.clone();
        
        relation redex_eq(Proc);
        redex_eq(q.clone()) <-- for p in terms.clone(), eq_proc(p.clone(), q);
        proc(q) <-- redex_eq(q);
        
        // relation path(Proc, Proc);
        // path(redex.clone(), redex.clone()) <-- for _ in [()];
        // path(redex.clone(), q.clone()) <-- redex_eq(q);
        // path(p.clone(),q.clone()) <-- rw_proc(p,q);
        // path(p.clone(),r.clone()) <-- rw_proc(p,q), path(q.clone(),r);
        
        // relation is_normal_form(Proc);

        // is_normal_form(t.clone()) <-- proc(t), !rw_proc(t.clone(),_);
        
        // relation path_full(Proc,Proc);
        // path_full(redex.clone(),z.clone()) <-- is_normal_form(z), path(redex.clone(), z);
    };

    println!("Terms: {}", prog.proc.len());
    println!("Proc equations: {}", prog.__eq_proc_ind_common.count_exact());
    for (lhs, rhs) in prog.__eq_proc_ind_common.iter_all_added() {
        println!("  {} = {}", lhs, rhs);
    }
    
    // let mut path_full = prog.path_full.clone();
    // path_full.sort_by(|a,b| a.0.cmp(&b.0));

    // println!("\n=== Paths to normal forms: {} ===", path_full.len());
    // println!("{}\n ~>", redex.clone());
    // for (_, t) in path_full {
    //     println!("{}", t);
    // }   

    let mut rewrites = prog.rw_proc.clone();
    rewrites.sort_by(|a,b| a.0.cmp(&b.0));
    println!("Rewrites: {}", rewrites.len());
    for (from, to) in rewrites.iter() {
        println!("  {} ~> {}", from, to);
    }
    println!();


    let elapsed = Instant::now().duration_since(start_time);
    println!("Time: {:?}", elapsed);
}

ascent_source! {
    rho:

    // Relations
relation proc(Proc);
relation name(Name);
#[ds(crate :: eqrel)] relation eq_proc(Proc, Proc);
#[ds(crate :: eqrel)] relation eq_name(Name, Name);
relation rw_proc(Proc, Proc);
relation rw_name(Name, Name);
relation ppar_contains(Proc, Proc);

    // Category rules
proc(c1) <--
    proc(c0),
    rw_proc(c0, c1);
name(field_0.as_ref().clone()) <--
    proc(t),
    if let Proc :: PDrop(field_0) = t;
name(field_0.as_ref().clone()), proc(field_1.as_ref().clone()) <--
    proc(t),
    if let Proc :: POutput(field_0, field_1) = t;
name(field_0.as_ref().clone()), proc(body.clone()) <--
    proc(t),
    if let Proc :: PInput(field_0, scope_field) = t,
    let body = (* scope_field.inner().unsafe_body).clone();
ppar_contains(parent.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter();
proc(elem) <--
    ppar_contains(_parent, elem);
name(c1) <--
    name(c0),
    rw_name(c0, c1);
proc(field_0.as_ref().clone()) <--
    name(t),
    if let Name :: NQuote(field_0) = t;

    // Equation rules
eq_proc(t.clone(), t.clone()) <--
    proc(t);
eq_name(t.clone(), t.clone()) <--
    name(t);
eq_proc(Proc :: PDrop(Box :: new(x0.clone())), Proc :: PDrop(Box :: new(y0.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone());
eq_proc(Proc :: POutput(Box :: new(x0.clone()), Box :: new(x1.clone())), Proc :: POutput(Box :: new(y0.clone()), Box :: new(y1.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone()),
    proc(x1),
    proc(y1),
    eq_proc(x1.clone(), y1.clone());
eq_name(Name :: NQuote(Box :: new(x0.clone())), Name :: NQuote(Box :: new(y0.clone()))) <--
    proc(x0),
    proc(y0),
    eq_proc(x0.clone(), y0.clone());
eq_name(p0, p1) <--
    name(p0),
    if let Name :: NQuote(p0_f0) = p0,
    let p0_f0_inner = p0_f0.as_ref(),
    if let Proc :: PDrop(p0_f0_inner_f0) = p0_f0_inner,
    let p0_f0_inner_f0_val = p0_f0_inner_f0.as_ref(),
    let n = p0_f0_inner_f0_val.clone(),
    let p1 = n.clone();

    // Rewrite rules
rw_proc(s, t) <--
    proc(s),
    if let Proc :: PDrop(s_f0) = s,
    let s_f0_inner = s_f0.as_ref(),
    if let Name :: NQuote(s_f0_inner_f0) = s_f0_inner,
    let s_f0_inner_f0_val = s_f0_inner_f0.as_ref(),
    let t = (s_f0_inner_f0_val.clone()).normalize();
relation pinput_proj_c2_b0_p0(Proc, Name, mettail_runtime :: Binder < String > , Proc, Proc);
pinput_proj_c2_b0_p0(parent.clone(), cap_chan.clone(), cap_x.clone(), cap_p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PInput(ref f0, ref f1) = elem,
    let cap_chan = (* * f0).clone(),
    let (binder_tmp, body_tmp) = (* f1).clone().unbind(),
    let cap_x = binder_tmp,
    let cap_p = * body_tmp;
relation poutput_proj_c2_b0_p1(Proc, Name, Proc, Proc);
poutput_proj_c2_b0_p1(parent.clone(), cap_chan.clone(), cap_q.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: POutput(ref f0, ref f1) = elem,
    let cap_chan = (* * f0).clone(),
    let cap_q = (* * f1).clone();
relation pdrop_proj_c2_b1_p0(Proc, Proc, Proc);
pdrop_proj_c2_b1_p0(parent.clone(), p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PDrop(elem_f0) = elem,
    let elem_f0_inner = elem_f0.as_ref(),
    if let Name :: NQuote(elem_f0_inner_f0) = elem_f0_inner,
    let elem_f0_inner_f0_val = elem_f0_inner_f0.as_ref(),
    let p = elem_f0_inner_f0_val.clone();
rw_proc(parent, result) <--
    pinput_proj_c2_b0_p0(parent, cap_chan_p0, cap_x_p0, cap_p_p0, elem_0),
    poutput_proj_c2_b0_p1(parent, cap_chan_p1, cap_q_p1, elem_1),
    eq_name(cap_chan_p0.clone(), cap_chan_p1.clone()),
    let rhs_term = (cap_p_p0.clone()).substitute_name(& cap_x_p0.clone().0, & Name :: NQuote(Box :: new(cap_q_p1.clone()))),
    if let Proc :: PPar(ref bag) = parent,
    let remaining = { let mut b = bag.clone();
b.remove(elem_0);
b.remove(elem_1);
b }, let result = Proc :: PPar({ let mut bag_result = remaining;
Proc :: insert_into_ppar(& mut bag_result, rhs_term);
bag_result }).normalize();
rw_proc(parent, result) <--
    pdrop_proj_c2_b1_p0(parent, cap_p_p0, elem_0),
    let rhs_term = cap_p_p0.clone(),
    if let Proc :: PPar(ref bag) = parent,
    let remaining = { let mut b = bag.clone();
b.remove(elem_0);
b }, let result = Proc :: PPar({ let mut bag_result = remaining;
Proc :: insert_into_ppar(& mut bag_result, rhs_term);
bag_result }).normalize();
}