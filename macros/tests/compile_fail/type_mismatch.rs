use mettail_macros::theory;

theory! {
    name: Invalid,
    
    exports {
        Proc
        Name
    }
    
    terms {
        PZero . Proc ::= "0" ;
        NQuote . Name ::= "@" (Proc) ;
        PDrop . Proc ::= "*" (Name) ;
    }
    
    equations {
        // Error: type mismatch - Proc != Name
        (PZero) == (NQuote (PZero))
    }
}

fn main() {}

