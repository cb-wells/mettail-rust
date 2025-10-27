// Tests for parsing full Rho Calculus syntax with the generated grammar
// This tests: a!(0) | b!(c!(0)) | for(a x){*x}

use mettail_runtime::{Var, Binder, Scope, BoundTerm};
use lalrpop_util::lalrpop_mod;

// Define the AST types matching the generated Rho Calculus types
#[derive(Debug, Clone, PartialEq, BoundTerm)]
pub enum Proc {
    PZero,
    PInput(Box<Name>, Scope<Binder<String>, Box<Proc>>),
    POutput(Box<Name>, Box<Proc>),
    PPar(Box<Proc>, Box<Proc>),
    PDrop(Box<Name>),
}

#[derive(Debug, Clone, PartialEq, BoundTerm)]
pub enum Name {
    NQuote(Box<Proc>),
    NVar(Var<String>),
}

// Include the generated parser
lalrpop_mod!(pub rhocalc);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_zero() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("0");
        assert!(result.is_ok(), "Failed to parse '0': {:?}", result.err());
        println!("✓ Parsed: 0");
    }
    
    #[test]
    fn test_parse_simple_output() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("a!(0)");
        assert!(result.is_ok(), "Failed to parse 'a!(0)': {:?}", result.err());
        println!("✓ Parsed: a!(0)");
    }
    
    #[test]
    fn test_parse_nested_output() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("b!(c!(0))");
        assert!(result.is_ok(), "Failed to parse 'b!(c!(0))': {:?}", result.err());
        println!("✓ Parsed: b!(c!(0))");
    }
    
    #[test]
    fn test_parse_parallel() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("a!(0) | b!(0)");
        assert!(result.is_ok(), "Failed to parse 'a!(0) | b!(0)': {:?}", result.err());
        println!("✓ Parsed: a!(0) | b!(0)");
    }
    
    #[test]
    fn test_parse_input() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("for(a x){*x}");
        assert!(result.is_ok(), "Failed to parse 'for(a x){{*x}}': {:?}", result.err());
        println!("✓ Parsed: for(a x){{*x}}");
    }
    
    #[test]
    fn test_parse_drop() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("*x");
        assert!(result.is_ok(), "Failed to parse '*x': {:?}", result.err());
        println!("✓ Parsed: *x");
    }
    
    #[test]
    fn test_parse_quote() {
        let parser = rhocalc::NameParser::new();
        let result = parser.parse("@(0)");
        assert!(result.is_ok(), "Failed to parse '@(0)': {:?}", result.err());
        println!("✓ Parsed: @(0)");
    }
    
    #[test]
    fn test_parse_parentheses() {
        let parser = rhocalc::ProcParser::new();
        let result = parser.parse("(a!(0))");
        assert!(result.is_ok(), "Failed to parse '(a!(0))': {:?}", result.err());
        println!("✓ Parsed: (a!(0))");
    }
    
    #[test]
    fn test_parse_complex_expression() {
        let parser = rhocalc::ProcParser::new();
        // The target expression: a!(0) | b!(c!(0)) | for(a x){*x}
        let result = parser.parse("a!(0) | b!(c!(0)) | for(a x){*x}");
        assert!(result.is_ok(), 
            "Failed to parse 'a!(0) | b!(c!(0)) | for(a x){{*x}}': {:?}", 
            result.err());
        println!("✓ Parsed: a!(0) | b!(c!(0)) | for(a x){{*x}}");
    }
    
    #[test]
    fn test_parse_left_associativity() {
        let parser = rhocalc::ProcParser::new();
        // Should parse as ((a!(0) | b!(0)) | c!(0))
        let result = parser.parse("a!(0) | b!(0) | c!(0)");
        assert!(result.is_ok(), 
            "Failed to parse 'a!(0) | b!(0) | c!(0)': {:?}", 
            result.err());
        
        // Verify structure is left-associative
        let ast = result.unwrap();
        match ast {
            Proc::PPar(left, _right) => {
                // Left should also be PPar (left-associative)
                match left.as_ref() {
                    Proc::PPar(_, _) => {
                        println!("✓ Verified left-associativity: ((a!(0) | b!(0)) | c!(0))");
                    }
                    _ => panic!("Expected left-associative parse"),
                }
            }
            _ => panic!("Expected PPar at top level"),
        }
    }
    
    #[test]
    fn test_parse_with_explicit_parentheses() {
        let parser = rhocalc::ProcParser::new();
        // Should parse as (a!(0) | (b!(0) | c!(0)))
        let result = parser.parse("a!(0) | (b!(0) | c!(0))");
        assert!(result.is_ok(), 
            "Failed to parse 'a!(0) | (b!(0) | c!(0))': {:?}", 
            result.err());
        
        // Verify structure respects parentheses
        let ast = result.unwrap();
        match ast {
            Proc::PPar(left, right) => {
                // Left should NOT be PPar (right-associative due to parens)
                match left.as_ref() {
                    Proc::PPar(_, _) => panic!("Unexpected left-associativity with explicit parens"),
                    Proc::POutput(_, _) => {
                        // Right should be PPar
                        match right.as_ref() {
                            Proc::PPar(_, _) => {
                                println!("✓ Verified right-associativity with parens: (a!(0) | (b!(0) | c!(0)))");
                            }
                            _ => panic!("Expected PPar on right"),
                        }
                    }
                    _ => panic!("Expected POutput on left"),
                }
            }
            _ => panic!("Expected PPar at top level"),
        }
    }
}

