#![allow(non_local_definitions)]

use mettail_macros::theory;

// Ambient Calculus Theory Definition
theory! {
    name: Ambient,
    exports {
        Proc
        Name
    },
    terms {
        PZero . Proc ::= "0" ;

        PIn . Proc ::= "in(" Name "," Proc ")";
        POut . Proc ::= "out(" Name "," Proc ")";
        POpen . Proc ::= "open(" Name "," Proc ")";

        PAmb . Proc ::= Name "[" Proc "]";
        PNew . Proc ::= "new(" <Name> "," Proc ")";

        PPar . Proc ::= HashBag(Proc) sep "|" delim "{" "}" ;

        PVar . Proc ::= Var;
        NVar . Name ::= Var ;
    },
    equations {
        if x # P then (PPar {(PNew x P), ...rest}) == (PNew x (PPar {P, ...rest}));
        if x # P then (PIn N (PNew x P)) == (PNew x (PIn N P));
        if x # P then (POut N (PNew x P)) == (PNew x (POut N P));
        if x # P then (POpen N (PNew x P)) == (PNew x (POpen N P));
        if x # P then (PAmb N (PNew x P)) == (PNew x (PAmb N P));
        (PNew x (PNew y P)) == (PNew y (PNew x P));
    },
    rewrites {
        // {n[{in(m,p), ...q}], m[r]} => {m[{n[{p, ...q}], r}]}
        (PPar {(PAmb N (PPar {(PIn M P) , ...rest})) , (PAmb M R)})
            => (PPar {(PAmb M (PPar {(PAmb N (PPar {P , ...rest})), R}))});

        // m[{n[{out(m,p), ...q}], r}] => {n[{p, ...q}], m[r]}
        (PAmb M (PPar {(PAmb N (PPar {(POut M P), ...rest})), R}))
            => (PPar {(PAmb N (PPar {P, ...rest})), (PAmb M R)});

        // {open(n,p), n[q]} => {p, q}
        (PPar {(POpen N P), (PAmb N Q)})
            => (PPar {P,Q});

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});

        if S => T then (PNew x S) => (PNew x T);
        if S => T then (PAmb N S) => (PAmb N T);
    }
}
