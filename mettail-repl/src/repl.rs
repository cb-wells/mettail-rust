use crate::registry::TheoryRegistry;
use crate::state::ReplState;
use anyhow::Result;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustyResult};

/// The main REPL
pub struct Repl {
    state: ReplState,
    registry: TheoryRegistry,
    editor: DefaultEditor,
}

impl Repl {
    /// Create a new REPL
    pub fn new(registry: TheoryRegistry) -> RustyResult<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self {
            state: ReplState::new(),
            registry,
            editor,
        })
    }
    
    /// Run the REPL
    pub fn run(&mut self) -> Result<()> {
        self.print_banner();
        
        loop {
            let prompt = self.make_prompt();
            match self.editor.readline(&prompt) {
                Ok(line) => {
                    self.editor.add_history_entry(&line)?;
                    
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    if let Err(e) = self.handle_command(line) {
                        eprintln!("{} {}", "Error:".red().bold(), e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("exit");
                    break;
                }
                Err(err) => {
                    eprintln!("{} {:?}", "Error:".red().bold(), err);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn print_banner(&self) {
        println!("{}", "╔════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║                   MeTTaIL Term Explorer                     ║".cyan());
        println!("{}", "║                      Version 0.1.0                          ║".cyan());
        println!("{}", "╚════════════════════════════════════════════════════════════╝".cyan());
        println!();
        println!("Type {} for available commands.", "'help'".green());
        println!();
    }
    
    fn make_prompt(&self) -> String {
        if let Some(theory_name) = self.state.theory_name() {
            format!("{}> ", theory_name.green())
        } else {
            "mettail> ".to_string()
        }
    }
    
    fn handle_command(&mut self, line: &str) -> Result<()> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        
        match parts[0] {
            "help" => self.cmd_help(),
            "load" => self.cmd_load(&parts[1..]),
            "list" | "list-theories" => self.cmd_list_theories(),
            "info" => self.cmd_info(),
            "rewrites" => self.cmd_rewrites(),
            "normal-forms" | "nf" => self.cmd_normal_forms(),
            "apply" => self.cmd_apply(&parts[1..]),
            "goto" => self.cmd_goto(&parts[1..]),
            "quit" | "exit" => {
                println!("Goodbye!");
                std::process::exit(0);
            }
            _ => {
                // Check if it's a term input
                if line.starts_with("term:") {
                    let term_str = line["term:".len()..].trim();
                    self.cmd_parse_term(term_str)
                } else {
                    anyhow::bail!("Unknown command: '{}'. Type 'help' for available commands.", parts[0])
                }
            }
        }
    }
    
    fn cmd_help(&self) -> Result<()> {
        println!();
        println!("{}", "Available commands:".bold());
        println!();
        println!("{}", "  Theory Management:".yellow());
        println!("    {}  Load a theory", "load <name>".green());
        println!("    {}        Show available theories", "list-theories".green());
        println!("    {}              Show theory information", "info".green());
        println!();
        println!("{}", "  Term Input:".yellow());
        println!("    {}    Parse and load a term", "term: <expr>".green());
        println!();
        println!("{}", "  Navigation:".yellow());
        println!("    {}           List rewrites from current term", "rewrites".green());
        println!("    {}        Show normal forms", "normal-forms".green());
        println!("    {} Apply rewrite N", "apply <N>".green());
        println!("    {}              Go to normal form N", "goto <N>".green());
        println!();
        println!("{}", "  General:".yellow());
        println!("    {}              Show this help", "help".green());
        println!("    {}        Exit REPL", "quit, exit".green());
        println!();
        Ok(())
    }
    
    fn cmd_load(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            anyhow::bail!("Usage: load <theory-name>");
        }
        
        let theory_name = args[0];
        
        if !self.registry.contains(theory_name) {
            anyhow::bail!("Theory '{}' not found. Use 'list-theories' to see available theories.", theory_name);
        }
        
        println!("Loading theory: {}", theory_name.green());
        
        // Get the theory from the registry (for display info)
        let theory = self.registry.get(theory_name)?;
        
        // Print theory info
        println!("  ✓ {} categories", theory.categories().len());
        println!("  ✓ {} constructors", theory.constructor_count());
        println!("  ✓ {} equations", theory.equation_count());
        println!("  ✓ {} rewrite rules", theory.rewrite_count());
        println!();
        
        // Store the theory name in state
        self.state.load_theory(theory_name.to_string());
        
        println!("{} Theory loaded successfully!", "✓".green());
        println!("Use {} to parse and execute a term.", "'term: <expr>'".cyan());
        println!();
        
        Ok(())
    }
    
    fn cmd_list_theories(&self) -> Result<()> {
        println!();
        println!("{}", "Available theories:".bold());
        println!();
        
        let theories = self.registry.list();
        if theories.is_empty() {
            println!("  {}", "No theories available.".yellow());
            println!("  {}", "Build mettail-examples first with: cargo build".dimmed());
        } else {
            for theory in theories {
                println!("  - {}", theory.green());
            }
        }
        
        println!();
        Ok(())
    }
    
    fn cmd_info(&self) -> Result<()> {
        if let Some(theory_name) = self.state.theory_name() {
            let theory = self.registry.get(theory_name)?;
            println!();
            println!("{} {}", "Theory:".bold(), theory.name().green());
            println!("  Categories: {}", theory.categories().len());
            println!("  Constructors: {}", theory.constructor_count());
            println!("  Equations: {}", theory.equation_count());
            println!("  Rewrites: {}", theory.rewrite_count());
            println!();
        } else {
            println!("{} No theory loaded. Use 'load <name>' first.", "Info:".yellow());
        }
        Ok(())
    }
    
    fn cmd_parse_term(&mut self, term_str: &str) -> Result<()> {
        // Get the loaded theory name
        let theory_name = self.state.theory_name()
            .ok_or_else(|| anyhow::anyhow!("No theory loaded. Use 'load <theory>' first."))?;

        // Get the theory from the registry
        let theory = self.registry.get(theory_name)?;

        println!();
        print!("Parsing... ");

        // Parse the term
        let term = theory.parse_term(term_str)?;
        println!("{}", "✓".green());

        print!("Running Ascent... ");

        // Run Ascent
        let results = theory.run_ascent(term.clone_box())?;
        println!("{}", "Done!".green());

        println!();
        println!("Computed:");
        println!("  - {} terms", results.all_terms.len());
        println!("  - {} rewrites", results.rewrites.len());
        println!("  - {} normal forms", results.normal_forms().len());
        println!();

        println!("Current term: {}", format!("{}", term).cyan());
        println!();

        // Store in state
        self.state.set_term(term, results)?;

        Ok(())
    }
    
    fn cmd_rewrites(&self) -> Result<()> {
        let results = self.state.ascent_results()
            .ok_or_else(|| anyhow::anyhow!("No term loaded. Use 'term: <expr>' first."))?;
        
        let current_id = self.state.current_graph_id()
            .ok_or_else(|| anyhow::anyhow!("No current term"))?;
        
        // Find rewrites from the current term
        let available_rewrites: Vec<_> = results.rewrites.iter()
            .filter(|r| r.from_id == current_id)
            .collect();
        
        println!();
        if available_rewrites.is_empty() {
            println!("{} No rewrites available from current term (it's a normal form).", "✓".green());
        } else {
            println!("{} available from current term:", "Rewrites".bold());
            println!();
            for (idx, rewrite) in available_rewrites.iter().enumerate() {
                // Find the target term display
                let target_display = results.all_terms.iter()
                    .find(|t| t.term_id == rewrite.to_id)
                    .map(|t| t.display.as_str())
                    .unwrap_or("<unknown>");
                
                println!("  {}) {} {}", 
                    idx.to_string().cyan(),
                    "→".yellow(),
                    target_display.green()
                );
            }
        }
        println!();
        Ok(())
    }
    
    fn cmd_normal_forms(&self) -> Result<()> {
        let results = self.state.ascent_results()
            .ok_or_else(|| anyhow::anyhow!("No term loaded. Use 'term: <expr>' first."))?;
        
        let normal_forms = results.normal_forms();
        
        println!();
        if normal_forms.is_empty() {
            println!("{} No normal forms computed.", "Warning:".yellow());
        } else {
            println!("{} ({} total):", "Normal forms".bold(), normal_forms.len());
            println!();
            for (idx, nf) in normal_forms.iter().enumerate() {
                println!("  {}) {}", idx.to_string().cyan(), nf.display.green());
            }
        }
        println!();
        Ok(())
    }
    
    fn cmd_apply(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            anyhow::bail!("Usage: apply <rewrite-number>");
        }
        
        let idx: usize = args[0].parse()
            .map_err(|_| anyhow::anyhow!("Invalid number: {}", args[0]))?;
        
        let theory_name = self.state.theory_name()
            .ok_or_else(|| anyhow::anyhow!("No theory loaded"))?;
        
        let theory = self.registry.get(theory_name)?;
        
        let results = self.state.ascent_results()
            .ok_or_else(|| anyhow::anyhow!("No term loaded"))?;
        
        let current_id = self.state.current_graph_id()
            .ok_or_else(|| anyhow::anyhow!("No current term"))?;
        
        // Find available rewrites
        let available_rewrites: Vec<_> = results.rewrites.iter()
            .filter(|r| r.from_id == current_id)
            .collect();
        
        if idx >= available_rewrites.len() {
            anyhow::bail!("Rewrite {} not found. Use 'rewrites' to see available rewrites.", idx);
        }
        
        let rewrite = available_rewrites[idx];
        
        // Find the target term
        let target_info = results.all_terms.iter()
            .find(|t| t.term_id == rewrite.to_id)
            .ok_or_else(|| anyhow::anyhow!("Target term not found"))?;
        
        // Parse the target term and update its ID to match what's in the graph
        let target_term = theory.parse_term(&target_info.display)?;
        
        println!();
        println!("Applied rewrite {} {}", "→".yellow(), target_info.display.green());
        println!();
        
        // Update state - pass the target_id so we can track position in the graph
        self.state.set_term_with_id(target_term, results.clone(), rewrite.to_id)?;
        
        Ok(())
    }
    
    fn cmd_goto(&mut self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            anyhow::bail!("Usage: goto <normal-form-number>");
        }
        
        let idx: usize = args[0].parse()
            .map_err(|_| anyhow::anyhow!("Invalid number: {}", args[0]))?;
        
        let theory_name = self.state.theory_name()
            .ok_or_else(|| anyhow::anyhow!("No theory loaded"))?;
        
        let theory = self.registry.get(theory_name)?;
        
        let results = self.state.ascent_results()
            .ok_or_else(|| anyhow::anyhow!("No term loaded"))?;
        
        let normal_forms = results.normal_forms();
        
        if idx >= normal_forms.len() {
            anyhow::bail!("Normal form {} not found. Use 'normal-forms' to see available normal forms.", idx);
        }
        
        let target_info = &normal_forms[idx];
        
        // Parse the target term
        let target_term = theory.parse_term(&target_info.display)?;
        
        println!();
        println!("Navigated to normal form: {}", target_info.display.green());
        println!();
        
        // Update state with the correct graph ID
        self.state.set_term_with_id(target_term, results.clone(), target_info.term_id)?;
        
        Ok(())
    }
}

