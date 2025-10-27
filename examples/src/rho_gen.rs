#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use mettail_macros::theory;
use lalrpop_util::lalrpop_mod;
pub enum Proc {
    PZero,
    PInput(
        Box<Name>,
        mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>,
    ),
    POutput(Box<Name>, Box<Proc>),
    PPar(Box<Proc>, Box<Proc>),
    PDrop(Box<Name>),
}
#[automatically_derived]
impl ::core::fmt::Debug for Proc {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Proc::PZero => ::core::fmt::Formatter::write_str(f, "PZero"),
            Proc::PInput(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "PInput",
                    __self_0,
                    &__self_1,
                )
            }
            Proc::POutput(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "POutput",
                    __self_0,
                    &__self_1,
                )
            }
            Proc::PPar(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "PPar",
                    __self_0,
                    &__self_1,
                )
            }
            Proc::PDrop(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PDrop", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Proc {
    #[inline]
    fn clone(&self) -> Proc {
        match self {
            Proc::PZero => Proc::PZero,
            Proc::PInput(__self_0, __self_1) => {
                Proc::PInput(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Proc::POutput(__self_0, __self_1) => {
                Proc::POutput(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Proc::PPar(__self_0, __self_1) => {
                Proc::PPar(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Proc::PDrop(__self_0) => Proc::PDrop(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Proc {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Proc {
    #[inline]
    fn eq(&self, other: &Proc) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Proc::PInput(__self_0, __self_1), Proc::PInput(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                (
                    Proc::POutput(__self_0, __self_1),
                    Proc::POutput(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                (Proc::PPar(__self_0, __self_1), Proc::PPar(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                (Proc::PDrop(__self_0), Proc::PDrop(__arg1_0)) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Proc {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Box<Name>>;
        let _: ::core::cmp::AssertParamIsEq<
            mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>,
        >;
        let _: ::core::cmp::AssertParamIsEq<Box<Name>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Name>>;
    }
}
#[allow(non_upper_case_globals)]
const _DERIVE_moniker_BoundTerm_String_FOR_Proc: () = {
    extern crate moniker;
    impl moniker::BoundTerm<String> for Proc {
        fn term_eq(&self, other: &Self) -> bool {
            match (self, other) {
                (&Proc::PZero, &Proc::PZero) => true,
                (
                    &Proc::PInput(ref __binding_lhs_0, ref __binding_lhs_1),
                    &Proc::PInput(ref __binding_rhs_0, ref __binding_rhs_1),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_1, __binding_rhs_1)
                }
                (
                    &Proc::POutput(ref __binding_lhs_0, ref __binding_lhs_1),
                    &Proc::POutput(ref __binding_rhs_0, ref __binding_rhs_1),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_1, __binding_rhs_1)
                }
                (
                    &Proc::PPar(ref __binding_lhs_0, ref __binding_lhs_1),
                    &Proc::PPar(ref __binding_rhs_0, ref __binding_rhs_1),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_1, __binding_rhs_1)
                }
                (
                    &Proc::PDrop(ref __binding_lhs_0),
                    &Proc::PDrop(ref __binding_rhs_0),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                }
                (_, _) => false,
            }
        }
        fn close_term(
            &mut self,
            __state: moniker::ScopeState,
            __on_free: &impl moniker::OnFreeFn<String>,
        ) {
            match *self {
                Proc::PZero => {}
                Proc::PInput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_0, __state, __on_free);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_1, __state, __on_free);
                    }
                }
                Proc::POutput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_0, __state, __on_free);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_1, __state, __on_free);
                    }
                }
                Proc::PPar(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_0, __state, __on_free);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_1, __state, __on_free);
                    }
                }
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::close_term(__binding_0, __state, __on_free);
                }
            }
        }
        fn open_term(
            &mut self,
            __state: moniker::ScopeState,
            __on_bound: &impl moniker::OnBoundFn<String>,
        ) {
            match *self {
                Proc::PZero => {}
                Proc::PInput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_0, __state, __on_bound);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_1, __state, __on_bound);
                    }
                }
                Proc::POutput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_0, __state, __on_bound);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_1, __state, __on_bound);
                    }
                }
                Proc::PPar(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_0, __state, __on_bound);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_1, __state, __on_bound);
                    }
                }
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::open_term(__binding_0, __state, __on_bound);
                }
            }
        }
        fn visit_vars(&self, __on_var: &mut impl FnMut(&moniker::Var<String>)) {
            match *self {
                Proc::PZero => {}
                Proc::PInput(ref __binding_0, ref __binding_1) => {
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_1, __on_var);
                    }
                }
                Proc::POutput(ref __binding_0, ref __binding_1) => {
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_1, __on_var);
                    }
                }
                Proc::PPar(ref __binding_0, ref __binding_1) => {
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_1, __on_var);
                    }
                }
                Proc::PDrop(ref __binding_0) => {
                    moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                }
            }
        }
        fn visit_mut_vars(
            &mut self,
            __on_var: &mut impl FnMut(&mut moniker::Var<String>),
        ) {
            match *self {
                Proc::PZero => {}
                Proc::PInput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_1, __on_var);
                    }
                }
                Proc::POutput(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_1, __on_var);
                    }
                }
                Proc::PPar(ref mut __binding_0, ref mut __binding_1) => {
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_1, __on_var);
                    }
                }
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<String>::visit_mut_vars(__binding_0, __on_var);
                }
            }
        }
    }
};
pub enum Name {
    NQuote(Box<Proc>),
    NVar(mettail_runtime::Var<String>),
}
#[automatically_derived]
impl ::core::fmt::Debug for Name {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Name::NQuote(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NQuote", &__self_0)
            }
            Name::NVar(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NVar", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Name {
    #[inline]
    fn clone(&self) -> Name {
        match self {
            Name::NQuote(__self_0) => Name::NQuote(::core::clone::Clone::clone(__self_0)),
            Name::NVar(__self_0) => Name::NVar(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Name {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Name {
    #[inline]
    fn eq(&self, other: &Name) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Name::NQuote(__self_0), Name::NQuote(__arg1_0)) => __self_0 == __arg1_0,
                (Name::NVar(__self_0), Name::NVar(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Name {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<mettail_runtime::Var<String>>;
    }
}
#[allow(non_upper_case_globals)]
const _DERIVE_moniker_BoundTerm_String_FOR_Name: () = {
    extern crate moniker;
    impl moniker::BoundTerm<String> for Name {
        fn term_eq(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    &Name::NQuote(ref __binding_lhs_0),
                    &Name::NQuote(ref __binding_rhs_0),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                }
                (&Name::NVar(ref __binding_lhs_0), &Name::NVar(ref __binding_rhs_0)) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
                }
                (_, _) => false,
            }
        }
        fn close_term(
            &mut self,
            __state: moniker::ScopeState,
            __on_free: &impl moniker::OnFreeFn<String>,
        ) {
            match *self {
                Name::NQuote(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::close_term(__binding_0, __state, __on_free);
                }
                Name::NVar(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::close_term(__binding_0, __state, __on_free);
                }
            }
        }
        fn open_term(
            &mut self,
            __state: moniker::ScopeState,
            __on_bound: &impl moniker::OnBoundFn<String>,
        ) {
            match *self {
                Name::NQuote(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::open_term(__binding_0, __state, __on_bound);
                }
                Name::NVar(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::open_term(__binding_0, __state, __on_bound);
                }
            }
        }
        fn visit_vars(&self, __on_var: &mut impl FnMut(&moniker::Var<String>)) {
            match *self {
                Name::NQuote(ref __binding_0) => {
                    moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                }
                Name::NVar(ref __binding_0) => {
                    moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                }
            }
        }
        fn visit_mut_vars(
            &mut self,
            __on_var: &mut impl FnMut(&mut moniker::Var<String>),
        ) {
            match *self {
                Name::NQuote(ref mut __binding_0) => {
                    moniker::BoundTerm::<String>::visit_mut_vars(__binding_0, __on_var);
                }
                Name::NVar(ref mut __binding_0) => {
                    moniker::BoundTerm::<String>::visit_mut_vars(__binding_0, __on_var);
                }
            }
        }
    }
};
impl Proc {
    /// Substitute `replacement` for free occurrences of `var` in this term
    ///
    /// This performs capture-avoiding substitution using moniker's BoundTerm trait.
    pub fn substitute(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Self,
    ) -> Self {
        match self {
            Proc::PZero => self.clone(),
            Proc::PInput(field_0, scope) => self.clone(),
            Proc::POutput(field_0, field_1) => {
                Proc::POutput(
                    Box::new((**field_0).substitute_proc(var, replacement)),
                    Box::new((**field_1).substitute(var, replacement)),
                )
            }
            Proc::PPar(field_0, field_1) => {
                Proc::PPar(
                    Box::new((**field_0).substitute(var, replacement)),
                    Box::new((**field_1).substitute(var, replacement)),
                )
            }
            Proc::PDrop(field_0) => {
                Proc::PDrop(Box::new((**field_0).substitute_proc(var, replacement)))
            }
        }
    }
    /// Alias for substitute(), provided for uniform cross-category substitution
    pub fn substitute_proc(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Self,
    ) -> Self {
        self.substitute(var, replacement)
    }
    /// Substitute `replacement` (of type #binder_cat) for free occurrences of `var` in this term
    ///
    /// This is used for cross-category substitution where a binder binds variables
    /// of a different category than the term itself.
    pub fn substitute_name(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Name,
    ) -> Self {
        match self {
            Proc::PZero => self.clone(),
            Proc::PInput(field_0, scope) => {
                let (binder, body) = scope.clone().unbind();
                if binder.0 == *var {
                    self.clone()
                } else {
                    let subst_body = body.substitute_name(var, replacement);
                    let new_scope = mettail_runtime::Scope::new(
                        binder,
                        Box::new(subst_body),
                    );
                    Proc::PInput(
                        Box::new((**field_0).substitute(var, replacement)),
                        new_scope.clone(),
                    )
                }
            }
            Proc::POutput(field_0, field_1) => {
                Proc::POutput(
                    Box::new((**field_0).substitute(var, replacement)),
                    Box::new((**field_1).substitute_name(var, replacement)),
                )
            }
            Proc::PPar(field_0, field_1) => {
                Proc::PPar(
                    Box::new((**field_0).substitute_name(var, replacement)),
                    Box::new((**field_1).substitute_name(var, replacement)),
                )
            }
            Proc::PDrop(field_0) => {
                Proc::PDrop(Box::new((**field_0).substitute(var, replacement)))
            }
        }
    }
}
impl Name {
    /// Substitute `replacement` for free occurrences of `var` in this term
    ///
    /// This performs capture-avoiding substitution using moniker's BoundTerm trait.
    pub fn substitute(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Self,
    ) -> Self {
        match self {
            Name::NQuote(field_0) => {
                Name::NQuote(Box::new((**field_0).substitute_name(var, replacement)))
            }
            Name::NVar(mettail_runtime::Var::Free(v)) if v == var => replacement.clone(),
            Name::NVar(_) => self.clone(),
        }
    }
    /// Alias for substitute(), provided for uniform cross-category substitution
    pub fn substitute_name(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Self,
    ) -> Self {
        self.substitute(var, replacement)
    }
    /// Substitute `replacement` (of type #binder_cat) for free occurrences of `var` in this term
    ///
    /// This is used for cross-category substitution where a binder binds variables
    /// of a different category than the term itself.
    pub fn substitute_proc(
        &self,
        var: &mettail_runtime::FreeVar<String>,
        replacement: &Proc,
    ) -> Self {
        match self {
            Name::NQuote(field_0) => {
                Name::NQuote(Box::new((**field_0).substitute(var, replacement)))
            }
            Name::NVar(_) => self.clone(),
        }
    }
}
impl std::fmt::Display for Proc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Proc::PZero => f.write_fmt(format_args!("0")),
            Proc::PInput(f0, scope) => {
                let (binder, body) = scope.clone().unbind();
                let binder_name = binder
                    .0
                    .pretty_name
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("_");
                f.write_fmt(format_args!("for({0} {1}){{{2}}}", f0, binder_name, body))
            }
            Proc::POutput(f0, f3) => f.write_fmt(format_args!("{0}!({1})", f0, f3)),
            Proc::PPar(f0, f2) => f.write_fmt(format_args!("{0}|{1}", f0, f2)),
            Proc::PDrop(f1) => f.write_fmt(format_args!("*{0}", f1)),
        }
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Name::NQuote(f2) => f.write_fmt(format_args!("@({0})", f2)),
            Name::NVar(f0) => {
                f.write_fmt(
                    format_args!(
                        "{0}",
                        match f0 {
                            mettail_runtime::Var::Free(fv) => {
                                fv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_")
                            }
                            mettail_runtime::Var::Bound(_) => "<bound>",
                        },
                    ),
                )
            }
        }
    }
}
#[rustfmt::skip]
#[allow(clippy::extra_unused_lifetimes)]
#[allow(clippy::needless_lifetimes)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::just_underscores_and_digits)]
pub mod rhocalc {
    use mettail_runtime::{Var, FreeVar, Binder, Scope};
    use super::{Proc, Name};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    #[rustfmt::skip]
    #[allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens,
        clippy::needless_lifetimes,
        clippy::type_complexity,
        clippy::needless_return,
        clippy::too_many_arguments,
        clippy::never_loop,
        clippy::match_single_binding,
        clippy::needless_raw_string_hashes
    )]
    mod __parse__Name {
        use mettail_runtime::{Var, FreeVar, Binder, Scope};
        use super::super::{Proc, Name};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(String),
            Variant2(Name),
            Variant3(Proc),
        }
        const __ACTION: &[i8] = &[
            0,
            0,
            0,
            0,
            0,
            12,
            0,
            0,
            0,
            0,
            13,
            0,
            3,
            0,
            4,
            18,
            12,
            19,
            0,
            0,
            0,
            13,
            0,
            3,
            0,
            4,
            18,
            12,
            19,
            0,
            0,
            0,
            13,
            0,
            0,
            0,
            0,
            0,
            12,
            0,
            0,
            0,
            0,
            13,
            0,
            3,
            0,
            4,
            18,
            12,
            19,
            0,
            0,
            0,
            13,
            0,
            0,
            0,
            0,
            0,
            12,
            0,
            0,
            0,
            0,
            13,
            0,
            3,
            0,
            4,
            18,
            12,
            19,
            0,
            0,
            0,
            13,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            13,
            0,
            3,
            0,
            4,
            18,
            12,
            19,
            0,
            0,
            0,
            13,
            -3,
            0,
            -3,
            0,
            0,
            0,
            0,
            0,
            -3,
            -3,
            -3,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -1,
            0,
            -1,
            0,
            0,
            0,
            0,
            0,
            -1,
            -1,
            -1,
            20,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            21,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -11,
            0,
            0,
            0,
            0,
            0,
            -11,
            -11,
            0,
            0,
            0,
            -4,
            0,
            0,
            0,
            0,
            0,
            5,
            -4,
            0,
            0,
            0,
            -6,
            0,
            0,
            0,
            0,
            0,
            -6,
            -6,
            0,
            0,
            6,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            7,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -2,
            0,
            -2,
            0,
            0,
            0,
            0,
            0,
            -2,
            -2,
            -2,
            0,
            0,
            25,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -9,
            0,
            0,
            0,
            0,
            0,
            -9,
            -9,
            0,
            0,
            0,
            -10,
            0,
            0,
            0,
            0,
            0,
            -10,
            -10,
            0,
            0,
            0,
            -5,
            0,
            0,
            0,
            0,
            0,
            -5,
            -5,
            0,
            0,
            0,
            28,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            29,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -8,
            0,
            0,
            0,
            0,
            0,
            -8,
            -8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            9,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            31,
            0,
            0,
            0,
            -7,
            0,
            0,
            0,
            0,
            0,
            -7,
            -7,
            0,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 11 + integer]
        }
        const __EOF_ACTION: &[i8] = &[
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -3,
            -12,
            0,
            -1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                0 => {
                    match state {
                        7 => 26,
                        _ => 9,
                    }
                }
                1 => {
                    match state {
                        5 => 7,
                        0 => 10,
                        3 => 22,
                        _ => 13,
                    }
                }
                2 => {
                    match state {
                        2 => 21,
                        6 => 25,
                        8 => 29,
                        _ => 14,
                    }
                }
                3 => {
                    match state {
                        4 => 23,
                        _ => 15,
                    }
                }
                4 => 16,
                _ => 0,
            }
        }
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""0""###,
            r###""@""###,
            r###""for""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
        ];
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        fn __expected_tokens_from_states<'input>(
            __states: &[i8],
            _: core::marker::PhantomData<(&'input ())>,
        ) -> alloc::vec::Vec<alloc::string::String> {
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    if __accepts(
                        None,
                        __states,
                        Some(index),
                        core::marker::PhantomData::<(&())>,
                    ) {
                        Some(alloc::string::ToString::to_string(terminal))
                    } else {
                        None
                    }
                })
                .collect()
        }
        struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = Name;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 11 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(
                &self,
                token_index: usize,
                token: Self::Token,
            ) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(
                &self,
                state: i8,
            ) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            fn expected_tokens_from_states(
                &self,
                states: &[i8],
            ) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("error recovery not enabled for this grammar"),
                    );
                }
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(
                &self,
                action: i8,
            ) -> __state_machine::SimulatedReduce<Self> {
                __simulate_reduce(action, core::marker::PhantomData::<(&())>)
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(4, _) if true => Some(3),
                Token(5, _) if true => Some(4),
                Token(6, _) if true => Some(5),
                Token(7, _) if true => Some(6),
                Token(8, _) if true => Some(7),
                Token(9, _) if true => Some(8),
                Token(10, _) if true => Some(9),
                Token(0, _) if true => Some(10),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            #[allow(clippy::manual_range_patterns)]
            match __token_index {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => {
                    match __token {
                        Token(1, __tok0)
                        | Token(2, __tok0)
                        | Token(3, __tok0)
                        | Token(4, __tok0)
                        | Token(5, __tok0)
                        | Token(6, __tok0)
                        | Token(7, __tok0)
                        | Token(8, __tok0)
                        | Token(9, __tok0)
                        | Token(10, __tok0)
                        | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn __simulate_reduce<'input>(
            __reduce_index: i8,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __state_machine::SimulatedReduce<__StateMachine<'input>> {
            match __reduce_index {
                0 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 0,
                    }
                }
                1 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 4,
                        nonterminal_produced: 1,
                    }
                }
                2 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 1,
                    }
                }
                3 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 2,
                    }
                }
                4 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 3,
                        nonterminal_produced: 3,
                    }
                }
                5 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 3,
                    }
                }
                6 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 8,
                        nonterminal_produced: 3,
                    }
                }
                7 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 5,
                        nonterminal_produced: 3,
                    }
                }
                8 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 2,
                        nonterminal_produced: 3,
                    }
                }
                9 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 3,
                        nonterminal_produced: 4,
                    }
                }
                10 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 4,
                    }
                }
                11 => __state_machine::SimulatedReduce::Accept,
                12 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 6,
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid reduction index {0}", __reduce_index),
                    );
                }
            }
        }
        pub struct NameParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl Default for NameParser {
            fn default() -> Self {
                Self::new()
            }
        }
        impl NameParser {
            pub fn new() -> NameParser {
                let __builder = super::__intern_token::new_builder();
                NameParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<
                Name,
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            > {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        fn __accepts<'input>(
            __error_state: Option<i8>,
            __states: &[i8],
            __opt_integer: Option<usize>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> bool {
            let mut __states = __states.to_vec();
            __states.extend(__error_state);
            loop {
                let mut __states_len = __states.len();
                let __top = __states[__states_len - 1];
                let __action = match __opt_integer {
                    None => __EOF_ACTION[__top as usize],
                    Some(__integer) => __action(__top, __integer),
                };
                if __action == 0 {
                    return false;
                }
                if __action > 0 {
                    return true;
                }
                let (__to_pop, __nt) = match __simulate_reduce(
                    -(__action + 1),
                    core::marker::PhantomData::<(&())>,
                ) {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop,
                        nonterminal_produced,
                    } => (states_to_pop, nonterminal_produced),
                    __state_machine::SimulatedReduce::Accept => return true,
                };
                __states_len -= __to_pop;
                __states.truncate(__states_len);
                let __top = __states[__states_len - 1];
                let __next_state = __goto(__top, __nt);
                __states.push(__next_state);
            }
        }
        fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<
            Result<Name, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
        > {
            let (__pop_states, __nonterminal) = match __action {
                0 => {
                    __reduce0(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                1 => {
                    __reduce1(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                2 => {
                    __reduce2(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                3 => {
                    __reduce3(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                4 => {
                    __reduce4(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                5 => {
                    __reduce5(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                6 => {
                    __reduce6(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                7 => {
                    __reduce7(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                8 => {
                    __reduce8(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                9 => {
                    __reduce9(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                10 => {
                    __reduce10(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                11 => {
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0;
                    let __end = __sym0.2;
                    let __nt = super::__action1(input, __sym0);
                    return Some(Ok(__nt));
                }
                12 => {
                    __reduce12(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid action code {0}", __action),
                    );
                }
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            {
                ::core::panicking::panic_fmt(format_args!("symbol type mismatch"));
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Name, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Proc, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action2(input, __sym0);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (1, 0)
        }
        fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 4) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
            }
            let __sym3 = __pop_Variant0(__symbols);
            let __sym2 = __pop_Variant3(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym3.2;
            let __nt = super::__action11(input, __sym0, __sym1, __sym2, __sym3);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (4, 1)
        }
        fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action12(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action3(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 2)
        }
        fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            }
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant3(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym2.2;
            let __nt = super::__action6(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action7(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 3)
        }
        fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 8) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 8")
            }
            let __sym7 = __pop_Variant0(__symbols);
            let __sym6 = __pop_Variant3(__symbols);
            let __sym5 = __pop_Variant0(__symbols);
            let __sym4 = __pop_Variant0(__symbols);
            let __sym3 = __pop_Variant1(__symbols);
            let __sym2 = __pop_Variant2(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym7.2;
            let __nt = super::__action8(
                input,
                __sym0,
                __sym1,
                __sym2,
                __sym3,
                __sym4,
                __sym5,
                __sym6,
                __sym7,
            );
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (8, 3)
        }
        fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 5) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
            }
            let __sym4 = __pop_Variant0(__symbols);
            let __sym3 = __pop_Variant3(__symbols);
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0;
            let __end = __sym4.2;
            let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3, __sym4);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (5, 3)
        }
        fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            }
            let __sym1 = __pop_Variant2(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym1.2;
            let __nt = super::__action10(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (2, 3)
        }
        fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            }
            let __sym2 = __pop_Variant3(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym2.2;
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 4)
        }
        fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 4)
        }
        fn __reduce12<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action0(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 6)
        }
    }
    #[allow(unused_imports)]
    pub use self::__parse__Name::NameParser;
    #[rustfmt::skip]
    #[allow(
        non_snake_case,
        non_camel_case_types,
        unused_mut,
        unused_variables,
        unused_imports,
        unused_parens,
        clippy::needless_lifetimes,
        clippy::type_complexity,
        clippy::needless_return,
        clippy::too_many_arguments,
        clippy::never_loop,
        clippy::match_single_binding,
        clippy::needless_raw_string_hashes
    )]
    mod __parse__Proc {
        use mettail_runtime::{Var, FreeVar, Binder, Scope};
        use super::super::{Proc, Name};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        use self::__lalrpop_util::lexer::Token;
        #[allow(dead_code)]
        pub(crate) enum __Symbol<'input> {
            Variant0(&'input str),
            Variant1(String),
            Variant2(Name),
            Variant3(Proc),
        }
        const __ACTION: &[i8] = &[
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            0,
            0,
            0,
            0,
            0,
            16,
            0,
            0,
            0,
            0,
            18,
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            0,
            0,
            0,
            0,
            0,
            16,
            0,
            0,
            0,
            0,
            18,
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            18,
            0,
            2,
            0,
            3,
            15,
            16,
            17,
            0,
            0,
            0,
            18,
            -3,
            0,
            -3,
            0,
            0,
            0,
            0,
            0,
            -3,
            -3,
            -3,
            19,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -11,
            0,
            0,
            0,
            0,
            0,
            -11,
            -11,
            0,
            0,
            0,
            -4,
            0,
            0,
            0,
            0,
            0,
            4,
            -4,
            0,
            0,
            0,
            -6,
            0,
            0,
            0,
            0,
            0,
            -6,
            -6,
            0,
            0,
            5,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            6,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -1,
            0,
            -1,
            0,
            0,
            0,
            0,
            0,
            -1,
            -1,
            -1,
            0,
            7,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            23,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -9,
            0,
            0,
            0,
            0,
            0,
            -9,
            -9,
            0,
            0,
            0,
            -10,
            0,
            0,
            0,
            0,
            0,
            -10,
            -10,
            0,
            0,
            0,
            -5,
            0,
            0,
            0,
            0,
            0,
            -5,
            -5,
            0,
            0,
            0,
            26,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            28,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -2,
            0,
            -2,
            0,
            0,
            0,
            0,
            0,
            -2,
            -2,
            -2,
            0,
            0,
            29,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -8,
            0,
            0,
            0,
            0,
            0,
            -8,
            -8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            9,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            31,
            0,
            0,
            0,
            -7,
            0,
            0,
            0,
            0,
            0,
            -7,
            -7,
            0,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 11 + integer]
        }
        const __EOF_ACTION: &[i8] = &[
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            -3,
            0,
            -13,
            -11,
            -4,
            -6,
            0,
            0,
            -1,
            0,
            0,
            -9,
            -10,
            -5,
            0,
            0,
            -2,
            0,
            -8,
            0,
            0,
            -7,
        ];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                0 => {
                    match state {
                        7 => 26,
                        _ => 9,
                    }
                }
                1 => {
                    match state {
                        5 => 7,
                        2 => 20,
                        _ => 10,
                    }
                }
                2 => {
                    match state {
                        1 => 19,
                        4 => 23,
                        6 => 24,
                        8 => 29,
                        _ => 11,
                    }
                }
                3 => {
                    match state {
                        3 => 21,
                        _ => 12,
                    }
                }
                4 => 13,
                _ => 0,
            }
        }
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""0""###,
            r###""@""###,
            r###""for""###,
            r###""{""###,
            r###""|""###,
            r###""}""###,
            r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
        ];
        fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    let next_state = __action(__state, index);
                    if next_state == 0 {
                        None
                    } else {
                        Some(alloc::string::ToString::to_string(terminal))
                    }
                })
                .collect()
        }
        fn __expected_tokens_from_states<'input>(
            __states: &[i8],
            _: core::marker::PhantomData<(&'input ())>,
        ) -> alloc::vec::Vec<alloc::string::String> {
            __TERMINAL
                .iter()
                .enumerate()
                .filter_map(|(index, terminal)| {
                    if __accepts(
                        None,
                        __states,
                        Some(index),
                        core::marker::PhantomData::<(&())>,
                    ) {
                        Some(alloc::string::ToString::to_string(terminal))
                    } else {
                        None
                    }
                })
                .collect()
        }
        struct __StateMachine<'input> {
            input: &'input str,
            __phantom: core::marker::PhantomData<(&'input ())>,
        }
        impl<'input> __state_machine::ParserDefinition for __StateMachine<'input> {
            type Location = usize;
            type Error = &'static str;
            type Token = Token<'input>;
            type TokenIndex = usize;
            type Symbol = __Symbol<'input>;
            type Success = Proc;
            type StateIndex = i8;
            type Action = i8;
            type ReduceIndex = i8;
            type NonterminalIndex = usize;
            #[inline]
            fn start_location(&self) -> Self::Location {
                Default::default()
            }
            #[inline]
            fn start_state(&self) -> Self::StateIndex {
                0
            }
            #[inline]
            fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
                __token_to_integer(token, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn action(&self, state: i8, integer: usize) -> i8 {
                __action(state, integer)
            }
            #[inline]
            fn error_action(&self, state: i8) -> i8 {
                __action(state, 11 - 1)
            }
            #[inline]
            fn eof_action(&self, state: i8) -> i8 {
                __EOF_ACTION[state as usize]
            }
            #[inline]
            fn goto(&self, state: i8, nt: usize) -> i8 {
                __goto(state, nt)
            }
            fn token_to_symbol(
                &self,
                token_index: usize,
                token: Self::Token,
            ) -> Self::Symbol {
                __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
            }
            fn expected_tokens(
                &self,
                state: i8,
            ) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens(state)
            }
            fn expected_tokens_from_states(
                &self,
                states: &[i8],
            ) -> alloc::vec::Vec<alloc::string::String> {
                __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
            }
            #[inline]
            fn uses_error_recovery(&self) -> bool {
                false
            }
            #[inline]
            fn error_recovery_symbol(
                &self,
                recovery: __state_machine::ErrorRecovery<Self>,
            ) -> Self::Symbol {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("error recovery not enabled for this grammar"),
                    );
                }
            }
            fn reduce(
                &mut self,
                action: i8,
                start_location: Option<&Self::Location>,
                states: &mut alloc::vec::Vec<i8>,
                symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
            ) -> Option<__state_machine::ParseResult<Self>> {
                __reduce(
                    self.input,
                    action,
                    start_location,
                    states,
                    symbols,
                    core::marker::PhantomData::<(&())>,
                )
            }
            fn simulate_reduce(
                &self,
                action: i8,
            ) -> __state_machine::SimulatedReduce<Self> {
                __simulate_reduce(action, core::marker::PhantomData::<(&())>)
            }
        }
        fn __token_to_integer<'input>(
            __token: &Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<usize> {
            match *__token {
                Token(1, _) if true => Some(0),
                Token(2, _) if true => Some(1),
                Token(3, _) if true => Some(2),
                Token(4, _) if true => Some(3),
                Token(5, _) if true => Some(4),
                Token(6, _) if true => Some(5),
                Token(7, _) if true => Some(6),
                Token(8, _) if true => Some(7),
                Token(9, _) if true => Some(8),
                Token(10, _) if true => Some(9),
                Token(0, _) if true => Some(10),
                _ => None,
            }
        }
        fn __token_to_symbol<'input>(
            __token_index: usize,
            __token: Token<'input>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __Symbol<'input> {
            #[allow(clippy::manual_range_patterns)]
            match __token_index {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => {
                    match __token {
                        Token(1, __tok0)
                        | Token(2, __tok0)
                        | Token(3, __tok0)
                        | Token(4, __tok0)
                        | Token(5, __tok0)
                        | Token(6, __tok0)
                        | Token(7, __tok0)
                        | Token(8, __tok0)
                        | Token(9, __tok0)
                        | Token(10, __tok0)
                        | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn __simulate_reduce<'input>(
            __reduce_index: i8,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> __state_machine::SimulatedReduce<__StateMachine<'input>> {
            match __reduce_index {
                0 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 0,
                    }
                }
                1 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 4,
                        nonterminal_produced: 1,
                    }
                }
                2 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 1,
                    }
                }
                3 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 2,
                    }
                }
                4 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 3,
                        nonterminal_produced: 3,
                    }
                }
                5 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 3,
                    }
                }
                6 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 8,
                        nonterminal_produced: 3,
                    }
                }
                7 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 5,
                        nonterminal_produced: 3,
                    }
                }
                8 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 2,
                        nonterminal_produced: 3,
                    }
                }
                9 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 3,
                        nonterminal_produced: 4,
                    }
                }
                10 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 4,
                    }
                }
                11 => {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop: 1,
                        nonterminal_produced: 5,
                    }
                }
                12 => __state_machine::SimulatedReduce::Accept,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid reduction index {0}", __reduce_index),
                    );
                }
            }
        }
        pub struct ProcParser {
            builder: __lalrpop_util::lexer::MatcherBuilder,
            _priv: (),
        }
        impl Default for ProcParser {
            fn default() -> Self {
                Self::new()
            }
        }
        impl ProcParser {
            pub fn new() -> ProcParser {
                let __builder = super::__intern_token::new_builder();
                ProcParser {
                    builder: __builder,
                    _priv: (),
                }
            }
            #[allow(dead_code)]
            pub fn parse<'input>(
                &self,
                input: &'input str,
            ) -> Result<
                Proc,
                __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
            > {
                let mut __tokens = self.builder.matcher(input);
                __state_machine::Parser::drive(
                    __StateMachine {
                        input,
                        __phantom: core::marker::PhantomData::<(&())>,
                    },
                    __tokens,
                )
            }
        }
        fn __accepts<'input>(
            __error_state: Option<i8>,
            __states: &[i8],
            __opt_integer: Option<usize>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> bool {
            let mut __states = __states.to_vec();
            __states.extend(__error_state);
            loop {
                let mut __states_len = __states.len();
                let __top = __states[__states_len - 1];
                let __action = match __opt_integer {
                    None => __EOF_ACTION[__top as usize],
                    Some(__integer) => __action(__top, __integer),
                };
                if __action == 0 {
                    return false;
                }
                if __action > 0 {
                    return true;
                }
                let (__to_pop, __nt) = match __simulate_reduce(
                    -(__action + 1),
                    core::marker::PhantomData::<(&())>,
                ) {
                    __state_machine::SimulatedReduce::Reduce {
                        states_to_pop,
                        nonterminal_produced,
                    } => (states_to_pop, nonterminal_produced),
                    __state_machine::SimulatedReduce::Accept => return true,
                };
                __states_len -= __to_pop;
                __states.truncate(__states_len);
                let __top = __states[__states_len - 1];
                let __next_state = __goto(__top, __nt);
                __states.push(__next_state);
            }
        }
        fn __reduce<'input>(
            input: &'input str,
            __action: i8,
            __lookahead_start: Option<&usize>,
            __states: &mut alloc::vec::Vec<i8>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> Option<
            Result<Proc, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>,
        > {
            let (__pop_states, __nonterminal) = match __action {
                0 => {
                    __reduce0(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                1 => {
                    __reduce1(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                2 => {
                    __reduce2(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                3 => {
                    __reduce3(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                4 => {
                    __reduce4(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                5 => {
                    __reduce5(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                6 => {
                    __reduce6(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                7 => {
                    __reduce7(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                8 => {
                    __reduce8(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                9 => {
                    __reduce9(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                10 => {
                    __reduce10(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                11 => {
                    __reduce11(
                        input,
                        __lookahead_start,
                        __symbols,
                        core::marker::PhantomData::<(&())>,
                    )
                }
                12 => {
                    let __sym0 = __pop_Variant3(__symbols);
                    let __start = __sym0.0;
                    let __end = __sym0.2;
                    let __nt = super::__action0(input, __sym0);
                    return Some(Ok(__nt));
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid action code {0}", __action),
                    );
                }
            };
            let __states_len = __states.len();
            __states.truncate(__states_len - __pop_states);
            let __state = *__states.last().unwrap();
            let __next_state = __goto(__state, __nonterminal);
            __states.push(__next_state);
            None
        }
        #[inline(never)]
        fn __symbol_type_mismatch() -> ! {
            {
                ::core::panicking::panic_fmt(format_args!("symbol type mismatch"));
            }
        }
        fn __pop_Variant2<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Name, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant3<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, Proc, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant1<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, String, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __pop_Variant0<'input>(
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
        ) -> (usize, &'input str, usize) {
            match __symbols.pop() {
                Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
                _ => __symbol_type_mismatch(),
            }
        }
        fn __reduce0<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action2(input, __sym0);
            __symbols.push((__start, __Symbol::Variant1(__nt), __end));
            (1, 0)
        }
        fn __reduce1<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 4) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
            }
            let __sym3 = __pop_Variant0(__symbols);
            let __sym2 = __pop_Variant3(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym3.2;
            let __nt = super::__action11(input, __sym0, __sym1, __sym2, __sym3);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (4, 1)
        }
        fn __reduce2<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant1(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action12(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 1)
        }
        fn __reduce3<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action3(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 2)
        }
        fn __reduce4<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            }
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant3(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym2.2;
            let __nt = super::__action6(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 3)
        }
        fn __reduce5<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action7(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 3)
        }
        fn __reduce6<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 8) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 8")
            }
            let __sym7 = __pop_Variant0(__symbols);
            let __sym6 = __pop_Variant3(__symbols);
            let __sym5 = __pop_Variant0(__symbols);
            let __sym4 = __pop_Variant0(__symbols);
            let __sym3 = __pop_Variant1(__symbols);
            let __sym2 = __pop_Variant2(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym7.2;
            let __nt = super::__action8(
                input,
                __sym0,
                __sym1,
                __sym2,
                __sym3,
                __sym4,
                __sym5,
                __sym6,
                __sym7,
            );
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (8, 3)
        }
        fn __reduce7<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 5) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
            }
            let __sym4 = __pop_Variant0(__symbols);
            let __sym3 = __pop_Variant3(__symbols);
            let __sym2 = __pop_Variant0(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0;
            let __end = __sym4.2;
            let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3, __sym4);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (5, 3)
        }
        fn __reduce8<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            }
            let __sym1 = __pop_Variant2(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym1.2;
            let __nt = super::__action10(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (2, 3)
        }
        fn __reduce9<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            if !(__symbols.len() >= 3) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
            }
            let __sym2 = __pop_Variant3(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym2.2;
            let __nt = super::__action4(input, __sym0, __sym1, __sym2);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (3, 4)
        }
        fn __reduce10<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant3(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action5(input, __sym0);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (1, 4)
        }
        fn __reduce11<'input>(
            input: &'input str,
            __lookahead_start: Option<&usize>,
            __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            _: core::marker::PhantomData<(&'input ())>,
        ) -> (usize, usize) {
            let __sym0 = __pop_Variant2(__symbols);
            let __start = __sym0.0;
            let __end = __sym0.2;
            let __nt = super::__action1(input, __sym0);
            __symbols.push((__start, __Symbol::Variant2(__nt), __end));
            (1, 5)
        }
    }
    #[allow(unused_imports)]
    pub use self::__parse__Proc::ProcParser;
    #[rustfmt::skip]
    mod __intern_token {
        #![allow(unused_imports)]
        use mettail_runtime::{Var, FreeVar, Binder, Scope};
        use super::super::{Proc, Name};
        #[allow(unused_extern_crates)]
        extern crate lalrpop_util as __lalrpop_util;
        #[allow(unused_imports)]
        use self::__lalrpop_util::state_machine as __state_machine;
        extern crate core;
        extern crate alloc;
        pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
            let __strs: &[(&str, bool)] = &[
                ("(?:[A-Z_a-z][0-9A-Z_a-z]*)", false),
                ("!", false),
                ("\\(", false),
                ("\\)", false),
                ("\\*", false),
                ("0", false),
                ("@", false),
                ("(?:for)", false),
                ("\\{", false),
                ("\\|", false),
                ("\\}", false),
                (r"\s+", true),
            ];
            __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
        }
    }
    pub(crate) use self::__lalrpop_util::lexer::Token;
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action0<'input>(input: &'input str, (_, __0, _): (usize, Proc, usize)) -> Proc {
        __0
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action1<'input>(input: &'input str, (_, __0, _): (usize, Name, usize)) -> Name {
        __0
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action2<'input>(
        input: &'input str,
        (_, __0, _): (usize, &'input str, usize),
    ) -> String {
        __0.to_string()
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action3<'input>(input: &'input str, (_, __0, _): (usize, Proc, usize)) -> Proc {
        __0
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action4<'input>(
        input: &'input str,
        (_, left, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, right, _): (usize, Proc, usize),
    ) -> Proc {
        Proc::PPar(Box::new(left), Box::new(right))
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action5<'input>(input: &'input str, (_, __0, _): (usize, Proc, usize)) -> Proc {
        __0
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action6<'input>(
        input: &'input str,
        (_, _, _): (usize, &'input str, usize),
        (_, __0, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Proc {
        __0
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action7<'input>(
        input: &'input str,
        (_, __0, _): (usize, &'input str, usize),
    ) -> Proc {
        Proc::PZero
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action8<'input>(
        input: &'input str,
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, f0, _): (usize, Name, usize),
        (_, x_1, _): (usize, String, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, body_2, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Proc {
        {
            let binder = Binder(FreeVar::fresh_named(x_1));
            let scope = Scope::new(binder, Box::new(body_2));
            Proc::PInput(Box::new(f0), scope)
        }
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action9<'input>(
        input: &'input str,
        (_, f0, _): (usize, Name, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, f1, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Proc {
        Proc::POutput(Box::new(f0), Box::new(f1))
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action10<'input>(
        input: &'input str,
        (_, _, _): (usize, &'input str, usize),
        (_, f0, _): (usize, Name, usize),
    ) -> Proc {
        Proc::PDrop(Box::new(f0))
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action11<'input>(
        input: &'input str,
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, f0, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Name {
        Name::NQuote(Box::new(f0))
    }
    #[allow(unused_variables)]
    #[allow(
        clippy::too_many_arguments,
        clippy::needless_lifetimes,
        clippy::just_underscores_and_digits
    )]
    fn __action12<'input>(
        input: &'input str,
        (_, v, _): (usize, String, usize),
    ) -> Name {
        Name::NVar(Var::Free(FreeVar::fresh_named(v)))
    }
    #[allow(clippy::type_complexity, dead_code)]
    pub trait __ToTriple<'input> {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        >;
    }
    impl<'input> __ToTriple<'input> for (usize, Token<'input>, usize) {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        > {
            Ok(value)
        }
    }
    impl<'input> __ToTriple<'input>
    for Result<(usize, Token<'input>, usize), &'static str> {
        fn to_triple(
            value: Self,
        ) -> Result<
            (usize, Token<'input>, usize),
            __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
        > {
            match value {
                Ok(v) => Ok(v),
                Err(error) => {
                    Err(__lalrpop_util::ParseError::User {
                        error,
                    })
                }
            }
        }
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("Rho Calculus Theory Compiled Successfully!\n"));
    };
}
