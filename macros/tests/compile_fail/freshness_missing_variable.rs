use mettail_macros::theory;

theory! {
    name: Invalid,

    exports {
        Name
    }

    terms {
        NZero . Name ::= "@0" ;
    }

    equations {
        // Error: freshness variable 'x' doesn't appear in equation
        if x # Q then (NZero) == (NZero)
    }
}

fn main() {}

