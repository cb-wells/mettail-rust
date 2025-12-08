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
    }

    equations {
        // Error: unknown constructor 'Unknown'
        (Unknown) == (PZero)
    }
}

fn main() {}

