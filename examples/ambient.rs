use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;
use std::time::Instant;
use ascent::*;

// the language specification
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

fn main() {
    let start_time = Instant::now();

    // Test DIRECT PNew term (not wrapped in collection)
    let rdx_str = "{p,{q}}";
    mettail_runtime::clear_var_cache();
    let parser = ambient::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();
    println!("Input: {}", rdx_str);
    // println!("Parsed: {:?}", redex.clone());

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

    let mut rewrites = prog.rw_proc.clone();
    rewrites.sort_by(|a,b| a.0.cmp(&b.0));

    // Check all terms in proc relation
    // println!("\nAll proc terms: {}", prog.proc.len());
    // for (p,) in prog.proc.iter().take(20) {
    //     println!("  {}", p);
    // }

    println!("Equations:");
    for (lhs, rhs) in prog.__eq_proc_ind_common.iter_all_added() {
        if lhs.to_string() != rhs.to_string() {
            println!("  {} = {}", lhs, rhs);
        }
    }
    
    // Check all rewrites in rw_proc
    println!("\nAll rewrites: {}", rewrites.len());
    for (from, to) in rewrites.iter().take(20) {
        println!("  {} ~> {}", from, to);
    }

    // Check if the input term can rewrite
    let direct_rewrites: Vec<_> = rewrites.iter()
        .filter(|(from, _)| from == &redex)
        .collect();
    
    println!("\nDirect rewrites from input: {}", direct_rewrites.len());
    for (from, to) in &direct_rewrites {
        println!("  {} ~> {}", from, to);
    }
    
    // Check if it's a normal form
    let is_nf = prog.is_normal_form.iter().any(|(t,)| t == &redex);
    println!("Is normal form: {}", is_nf);


    let elapsed = Instant::now().duration_since(start_time);
    println!("\nTime: {:?}", elapsed);  
}



//// GENERATED CODE ////
ascent_source! {
    amb:

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
name(field_0.as_ref().clone()), proc(field_1.as_ref().clone()) <--
    proc(t),
    if let Proc :: PIn(field_0, field_1) = t;
name(field_0.as_ref().clone()), proc(field_1.as_ref().clone()) <--
    proc(t),
    if let Proc :: POut(field_0, field_1) = t;
name(field_0.as_ref().clone()), proc(field_1.as_ref().clone()) <--
    proc(t),
    if let Proc :: POpen(field_0, field_1) = t;
name(field_0.as_ref().clone()), proc(field_1.as_ref().clone()) <--
    proc(t),
    if let Proc :: PAmb(field_0, field_1) = t;
proc(body_value) <--
    proc(t),
    if let Proc :: PNew(scope) = t,
    let body_value = scope.inner().unsafe_body.as_ref().clone();
ppar_contains(parent.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter();
proc(elem) <--
    ppar_contains(_parent, elem);
name(c1) <--
    name(c0),
    rw_name(c0, c1);

    // Equation rules
eq_proc(t.clone(), t.clone()) <--
    proc(t);
eq_name(t.clone(), t.clone()) <--
    name(t);
eq_proc(Proc :: PIn(Box :: new(x0.clone()), Box :: new(x1.clone())), Proc :: PIn(Box :: new(y0.clone()), Box :: new(y1.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone()),
    proc(x1),
    proc(y1),
    eq_proc(x1.clone(), y1.clone());
eq_proc(Proc :: POut(Box :: new(x0.clone()), Box :: new(x1.clone())), Proc :: POut(Box :: new(y0.clone()), Box :: new(y1.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone()),
    proc(x1),
    proc(y1),
    eq_proc(x1.clone(), y1.clone());
eq_proc(Proc :: POpen(Box :: new(x0.clone()), Box :: new(x1.clone())), Proc :: POpen(Box :: new(y0.clone()), Box :: new(y1.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone()),
    proc(x1),
    proc(y1),
    eq_proc(x1.clone(), y1.clone());
eq_proc(Proc :: PAmb(Box :: new(x0.clone()), Box :: new(x1.clone())), Proc :: PAmb(Box :: new(y0.clone()), Box :: new(y1.clone()))) <--
    name(x0),
    name(y0),
    eq_name(x0.clone(), y0.clone()),
    proc(x1),
    proc(y1),
    eq_proc(x1.clone(), y1.clone());
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc :: PPar(p0_bag) = p0,
    for (p0_elem_0, _count_p0_0) in p0_bag.iter(),
    for (p0_elem_1, _count_p0_1) in p0_bag.iter(),
    if & p0_elem_1 != & p0_elem_0,
    if let Proc :: PNew(p0_elem_1_f0) = p0_elem_1,
    let binder_1 = p0_elem_1_f0.inner().unsafe_pattern.clone(),
    let body_1 = p0_elem_1_f0.inner().unsafe_body.as_ref().clone(),
    let p = p0_elem_0.clone(),
    let c = body_1.clone(),
    let x = binder_1.clone(),
    if is_fresh(& x, & c),
    let p1 = (Proc :: PNew(mettail_runtime :: Scope :: from_parts_unsafe(x.clone(), Box :: new(Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
bag.insert(p.clone());
bag.insert(c.clone());
bag }))))).normalize();
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc :: PNew(p0_f0) = p0,
    let binder_0 = p0_f0.inner().unsafe_pattern.clone(),
    let body_0 = p0_f0.inner().unsafe_body.as_ref().clone(),
    if let Proc :: PPar(body_0_f0) = body_0,
    for (body_0_f0_elem_0, _count_body_0_f0_0) in body_0_f0.iter(),
    for (body_0_f0_elem_1, _count_body_0_f0_1) in body_0_f0.iter(),
    if & body_0_f0_elem_1 != & body_0_f0_elem_0,
    if let Proc :: PIn(body_0_f0_elem_1_f0, body_0_f0_elem_1_f1) = body_0_f0_elem_1,
    let body_0_f0_elem_1_f0_val = body_0_f0_elem_1_f0.as_ref(),
    let body_0_f0_elem_1_f1_val = body_0_f0_elem_1_f1.as_ref(),
    let p = body_0_f0_elem_0.clone(),
    let x = binder_0.clone(),
    let q = body_0_f0_elem_1_f1_val.clone(),
    let n = body_0_f0_elem_1_f0_val.clone(),
    if is_fresh(& x, & n),
    let p1 = (Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
bag.insert(p.clone());
bag.insert(Proc :: PIn(Box :: new(n.clone()), Box :: new(Proc :: PNew(mettail_runtime :: Scope :: from_parts_unsafe(x.clone(), Box :: new(q.clone()))))));
bag })).normalize();
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc :: PNew(p0_f0) = p0,
    let binder_0 = p0_f0.inner().unsafe_pattern.clone(),
    let body_0 = p0_f0.inner().unsafe_body.as_ref().clone(),
    if let Proc :: PPar(body_0_f0) = body_0,
    for (body_0_f0_elem_0, _count_body_0_f0_0) in body_0_f0.iter(),
    for (body_0_f0_elem_1, _count_body_0_f0_1) in body_0_f0.iter(),
    if & body_0_f0_elem_1 != & body_0_f0_elem_0,
    if let Proc :: POut(body_0_f0_elem_1_f0, body_0_f0_elem_1_f1) = body_0_f0_elem_1,
    let body_0_f0_elem_1_f0_val = body_0_f0_elem_1_f0.as_ref(),
    let body_0_f0_elem_1_f1_val = body_0_f0_elem_1_f1.as_ref(),
    let q = body_0_f0_elem_1_f1_val.clone(),
    let n = body_0_f0_elem_1_f0_val.clone(),
    let x = binder_0.clone(),
    let p = body_0_f0_elem_0.clone(),
    if is_fresh(& x, & n),
    let p1 = (Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
bag.insert(p.clone());
bag.insert(Proc :: POut(Box :: new(n.clone()), Box :: new(Proc :: PNew(mettail_runtime :: Scope :: from_parts_unsafe(x.clone(), Box :: new(q.clone()))))));
bag })).normalize();
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc :: PNew(p0_f0) = p0,
    let binder_0 = p0_f0.inner().unsafe_pattern.clone(),
    let body_0 = p0_f0.inner().unsafe_body.as_ref().clone(),
    if let Proc :: PPar(body_0_f0) = body_0,
    for (body_0_f0_elem_0, _count_body_0_f0_0) in body_0_f0.iter(),
    for (body_0_f0_elem_1, _count_body_0_f0_1) in body_0_f0.iter(),
    if & body_0_f0_elem_1 != & body_0_f0_elem_0,
    if let Proc :: POpen(body_0_f0_elem_1_f0, body_0_f0_elem_1_f1) = body_0_f0_elem_1,
    let body_0_f0_elem_1_f0_val = body_0_f0_elem_1_f0.as_ref(),
    let body_0_f0_elem_1_f1_val = body_0_f0_elem_1_f1.as_ref(),
    let q = body_0_f0_elem_1_f1_val.clone(),
    let p = body_0_f0_elem_0.clone(),
    let n = body_0_f0_elem_1_f0_val.clone(),
    let x = binder_0.clone(),
    if is_fresh(& x, & n),
    let p1 = (Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
bag.insert(p.clone());
bag.insert(Proc :: POpen(Box :: new(n.clone()), Box :: new(Proc :: PNew(mettail_runtime :: Scope :: from_parts_unsafe(x.clone(), Box :: new(q.clone()))))));
bag })).normalize();
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc :: PNew(p0_f0) = p0,
    let binder_0 = p0_f0.inner().unsafe_pattern.clone(),
    let body_0 = p0_f0.inner().unsafe_body.as_ref().clone(),
    if let Proc :: PPar(body_0_f0) = body_0,
    for (body_0_f0_elem_0, _count_body_0_f0_0) in body_0_f0.iter(),
    for (body_0_f0_elem_1, _count_body_0_f0_1) in body_0_f0.iter(),
    if & body_0_f0_elem_1 != & body_0_f0_elem_0,
    if let Proc :: PAmb(body_0_f0_elem_1_f0, body_0_f0_elem_1_f1) = body_0_f0_elem_1,
    let body_0_f0_elem_1_f0_val = body_0_f0_elem_1_f0.as_ref(),
    let body_0_f0_elem_1_f1_val = body_0_f0_elem_1_f1.as_ref(),
    let x = binder_0.clone(),
    let n = body_0_f0_elem_1_f0_val.clone(),
    let p = body_0_f0_elem_0.clone(),
    let q = body_0_f0_elem_1_f1_val.clone(),
    if is_fresh(& x, & n),
    let p1 = (Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
bag.insert(p.clone());
bag.insert(Proc :: PAmb(Box :: new(n.clone()), Box :: new(Proc :: PNew(mettail_runtime :: Scope :: from_parts_unsafe(x.clone(), Box :: new(q.clone()))))));
bag })).normalize();

    // Rewrite rules
rw_proc(s1, t) <--
    rw_proc(s0, t),
    eq_proc(s0, s1);
rw_proc(s, t1) <--
    rw_proc(s, t0),
    eq_proc(t0, t1);
rw_name(s1, t) <--
    rw_name(s0, t),
    eq_name(s0, s1);
rw_name(s, t1) <--
    rw_name(s, t0),
    eq_name(t0, t1);
rw_proc(s, t) <--
    proc(s),
    if let Proc :: PAmb(s_f0, s_f1) = s,
    let s_f0_val = s_f0.as_ref(),
    let s_f1_inner = s_f1.as_ref(),
    if let Proc :: PPar(s_f1_inner_f0) = s_f1_inner,
    for (s_f1_inner_f0_elem_0, _count_s_f1_inner_f0_0) in s_f1_inner_f0.iter(),
    if let Proc :: PAmb(s_f1_inner_f0_elem_0_f0, s_f1_inner_f0_elem_0_f1) = s_f1_inner_f0_elem_0,
    let s_f1_inner_f0_elem_0_f0_val = s_f1_inner_f0_elem_0_f0.as_ref(),
    let s_f1_inner_f0_elem_0_f1_inner = s_f1_inner_f0_elem_0_f1.as_ref(),
    if let Proc :: PPar(s_f1_inner_f0_elem_0_f1_inner_f0) = s_f1_inner_f0_elem_0_f1_inner,
    for (s_f1_inner_f0_elem_0_f1_inner_f0_elem_0, _count_s_f1_inner_f0_elem_0_f1_inner_f0_0) in s_f1_inner_f0_elem_0_f1_inner_f0.iter(),
    if let Proc :: POut(s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0, s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1) = s_f1_inner_f0_elem_0_f1_inner_f0_elem_0,
    let s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0_val = s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0.as_ref(),
    let s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1_val = s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1.as_ref(),
    let s_f1_inner_f0_elem_0_f1_inner_f0_rest = { let mut bag = s_f1_inner_f0_elem_0_f1_inner_f0.clone();
bag.remove(& s_f1_inner_f0_elem_0_f1_inner_f0_elem_0);
bag }, for (s_f1_inner_f0_elem_1, _count_s_f1_inner_f0_1) in s_f1_inner_f0.iter(), if & s_f1_inner_f0_elem_1 != & s_f1_inner_f0_elem_0, eq_name(s_f0_val.clone(), s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0_val.clone()), let t = (Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
Proc :: insert_into_ppar(& mut bag, Proc :: PAmb(Box :: new(s_f1_inner_f0_elem_0_f0_val.clone()), Box :: new(Proc :: PPar({ let mut bag = (s_f1_inner_f0_elem_0_f1_inner_f0_rest.clone()).clone();
Proc :: insert_into_ppar(& mut bag, s_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1_val.clone());
bag }))));
Proc :: insert_into_ppar(& mut bag, Proc :: PAmb(Box :: new(s_f0_val.clone()), Box :: new(s_f1_inner_f0_elem_1.clone())));
bag })).normalize();
relation pamb_proj_c3_b0_p0(Proc, mettail_runtime :: HashBag < Proc > , Name, Name, Proc, Proc);
pamb_proj_c3_b0_p0(parent.clone(), rest.clone(), n.clone(), m.clone(), p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PAmb(elem_f0, elem_f1) = elem,
    let elem_f0_val = elem_f0.as_ref(),
    let elem_f1_inner = elem_f1.as_ref(),
    if let Proc :: PPar(elem_f1_inner_f0) = elem_f1_inner,
    for (elem_f1_inner_f0_elem_0, _count_elem_f1_inner_f0_0) in elem_f1_inner_f0.iter(),
    if let Proc :: PIn(elem_f1_inner_f0_elem_0_f0, elem_f1_inner_f0_elem_0_f1) = elem_f1_inner_f0_elem_0,
    let elem_f1_inner_f0_elem_0_f0_val = elem_f1_inner_f0_elem_0_f0.as_ref(),
    let elem_f1_inner_f0_elem_0_f1_val = elem_f1_inner_f0_elem_0_f1.as_ref(),
    let elem_f1_inner_f0_rest = { let mut bag = elem_f1_inner_f0.clone();
bag.remove(& elem_f1_inner_f0_elem_0);
bag }, let rest = elem_f1_inner_f0_rest.clone(), let n = elem_f0_val.clone(), let m = elem_f1_inner_f0_elem_0_f0_val.clone(), let p = elem_f1_inner_f0_elem_0_f1_val.clone();
relation pamb_proj_c3_b0_p1(Proc, Name, Proc, Proc);
pamb_proj_c3_b0_p1(parent.clone(), cap_m.clone(), cap_r.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PAmb(ref f0, ref f1) = elem,
    let cap_m = (* * f0).clone(),
    let cap_r = (* * f1).clone();
relation pamb_proj_c3_b1_p0(Proc, mettail_runtime :: HashBag < Proc > , Name, Proc, Name, Proc, Proc);
pamb_proj_c3_b1_p0(parent.clone(), rest.clone(), m.clone(), p.clone(), n.clone(), r.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PAmb(elem_f0, elem_f1) = elem,
    let elem_f0_val = elem_f0.as_ref(),
    let elem_f1_inner = elem_f1.as_ref(),
    if let Proc :: PPar(elem_f1_inner_f0) = elem_f1_inner,
    for (elem_f1_inner_f0_elem_0, _count_elem_f1_inner_f0_0) in elem_f1_inner_f0.iter(),
    if let Proc :: PAmb(elem_f1_inner_f0_elem_0_f0, elem_f1_inner_f0_elem_0_f1) = elem_f1_inner_f0_elem_0,
    let elem_f1_inner_f0_elem_0_f0_val = elem_f1_inner_f0_elem_0_f0.as_ref(),
    let elem_f1_inner_f0_elem_0_f1_inner = elem_f1_inner_f0_elem_0_f1.as_ref(),
    if let Proc :: PPar(elem_f1_inner_f0_elem_0_f1_inner_f0) = elem_f1_inner_f0_elem_0_f1_inner,
    for (elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0, _count_elem_f1_inner_f0_elem_0_f1_inner_f0_0) in elem_f1_inner_f0_elem_0_f1_inner_f0.iter(),
    if let Proc :: POut(elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0, elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1) = elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0,
    let elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0_val = elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0.as_ref(),
    let elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1_val = elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1.as_ref(),
    let elem_f1_inner_f0_elem_0_f1_inner_f0_rest = { let mut bag = elem_f1_inner_f0_elem_0_f1_inner_f0.clone();
bag.remove(& elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0);
bag }, for (elem_f1_inner_f0_elem_1, _count_elem_f1_inner_f0_1) in elem_f1_inner_f0.iter(), if & elem_f1_inner_f0_elem_1 != & elem_f1_inner_f0_elem_0, let rest = elem_f1_inner_f0_elem_0_f1_inner_f0_rest.clone(), let m = elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f0_val.clone(), let p = elem_f1_inner_f0_elem_0_f1_inner_f0_elem_0_f1_val.clone(), let n = elem_f1_inner_f0_elem_0_f0_val.clone(), let r = elem_f1_inner_f0_elem_1.clone();
relation popen_proj_c3_b2_p0(Proc, Name, Proc, Proc);
popen_proj_c3_b2_p0(parent.clone(), cap_n.clone(), cap_p.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: POpen(ref f0, ref f1) = elem,
    let cap_n = (* * f0).clone(),
    let cap_p = (* * f1).clone();
relation pamb_proj_c3_b2_p1(Proc, Name, Proc, Proc);
pamb_proj_c3_b2_p1(parent.clone(), cap_n.clone(), cap_q.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PAmb(ref f0, ref f1) = elem,
    let cap_n = (* * f0).clone(),
    let cap_q = (* * f1).clone();
relation pnew_proj_c3_r0(Proc, mettail_runtime :: Binder < String > , Proc, Proc);
pnew_proj_c3_r0(parent.clone(), binder_var.clone(), rewrite_field.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PNew(ref scope) = elem,
    let binder_var = scope.inner().unsafe_pattern.clone(),
    let rewrite_field = scope.inner().unsafe_body.as_ref().clone();
relation pamb_proj_c3_r1(Proc, Proc, Proc);
pamb_proj_c3_r1(parent.clone(), rewrite_field.clone(), elem.clone()) <--
    proc(parent),
    if let Proc :: PPar(ref bag_field) = parent,
    for (elem, _count) in bag_field.iter(),
    if let Proc :: PAmb(_field0, ref rewrite_field_box) = elem,
    let rewrite_field = (* * rewrite_field_box).clone();
rw_proc(parent, result) <--
    pamb_proj_c3_b0_p0(parent, cap_rest_p0, cap_n_p0, cap_m, cap_p_p0, elem_0),
    pamb_proj_c3_b0_p1(parent, cap_m, cap_r_p1, elem_1),
    let rhs_term = Proc :: PAmb(Box :: new(cap_m.clone()), Box :: new(Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
Proc :: insert_into_ppar(& mut bag, Proc :: PAmb(Box :: new(cap_n_p0.clone()), Box :: new(Proc :: PPar({ let mut bag = (cap_rest_p0.clone()).clone();
Proc :: insert_into_ppar(& mut bag, cap_p_p0.clone());
bag }))));
Proc :: insert_into_ppar(& mut bag, cap_r_p1.clone());
bag }))), if let Proc :: PPar(ref bag) = parent, let remaining = { let mut b = bag.clone();
b.remove(elem_0);
b.remove(elem_1);
b }, let result = Proc :: PPar({ let mut bag_result = remaining;
Proc :: insert_into_ppar(& mut bag_result, rhs_term);
bag_result }).normalize();
rw_proc(parent, result) <--
    pamb_proj_c3_b1_p0(parent, cap_rest_p0, cap_m_p0, cap_p_p0, cap_n_p0, cap_r_p0, elem_0),
    let rhs_term = Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
Proc :: insert_into_ppar(& mut bag, Proc :: PAmb(Box :: new(cap_n_p0.clone()), Box :: new(Proc :: PPar({ let mut bag = (cap_rest_p0.clone()).clone();
Proc :: insert_into_ppar(& mut bag, cap_p_p0.clone());
bag }))));
Proc :: insert_into_ppar(& mut bag, Proc :: PAmb(Box :: new(cap_m_p0.clone()), Box :: new(cap_r_p0.clone())));
bag }), if let Proc :: PPar(ref bag) = parent, let remaining = { let mut b = bag.clone();
b.remove(elem_0);
b }, let result = Proc :: PPar({ let mut bag_result = remaining;
Proc :: insert_into_ppar(& mut bag_result, rhs_term);
bag_result }).normalize();
rw_proc(parent, result) <--
    popen_proj_c3_b2_p0(parent, cap_n, cap_p_p0, elem_0),
    pamb_proj_c3_b2_p1(parent, cap_n, cap_q_p1, elem_1),
    let rhs_term = Proc :: PPar({ let mut bag = mettail_runtime :: HashBag :: new();
Proc :: insert_into_ppar(& mut bag, cap_p_p0.clone());
Proc :: insert_into_ppar(& mut bag, cap_q_p1.clone());
bag }), if let Proc :: PPar(ref bag) = parent, let remaining = { let mut b = bag.clone();
b.remove(elem_0);
b.remove(elem_1);
b }, let result = Proc :: PPar({ let mut bag_result = remaining;
Proc :: insert_into_ppar(& mut bag_result, rhs_term);
bag_result }).normalize();
rw_proc(parent, result) <--
    pnew_proj_c3_r0(parent, binder_var, body, elem),
    rw_proc(body, body_rewritten),
    if let Proc :: PPar(ref bag) = parent,
    let remaining = { let mut b = bag.clone();
b.remove(elem);
b }, let scope_tmp = mettail_runtime :: Scope :: from_parts_unsafe(binder_var.clone(), Box :: new(body_rewritten.clone())), let rewritten = Proc :: PNew(scope_tmp), let result = Proc :: PPar({ let mut bag = remaining;
Proc :: insert_into_ppar(& mut bag, rewritten);
bag }).normalize();
rw_proc(parent, result) <--
    pamb_proj_c3_r1(parent, body, elem),
    rw_proc(body, body_rewritten),
    if let Proc :: PPar(ref bag) = parent,
    let remaining = { let mut b = bag.clone();
b.remove(elem);
b }, let rewritten = body_rewritten.clone(), let result = Proc :: PPar({ let mut bag = remaining;
Proc :: insert_into_ppar(& mut bag, rewritten);
bag }).normalize();
relation pnew_direct_congruence_proj(Proc, mettail_runtime :: Binder < String > , Proc);
pnew_direct_congruence_proj(parent, binder_var, body) <--
    proc(parent),
    if let Proc :: PNew(ref scope) = parent,
    let binder_var = scope.inner().unsafe_pattern.clone(),
    let body = scope.inner().unsafe_body.as_ref().clone();
rw_proc(parent, result) <--
    pnew_direct_congruence_proj(parent, binder_var, body),
    rw_proc(body, body_rewritten),
    let scope_tmp = mettail_runtime :: Scope :: from_parts_unsafe(binder_var.clone(), Box :: new(body_rewritten.clone())),
    let result = Proc :: PNew(scope_tmp).normalize();
rw_proc(s, t) <--
    proc(s),
    if let Proc :: PAmb(n, s0) = s,
    rw_proc(* * s0, t0),
    let t = Proc :: PAmb(n.clone(), Box :: new(t0.clone()));
}