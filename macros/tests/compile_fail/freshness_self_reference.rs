use mettail_macros::theory;

theory! {
    name: Invalid,

    exports {
        Name
    }

    terms {
        NVar . Name ::= "var" ;
    }

    equations {
        // Error: x # x is invalid (variable fresh in itself)
        if x # x then x == (NVar)
    }
}

fn main() {}

