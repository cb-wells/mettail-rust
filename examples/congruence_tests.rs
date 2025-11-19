use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;
use ascent::*;

// Re-use the language specification from ambient.rs
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
        P == (PPar {P, PZero});
        if x # C then (PPar {P, (PNew x C)}) == (PNew x (PPar {P, C}));
        if x # N then (PNew x {P, (PIn N P)}) == {P, (PIn N (PNew x P))};
        if x # N then (PNew x {P, (POut N P)}) == {P, (POut N (PNew x P))};
        if x # N then (PNew x {P, (POpen N P)}) == {P, (POpen N (PNew x P))};
        if x # N then (PNew x {P, (PAmb N P)}) == {P, (PAmb N (PNew x P))};
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

struct CongruenceTest {
    name: &'static str,
    input: &'static str,
    expected_rewrite: Option<&'static str>,
    description: &'static str,
}

fn run_congruence_test(test: &CongruenceTest) -> Result<(), String> {
    println!("CONGRUENCE TEST: {}", test.name);
    println!("Description: {}", test.description);
    println!("Input: {}", test.input);
    
    mettail_runtime::clear_var_cache();
    let parser = ambient::ProcParser::new();
    let input_term = parser.parse(test.input)
        .map_err(|e| format!("Parse error: {:?}", e))?;
    
    println!("Parsed: {}", input_term);
    
    let prog = ascent_run! {
        include_source!(ambient_source);
        proc(p) <-- for p in [input_term.clone()];

        relation path(Proc, Proc);
        path(p1, p2) <-- rw_proc(p1,p2);
        path(p1, p3) <-- path(p1,p2), path(p2,p3);

        relation is_normal_form(Proc);
        is_normal_form(t.clone()) <-- proc(t), !rw_proc(t.clone(),_);
    };

    println!("\nTotal proc terms: {}", prog.proc.len());
    
    // Direct rewrites from input
    let direct_rewrites: Vec<_> = prog.rw_proc.iter()
        .filter(|(from, _)| from == &input_term)
        .collect();
    
    println!("Direct rewrites from input: {}", direct_rewrites.len());
    for (from, to) in &direct_rewrites {
        println!("  {} ~> {}", from, to);
    }
    
    // Check if expected rewrite exists
    if let Some(expected_str) = test.expected_rewrite {
        let expected = parser.parse(expected_str)
            .map_err(|e| format!("Parse error in expected: {:?}", e))?;
        
        // Parse the actual rewrites to compare structurally (not string comparison)
        let found = direct_rewrites.iter().any(|(_, to)| to == &expected);
        
        if !found {
            // Debug: show all rewrites
            println!("\nAll rewrites in relation:");
            for (from, to) in prog.rw_proc.iter().take(10) {
                println!("  {} ~> {}", from, to);
            }
            
            println!("\nExpected: {}", expected);
            println!("Expected (debug): {:?}", expected);
            
            return Err(format!(
                "Expected rewrite to '{}' not found among direct rewrites",
                expected_str
            ));
        }
        println!("\n✓ Expected rewrite found!");
    }
    
    println!("\n✓ Test passed!");
    Ok(())
}

fn main() {
    let tests = vec![
        // Test ambient congruence: if S => T then amb[S] => amb[T]
        CongruenceTest {
            name: "amb_congruence",
            input: "outer[{n[{in(m,p)}], m[r]}]",
            expected_rewrite: Some("outer[{m[{n[{p}], r}]}]"),
            description: "Ambient congruence: rewrite inside ambient body",
        },
        
        // Test new congruence: if S => T then new(x, S) => new(x, T)
        CongruenceTest {
            name: "new_congruence",
            input: "new(x, {agent[{in(parent,data)}], parent[0]})",
            expected_rewrite: Some("new(x, {parent[{0, agent[{data}]}]})"),
            description: "New congruence: rewrite inside binder body",
        },
        
        // Test parallel congruence: if S => T then {S, ...rest} => {T, ...rest}
        CongruenceTest {
            name: "par_congruence",
            input: "{{n[{in(m,p)}], m[r]}, observer}",
            expected_rewrite: None, // Multiple possible
            description: "Parallel congruence: rewrite inside parallel composition",
        },
        
        // Test nested congruences
        CongruenceTest {
            name: "nested_amb_new",
            input: "outer[new(x, {agent[{in(parent,data)}], parent[0]})]",
            expected_rewrite: Some("outer[new(x, {parent[{0, agent[{data}]}]})]"),
            description: "Nested congruences: ambient containing new containing rewrite",
        },
        
        // Test new with rest pattern
        CongruenceTest {
            name: "new_with_rest",
            input: "new(x, {a[{in(b,p), state}], b[r], observer})",
            expected_rewrite: Some("new(x, {b[{a[{p, state}], r}], observer})"),
            description: "New congruence with rest pattern in parallel",
        },
        
        // Test collection congruence for new
        CongruenceTest {
            name: "new_in_collection",
            input: "{new(x, {agent[{in(parent,data)}], parent[0]}), observer}",
            expected_rewrite: Some("{new(x, {parent[{0, agent[{data}]}]}), observer}"),
            description: "Collection congruence: rewrite new inside parallel",
        },
    ];
    
    println!("\nRunning {} congruence tests...", tests.len());
    
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    
    for test in &tests {
        match run_congruence_test(test) {
            Ok(()) => passed += 1,
            Err(e) => {
                failed += 1;
                errors.push((test.name, e));
            }
        }
    }
    
    println!("CONGRUENCE TEST SUMMARY");
    println!("Total: {}", tests.len());
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    
    if !errors.is_empty() {
        println!("FAILURES:");
        for (name, error) in &errors {
            println!("\n✗ {}", name);
            println!("  Error: {}", error);
        }
    }
    
    if failed > 0 {
        std::process::exit(1);
    }
}

