use ascent_byods_rels::*;
use ascent::*;
use mettail_theories::ambient::*;


fn test_equation(name: &str, lhs_str: &str, rhs_str: &str) {
    let parser = ambient::ProcParser::new();
    
    let lhs = parser.parse(lhs_str)
        .expect(&format!("Failed to parse LHS: {}", lhs_str));
    let rhs = parser.parse(rhs_str)
        .expect(&format!("Failed to parse RHS: {}", rhs_str));
    
    println!("  LHS: {}", lhs_str);
    println!("  RHS: {}", rhs_str);
    
    // Run Ascent to compute equalities
    let prog = ascent_run! {
        include_source!(ambient_source);
        proc(p) <-- for p in [lhs.clone(), rhs.clone()];
    };
    
    println!("  proc relation size: {}", prog.proc.len());
    println!("  eq_proc count_exact: {}", prog.__eq_proc_ind_common.count_exact());
    
    // For eqrel, iterate over all added pairs to check equality
    let are_equal = prog.__eq_proc_ind_common.iter_all_added()
        .any(|(p1, p2)| {
            (p1 == &lhs && p2 == &rhs) || (p1 == &rhs && p2 == &lhs)
        });
    
    if are_equal {
        println!("  ✓ Equal");
    } else {
        println!("  ✗ NOT Equal");
    }
    
    assert!(are_equal, "{} failed: terms should be equal", name);
}

fn main() {
    println!("Testing Ambient Calculus Equations\n");
    
    // Test 1: Zero identity - {P, 0} == P
    println!("=== Equation 1: Zero Identity ===");
    test_equation(
        "zero_identity",
        "{n[0], 0}",
        "n[0]"
    );
    
    // Test 2: Scope extrusion - {P, new(x,C)} == new(x, {P,C})
    println!("\n=== Equation 2: Scope Extrusion ===");
    test_equation(
        "scope_extrusion",
        "{p, new(x, q)}",
        "new(x, {p, q})"
    );
    
    // Test 3: Extrusion with in capability
    println!("\n=== Equation 3: Extrusion with In Capability ===");
    test_equation(
        "extrusion_in",
        "new(x, {p, in(n, q)})",  // Both P's are m[0]
        "{p, in(n, new(x, q))}"
    );
    
    // Test 4: Extrusion with out capability
    println!("\n=== Equation 4: Extrusion with Out Capability ===");
    test_equation(
        "extrusion_out",
        "new(y, {p, out(n, q)})",
        "{p, out(n, new(y, q))}"
    );
    
    // Test 5: Extrusion with open capability
    println!("\n=== Equation 5: Extrusion with Open Capability ===");
    test_equation(
        "extrusion_open",
        "new(z, {p, open(n, q)})",
        "{p, open(n, new(z, q))}"
    );
    
    // Test 6: Extrusion with ambient capability
    println!("\n=== Equation 6: Extrusion with Amb Capability ===");
    test_equation(
        "extrusion_amb",        
        "new(w, {p, n[q]})",
        "{p, n[new(w, q)]}"
    );
    
}
