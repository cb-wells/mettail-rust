// Simple lambda calculus example to test binders with moniker

use mettail_macros::theory;

theory! {
    name: LambdaCalc,
    
    exports {
        Expr
    },
    
    terms {
        // Variables will be handled by moniker
        EVar . Expr ::= "var" ;
        
        // Lambda: \x.e where x binds in e
        ELam . Expr ::= "\\" <Var> "." Expr ;
        
        // Application: e1 e2
        EApp . Expr ::= Expr Expr ;
    }
}

fn main() {
    println!("Lambda calculus example with binders!");
    
    // TODO: Create and evaluate lambda expressions
    // Example: (\x.x) y should evaluate to y
}

