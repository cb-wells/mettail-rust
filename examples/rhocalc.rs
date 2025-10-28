use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;

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
            => (subst P x (NQuote Q))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mettail_runtime::{FreeVar, Binder, Scope, Var};
    
    #[test]
    fn test_communication_substitution() {
        println!("\n=== Testing Rho Calculus Communication ===\n");
        
        // Example 1: for(x y){*y} | x!(0) => *@0
        // Here x is the channel, y is the bound variable
        println!("Example 1: for(x y){{*y}} | x!(0) => *@0");
        
        // Create channel name 'x'
        let x_name = Name::NVar(Var::Free(FreeVar::fresh_named("x")));
        
        // Create bound variable 'y' in the input
        let y = FreeVar::fresh_named("y");
        let body = Proc::PDrop(Box::new(Name::NVar(Var::Free(y.clone()))));
        let input = Proc::PInput(
            Box::new(x_name.clone()),
            Scope::new(Binder(y.clone()), Box::new(body))
        );
        
        // Create output: x!(0)
        let zero = Proc::PZero;
        let output = Proc::POutput(Box::new(x_name.clone()), Box::new(zero));
        
        // Parallel composition: for(x y){*y} | x!(0)
        let parallel = Proc::PPar(Box::new(input), Box::new(output));
        
        println!("  Before: {:?}", parallel);
        
        // Simulate communication: P[@Q/y] where P = *y, Q = 0
        // We substitute @Q for y in P
        let quoted_zero = Name::NQuote(Box::new(Proc::PZero));
        
        // Extract the input body and perform substitution
        let (_chan, binder, input_body) = match &parallel {
            Proc::PPar(left, _right) => {
                match left.as_ref() {
                    Proc::PInput(chan, scope) => {
                        let (b, body) = scope.clone().unbind();
                        (chan, b, body)
                    }
                    _ => panic!("Expected PInput"),
                }
            }
            _ => panic!("Expected PPar"),
        };
        
        println!("  Binder: {:?}", binder);
        println!("  Body before subst: {:?}", input_body);
        
        // NOW WE CAN USE CROSS-CATEGORY SUBSTITUTION! ✅
        // The body is a Proc (*y), and y is bound as a Name
        // We use substitute_name to substitute @0 for y
        let result = input_body.substitute_name(&binder.0, &quoted_zero);
        
        // Expected result: *@0
        let expected = Proc::PDrop(Box::new(quoted_zero.clone()));
        
        assert_eq!(result, expected, "Communication should produce *@0");
        println!("  After:  {:?}", result);
        println!("  ✓ Communication: for(x y){{*y}} | x!(0) => *@0\n");
    }
    
    #[test]
    fn test_generated_display() {
        println!("\n=== Testing Generated Display ===\n");
        
        // Test Display for various Proc constructors
        let zero = Proc::PZero;
        assert_eq!(format!("{}", zero), "0");
        println!("✓ PZero displays as: {}", zero);
        
        // Test variable name
        let x_name = Name::NVar(Var::Free(FreeVar::fresh_named("x")));
        let display = format!("{}", x_name);
        assert_eq!(display, "x");
        println!("✓ NVar displays as: {}", display);
        
        // Test output
        let output = Proc::POutput(Box::new(x_name.clone()), Box::new(Proc::PZero));
        let display = format!("{}", output);
        assert_eq!(display, "x!(0)");
        println!("✓ POutput displays as: {}", display);
        
        // Test drop
        let drop = Proc::PDrop(Box::new(x_name.clone()));
        let display = format!("{}", drop);
        assert_eq!(display, "*x");
        println!("✓ PDrop displays as: {}", display);
        
        // Test quote
        let quote = Name::NQuote(Box::new(Proc::PZero));
        let display = format!("{}", quote);
        assert_eq!(display, "@(0)");
        println!("✓ NQuote displays as: {}", display);
        
        // Test parallel
        let par = Proc::PPar(
            Box::new(Proc::POutput(Box::new(Name::NVar(Var::Free(FreeVar::fresh_named("a")))), Box::new(Proc::PZero))),
            Box::new(Proc::POutput(Box::new(Name::NVar(Var::Free(FreeVar::fresh_named("b")))), Box::new(Proc::PZero)))
        );
        let display = format!("{}", par);
        assert_eq!(display, "a!(0)|b!(0)");  // Note: No spaces around | - matches grammar
        println!("✓ PPar displays as: {}", display);
        
        // Test binder (PInput)
        let chan = Name::NVar(Var::Free(FreeVar::fresh_named("ch")));
        let x_var = FreeVar::fresh_named("x");
        let body = Proc::PDrop(Box::new(Name::NVar(Var::Free(x_var.clone()))));
        let input = Proc::PInput(
            Box::new(chan),
            Scope::new(Binder(x_var), Box::new(body))
        );
        let display = format!("{}", input);
        assert_eq!(display, "for(ch<-x){*x}");  // Uses <- arrow for binder
        println!("✓ PInput displays as: {}", display);
        
        println!("\n✅ All Display implementations working correctly!");
    }
    
    #[test]
    fn test_round_trip() {
        println!("\n=== Testing Round-Trip Parsing ===\n");
        
        // Test that parse -> display -> parse produces stable output
        let test_cases = vec![
            "0",
            "*x",
            "a!(0)",
            "b!(c!(0))",
            "a!(0)|b!(0)",
            "for(a<-x){*x}",
            "a!(0)|for(a<-x){*x}",
        ];
        
        for input in test_cases {
            let parser = rhocalc::ProcParser::new();
            
            // Parse
            let ast = parser.parse(input)
                .unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", input, e));
            
            // Display
            let output = format!("{}", ast);
            
            // Parse again
            let ast2 = parser.parse(&output)
                .unwrap_or_else(|e| panic!("Failed to re-parse '{}': {:?}", output, e));
            
            // Display again - should be stable
            let output2 = format!("{}", ast2);
            
            assert_eq!(output, output2, 
                "Round-trip not stable for input '{}':\n  First:  {}\n  Second: {}", 
                input, output, output2);
            
            println!("✓ Round-trip OK: {} → {} → {}", input, output, output2);
        }
        
        println!("\n✅ All round-trip tests passed!");
    }
}

fn main() {
    println!("=== Rho Calculus Rewrite Demo ===\n");
    
    // Step function that tries all rewrite rules
    fn step_once(proc: &Proc) -> Option<Proc> {
        try_rewrite_rule_0(proc)
    }
    
    // Multi-step execution with visualization
    fn execute(input: &str, max_steps: usize) {
        let parser = rhocalc::ProcParser::new();
        let mut current = parser.parse(input)
            .unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", input, e));
        
        println!("Input:  {}", input);
        
        for step in 1..=max_steps {
            match step_once(&current) {
                Some(next) => {
                    current = next;
                    println!("Step {}: {}\n", step, current);
                }
                None => {
                    println!("→ Normal form reached after {} step(s)\n", step - 1);
                    return;
                }
            }
        }
        
        println!("→ Stopped after {} step(s) (max reached)\n", max_steps);
    }
    
    execute("for(a<-x){*x}|a!(0)", 5);
}

