use mettail_macros::theory;

theory! {
    name: Invalid,

    exports {
        Proc
    }

    terms {
        // Error: references undefined 'Name' category
        Drop . Proc ::= "*" (Name) ;
    }
}

fn main() {}

