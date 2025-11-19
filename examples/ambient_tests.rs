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

struct TestCase {
    name: &'static str,
    input: &'static str,
    expected_output: Option<&'static str>,
    should_normalize: bool,
    min_rewrites: usize,
    description: &'static str,
}

fn run_test(test: &TestCase) -> Result<(), String> {
    println!("TEST: {}", test.name);
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
        
        relation path_full(Proc, Proc);
        path_full(input_term.clone(), z.clone()) <-- is_normal_form(z), path(input_term.clone(), z.clone());
    };

    let mut rewrites: Vec<_> = prog.rw_proc.iter().collect();
    rewrites.sort_by(|a, b| a.0.cmp(&b.0));
    
    println!("\nRewrites found: {}", rewrites.len());
    for (i, (s, t)) in rewrites.iter().enumerate() {
        println!("  [{}] {} ~> {}", i+1, s, t);
    }
    
    // Check minimum rewrites
    if rewrites.len() < test.min_rewrites {
        return Err(format!(
            "Expected at least {} rewrites, found {}",
            test.min_rewrites, rewrites.len()
        ));
    }
    
    // Check normalization
    let normal_forms: Vec<_> = prog.is_normal_form.iter().collect();
    println!("\nNormal forms: {}", normal_forms.len());
    for nf in &normal_forms {
        println!("  {}", nf.0);
    }
    
    if test.should_normalize && normal_forms.is_empty() {
        return Err("Expected to find a normal form, but none found".to_string());
    }
    
    // Check expected output if provided
    if let Some(expected_str) = test.expected_output {
        let expected = parser.parse(expected_str)
            .map_err(|e| format!("Parse error in expected: {:?}", e))?;
        
        // Check if expected output is in the rewrite relation
        let found = rewrites.iter().any(|(from, to)| {
            from == &input_term && to == &expected
        }) || prog.path.iter().any(|(from, to)| {
            from == &input_term && to == &expected
        });
        
        if !found {
            // Also check if it's in normal forms
            let in_normal_forms = normal_forms.iter().any(|nf| nf.0 == expected);
            if !in_normal_forms {
                return Err(format!(
                    "Expected output '{}' not found in rewrites or normal forms",
                    expected_str
                ));
            }
        }
        println!("\n✓ Expected output found: {}", expected_str);
    }
    
    println!("\n✓ Test passed!");
    Ok(())
}

fn main() {
    let tests = vec![
        // Basic rest patterns - empty context
        TestCase {
            name: "enter_empty_rest",
            input: "{n[{in(m,p)}], m[r]}",
            expected_output: Some("m[{n[{p}], r}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Basic entry with empty rest pattern: n enters m",
        },
        
        TestCase {
            name: "exit_empty_rest",
            input: "m[{n[{out(m,p)}], r}]",
            expected_output: Some("{n[{p}], m[r]}"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Basic exit with empty rest pattern: n exits m",
        },
        
        TestCase {
            name: "open_basic",
            input: "{open(n,p), n[q]}",
            expected_output: Some("{p, q}"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Basic open capability",
        },
        
        // Rest patterns - non-empty context
        TestCase {
            name: "enter_nonempty_rest",
            input: "{n[{in(m,p), q}], m[r]}",
            expected_output: Some("m[{n[{p, q}], r}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Entry with non-empty rest: preserves q during move",
        },
        
        TestCase {
            name: "enter_multiple_rest",
            input: "{n[{in(m,p), q, s}], m[r]}",
            expected_output: Some("m[{n[{p, q, s}], r}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Entry with multiple items in rest",
        },
        
        TestCase {
            name: "exit_nonempty_rest",
            input: "m[{n[{out(m,p), q}], r}]",
            expected_output: Some("{n[{p, q}], m[r]}"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Exit with non-empty rest: preserves q during exit",
        },
        
        TestCase {
            name: "exit_multiple_rest",
            input: "m[{n[{out(m,p), q, s, t}], r}]",
            expected_output: Some("{n[{p, q, s, t}], m[r]}"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Exit with multiple items in rest",
        },
        
        // Context preservation
        TestCase {
            name: "context_preservation",
            input: "{n[{in(m,p), state1, state2}], m[{r, local}]}",
            expected_output: Some("m[{n[{p, state1, state2}], r, local}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Both ambients preserve their local state",
        },
        
        // Parallel operations
        TestCase {
            name: "parallel_entry",
            input: "{a[{in(parent,x)}], b[{in(parent,y)}], parent[z]}",
            expected_output: None, // Multiple possible outcomes
            should_normalize: false, // May have multiple normal forms
            min_rewrites: 2, // At least two rewrites (one for each entry)
            description: "Two ambients entering the same parent in parallel",
        },
        
        // Sequential operations
        TestCase {
            name: "sequential_mobility",
            input: "{agent[{in(loc1, in(loc2, 0))}], loc1[0], loc2[0]}",
            expected_output: None, // Complex chain
            should_normalize: false,
            min_rewrites: 1, // At least the first move
            description: "Agent moves through multiple locations",
        },
        
        // Nested mobility
        TestCase {
            name: "nested_mobility",
            input: "{parent[{in(grandparent,0), child[0]}], grandparent[0]}",
            expected_output: Some("grandparent[{0, parent[{0, child[0]}]}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Parent with child moves together",
        },
        
        // Complex interactions
        TestCase {
            name: "entry_then_exit",
            input: "{n[{in(m, out(m, p))}], m[r]}",
            expected_output: None, // Chain of rewrites
            should_normalize: true,
            min_rewrites: 2, // Enter, then exit
            description: "Agent enters then immediately exits",
        },
        
        TestCase {
            name: "open_after_entry",
            input: "{agent[{in(container, 0)}], container[{open(agent, result)}]}",
            expected_output: None, // Multi-step
            should_normalize: true,
            min_rewrites: 2, // Entry then open
            description: "Agent enters container, then is opened",
        },
        
        // Edge cases
        TestCase {
            name: "zero_in_context",
            input: "{n[{in(m,p), 0}], m[r]}",
            expected_output: Some("m[{n[{p, 0}], r}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Zero explicitly in rest pattern",
        },
        
        TestCase {
            name: "nested_ambients_in_rest",
            input: "{n[{in(m,p), inner[data]}], m[r]}",
            expected_output: Some("m[{n[{p, inner[data]}], r}]"),
            should_normalize: true,
            min_rewrites: 1,
            description: "Nested ambient preserved in rest during move",
        },
        
        // Congruence closure tests
        TestCase {
            name: "congruence_in_ambient",
            input: "outer[{n[{in(m,p)}], m[r]}]",
            expected_output: None, // Rewrites under ambient
            should_normalize: false,
            min_rewrites: 1,
            description: "Rewrite applies under ambient constructor",
        },
        
        TestCase {
            name: "congruence_in_parallel",
            input: "{{n[{in(m,p)}], m[r]}, observer}",
            expected_output: None, // Rewrites in parallel context
            should_normalize: false,
            min_rewrites: 1,
            description: "Rewrite applies within parallel composition",
        },
    ];
    
    println!("\nRunning {} ambient calculus tests...", tests.len());
    
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    
    for test in &tests {
        match run_test(test) {
            Ok(()) => passed += 1,
            Err(e) => {
                failed += 1;
                errors.push((test.name, e));
            }
        }
    }
    
    println!("TEST SUMMARY");
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

