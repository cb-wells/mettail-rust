use mettail_macros::theory;

theory! {
    name: Invalid,
    
    exports {
        Proc
    }
    
    terms {
        // Error: 'Name' is not exported
        Quote . Name ::= "@" (Proc) ;
    }
}

fn main() {}

