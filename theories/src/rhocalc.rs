use mettail_macros::theory;

// RhoCalc Theory Definition
theory! {
    name: RhoCalc,
    
    exports {
        Proc
        Name
    },
    
    terms {
        PZero . Proc ::= "0" ;
        PDrop . Proc ::= "*" "(" Name ")" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;

        PPar . Proc ::= HashBag(Proc) sep "|" delim "{" "}" ;

        NQuote . Name ::= "@" "(" Proc ")" ;

        PVar . Proc ::= Var;
        NVar . Name ::= Var;
    },
    
    equations {
        (NQuote (PDrop N)) == N ;
    },
        
    rewrites {
        (PPar {(PInput N x P), (POutput N Q)})
            => (PPar {(subst P x (NQuote Q))});
        
        (PDrop (NQuote P)) => P;

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
    },
}
