#![allow(
    non_local_definitions,
    clippy::crate_in_macro_def,
    clippy::empty_line_after_outer_attr
)]

use mettail_macros::theory;

// Simple integer calculator theory: supports integer literals, + and -
// Uses native i32 type for direct integer support
theory! {
    name: Calculator,
    exports {
        ![i32] as Int
    },
    terms {
        // Variables parse as VarRef nodes
        VarRef . Int ::= Var ;
        // Integer literals - uses Integer keyword for native integer type
        NumLit . Int ::= Integer ;

        Add . Int ::= Int "+" Int ;
        Sub . Int ::= Int "-" Int ;
        
        // Assignment: x = expr evaluates expr and stores result
        Assign . Int ::= Var "=" Int ;
    },
    equations {
    },
    rewrites {
        // Variable substitution: if env_var(x, v) then VarRef(x) => NumLit(v)
        if env_var(x, v) then (VarRef x) => (NumLit v);
        
        // Congruence rules: propagate rewrites through Add, Sub, and Assign
        if S => T then (Add S R) => (Add T R);
        if S => T then (Add L S) => (Add L T);
        if S => T then (Sub S R) => (Sub T R);
        if S => T then (Sub L S) => (Sub L T);
        if S => T then (Assign x S) => (Assign x T);
    },
    semantics {
        Add: +,
        Sub: -,
    }
}
