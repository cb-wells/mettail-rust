// Ambient Calculus demonstration - uses theory from mettail-theories library
//
// This example shows the Ambient Calculus with capabilities:
// in, out, and open operations on nested ambients.

use ascent_byods_rels::*;
use mettail_theories::ambient::*;
use ascent::*;
use std::time::Instant;

fn main() {
    println!("=== Ambient Calculus Demo ===\n");
    
    let start_time = Instant::now();
    
    // Example 1: In capability
    println!("Example 1: In capability");
    let input1 = "{n[in(m,p)]|m[r]}";
    demonstrate_rewrite(input1);
    
    // Example 2: Out capability
    println!("\nExample 2: Out capability");
    let input2 = "m[{n[out(m,p)]|r}]";
    demonstrate_rewrite(input2);
    
    // Example 3: Open capability
    println!("\nExample 3: Open capability");
    let input3 = "{open(n,p)|n[q]}";
    demonstrate_rewrite(input3);
    
    let elapsed = Instant::now().duration_since(start_time);
    println!("\n✅ Total time: {:?}", elapsed);
}

fn demonstrate_rewrite(input: &str) {
    println!("  Input:  {}", input);
    
    // Parse the term
    mettail_runtime::clear_var_cache();
    let parser = ambient::ProcParser::new();
    let term = parser.parse(input).unwrap_or_else(|e| {
        eprintln!("  Parse error: {:?}", e);
        std::process::exit(1);
    });
    
    // Run Ascent - use module path to locate the macro
    let prog = ascent_run! {
        include_source!(mettail_theories::ambient::ambient_source);
        proc(term.clone());
    };
    
    // Show results
    let procs: Vec<_> = prog.proc.iter().map(|(p,)| p).collect();
    let rewrites: Vec<_> = prog.rw_proc.iter().collect();
    
    println!("  Terms reachable: {}", procs.len());
    println!("  Rewrites: {}", rewrites.len());
    
    // Find normal forms
    let normal_forms: Vec<_> = procs.iter()
        .filter(|p| !rewrites.iter().any(|(from, _)| from == **p))
        .collect();
    
    if normal_forms.is_empty() {
        println!("  No normal form found");
    } else {
        println!("  Normal forms: {}", normal_forms.len());
        for nf in normal_forms.iter().take(5) {
            println!("    → {}", nf);
        }
        if normal_forms.len() > 5 {
            println!("    ... and {} more", normal_forms.len() - 5);
        }
    }
}

