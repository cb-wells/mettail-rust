use anyhow::Result;
use clap::Parser;
use mettail_repl::{Repl, build_registry};

/// MeTTaIL Term Explorer - Interactive REPL for exploring rewrite systems
#[derive(Parser, Debug)]
#[command(name = "mettail")]
#[command(about = "Interactive term exploration for MeTTaIL theories", long_about = None)]
struct Args {
    /// Theory to load on startup
    #[arg(value_name = "THEORY")]
    theory: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Build the theory registry
    let registry = build_registry().unwrap_or_else(|e| {
        eprintln!("Warning: {}", e);
        eprintln!("Continuing with empty registry...");
        mettail_repl::TheoryRegistry::new()
    });
    
    // Create and run the REPL
    let mut repl = Repl::new(registry)?;
    
    // If a theory was specified, try to load it
    if let Some(theory_name) = args.theory {
        println!("Auto-loading theory: {}", theory_name);
        // TODO: Actually load the theory
    }
    
    repl.run()?;
    
    Ok(())
}

