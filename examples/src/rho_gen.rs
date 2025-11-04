#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use mettail_macros::theory;
use mettail_runtime;
use lalrpop_util::lalrpop_mod;
use ascent::*;
pub enum Proc {
    PZero,
    PDrop(Box<Name>),
    POutput(Box<Name>, Box<Proc>),
    PInput(
        Box<Name>,
        mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>,
    ),
    PPar(Box<Proc>, Box<Proc>),
}
#[automatically_derived]
impl ::core::fmt::Debug for Proc {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Proc::PZero => ::core::fmt::Formatter::write_str(f, "PZero"),
            Proc::PDrop(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PDrop", &__self_0)
            }
            Proc::POutput(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "POutput",
                    __self_0,
                    &__self_1,
                )
            }
            Proc::PInput(__self_0, __self_1) => {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "PInput",
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
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Proc {
    #[inline]
    fn clone(&self) -> Proc {
        match self {
            Proc::PZero => Proc::PZero,
            Proc::PDrop(__self_0) => Proc::PDrop(::core::clone::Clone::clone(__self_0)),
            Proc::POutput(__self_0, __self_1) => {
                Proc::POutput(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                )
            }
            Proc::PInput(__self_0, __self_1) => {
                Proc::PInput(
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
                (Proc::PDrop(__self_0), Proc::PDrop(__arg1_0)) => __self_0 == __arg1_0,
                (
                    Proc::POutput(__self_0, __self_1),
                    Proc::POutput(__arg1_0, __arg1_1),
                ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                (Proc::PInput(__self_0, __self_1), Proc::PInput(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
                (Proc::PPar(__self_0, __self_1), Proc::PPar(__arg1_0, __arg1_1)) => {
                    __self_0 == __arg1_0 && __self_1 == __arg1_1
                }
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
        let _: ::core::cmp::AssertParamIsEq<Box<Name>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Name>>;
        let _: ::core::cmp::AssertParamIsEq<
            mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>,
        >;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
        let _: ::core::cmp::AssertParamIsEq<Box<Proc>>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Proc {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Proc,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Proc::PDrop(__self_0), Proc::PDrop(__arg1_0)) => {
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
            }
            (Proc::POutput(__self_0, __self_1), Proc::POutput(__arg1_0, __arg1_1)) => {
                match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                    }
                    cmp => cmp,
                }
            }
            (Proc::PInput(__self_0, __self_1), Proc::PInput(__arg1_0, __arg1_1)) => {
                match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                    }
                    cmp => cmp,
                }
            }
            (Proc::PPar(__self_0, __self_1), Proc::PPar(__arg1_0, __arg1_1)) => {
                match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                    }
                    cmp => cmp,
                }
            }
            _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Proc {
    #[inline]
    fn cmp(&self, other: &Proc) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
            ::core::cmp::Ordering::Equal => {
                match (self, other) {
                    (Proc::PDrop(__self_0), Proc::PDrop(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (
                        Proc::POutput(__self_0, __self_1),
                        Proc::POutput(__arg1_0, __arg1_1),
                    ) => {
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        }
                    }
                    (
                        Proc::PInput(__self_0, __self_1),
                        Proc::PInput(__arg1_0, __arg1_1),
                    ) => {
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        }
                    }
                    (Proc::PPar(__self_0, __self_1), Proc::PPar(__arg1_0, __arg1_1)) => {
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        }
                    }
                    _ => ::core::cmp::Ordering::Equal,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Proc {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state);
        match self {
            Proc::PDrop(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Proc::POutput(__self_0, __self_1) => {
                ::core::hash::Hash::hash(__self_0, state);
                ::core::hash::Hash::hash(__self_1, state)
            }
            Proc::PInput(__self_0, __self_1) => {
                ::core::hash::Hash::hash(__self_0, state);
                ::core::hash::Hash::hash(__self_1, state)
            }
            Proc::PPar(__self_0, __self_1) => {
                ::core::hash::Hash::hash(__self_0, state);
                ::core::hash::Hash::hash(__self_1, state)
            }
            _ => {}
        }
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
                    &Proc::PDrop(ref __binding_lhs_0),
                    &Proc::PDrop(ref __binding_rhs_0),
                ) => {
                    true
                        && moniker::BoundTerm::<
                            String,
                        >::term_eq(__binding_lhs_0, __binding_rhs_0)
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
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::close_term(__binding_0, __state, __on_free);
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
            }
        }
        fn open_term(
            &mut self,
            __state: moniker::ScopeState,
            __on_bound: &impl moniker::OnBoundFn<String>,
        ) {
            match *self {
                Proc::PZero => {}
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<
                        String,
                    >::open_term(__binding_0, __state, __on_bound);
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
            }
        }
        fn visit_vars(&self, __on_var: &mut impl FnMut(&moniker::Var<String>)) {
            match *self {
                Proc::PZero => {}
                Proc::PDrop(ref __binding_0) => {
                    moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                }
                Proc::POutput(ref __binding_0, ref __binding_1) => {
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                    }
                    {
                        moniker::BoundTerm::<String>::visit_vars(__binding_1, __on_var);
                    }
                }
                Proc::PInput(ref __binding_0, ref __binding_1) => {
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
            }
        }
        fn visit_mut_vars(
            &mut self,
            __on_var: &mut impl FnMut(&mut moniker::Var<String>),
        ) {
            match *self {
                Proc::PZero => {}
                Proc::PDrop(ref mut __binding_0) => {
                    moniker::BoundTerm::<String>::visit_mut_vars(__binding_0, __on_var);
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
            }
        }
    }
};
pub enum Name {
    NQuote(Box<Proc>),
    NVar(mettail_runtime::OrdVar),
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
        let _: ::core::cmp::AssertParamIsEq<mettail_runtime::OrdVar>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Name {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Name,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Name::NQuote(__self_0), Name::NQuote(__arg1_0)) => {
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
            }
            (Name::NVar(__self_0), Name::NVar(__arg1_0)) => {
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
            }
            _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Name {
    #[inline]
    fn cmp(&self, other: &Name) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
            ::core::cmp::Ordering::Equal => {
                match (self, other) {
                    (Name::NQuote(__self_0), Name::NQuote(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (Name::NVar(__self_0), Name::NVar(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Name {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state);
        match self {
            Name::NQuote(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Name::NVar(__self_0) => ::core::hash::Hash::hash(__self_0, state),
        }
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
            Proc::PDrop(field_0) => {
                Proc::PDrop(Box::new((**field_0).substitute_proc(var, replacement)))
            }
            Proc::POutput(field_0, field_1) => {
                Proc::POutput(
                    Box::new((**field_0).substitute_proc(var, replacement)),
                    Box::new((**field_1).substitute(var, replacement)),
                )
            }
            Proc::PInput(field_0, scope) => self.clone(),
            Proc::PPar(field_0, field_1) => {
                Proc::PPar(
                    Box::new((**field_0).substitute(var, replacement)),
                    Box::new((**field_1).substitute(var, replacement)),
                )
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
            Proc::PDrop(field_0) => {
                Proc::PDrop(Box::new((**field_0).substitute(var, replacement)))
            }
            Proc::POutput(field_0, field_1) => {
                Proc::POutput(
                    Box::new((**field_0).substitute(var, replacement)),
                    Box::new((**field_1).substitute_name(var, replacement)),
                )
            }
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
            Proc::PPar(field_0, field_1) => {
                Proc::PPar(
                    Box::new((**field_0).substitute_name(var, replacement)),
                    Box::new((**field_1).substitute_name(var, replacement)),
                )
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
            Name::NVar(
                mettail_runtime::OrdVar(mettail_runtime::Var::Free(v)),
            ) if v == var => replacement.clone(),
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
            Proc::PDrop(f1) => f.write_fmt(format_args!("*{0}", f1)),
            Proc::POutput(f0, f3) => f.write_fmt(format_args!("{0}!({1})", f0, f3)),
            Proc::PInput(f0, scope) => {
                let (binder, body) = scope.clone().unbind();
                let binder_name = binder
                    .0
                    .pretty_name
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("_");
                f.write_fmt(format_args!("for({0}->{1}){{{2}}}", f0, binder_name, body))
            }
            Proc::PPar(f0, f2) => f.write_fmt(format_args!("{0}|{1}", f0, f2)),
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
                        match &(f0).0 {
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
struct GenerationContext {
    vars: Vec<String>,
    max_depth: usize,
    proc_by_depth: std::collections::HashMap<usize, Vec<Proc>>,
    name_by_depth: std::collections::HashMap<usize, Vec<Name>>,
}
impl GenerationContext {
    fn new(vars: Vec<String>, max_depth: usize) -> Self {
        Self {
            vars,
            max_depth,
            proc_by_depth: std::collections::HashMap::new(),
            name_by_depth: std::collections::HashMap::new(),
        }
    }
    fn generate_all(mut self) -> Self {
        for depth in 0..=self.max_depth {
            self.generate_proc(depth);
            self.generate_name(depth);
        }
        self
    }
    fn generate_proc(&mut self, depth: usize) {
        let mut terms: Vec<Proc> = Vec::new();
        if depth == 0 {
            terms.push(Proc::PZero);
        } else {
            for d1 in 0..depth {
                if let Some(args1) = self.name_by_depth.get(&d1) {
                    for arg1 in args1 {
                        terms.push(Proc::PDrop(Box::new(arg1.clone())));
                    }
                }
            }
            for d1 in 0..depth {
                for d2 in 0..depth {
                    if d1.max(d2) + 1 == depth {
                        if let Some(args1) = self.name_by_depth.get(&d1) {
                            if let Some(args2) = self.proc_by_depth.get(&d2) {
                                for arg1 in args1 {
                                    for arg2 in args2 {
                                        terms
                                            .push(
                                                Proc::POutput(
                                                    Box::new(arg1.clone()),
                                                    Box::new(arg2.clone()),
                                                ),
                                            );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for d1 in 0..depth {
                for d2 in 0..depth {
                    if d1.max(d2) + 1 == depth {
                        if let Some(args1) = self.name_by_depth.get(&d1) {
                            if let Some(bodies) = self.proc_by_depth.get(&d2) {
                                for arg1 in args1 {
                                    for body in bodies {
                                        let binder = mettail_runtime::Binder(
                                            mettail_runtime::get_or_create_var("x"),
                                        );
                                        let scope = mettail_runtime::Scope::new(
                                            binder,
                                            Box::new(body.clone()),
                                        );
                                        terms.push(Proc::PInput(Box::new(arg1.clone()), scope));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for d1 in 0..depth {
                for d2 in 0..depth {
                    if d1.max(d2) + 1 == depth {
                        if let Some(args1) = self.proc_by_depth.get(&d1) {
                            if let Some(args2) = self.proc_by_depth.get(&d2) {
                                for arg1 in args1 {
                                    for arg2 in args2 {
                                        terms
                                            .push(
                                                Proc::PPar(Box::new(arg1.clone()), Box::new(arg2.clone())),
                                            );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        terms.sort();
        terms.dedup();
        self.proc_by_depth.insert(depth, terms);
    }
    fn generate_name(&mut self, depth: usize) {
        let mut terms: Vec<Name> = Vec::new();
        if depth == 0 {
            for var_name in &self.vars {
                terms
                    .push(
                        Name::NVar(
                            mettail_runtime::OrdVar(
                                mettail_runtime::Var::Free(
                                    mettail_runtime::get_or_create_var(var_name),
                                ),
                            ),
                        ),
                    );
            }
        } else {
            for d1 in 0..depth {
                if let Some(args1) = self.proc_by_depth.get(&d1) {
                    for arg1 in args1 {
                        terms.push(Name::NQuote(Box::new(arg1.clone())));
                    }
                }
            }
        }
        terms.sort();
        terms.dedup();
        self.name_by_depth.insert(depth, terms);
    }
}
impl Proc {
    /// Generate all terms up to max_depth
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `max_depth` - Maximum operator nesting level
    ///
    /// # Returns
    /// Sorted, deduplicated vector of terms
    ///
    /// # Warning
    /// Number of terms grows exponentially with depth!
    /// Recommend max_depth <= 3 for most use cases.
    pub fn generate_terms(vars: &[String], max_depth: usize) -> Vec<Proc> {
        let ctx = GenerationContext::new(vars.to_vec(), max_depth);
        let ctx = ctx.generate_all();
        let mut all_terms = Vec::new();
        for depth in 0..=max_depth {
            if let Some(terms) = ctx.proc_by_depth.get(&depth) {
                all_terms.extend(terms.clone());
            }
        }
        all_terms.sort();
        all_terms.dedup();
        all_terms
    }
}
impl Name {
    /// Generate all terms up to max_depth
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `max_depth` - Maximum operator nesting level
    ///
    /// # Returns
    /// Sorted, deduplicated vector of terms
    ///
    /// # Warning
    /// Number of terms grows exponentially with depth!
    /// Recommend max_depth <= 3 for most use cases.
    pub fn generate_terms(vars: &[String], max_depth: usize) -> Vec<Name> {
        let ctx = GenerationContext::new(vars.to_vec(), max_depth);
        let ctx = ctx.generate_all();
        let mut all_terms = Vec::new();
        for depth in 0..=max_depth {
            if let Some(terms) = ctx.name_by_depth.get(&depth) {
                all_terms.extend(terms.clone());
            }
        }
        all_terms.sort();
        all_terms.dedup();
        all_terms
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
            0,
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
            0,
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
            0,
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
            0,
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
            0,
            13,
            0,
            3,
            0,
            4,
            0,
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
            -3,
            0,
            0,
            0,
            0,
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
            0,
            -1,
            0,
            -1,
            0,
            -1,
            0,
            0,
            0,
            0,
            -1,
            -1,
            0,
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
            0,
            -11,
            0,
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
            -2,
            0,
            -2,
            0,
            -2,
            0,
            0,
            0,
            0,
            -2,
            -2,
            0,
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
            0,
            -7,
            0,
            0,
            0,
            0,
            0,
            0,
            -7,
            -7,
            0,
            0,
            0,
            -10,
            0,
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
            0,
            -5,
            -5,
            0,
            0,
            0,
            0,
            0,
            8,
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
            0,
            0,
            0,
            -8,
            0,
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
            30,
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
            0,
            32,
            0,
            0,
            0,
            -9,
            0,
            0,
            0,
            0,
            0,
            0,
            -9,
            -9,
            0,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 12 + integer]
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
            0,
        ];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                0 => {
                    match state {
                        7 => 28,
                        _ => 9,
                    }
                }
                1 => {
                    match state {
                        0 => 10,
                        3 => 22,
                        5 => 25,
                        _ => 13,
                    }
                }
                2 => {
                    match state {
                        2 => 21,
                        6 => 26,
                        8 => 30,
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
            r###""->""###,
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
                __action(state, 12 - 1)
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
                Token(11, _) if true => Some(10),
                Token(0, _) if true => Some(11),
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
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
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
                        | Token(11, __tok0)
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
                        states_to_pop: 2,
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
                        states_to_pop: 9,
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
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            }
            let __sym1 = __pop_Variant2(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym1.2;
            let __nt = super::__action8(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (2, 3)
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
            if !(__symbols.len() >= 9) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 9")
            }
            let __sym8 = __pop_Variant0(__symbols);
            let __sym7 = __pop_Variant3(__symbols);
            let __sym6 = __pop_Variant0(__symbols);
            let __sym5 = __pop_Variant0(__symbols);
            let __sym4 = __pop_Variant1(__symbols);
            let __sym3 = __pop_Variant0(__symbols);
            let __sym2 = __pop_Variant2(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym8.2;
            let __nt = super::__action10(
                input,
                __sym0,
                __sym1,
                __sym2,
                __sym3,
                __sym4,
                __sym5,
                __sym6,
                __sym7,
                __sym8,
            );
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (9, 3)
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
            0,
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
            0,
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
            0,
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
            0,
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
            0,
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
            0,
            18,
            0,
            2,
            0,
            3,
            0,
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
            -3,
            0,
            0,
            0,
            0,
            -3,
            -3,
            0,
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
            0,
            0,
            -11,
            0,
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
            -1,
            0,
            -1,
            0,
            -1,
            0,
            0,
            0,
            0,
            -1,
            -1,
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
            0,
            -7,
            0,
            0,
            0,
            0,
            0,
            0,
            -7,
            -7,
            0,
            0,
            0,
            -10,
            0,
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
            0,
            -5,
            -5,
            0,
            0,
            0,
            27,
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
            8,
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
            0,
            -2,
            0,
            -2,
            0,
            -2,
            0,
            0,
            0,
            0,
            -2,
            -2,
            0,
            0,
            0,
            -8,
            0,
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
            30,
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
            0,
            32,
            0,
            0,
            0,
            -9,
            0,
            0,
            0,
            0,
            0,
            0,
            -9,
            -9,
            0,
        ];
        fn __action(state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 12 + integer]
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
            -7,
            -10,
            -5,
            0,
            0,
            0,
            -2,
            -8,
            0,
            0,
            0,
            -9,
        ];
        fn __goto(state: i8, nt: usize) -> i8 {
            match nt {
                0 => {
                    match state {
                        7 => 28,
                        _ => 9,
                    }
                }
                1 => {
                    match state {
                        2 => 20,
                        5 => 24,
                        _ => 10,
                    }
                }
                2 => {
                    match state {
                        1 => 19,
                        4 => 23,
                        6 => 25,
                        8 => 30,
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
            r###""->""###,
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
                __action(state, 12 - 1)
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
                Token(11, _) if true => Some(10),
                Token(0, _) if true => Some(11),
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
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
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
                        | Token(11, __tok0)
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
                        states_to_pop: 2,
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
                        states_to_pop: 9,
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
            if !(__symbols.len() >= 2) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
            }
            let __sym1 = __pop_Variant2(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym1.2;
            let __nt = super::__action8(input, __sym0, __sym1);
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (2, 3)
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
            if !(__symbols.len() >= 9) {
                ::core::panicking::panic("assertion failed: __symbols.len() >= 9")
            }
            let __sym8 = __pop_Variant0(__symbols);
            let __sym7 = __pop_Variant3(__symbols);
            let __sym6 = __pop_Variant0(__symbols);
            let __sym5 = __pop_Variant0(__symbols);
            let __sym4 = __pop_Variant1(__symbols);
            let __sym3 = __pop_Variant0(__symbols);
            let __sym2 = __pop_Variant2(__symbols);
            let __sym1 = __pop_Variant0(__symbols);
            let __sym0 = __pop_Variant0(__symbols);
            let __start = __sym0.0;
            let __end = __sym8.2;
            let __nt = super::__action10(
                input,
                __sym0,
                __sym1,
                __sym2,
                __sym3,
                __sym4,
                __sym5,
                __sym6,
                __sym7,
                __sym8,
            );
            __symbols.push((__start, __Symbol::Variant3(__nt), __end));
            (9, 3)
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
                ("(?:\\->)", false),
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
        (_, _, _): (usize, &'input str, usize),
        (_, f0, _): (usize, Name, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, x_1, _): (usize, String, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, _, _): (usize, &'input str, usize),
        (_, body_2, _): (usize, Proc, usize),
        (_, _, _): (usize, &'input str, usize),
    ) -> Proc {
        {
            use mettail_runtime::BoundTerm;
            let free_vars = body_2.free_vars();
            let binder = if let Some(fv) = free_vars
                .iter()
                .find(|fv| fv.pretty_name.as_deref() == Some(&x_1))
            {
                Binder((*fv).clone())
            } else {
                Binder(mettail_runtime::get_or_create_var(x_1))
            };
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
        Name::NVar(
            mettail_runtime::OrdVar(Var::Free(mettail_runtime::get_or_create_var(v))),
        )
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
fn is_fresh<T>(binder: &mettail_runtime::Binder<String>, term: &T) -> bool
where
    T: mettail_runtime::BoundTerm<String>,
{
    use mettail_runtime::BoundTerm;
    let mut is_fresh = true;
    term.visit_vars(
        &mut |v| {
            if let mettail_runtime::Var::Free(fv) = v {
                if fv == &binder.0 {
                    is_fresh = false;
                }
            }
        },
    );
    is_fresh
}
pub fn try_rewrite_rule_0(term: &Proc) -> Option<Proc> {
    if let Proc::PPar(field_0, field_1) = term {
        let field_0_inner = &(**field_0);
        if let Proc::PInput(field_0_inner_0, scope_field) = field_0_inner {
            let (binder, body) = scope_field.clone().unbind();
            let field_1_inner = &(**field_1);
            if let Proc::POutput(field_1_inner_0, field_1_inner_1) = field_1_inner {
                if !((**field_0_inner_0).clone() == (**field_1_inner_0).clone()) {
                    return None;
                }
                if !is_fresh(&binder.clone(), &(**field_1_inner_1).clone()) {
                    return None;
                }
                return Some(
                    ((*body).clone())
                        .substitute_name(
                            &(binder.clone()).0,
                            &Name::NQuote(Box::new((**field_1_inner_1).clone())),
                        ),
                );
            }
        }
    }
    None
}
pub use ascent_source_theory_source as theory_source;
fn main() {
    {
        ::std::io::_print(format_args!("=== Rho Calculus Demo ===\n\n"));
    };
    {
        ::std::io::_print(format_args!("--- Term Generation ---\n"));
    };
    {
        ::std::io::_print(
            format_args!("Generating Proc terms up to depth 2 with vars [a, b]...\n\n"),
        );
    };
    let vars = <[_]>::into_vec(
        ::alloc::boxed::box_new(["a".to_string(), "b".to_string()]),
    );
    let terms = Proc::generate_terms(&vars, 2);
    {
        ::std::io::_print(format_args!("Generated {0} terms total\n", terms.len()));
    };
    {
        ::std::io::_print(format_args!("\nFirst 20 terms:\n"));
    };
    for (i, term) in terms.iter().take(20).enumerate() {
        {
            ::std::io::_print(format_args!("  {0}: {1}\n", i + 1, term));
        };
    }
    {
        ::std::io::_print(format_args!("\nGenerating Name terms up to depth 1...\n"));
    };
    let names = Name::generate_terms(&vars, 1);
    {
        ::std::io::_print(format_args!("Generated {0} names:\n", names.len()));
    };
    for (i, name) in names.iter().take(10).enumerate() {
        {
            ::std::io::_print(format_args!("  {0}: {1}\n", i + 1, name));
        };
    }
    {
        ::std::io::_print(format_args!("\n--- Rewrite Engine Demo ---\n"));
    };
    let rdx_str = "for(a->x){*x}|a!(b!(*n))|for(b->y){*y}|b!(0)";
    mettail_runtime::clear_var_cache();
    let parser = rhocalc::ProcParser::new();
    let redex = parser.parse(rdx_str).unwrap();
    let prog = {
        {
            #![allow(
                unused_imports,
                noop_method_call,
                suspicious_double_ref_op,
                clippy::all
            )]
            pub struct AscentProgram {
                /**
logical indices: eq_indices_0; eq_indices_0_1; eq_indices_1; eq_indices_none*/
                pub eq: ::std::vec::Vec<(Proc, Proc)>,
                __eq_ind_common: (),
                eq_indices_0: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                eq_indices_0_1: ascent::internal::RelFullIndexType<(Proc, Proc), ()>,
                eq_indices_1: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                eq_indices_none: ascent::rel::ToRelIndexType<(), (Proc, Proc)>,
                /**
logical indices: full_path_indices_0_1*/
                pub full_path: ::std::vec::Vec<(Proc, Vec<Proc>)>,
                __full_path_ind_common: (),
                full_path_indices_0_1: ascent::internal::RelFullIndexType<
                    (Proc, Vec<Proc>),
                    (),
                >,
                /**
logical indices: path_indices_0; path_indices_0_1; path_indices_none*/
                pub path: ::std::vec::Vec<(Proc, Vec<Proc>)>,
                __path_ind_common: (),
                path_indices_0: ascent::rel::ToRelIndexType<(Proc,), (Vec<Proc>,)>,
                path_indices_0_1: ascent::internal::RelFullIndexType<
                    (Proc, Vec<Proc>),
                    (),
                >,
                path_indices_none: ascent::rel::ToRelIndexType<(), (Proc, Vec<Proc>)>,
                /**
logical indices: path_terminal_indices_0_1; path_terminal_indices_none*/
                pub path_terminal: ::std::vec::Vec<(Proc, Vec<Proc>)>,
                __path_terminal_ind_common: (),
                path_terminal_indices_0_1: ascent::internal::RelFullIndexType<
                    (Proc, Vec<Proc>),
                    (),
                >,
                path_terminal_indices_none: ascent::rel::ToRelIndexType<
                    (),
                    (Proc, Vec<Proc>),
                >,
                /**
logical indices: proc_indices_0; proc_indices_none*/
                pub proc: ::std::vec::Vec<(Proc,)>,
                __proc_ind_common: (),
                proc_indices_0: ascent::internal::RelFullIndexType<(Proc,), ()>,
                proc_indices_none: ascent::rel::ToRelIndexType<(), (Proc,)>,
                /**
logical indices: rw_indices_0; rw_indices_0_1; rw_indices_1; rw_indices_none*/
                pub rw: ::std::vec::Vec<(Proc, Proc)>,
                __rw_ind_common: (),
                rw_indices_0: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                rw_indices_0_1: ascent::internal::RelFullIndexType<(Proc, Proc), ()>,
                rw_indices_1: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                rw_indices_none: ascent::rel::ToRelIndexType<(), (Proc, Proc)>,
                scc_times: [std::time::Duration; 6usize],
                scc_iters: [usize; 6usize],
                update_time_nanos: std::sync::atomic::AtomicU64,
                update_indices_duration: std::time::Duration,
            }
            impl AscentProgram {
                #[allow(noop_method_call, suspicious_double_ref_op)]
                fn update_indices_priv(&mut self) {
                    #![allow(clippy::all)]
                    let before = ::ascent::internal::Instant::now();
                    use ascent::internal::ToRelIndex0;
                    use ascent::internal::RelIndexWrite;
                    for (_i, tuple) in self.eq.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.eq_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__eq_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.eq_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__eq_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = (tuple.1.clone(),);
                        let rel_ind = &mut self.eq_indices_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__eq_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.eq_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__eq_ind_common),
                            selection_tuple,
                            (tuple.0.clone(), tuple.1.clone()),
                        );
                    }
                    for (_i, tuple) in self.full_path.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.full_path_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__full_path_ind_common),
                            selection_tuple,
                            (),
                        );
                    }
                    for (_i, tuple) in self.path.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.path_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__path_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.path_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__path_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.path_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__path_ind_common),
                            selection_tuple,
                            (tuple.0.clone(), tuple.1.clone()),
                        );
                    }
                    for (_i, tuple) in self.path_terminal.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.path_terminal_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__path_terminal_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.path_terminal_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__path_terminal_ind_common),
                            selection_tuple,
                            (tuple.0.clone(), tuple.1.clone()),
                        );
                    }
                    for (_i, tuple) in self.proc.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.proc_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__proc_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.proc_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__proc_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                    }
                    for (_i, tuple) in self.rw.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.rw_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__rw_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.rw_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__rw_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = (tuple.1.clone(),);
                        let rel_ind = &mut self.rw_indices_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__rw_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.rw_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__rw_ind_common),
                            selection_tuple,
                            (tuple.0.clone(), tuple.1.clone()),
                        );
                    }
                    self.update_indices_duration += before.elapsed();
                }
                #[deprecated = "Explicit call to update_indices not required anymore."]
                pub fn update_indices(&mut self) {
                    self.update_indices_priv();
                }
                fn type_constraints() {
                    #![allow(clippy::all)]
                    let _type_constraints: ascent::internal::TypeConstraints<Proc>;
                    let _type_constraints: ascent::internal::TypeConstraints<Vec<Proc>>;
                }
                pub fn summary(&self) -> &'static str {
                    "scc 0, is_looping: false:\n  proc <-- for_p\n  dynamic relations: proc\nscc 1, is_looping: true:\n  proc <-- proc_indices_0_delta, rw_indices_0_total+delta [SIMPLE JOIN]\n  proc <-- proc_indices_0_total, rw_indices_0_delta [SIMPLE JOIN]\n  proc <-- proc_indices_0_delta, eq_indices_0_total+delta [SIMPLE JOIN]\n  proc <-- proc_indices_0_total, eq_indices_0_delta [SIMPLE JOIN]\n  proc, proc <-- proc_indices_none_delta, if let ⋯\n  rw <-- proc_indices_none_delta, if let ⋯, if let ⋯\n  rw <-- proc_indices_none_delta, if let ⋯\n  rw <-- proc_indices_none_delta, if let ⋯, rw_indices_0_total+delta, let ⋯\n  rw <-- proc_indices_none_total, if let ⋯, rw_indices_0_delta, let ⋯\n  rw <-- rw_indices_0_delta, eq_indices_0_total+delta [SIMPLE JOIN]\n  rw <-- rw_indices_0_total, eq_indices_0_delta [SIMPLE JOIN]\n  eq <-- proc_indices_none_delta, if let ⋯, let ⋯\n  eq <-- proc_indices_none_delta, if let ⋯, if let ⋯, let ⋯\n  eq <-- proc_indices_none_delta\n  eq <-- eq_indices_none_delta\n  eq <-- eq_indices_1_delta, eq_indices_0_total+delta [SIMPLE JOIN]\n  eq <-- eq_indices_1_total, eq_indices_0_delta [SIMPLE JOIN]\n  dynamic relations: eq, proc, rw\nscc 2, is_looping: false:\n  path <-- rw_indices_none_total\n  dynamic relations: path\nscc 3, is_looping: true:\n  path <-- rw_indices_1_total, path_indices_0_delta, let ⋯ [SIMPLE JOIN]\n  dynamic relations: path\nscc 4, is_looping: false:\n  path_terminal <-- path_indices_none_total, let ⋯, agg rw_indices_0\n  dynamic relations: path_terminal\nscc 5, is_looping: false:\n  full_path <-- path_terminal_indices_none_total, eq_indices_0_1_total\n  dynamic relations: full_path\n"
                }
                pub fn relation_sizes_summary(&self) -> String {
                    #![allow(clippy::all)]
                    use std::fmt::Write;
                    let mut res = String::new();
                    (&mut res)
                        .write_fmt(format_args!("{0} size: {1}\n", "eq", self.eq.len()))
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "full_path",
                                self.full_path.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!("{0} size: {1}\n", "path", self.path.len()),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "path_terminal",
                                self.path_terminal.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!("{0} size: {1}\n", "proc", self.proc.len()),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(format_args!("{0} size: {1}\n", "rw", self.rw.len()))
                        .unwrap();
                    res
                }
                pub fn scc_times_summary(&self) -> String {
                    #![allow(clippy::all)]
                    use std::fmt::Write;
                    let mut res = String::new();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "update_indices time: {0:?}\n",
                                self.update_indices_duration,
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "0",
                                self.scc_iters[0usize],
                                self.scc_times[0usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "1",
                                self.scc_iters[1usize],
                                self.scc_times[1usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "2",
                                self.scc_iters[2usize],
                                self.scc_times[2usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "3",
                                self.scc_iters[3usize],
                                self.scc_times[3usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "4",
                                self.scc_iters[4usize],
                                self.scc_times[4usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "5",
                                self.scc_iters[5usize],
                                self.scc_times[5usize],
                            ),
                        )
                        .unwrap();
                    res
                }
            }
            impl Default for AscentProgram {
                fn default() -> Self {
                    let mut _self = AscentProgram {
                        eq: Default::default(),
                        __eq_ind_common: Default::default(),
                        eq_indices_0: Default::default(),
                        eq_indices_0_1: Default::default(),
                        eq_indices_1: Default::default(),
                        eq_indices_none: Default::default(),
                        full_path: Default::default(),
                        __full_path_ind_common: Default::default(),
                        full_path_indices_0_1: Default::default(),
                        path: Default::default(),
                        __path_ind_common: Default::default(),
                        path_indices_0: Default::default(),
                        path_indices_0_1: Default::default(),
                        path_indices_none: Default::default(),
                        path_terminal: Default::default(),
                        __path_terminal_ind_common: Default::default(),
                        path_terminal_indices_0_1: Default::default(),
                        path_terminal_indices_none: Default::default(),
                        proc: Default::default(),
                        __proc_ind_common: Default::default(),
                        proc_indices_0: Default::default(),
                        proc_indices_none: Default::default(),
                        rw: Default::default(),
                        __rw_ind_common: Default::default(),
                        rw_indices_0: Default::default(),
                        rw_indices_0_1: Default::default(),
                        rw_indices_1: Default::default(),
                        rw_indices_none: Default::default(),
                        scc_times: [std::time::Duration::ZERO; 6usize],
                        scc_iters: [0; 6usize],
                        update_time_nanos: Default::default(),
                        update_indices_duration: std::time::Duration::default(),
                    };
                    _self
                }
            }
            let mut __run_res: AscentProgram = AscentProgram::default();
            {
                ascent::internal::comment("running...");
                use core::cmp::PartialEq;
                use ascent::internal::{
                    RelIndexRead, RelIndexReadAll, ToRelIndex0, TupleOfBorrowed,
                };
                use ascent::internal::RelIndexWrite;
                let _self = &mut __run_res;
                ascent::internal::comment("scc 0");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __proc_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__proc_ind_common,
                    );
                    let mut __proc_ind_common_total: () = Default::default();
                    let mut __proc_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __proc_ind_common_new,
                        &mut __proc_ind_common_delta,
                        &mut __proc_ind_common_total,
                    );
                    let mut proc_indices_0_delta: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = ::std::mem::take(&mut _self.proc_indices_0);
                    let mut proc_indices_0_total: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    let mut proc_indices_0_new: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut proc_indices_0_new
                            .to_rel_index_write(&mut __proc_ind_common_new),
                        &mut proc_indices_0_delta
                            .to_rel_index_write(&mut __proc_ind_common_delta),
                        &mut proc_indices_0_total
                            .to_rel_index_write(&mut __proc_ind_common_total),
                    );
                    let mut proc_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.proc_indices_none);
                    let mut proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    let mut proc_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut proc_indices_none_new
                            .to_rel_index_write(&mut __proc_ind_common_new),
                        &mut proc_indices_none_delta
                            .to_rel_index_write(&mut __proc_ind_common_delta),
                        &mut proc_indices_none_total
                            .to_rel_index_write(&mut __proc_ind_common_total),
                    );
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment("proc <-- for_p");
                        {
                            for p in [redex.clone()] {
                                let __new_row: (Proc,) = (
                                    ascent::internal::Convert::convert(p),
                                );
                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                    &proc_indices_0_total
                                        .to_rel_index(&__proc_ind_common_total),
                                    &__new_row,
                                )
                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                        &proc_indices_0_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                        &__new_row,
                                    )
                                {
                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                        &mut proc_indices_0_new
                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                        &__new_row,
                                        (),
                                    ) {
                                        let __new_row_ind = _self.proc.len();
                                        _self.proc.push((__new_row.0.clone(),));
                                        ::ascent::internal::RelIndexWrite::index_insert(
                                            &mut proc_indices_none_new
                                                .to_rel_index_write(&mut __proc_ind_common_new),
                                            (),
                                            (__new_row.0.clone(),),
                                        );
                                        __changed = true;
                                    }
                                }
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __proc_ind_common_new,
                            &mut __proc_ind_common_delta,
                            &mut __proc_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_0_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_0_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_0_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_none_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_none_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_none_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __proc_ind_common_new,
                            &mut __proc_ind_common_delta,
                            &mut __proc_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_0_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_0_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_0_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_none_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_none_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_none_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        _self.scc_iters[0usize] += 1;
                    }
                    _self.__proc_ind_common = __proc_ind_common_total;
                    _self.proc_indices_0 = proc_indices_0_total;
                    _self.proc_indices_none = proc_indices_none_total;
                    _self.scc_times[0usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 1");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __eq_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__eq_ind_common,
                    );
                    let mut __eq_ind_common_total: () = Default::default();
                    let mut __eq_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __eq_ind_common_new,
                        &mut __eq_ind_common_delta,
                        &mut __eq_ind_common_total,
                    );
                    let mut eq_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.eq_indices_0);
                    let mut eq_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut eq_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_indices_0_new
                            .to_rel_index_write(&mut __eq_ind_common_new),
                        &mut eq_indices_0_delta
                            .to_rel_index_write(&mut __eq_ind_common_delta),
                        &mut eq_indices_0_total
                            .to_rel_index_write(&mut __eq_ind_common_total),
                    );
                    let mut eq_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.eq_indices_0_1);
                    let mut eq_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut eq_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_indices_0_1_new
                            .to_rel_index_write(&mut __eq_ind_common_new),
                        &mut eq_indices_0_1_delta
                            .to_rel_index_write(&mut __eq_ind_common_delta),
                        &mut eq_indices_0_1_total
                            .to_rel_index_write(&mut __eq_ind_common_total),
                    );
                    let mut eq_indices_1_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.eq_indices_1);
                    let mut eq_indices_1_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut eq_indices_1_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_indices_1_new
                            .to_rel_index_write(&mut __eq_ind_common_new),
                        &mut eq_indices_1_delta
                            .to_rel_index_write(&mut __eq_ind_common_delta),
                        &mut eq_indices_1_total
                            .to_rel_index_write(&mut __eq_ind_common_total),
                    );
                    let mut eq_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = ::std::mem::take(&mut _self.eq_indices_none);
                    let mut eq_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    let mut eq_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_indices_none_new
                            .to_rel_index_write(&mut __eq_ind_common_new),
                        &mut eq_indices_none_delta
                            .to_rel_index_write(&mut __eq_ind_common_delta),
                        &mut eq_indices_none_total
                            .to_rel_index_write(&mut __eq_ind_common_total),
                    );
                    let mut __proc_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__proc_ind_common,
                    );
                    let mut __proc_ind_common_total: () = Default::default();
                    let mut __proc_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __proc_ind_common_new,
                        &mut __proc_ind_common_delta,
                        &mut __proc_ind_common_total,
                    );
                    let mut proc_indices_0_delta: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = ::std::mem::take(&mut _self.proc_indices_0);
                    let mut proc_indices_0_total: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    let mut proc_indices_0_new: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut proc_indices_0_new
                            .to_rel_index_write(&mut __proc_ind_common_new),
                        &mut proc_indices_0_delta
                            .to_rel_index_write(&mut __proc_ind_common_delta),
                        &mut proc_indices_0_total
                            .to_rel_index_write(&mut __proc_ind_common_total),
                    );
                    let mut proc_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.proc_indices_none);
                    let mut proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    let mut proc_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut proc_indices_none_new
                            .to_rel_index_write(&mut __proc_ind_common_new),
                        &mut proc_indices_none_delta
                            .to_rel_index_write(&mut __proc_ind_common_delta),
                        &mut proc_indices_none_total
                            .to_rel_index_write(&mut __proc_ind_common_total),
                    );
                    let mut __rw_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__rw_ind_common,
                    );
                    let mut __rw_ind_common_total: () = Default::default();
                    let mut __rw_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __rw_ind_common_new,
                        &mut __rw_ind_common_delta,
                        &mut __rw_ind_common_total,
                    );
                    let mut rw_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.rw_indices_0);
                    let mut rw_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut rw_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_indices_0_new
                            .to_rel_index_write(&mut __rw_ind_common_new),
                        &mut rw_indices_0_delta
                            .to_rel_index_write(&mut __rw_ind_common_delta),
                        &mut rw_indices_0_total
                            .to_rel_index_write(&mut __rw_ind_common_total),
                    );
                    let mut rw_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.rw_indices_0_1);
                    let mut rw_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut rw_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_indices_0_1_new
                            .to_rel_index_write(&mut __rw_ind_common_new),
                        &mut rw_indices_0_1_delta
                            .to_rel_index_write(&mut __rw_ind_common_delta),
                        &mut rw_indices_0_1_total
                            .to_rel_index_write(&mut __rw_ind_common_total),
                    );
                    let mut rw_indices_1_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.rw_indices_1);
                    let mut rw_indices_1_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut rw_indices_1_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_indices_1_new
                            .to_rel_index_write(&mut __rw_ind_common_new),
                        &mut rw_indices_1_delta
                            .to_rel_index_write(&mut __rw_ind_common_delta),
                        &mut rw_indices_1_total
                            .to_rel_index_write(&mut __rw_ind_common_total),
                    );
                    let mut rw_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = ::std::mem::take(&mut _self.rw_indices_none);
                    let mut rw_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    let mut rw_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_indices_none_new
                            .to_rel_index_write(&mut __rw_ind_common_new),
                        &mut rw_indices_none_delta
                            .to_rel_index_write(&mut __rw_ind_common_delta),
                        &mut rw_indices_none_total
                            .to_rel_index_write(&mut __rw_ind_common_total),
                    );
                    #[allow(unused_assignments, unused_variables)]
                    loop {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "proc <-- proc_indices_0_delta, rw_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &rw_indices_0_total.to_rel_index(&__rw_ind_common_total),
                                        &rw_indices_0_delta.to_rel_index(&__rw_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                proc_indices_0_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &rw_indices_0_total.to_rel_index(&__rw_ind_common_total),
                                                &rw_indices_0_delta.to_rel_index(&__rw_ind_common_delta),
                                            )
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                ascent::internal::RelIndexCombined::new(
                                        &rw_indices_0_total.to_rel_index(&__rw_ind_common_total),
                                        &rw_indices_0_delta.to_rel_index(&__rw_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_delta
                                            .to_rel_index(&__proc_ind_common_delta)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "proc <-- proc_indices_0_total, rw_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_total
                                .to_rel_index(&__proc_ind_common_total)
                                .len_estimate()
                                <= rw_indices_0_delta
                                    .to_rel_index(&__rw_ind_common_delta)
                                    .len_estimate()
                            {
                                proc_indices_0_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_indices_0_delta
                                            .to_rel_index(&__rw_ind_common_delta)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                rw_indices_0_delta
                                    .to_rel_index(&__rw_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_total
                                            .to_rel_index(&__proc_ind_common_total)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "proc <-- proc_indices_0_delta, eq_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                proc_indices_0_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                                &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                            )
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_delta
                                            .to_rel_index(&__proc_ind_common_delta)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "proc <-- proc_indices_0_total, eq_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_total
                                .to_rel_index(&__proc_ind_common_total)
                                .len_estimate()
                                <= eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .len_estimate()
                            {
                                proc_indices_0_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_indices_0_delta
                                            .to_rel_index(&__eq_ind_common_delta)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_total
                                            .to_rel_index(&__proc_ind_common_total)
                                            .index_get(&(p0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(p1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &proc_indices_0_total
                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &proc_indices_0_delta
                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut proc_indices_0_new
                                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.proc.len();
                                                                    _self.proc.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut proc_indices_none_new
                                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "proc, proc <-- proc_indices_none_delta, if let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p0: &Proc = __val.0;
                                        if let Proc::PPar(p, q) = p0 {
                                            let __new_row: (Proc,) = (*p.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &proc_indices_0_total
                                                    .to_rel_index(&__proc_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &proc_indices_0_delta
                                                        .to_rel_index(&__proc_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut proc_indices_0_new
                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.proc.len();
                                                    _self.proc.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut proc_indices_none_new
                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                            let __new_row: (Proc,) = (*q.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &proc_indices_0_total
                                                    .to_rel_index(&__proc_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &proc_indices_0_delta
                                                        .to_rel_index(&__proc_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut proc_indices_0_new
                                                        .to_rel_index_write(&mut __proc_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.proc.len();
                                                    _self.proc.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut proc_indices_none_new
                                                            .to_rel_index_write(&mut __proc_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- proc_indices_none_delta, if let ⋯, if let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let s: &Proc = __val.0;
                                        if let Proc::PDrop(n) = s {
                                            if let Name::NQuote(p) = &**n {
                                                let __new_row: (Proc, Proc) = (
                                                    ascent::internal::Convert::convert(s),
                                                    *p.clone(),
                                                );
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                        &__new_row,
                                                    )
                                                {
                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut rw_indices_0_1_new
                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                        &__new_row,
                                                        (),
                                                    ) {
                                                        let __new_row_ind = _self.rw.len();
                                                        _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_indices_0_new
                                                                .to_rel_index_write(&mut __rw_ind_common_new),
                                                            (__new_row.0.clone(),),
                                                            (__new_row.1.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_indices_1_new
                                                                .to_rel_index_write(&mut __rw_ind_common_new),
                                                            (__new_row.1.clone(),),
                                                            (__new_row.0.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_indices_none_new
                                                                .to_rel_index_write(&mut __rw_ind_common_new),
                                                            (),
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                        );
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- proc_indices_none_delta, if let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let s: &Proc = __val.0;
                                        if let Some(t) = try_rewrite_rule_0(&s) {
                                            let __new_row: (Proc, Proc) = (
                                                ascent::internal::Convert::convert(s),
                                                t.clone(),
                                            );
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut rw_indices_0_1_new
                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.rw.len();
                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut rw_indices_0_new
                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                        (__new_row.0.clone(),),
                                                        (__new_row.1.clone(),),
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut rw_indices_1_new
                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                        (__new_row.1.clone(),),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut rw_indices_none_new
                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- proc_indices_none_delta, if let ⋯, rw_indices_0_total+delta, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &rw_indices_0_total.to_rel_index(&__rw_ind_common_total),
                                        &rw_indices_0_delta.to_rel_index(&__rw_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if let Some(__matching) = proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .index_get(&())
                                {
                                    __matching
                                        .for_each(|__val| {
                                            let __val = __val.tuple_of_borrowed();
                                            let s: &Proc = __val.0;
                                            if let Proc::PPar(s0, p) = s {
                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                        &rw_indices_0_total.to_rel_index(&__rw_ind_common_total),
                                                        &rw_indices_0_delta.to_rel_index(&__rw_ind_common_delta),
                                                    )
                                                    .index_get(&((**s0).clone(),))
                                                {
                                                    __matching
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t0: &Proc = __val.0;
                                                            let t = Proc::PPar(Box::new(t0.clone()), p.clone());
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                }
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- proc_indices_none_total, if let ⋯, rw_indices_0_delta, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || rw_indices_0_delta
                                    .to_rel_index(&__rw_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if let Some(__matching) = proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .index_get(&())
                                {
                                    __matching
                                        .for_each(|__val| {
                                            let __val = __val.tuple_of_borrowed();
                                            let s: &Proc = __val.0;
                                            if let Proc::PPar(s0, p) = s {
                                                if let Some(__matching) = rw_indices_0_delta
                                                    .to_rel_index(&__rw_ind_common_delta)
                                                    .index_get(&((**s0).clone(),))
                                                {
                                                    __matching
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t0: &Proc = __val.0;
                                                            let t = Proc::PPar(Box::new(t0.clone()), p.clone());
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                }
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- rw_indices_0_delta, eq_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_indices_0_delta
                                .to_rel_index(&__rw_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                rw_indices_0_delta
                                    .to_rel_index(&__rw_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                                &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                            )
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s1: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_indices_0_delta
                                            .to_rel_index(&__rw_ind_common_delta)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "rw <-- rw_indices_0_total, eq_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_indices_0_total
                                .to_rel_index(&__rw_ind_common_total)
                                .len_estimate()
                                <= eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_indices_0_total
                                    .to_rel_index(&__rw_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_indices_0_delta
                                            .to_rel_index(&__eq_ind_common_delta)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s1: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_indices_0_total
                                            .to_rel_index(&__rw_ind_common_total)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_indices_0_1_total.to_rel_index(&__rw_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_indices_0_1_delta.to_rel_index(&__rw_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw.len();
                                                                    _self.rw.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq <-- proc_indices_none_delta, if let ⋯, let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p0: &Proc = __val.0;
                                        if let Proc::PPar(p, q) = p0 {
                                            let p1 = Proc::PPar(q.clone(), p.clone());
                                            let __new_row: (Proc, Proc) = (
                                                ascent::internal::Convert::convert(p0),
                                                ascent::internal::Convert::convert(p1),
                                            );
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut eq_indices_0_1_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.eq.len();
                                                    _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut eq_indices_0_new
                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                        (__new_row.0.clone(),),
                                                        (__new_row.1.clone(),),
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut eq_indices_1_new
                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                        (__new_row.1.clone(),),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut eq_indices_none_new
                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq <-- proc_indices_none_delta, if let ⋯, if let ⋯, let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p0: &Proc = __val.0;
                                        if let Proc::PPar(t, r) = p0 {
                                            if let Proc::PPar(p, q) = &**t {
                                                let p1 = Proc::PPar(
                                                    p.clone(),
                                                    Box::new(Proc::PPar(q.clone(), r.clone())),
                                                );
                                                let __new_row: (Proc, Proc) = (
                                                    ascent::internal::Convert::convert(p0),
                                                    ascent::internal::Convert::convert(p1),
                                                );
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                        &__new_row,
                                                    )
                                                {
                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut eq_indices_0_1_new
                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                        &__new_row,
                                                        (),
                                                    ) {
                                                        let __new_row_ind = _self.eq.len();
                                                        _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut eq_indices_0_new
                                                                .to_rel_index_write(&mut __eq_ind_common_new),
                                                            (__new_row.0.clone(),),
                                                            (__new_row.1.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut eq_indices_1_new
                                                                .to_rel_index_write(&mut __eq_ind_common_new),
                                                            (__new_row.1.clone(),),
                                                            (__new_row.0.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut eq_indices_none_new
                                                                .to_rel_index_write(&mut __eq_ind_common_new),
                                                            (),
                                                            (__new_row.0.clone(), __new_row.1.clone()),
                                                        );
                                                        __changed = true;
                                                    }
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment("eq <-- proc_indices_none_delta");
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p: &Proc = __val.0;
                                        let __new_row: (Proc, Proc) = (
                                            ascent::internal::Convert::convert(p),
                                            ascent::internal::Convert::convert(p),
                                        );
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut eq_indices_0_1_new
                                                    .to_rel_index_write(&mut __eq_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.eq.len();
                                                _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_0_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (__new_row.0.clone(),),
                                                    (__new_row.1.clone(),),
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_1_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (__new_row.1.clone(),),
                                                    (__new_row.0.clone(),),
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_none_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (),
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment("eq <-- eq_indices_none_delta");
                        {
                            if let Some(__matching) = eq_indices_none_delta
                                .to_rel_index(&__eq_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p: &Proc = __val.0;
                                        let q: &Proc = __val.1;
                                        let __new_row: (Proc, Proc) = (
                                            ascent::internal::Convert::convert(q),
                                            ascent::internal::Convert::convert(p),
                                        );
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut eq_indices_0_1_new
                                                    .to_rel_index_write(&mut __eq_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.eq.len();
                                                _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_0_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (__new_row.0.clone(),),
                                                    (__new_row.1.clone(),),
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_1_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (__new_row.1.clone(),),
                                                    (__new_row.0.clone(),),
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_indices_none_new
                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                    (),
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq <-- eq_indices_1_delta, eq_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if eq_indices_1_delta
                                .to_rel_index(&__eq_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                eq_indices_1_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let q = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                                &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                            )
                                            .index_get(&(q.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let r: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(p),
                                                                ascent::internal::Convert::convert(r),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut eq_indices_0_1_new
                                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.eq.len();
                                                                    _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_0_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_1_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_none_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                ascent::internal::RelIndexCombined::new(
                                        &eq_indices_0_total.to_rel_index(&__eq_ind_common_total),
                                        &eq_indices_0_delta.to_rel_index(&__eq_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let q = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_indices_1_delta
                                            .to_rel_index(&__eq_ind_common_delta)
                                            .index_get(&(q.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let r: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(p),
                                                                ascent::internal::Convert::convert(r),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut eq_indices_0_1_new
                                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.eq.len();
                                                                    _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_0_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_1_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_none_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq <-- eq_indices_1_total, eq_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if eq_indices_1_total
                                .to_rel_index(&__eq_ind_common_total)
                                .len_estimate()
                                <= eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .len_estimate()
                            {
                                eq_indices_1_total
                                    .to_rel_index(&__eq_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let q = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_indices_0_delta
                                            .to_rel_index(&__eq_ind_common_delta)
                                            .index_get(&(q.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let r: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(p),
                                                                ascent::internal::Convert::convert(r),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut eq_indices_0_1_new
                                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.eq.len();
                                                                    _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_0_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_1_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_none_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                eq_indices_0_delta
                                    .to_rel_index(&__eq_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let q = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_indices_1_total
                                            .to_rel_index(&__eq_ind_common_total)
                                            .index_get(&(q.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let r: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(p),
                                                                ascent::internal::Convert::convert(r),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &eq_indices_0_1_total.to_rel_index(&__eq_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &eq_indices_0_1_delta.to_rel_index(&__eq_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut eq_indices_0_1_new
                                                                        .to_rel_index_write(&mut __eq_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.eq.len();
                                                                    _self.eq.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_0_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_1_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut eq_indices_none_new
                                                                            .to_rel_index_write(&mut __eq_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __eq_ind_common_new,
                            &mut __eq_ind_common_delta,
                            &mut __eq_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_indices_0_new
                                .to_rel_index_write(&mut __eq_ind_common_new),
                            &mut eq_indices_0_delta
                                .to_rel_index_write(&mut __eq_ind_common_delta),
                            &mut eq_indices_0_total
                                .to_rel_index_write(&mut __eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_indices_0_1_new
                                .to_rel_index_write(&mut __eq_ind_common_new),
                            &mut eq_indices_0_1_delta
                                .to_rel_index_write(&mut __eq_ind_common_delta),
                            &mut eq_indices_0_1_total
                                .to_rel_index_write(&mut __eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_indices_1_new
                                .to_rel_index_write(&mut __eq_ind_common_new),
                            &mut eq_indices_1_delta
                                .to_rel_index_write(&mut __eq_ind_common_delta),
                            &mut eq_indices_1_total
                                .to_rel_index_write(&mut __eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_indices_none_new
                                .to_rel_index_write(&mut __eq_ind_common_new),
                            &mut eq_indices_none_delta
                                .to_rel_index_write(&mut __eq_ind_common_delta),
                            &mut eq_indices_none_total
                                .to_rel_index_write(&mut __eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __proc_ind_common_new,
                            &mut __proc_ind_common_delta,
                            &mut __proc_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_0_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_0_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_0_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut proc_indices_none_new
                                .to_rel_index_write(&mut __proc_ind_common_new),
                            &mut proc_indices_none_delta
                                .to_rel_index_write(&mut __proc_ind_common_delta),
                            &mut proc_indices_none_total
                                .to_rel_index_write(&mut __proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __rw_ind_common_new,
                            &mut __rw_ind_common_delta,
                            &mut __rw_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_indices_0_new
                                .to_rel_index_write(&mut __rw_ind_common_new),
                            &mut rw_indices_0_delta
                                .to_rel_index_write(&mut __rw_ind_common_delta),
                            &mut rw_indices_0_total
                                .to_rel_index_write(&mut __rw_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_indices_0_1_new
                                .to_rel_index_write(&mut __rw_ind_common_new),
                            &mut rw_indices_0_1_delta
                                .to_rel_index_write(&mut __rw_ind_common_delta),
                            &mut rw_indices_0_1_total
                                .to_rel_index_write(&mut __rw_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_indices_1_new
                                .to_rel_index_write(&mut __rw_ind_common_new),
                            &mut rw_indices_1_delta
                                .to_rel_index_write(&mut __rw_ind_common_delta),
                            &mut rw_indices_1_total
                                .to_rel_index_write(&mut __rw_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_indices_none_new
                                .to_rel_index_write(&mut __rw_ind_common_new),
                            &mut rw_indices_none_delta
                                .to_rel_index_write(&mut __rw_ind_common_delta),
                            &mut rw_indices_none_total
                                .to_rel_index_write(&mut __rw_ind_common_total),
                        );
                        _self.scc_iters[1usize] += 1;
                        if !__changed {
                            break;
                        }
                    }
                    _self.__eq_ind_common = __eq_ind_common_total;
                    _self.eq_indices_0 = eq_indices_0_total;
                    _self.eq_indices_0_1 = eq_indices_0_1_total;
                    _self.eq_indices_1 = eq_indices_1_total;
                    _self.eq_indices_none = eq_indices_none_total;
                    _self.__proc_ind_common = __proc_ind_common_total;
                    _self.proc_indices_0 = proc_indices_0_total;
                    _self.proc_indices_none = proc_indices_none_total;
                    _self.__rw_ind_common = __rw_ind_common_total;
                    _self.rw_indices_0 = rw_indices_0_total;
                    _self.rw_indices_0_1 = rw_indices_0_1_total;
                    _self.rw_indices_1 = rw_indices_1_total;
                    _self.rw_indices_none = rw_indices_none_total;
                    _self.scc_times[1usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 2");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __path_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__path_ind_common,
                    );
                    let mut __path_ind_common_total: () = Default::default();
                    let mut __path_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __path_ind_common_new,
                        &mut __path_ind_common_delta,
                        &mut __path_ind_common_total,
                    );
                    let mut path_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_0_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_0_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_0_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let mut path_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_0_1_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_0_1_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_0_1_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let mut path_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = ::std::mem::take(&mut _self.path_indices_none);
                    let mut path_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    let mut path_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_none_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_none_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_none_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let __rw_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_ind_common,
                    );
                    let rw_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = std::mem::take(&mut _self.rw_indices_none);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment("path <-- rw_indices_none_total");
                        {
                            if let Some(__matching) = rw_indices_none_total
                                .to_rel_index(&__rw_ind_common_total)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p1: &Proc = __val.0;
                                        let p2: &Proc = __val.1;
                                        let __new_row: (Proc, Vec<Proc>) = (
                                            ascent::internal::Convert::convert(p1),
                                            <[_]>::into_vec(::alloc::boxed::box_new([p2.clone()])),
                                        );
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &path_indices_0_1_total
                                                .to_rel_index(&__path_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &path_indices_0_1_delta
                                                    .to_rel_index(&__path_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut path_indices_0_1_new
                                                    .to_rel_index_write(&mut __path_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.path.len();
                                                _self.path.push((__new_row.0.clone(), __new_row.1.clone()));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut path_indices_0_new
                                                        .to_rel_index_write(&mut __path_ind_common_new),
                                                    (__new_row.0.clone(),),
                                                    (__new_row.1.clone(),),
                                                );
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut path_indices_none_new
                                                        .to_rel_index_write(&mut __path_ind_common_new),
                                                    (),
                                                    (__new_row.0.clone(), __new_row.1.clone()),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_ind_common_new,
                            &mut __path_ind_common_delta,
                            &mut __path_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_1_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_1_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_1_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_none_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_none_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_none_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_ind_common_new,
                            &mut __path_ind_common_delta,
                            &mut __path_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_1_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_1_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_1_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_none_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_none_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_none_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        _self.scc_iters[2usize] += 1;
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.path_indices_none = path_indices_none_total;
                    _self.__rw_ind_common = __rw_ind_common_total;
                    _self.rw_indices_none = rw_indices_none_total;
                    _self.scc_times[2usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 3");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __path_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__path_ind_common,
                    );
                    let mut __path_ind_common_total: () = Default::default();
                    let mut __path_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __path_ind_common_new,
                        &mut __path_ind_common_delta,
                        &mut __path_ind_common_total,
                    );
                    let mut path_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Vec<Proc>,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_0_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_0_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_0_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let mut path_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_0_1_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_0_1_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_0_1_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let mut path_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = ::std::mem::take(&mut _self.path_indices_none);
                    let mut path_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    let mut path_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_indices_none_new
                            .to_rel_index_write(&mut __path_ind_common_new),
                        &mut path_indices_none_delta
                            .to_rel_index_write(&mut __path_ind_common_delta),
                        &mut path_indices_none_total
                            .to_rel_index_write(&mut __path_ind_common_total),
                    );
                    let __rw_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_ind_common,
                    );
                    let rw_indices_1_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = std::mem::take(&mut _self.rw_indices_1);
                    #[allow(unused_assignments, unused_variables)]
                    loop {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "path <-- rw_indices_1_total, path_indices_0_delta, let ⋯ [SIMPLE JOIN]",
                        );
                        {
                            if rw_indices_1_total
                                .to_rel_index(&__rw_ind_common_total)
                                .len_estimate()
                                <= path_indices_0_delta
                                    .to_rel_index(&__path_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_indices_1_total
                                    .to_rel_index(&__rw_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p2 = __cl1_joined_columns.0;
                                        if let Some(__matching) = path_indices_0_delta
                                            .to_rel_index(&__path_ind_common_delta)
                                            .index_get(&(p2.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let p1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let qs: &Vec<Proc> = __val.0;
                                                            let ps = [
                                                                <[_]>::into_vec(::alloc::boxed::box_new([p2.clone()])),
                                                                qs.clone(),
                                                            ]
                                                                .concat();
                                                            let __new_row: (Proc, Vec<Proc>) = (
                                                                ascent::internal::Convert::convert(p1),
                                                                ascent::internal::Convert::convert(ps),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &path_indices_0_1_total
                                                                    .to_rel_index(&__path_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &path_indices_0_1_delta
                                                                        .to_rel_index(&__path_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut path_indices_0_1_new
                                                                        .to_rel_index_write(&mut __path_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.path.len();
                                                                    _self.path.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut path_indices_0_new
                                                                            .to_rel_index_write(&mut __path_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut path_indices_none_new
                                                                            .to_rel_index_write(&mut __path_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            } else {
                                path_indices_0_delta
                                    .to_rel_index(&__path_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let p2 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_indices_1_total
                                            .to_rel_index(&__rw_ind_common_total)
                                            .index_get(&(p2.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let qs: &Vec<Proc> = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let p1: &Proc = __val.0;
                                                            let ps = [
                                                                <[_]>::into_vec(::alloc::boxed::box_new([p2.clone()])),
                                                                qs.clone(),
                                                            ]
                                                                .concat();
                                                            let __new_row: (Proc, Vec<Proc>) = (
                                                                ascent::internal::Convert::convert(p1),
                                                                ascent::internal::Convert::convert(ps),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &path_indices_0_1_total
                                                                    .to_rel_index(&__path_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &path_indices_0_1_delta
                                                                        .to_rel_index(&__path_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut path_indices_0_1_new
                                                                        .to_rel_index_write(&mut __path_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.path.len();
                                                                    _self.path.push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut path_indices_0_new
                                                                            .to_rel_index_write(&mut __path_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut path_indices_none_new
                                                                            .to_rel_index_write(&mut __path_ind_common_new),
                                                                        (),
                                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                                    );
                                                                    __changed = true;
                                                                }
                                                            }
                                                        });
                                                });
                                        }
                                    });
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_ind_common_new,
                            &mut __path_ind_common_delta,
                            &mut __path_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_0_1_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_0_1_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_0_1_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_indices_none_new
                                .to_rel_index_write(&mut __path_ind_common_new),
                            &mut path_indices_none_delta
                                .to_rel_index_write(&mut __path_ind_common_delta),
                            &mut path_indices_none_total
                                .to_rel_index_write(&mut __path_ind_common_total),
                        );
                        _self.scc_iters[3usize] += 1;
                        if !__changed {
                            break;
                        }
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.path_indices_none = path_indices_none_total;
                    _self.__rw_ind_common = __rw_ind_common_total;
                    _self.rw_indices_1 = rw_indices_1_total;
                    _self.scc_times[3usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 4");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __path_terminal_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__path_terminal_ind_common,
                    );
                    let mut __path_terminal_ind_common_total: () = Default::default();
                    let mut __path_terminal_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __path_terminal_ind_common_new,
                        &mut __path_terminal_ind_common_delta,
                        &mut __path_terminal_ind_common_total,
                    );
                    let mut path_terminal_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = ::std::mem::take(&mut _self.path_terminal_indices_0_1);
                    let mut path_terminal_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    let mut path_terminal_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_terminal_indices_0_1_new
                            .to_rel_index_write(&mut __path_terminal_ind_common_new),
                        &mut path_terminal_indices_0_1_delta
                            .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                        &mut path_terminal_indices_0_1_total
                            .to_rel_index_write(&mut __path_terminal_ind_common_total),
                    );
                    let mut path_terminal_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = ::std::mem::take(&mut _self.path_terminal_indices_none);
                    let mut path_terminal_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    let mut path_terminal_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_terminal_indices_none_new
                            .to_rel_index_write(&mut __path_terminal_ind_common_new),
                        &mut path_terminal_indices_none_delta
                            .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                        &mut path_terminal_indices_none_total
                            .to_rel_index_write(&mut __path_terminal_ind_common_total),
                    );
                    let __path_ind_common_total: () = std::mem::take(
                        &mut _self.__path_ind_common,
                    );
                    let path_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = std::mem::take(&mut _self.path_indices_none);
                    let __rw_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_ind_common,
                    );
                    let rw_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = std::mem::take(&mut _self.rw_indices_0);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "path_terminal <-- path_indices_none_total, let ⋯, agg rw_indices_0",
                        );
                        {
                            if let Some(__matching) = path_indices_none_total
                                .to_rel_index(&__path_ind_common_total)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p: &Proc = __val.0;
                                        let ps: &Vec<Proc> = __val.1;
                                        let z = ps.last().unwrap();
                                        let __aggregated_rel = rw_indices_0_total
                                            .to_rel_index(&__rw_ind_common_total);
                                        let __matching = __aggregated_rel.index_get(&(z.clone(),));
                                        let __agg_args = __matching
                                            .into_iter()
                                            .flatten()
                                            .map(|__val| { () });
                                        for () in ::ascent::aggregators::not(__agg_args) {
                                            let __new_row: (Proc, Vec<Proc>) = (
                                                ascent::internal::Convert::convert(p),
                                                ascent::internal::Convert::convert(ps),
                                            );
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &path_terminal_indices_0_1_total
                                                    .to_rel_index(&__path_terminal_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &path_terminal_indices_0_1_delta
                                                        .to_rel_index(&__path_terminal_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut path_terminal_indices_0_1_new
                                                        .to_rel_index_write(&mut __path_terminal_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.path_terminal.len();
                                                    _self
                                                        .path_terminal
                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut path_terminal_indices_none_new
                                                            .to_rel_index_write(&mut __path_terminal_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(), __new_row.1.clone()),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_terminal_ind_common_new,
                            &mut __path_terminal_ind_common_delta,
                            &mut __path_terminal_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_terminal_indices_0_1_new
                                .to_rel_index_write(&mut __path_terminal_ind_common_new),
                            &mut path_terminal_indices_0_1_delta
                                .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                            &mut path_terminal_indices_0_1_total
                                .to_rel_index_write(&mut __path_terminal_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_terminal_indices_none_new
                                .to_rel_index_write(&mut __path_terminal_ind_common_new),
                            &mut path_terminal_indices_none_delta
                                .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                            &mut path_terminal_indices_none_total
                                .to_rel_index_write(&mut __path_terminal_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_terminal_ind_common_new,
                            &mut __path_terminal_ind_common_delta,
                            &mut __path_terminal_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_terminal_indices_0_1_new
                                .to_rel_index_write(&mut __path_terminal_ind_common_new),
                            &mut path_terminal_indices_0_1_delta
                                .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                            &mut path_terminal_indices_0_1_total
                                .to_rel_index_write(&mut __path_terminal_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_terminal_indices_none_new
                                .to_rel_index_write(&mut __path_terminal_ind_common_new),
                            &mut path_terminal_indices_none_delta
                                .to_rel_index_write(&mut __path_terminal_ind_common_delta),
                            &mut path_terminal_indices_none_total
                                .to_rel_index_write(&mut __path_terminal_ind_common_total),
                        );
                        _self.scc_iters[4usize] += 1;
                    }
                    _self.__path_terminal_ind_common = __path_terminal_ind_common_total;
                    _self.path_terminal_indices_0_1 = path_terminal_indices_0_1_total;
                    _self.path_terminal_indices_none = path_terminal_indices_none_total;
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_none = path_indices_none_total;
                    _self.__rw_ind_common = __rw_ind_common_total;
                    _self.rw_indices_0 = rw_indices_0_total;
                    _self.scc_times[4usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 5");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __full_path_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__full_path_ind_common,
                    );
                    let mut __full_path_ind_common_total: () = Default::default();
                    let mut __full_path_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __full_path_ind_common_new,
                        &mut __full_path_ind_common_delta,
                        &mut __full_path_ind_common_total,
                    );
                    let mut full_path_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = ::std::mem::take(&mut _self.full_path_indices_0_1);
                    let mut full_path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    let mut full_path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Vec<Proc>),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut full_path_indices_0_1_new
                            .to_rel_index_write(&mut __full_path_ind_common_new),
                        &mut full_path_indices_0_1_delta
                            .to_rel_index_write(&mut __full_path_ind_common_delta),
                        &mut full_path_indices_0_1_total
                            .to_rel_index_write(&mut __full_path_ind_common_total),
                    );
                    let __eq_ind_common_total: () = std::mem::take(
                        &mut _self.__eq_ind_common,
                    );
                    let eq_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = std::mem::take(&mut _self.eq_indices_0_1);
                    let __path_terminal_ind_common_total: () = std::mem::take(
                        &mut _self.__path_terminal_ind_common,
                    );
                    let path_terminal_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Vec<Proc>),
                    > = std::mem::take(&mut _self.path_terminal_indices_none);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "full_path <-- path_terminal_indices_none_total, eq_indices_0_1_total",
                        );
                        {
                            let any_rel_empty = path_terminal_indices_none_total
                                .to_rel_index(&__path_terminal_ind_common_total)
                                .is_empty()
                                || eq_indices_0_1_total
                                    .to_rel_index(&__eq_ind_common_total)
                                    .is_empty();
                            if !any_rel_empty {
                                if let Some(__matching) = path_terminal_indices_none_total
                                    .to_rel_index(&__path_terminal_ind_common_total)
                                    .index_get(&())
                                {
                                    __matching
                                        .for_each(|__val| {
                                            let __val = __val.tuple_of_borrowed();
                                            let s: &Proc = __val.0;
                                            let ps: &Vec<Proc> = __val.1;
                                            if let Some(__matching) = eq_indices_0_1_total
                                                .to_rel_index(&__eq_ind_common_total)
                                                .index_get(&(s.clone(), (redex.clone()).clone()))
                                            {
                                                __matching
                                                    .for_each(|__val| {
                                                        let __new_row: (Proc, Vec<Proc>) = (
                                                            ascent::internal::Convert::convert(s),
                                                            ascent::internal::Convert::convert(ps),
                                                        );
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &full_path_indices_0_1_total
                                                                .to_rel_index(&__full_path_ind_common_total),
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &full_path_indices_0_1_delta
                                                                    .to_rel_index(&__full_path_ind_common_delta),
                                                                &__new_row,
                                                            )
                                                        {
                                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut full_path_indices_0_1_new
                                                                    .to_rel_index_write(&mut __full_path_ind_common_new),
                                                                &__new_row,
                                                                (),
                                                            ) {
                                                                let __new_row_ind = _self.full_path.len();
                                                                _self.full_path.push((__new_row.0, __new_row.1));
                                                                __changed = true;
                                                            }
                                                        }
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __full_path_ind_common_new,
                            &mut __full_path_ind_common_delta,
                            &mut __full_path_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut full_path_indices_0_1_new
                                .to_rel_index_write(&mut __full_path_ind_common_new),
                            &mut full_path_indices_0_1_delta
                                .to_rel_index_write(&mut __full_path_ind_common_delta),
                            &mut full_path_indices_0_1_total
                                .to_rel_index_write(&mut __full_path_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __full_path_ind_common_new,
                            &mut __full_path_ind_common_delta,
                            &mut __full_path_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut full_path_indices_0_1_new
                                .to_rel_index_write(&mut __full_path_ind_common_new),
                            &mut full_path_indices_0_1_delta
                                .to_rel_index_write(&mut __full_path_ind_common_delta),
                            &mut full_path_indices_0_1_total
                                .to_rel_index_write(&mut __full_path_ind_common_total),
                        );
                        _self.scc_iters[5usize] += 1;
                    }
                    _self.__full_path_ind_common = __full_path_ind_common_total;
                    _self.full_path_indices_0_1 = full_path_indices_0_1_total;
                    _self.__eq_ind_common = __eq_ind_common_total;
                    _self.eq_indices_0_1 = eq_indices_0_1_total;
                    _self.__path_terminal_ind_common = __path_terminal_ind_common_total;
                    _self.path_terminal_indices_none = path_terminal_indices_none_total;
                    _self.scc_times[5usize] += _scc_start_time.elapsed();
                }
            }
            __run_res
        }
    };
    let mut paths = prog.full_path.clone();
    paths.sort_by(|a, b| a.0.cmp(&b.0));
    {
        ::std::io::_print(format_args!("Paths found: {0}\n", paths.len()));
    };
    for (s, ps) in paths.iter().take(3) {
        {
            ::std::io::_print(
                format_args!(
                    "{0} ~> {1}\n",
                    s,
                    ps
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(" ~> "),
                ),
            );
        };
        {
            ::std::io::_print(format_args!("\n"));
        };
    }
}
