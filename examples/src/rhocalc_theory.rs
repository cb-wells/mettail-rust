use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;

// Re-export generated parser (the macro uses lalrpop_mod! internally)

theory! {
    name: RhoCalc,
    
    exports {
        Proc
        Name
    },
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name "->" <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        PPar . Proc ::= HashBag(Proc) sep "," delim "{" "}" ;
        PDrop . Proc ::= "*" Name ;

        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    },
    
    equations {
        (NQuote (PDrop N)) == N ;
        (PPar {P}) == P;
        (PPar {}) == PZero ;
    },
        
    rewrites {
        (PPar {(PInput chan x P) , (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
        
        (PPar {(PDrop (NQuote P)), ...rest}) => (PPar {P, ...rest});

        if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest});
    }
}
