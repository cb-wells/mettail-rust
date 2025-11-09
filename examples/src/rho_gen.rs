#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use mettail_macros::theory;
use mettail_runtime;
use lalrpop_util::lalrpop_mod;
use ascent_byods_rels::*;
use std::time::Instant;
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
    initial_var_count: usize,
    max_depth: usize,
    proc_by_depth: std::collections::HashMap<usize, Vec<Proc>>,
    name_by_depth: std::collections::HashMap<usize, Vec<Name>>,
}
impl GenerationContext {
    fn new(vars: Vec<String>, max_depth: usize) -> Self {
        let initial_var_count = vars.len();
        Self {
            vars,
            initial_var_count,
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
            let current_binding_depth = self.vars.len() - self.initial_var_count;
            let binder_name = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("x{0}", current_binding_depth))
            });
            let mut extended_vars = self.vars.clone();
            extended_vars.push(binder_name.clone());
            let mut temp_ctx = GenerationContext::new(extended_vars, depth - 1);
            temp_ctx = temp_ctx.generate_all();
            let mut bodies_with_binder = Vec::new();
            for d in 0..depth {
                if let Some(terms) = temp_ctx.proc_by_depth.get(&d) {
                    bodies_with_binder.extend(terms.clone());
                }
            }
            for d1 in 0..depth {
                if let Some(args1) = self.name_by_depth.get(&d1) {
                    for arg1 in args1 {
                        for body in &bodies_with_binder {
                            let binder_var = mettail_runtime::get_or_create_var(
                                &binder_name,
                            );
                            let binder = mettail_runtime::Binder(binder_var);
                            let scope = mettail_runtime::Scope::new(
                                binder,
                                Box::new(body.clone()),
                            );
                            terms.push(Proc::PInput(Box::new(arg1.clone()), scope));
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
impl Proc {
    /// Generate a random term at exactly the given depth
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `depth` - Target depth (operator nesting level)
    ///
    /// # Example
    /// ```ignore
    /// let term = Proc::generate_random_at_depth(&["a".into(), "b".into()], 25);
    /// ```
    pub fn generate_random_at_depth(vars: &[String], depth: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::generate_random_at_depth_internal(vars, depth, &mut rng, 0)
    }
    /// Generate a random term at exactly the given depth with a seed
    ///
    /// This is deterministic - same seed produces same term.
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `depth` - Target depth (operator nesting level)
    /// * `seed` - Random seed for reproducibility
    pub fn generate_random_at_depth_with_seed(
        vars: &[String],
        depth: usize,
        seed: u64,
    ) -> Self {
        use rand::{SeedableRng, Rng};
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        Self::generate_random_at_depth_internal(vars, depth, &mut rng, 0)
    }
    fn generate_random_at_depth_internal<R: rand::Rng>(
        vars: &[String],
        depth: usize,
        rng: &mut R,
        binding_depth: usize,
    ) -> Self {
        if depth == 0 {
            Proc::PZero
        } else {
            {
                let choice = rng.gen_range(0..4usize);
                match choice {
                    0usize => {
                        let arg = Name::generate_random_at_depth_internal(
                            vars,
                            depth - 1,
                            rng,
                            binding_depth,
                        );
                        Proc::PDrop(Box::new(arg))
                    }
                    1usize => {
                        let d1 = rng.gen_range(0..depth);
                        let d2 = if d1 == depth - 1 {
                            rng.gen_range(0..depth)
                        } else {
                            depth - 1
                        };
                        let arg1 = Name::generate_random_at_depth_internal(
                            vars,
                            d1,
                            rng,
                            binding_depth,
                        );
                        let arg2 = Proc::generate_random_at_depth_internal(
                            vars,
                            d2,
                            rng,
                            binding_depth,
                        );
                        Proc::POutput(Box::new(arg1), Box::new(arg2))
                    }
                    2usize => {
                        let d1 = rng.gen_range(0..depth);
                        let d2 = if d1 == depth - 1 {
                            rng.gen_range(0..depth)
                        } else {
                            depth - 1
                        };
                        let arg1 = Name::generate_random_at_depth_internal(
                            vars,
                            d1,
                            rng,
                            binding_depth,
                        );
                        let binder_name = ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("x{0}", binding_depth))
                        });
                        let mut extended_vars = vars.to_vec();
                        extended_vars.push(binder_name.clone());
                        let body = Proc::generate_random_at_depth_internal(
                            &extended_vars,
                            d2,
                            rng,
                            binding_depth + 1,
                        );
                        let binder_var = mettail_runtime::get_or_create_var(
                            &binder_name,
                        );
                        let binder = mettail_runtime::Binder(binder_var);
                        let scope = mettail_runtime::Scope::new(binder, Box::new(body));
                        Proc::PInput(Box::new(arg1), scope)
                    }
                    3usize => {
                        let d1 = rng.gen_range(0..depth);
                        let d2 = if d1 == depth - 1 {
                            rng.gen_range(0..depth)
                        } else {
                            depth - 1
                        };
                        let arg1 = Proc::generate_random_at_depth_internal(
                            vars,
                            d1,
                            rng,
                            binding_depth,
                        );
                        let arg2 = Proc::generate_random_at_depth_internal(
                            vars,
                            d2,
                            rng,
                            binding_depth,
                        );
                        Proc::PPar(Box::new(arg1), Box::new(arg2))
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
    }
}
impl Name {
    /// Generate a random term at exactly the given depth
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `depth` - Target depth (operator nesting level)
    ///
    /// # Example
    /// ```ignore
    /// let term = Proc::generate_random_at_depth(&["a".into(), "b".into()], 25);
    /// ```
    pub fn generate_random_at_depth(vars: &[String], depth: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::generate_random_at_depth_internal(vars, depth, &mut rng, 0)
    }
    /// Generate a random term at exactly the given depth with a seed
    ///
    /// This is deterministic - same seed produces same term.
    ///
    /// # Arguments
    /// * `vars` - Pool of variable names for free variables
    /// * `depth` - Target depth (operator nesting level)
    /// * `seed` - Random seed for reproducibility
    pub fn generate_random_at_depth_with_seed(
        vars: &[String],
        depth: usize,
        seed: u64,
    ) -> Self {
        use rand::{SeedableRng, Rng};
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        Self::generate_random_at_depth_internal(vars, depth, &mut rng, 0)
    }
    fn generate_random_at_depth_internal<R: rand::Rng>(
        vars: &[String],
        depth: usize,
        rng: &mut R,
        binding_depth: usize,
    ) -> Self {
        if depth == 0 {
            if !vars.is_empty() {
                let idx = rng.gen_range(0..vars.len());
                Name::NVar(
                    mettail_runtime::OrdVar(
                        mettail_runtime::Var::Free(
                            mettail_runtime::get_or_create_var(&vars[idx]),
                        ),
                    ),
                )
            } else {
                Name::NVar(
                    mettail_runtime::OrdVar(
                        mettail_runtime::Var::Free(
                            mettail_runtime::get_or_create_var("_"),
                        ),
                    ),
                )
            }
        } else {
            {
                let choice = rng.gen_range(0..1usize);
                match choice {
                    0usize => {
                        let arg = Proc::generate_random_at_depth_internal(
                            vars,
                            depth - 1,
                            rng,
                            binding_depth,
                        );
                        Name::NQuote(Box::new(arg))
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
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
pub use ascent_source_rhocalc_source as rhocalc_source;
fn main() {
    let start_time = Instant::now();
    let vars = <[_]>::into_vec(
        ::alloc::boxed::box_new(["a".to_string(), "b".to_string()]),
    );
    let redex = Proc::generate_random_at_depth(&vars, 6);
    {
        ::std::io::_print(format_args!("Initial: {0}\n", redex.clone()));
    };
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
logical indices: eq_name_indices_0; eq_name_indices_0_1*/
                pub eq_name: ::ascent_byods_rels::fake_vec::FakeVec<(Name, Name)>,
                __eq_name_ind_common: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                    Name,
                >,
                eq_name_indices_0: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<Name>,
                eq_name_indices_0_1: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<Name>,
                /**
logical indices: eq_proc_indices_0; eq_proc_indices_0_1*/
                pub eq_proc: ::ascent_byods_rels::fake_vec::FakeVec<(Proc, Proc)>,
                __eq_proc_ind_common: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                    Proc,
                >,
                eq_proc_indices_0: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<Proc>,
                eq_proc_indices_0_1: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<Proc>,
                /**
logical indices: is_normal_form_indices_0; is_normal_form_indices_none*/
                pub is_normal_form: ::std::vec::Vec<(Proc,)>,
                __is_normal_form_ind_common: (),
                is_normal_form_indices_0: ascent::internal::RelFullIndexType<
                    (Proc,),
                    (),
                >,
                is_normal_form_indices_none: ascent::rel::ToRelIndexType<(), (Proc,)>,
                /**
logical indices: name_indices_0; name_indices_none*/
                pub name: ::std::vec::Vec<(Name,)>,
                __name_ind_common: (),
                name_indices_0: ascent::internal::RelFullIndexType<(Name,), ()>,
                name_indices_none: ascent::rel::ToRelIndexType<(), (Name,)>,
                /**
logical indices: path_indices_0; path_indices_0_1*/
                pub path: ::std::vec::Vec<(Proc, Proc)>,
                __path_ind_common: (),
                path_indices_0: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                path_indices_0_1: ascent::internal::RelFullIndexType<(Proc, Proc), ()>,
                /**
logical indices: path_full_indices_0_1*/
                pub path_full: ::std::vec::Vec<(Proc, Proc)>,
                __path_full_ind_common: (),
                path_full_indices_0_1: ascent::internal::RelFullIndexType<
                    (Proc, Proc),
                    (),
                >,
                /**
logical indices: proc_indices_0; proc_indices_none*/
                pub proc: ::std::vec::Vec<(Proc,)>,
                __proc_ind_common: (),
                proc_indices_0: ascent::internal::RelFullIndexType<(Proc,), ()>,
                proc_indices_none: ascent::rel::ToRelIndexType<(), (Proc,)>,
                /**
logical indices: redex_eq_indices_0; redex_eq_indices_none*/
                pub redex_eq: ::std::vec::Vec<(Proc,)>,
                __redex_eq_ind_common: (),
                redex_eq_indices_0: ascent::internal::RelFullIndexType<(Proc,), ()>,
                redex_eq_indices_none: ascent::rel::ToRelIndexType<(), (Proc,)>,
                /**
logical indices: rw_name_indices_0; rw_name_indices_0_1; rw_name_indices_1*/
                pub rw_name: ::std::vec::Vec<(Name, Name)>,
                __rw_name_ind_common: (),
                rw_name_indices_0: ascent::rel::ToRelIndexType<(Name,), (Name,)>,
                rw_name_indices_0_1: ascent::internal::RelFullIndexType<
                    (Name, Name),
                    (),
                >,
                rw_name_indices_1: ascent::rel::ToRelIndexType<(Name,), (Name,)>,
                /**
logical indices: rw_proc_indices_0; rw_proc_indices_0_1; rw_proc_indices_1; rw_proc_indices_none*/
                pub rw_proc: ::std::vec::Vec<(Proc, Proc)>,
                __rw_proc_ind_common: (),
                rw_proc_indices_0: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                rw_proc_indices_0_1: ascent::internal::RelFullIndexType<
                    (Proc, Proc),
                    (),
                >,
                rw_proc_indices_1: ascent::rel::ToRelIndexType<(Proc,), (Proc,)>,
                rw_proc_indices_none: ascent::rel::ToRelIndexType<(), (Proc, Proc)>,
                scc_times: [std::time::Duration; 8usize],
                scc_iters: [usize; 8usize],
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
                    for (_i, tuple) in self.eq_name.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.eq_name_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__eq_name_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.eq_name_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__eq_name_ind_common),
                            selection_tuple,
                            (),
                        );
                    }
                    for (_i, tuple) in self.eq_proc.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.eq_proc_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__eq_proc_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.eq_proc_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__eq_proc_ind_common),
                            selection_tuple,
                            (),
                        );
                    }
                    for (_i, tuple) in self.is_normal_form.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.is_normal_form_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__is_normal_form_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.is_normal_form_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__is_normal_form_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                    }
                    for (_i, tuple) in self.name.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.name_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__name_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.name_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind.to_rel_index_write(&mut self.__name_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
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
                    }
                    for (_i, tuple) in self.path_full.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.path_full_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__path_full_ind_common),
                            selection_tuple,
                            (),
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
                    for (_i, tuple) in self.redex_eq.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.redex_eq_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__redex_eq_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.redex_eq_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__redex_eq_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                    }
                    for (_i, tuple) in self.rw_name.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.rw_name_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_name_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.rw_name_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_name_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = (tuple.1.clone(),);
                        let rel_ind = &mut self.rw_name_indices_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_name_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                    }
                    for (_i, tuple) in self.rw_proc.iter().enumerate() {
                        let selection_tuple = (tuple.0.clone(),);
                        let rel_ind = &mut self.rw_proc_indices_0;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_proc_ind_common),
                            selection_tuple,
                            (tuple.1.clone(),),
                        );
                        let selection_tuple = (tuple.0.clone(), tuple.1.clone());
                        let rel_ind = &mut self.rw_proc_indices_0_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_proc_ind_common),
                            selection_tuple,
                            (),
                        );
                        let selection_tuple = (tuple.1.clone(),);
                        let rel_ind = &mut self.rw_proc_indices_1;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_proc_ind_common),
                            selection_tuple,
                            (tuple.0.clone(),),
                        );
                        let selection_tuple = ();
                        let rel_ind = &mut self.rw_proc_indices_none;
                        ascent::internal::RelIndexWrite::index_insert(
                            &mut rel_ind
                                .to_rel_index_write(&mut self.__rw_proc_ind_common),
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
                    let _type_constraints: ascent::internal::TypeConstraints<Name>;
                    let _type_constraints: ascent::internal::TypeConstraints<Proc>;
                }
                pub fn summary(&self) -> &'static str {
                    "scc 0, is_looping: false:\n  proc <-- for_p\n  dynamic relations: proc\nscc 1, is_looping: true:\n  proc <-- proc_indices_0_delta, rw_proc_indices_0_total+delta [SIMPLE JOIN]\n  proc <-- proc_indices_0_total, rw_proc_indices_0_delta [SIMPLE JOIN]\n  name, proc <-- proc_indices_none_delta, if let ⋯\n  name, proc <-- proc_indices_none_delta, if let ⋯, let ⋯\n  proc, proc <-- proc_indices_none_delta, if let ⋯\n  proc <-- name_indices_none_delta, if let ⋯\n  proc <-- redex_eq_indices_none_delta\n  rw_proc <-- rw_proc_indices_0_delta, eq_proc_indices_0_total+delta [SIMPLE JOIN]\n  rw_proc <-- rw_proc_indices_0_total, eq_proc_indices_0_delta [SIMPLE JOIN]\n  rw_proc <-- rw_proc_indices_1_delta, eq_proc_indices_0_total+delta [SIMPLE JOIN]\n  rw_proc <-- rw_proc_indices_1_total, eq_proc_indices_0_delta [SIMPLE JOIN]\n  rw_proc <-- proc_indices_none_delta, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, eq_name_indices_0_1_total+delta, if ⋯, let ⋯\n  rw_proc <-- proc_indices_none_total, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, eq_name_indices_0_1_delta, if ⋯, let ⋯\n  rw_proc <-- proc_indices_none_delta, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯\n  rw_proc <-- proc_indices_none_delta, if let ⋯, rw_proc_indices_0_total+delta, let ⋯\n  rw_proc <-- proc_indices_none_total, if let ⋯, rw_proc_indices_0_delta, let ⋯\n  name <-- proc_indices_none_delta, if let ⋯\n  name <-- name_indices_0_delta, rw_name_indices_0_total+delta [SIMPLE JOIN]\n  name <-- name_indices_0_total, rw_name_indices_0_delta [SIMPLE JOIN]\n  rw_name <-- rw_name_indices_0_delta, eq_name_indices_0_total+delta [SIMPLE JOIN]\n  rw_name <-- rw_name_indices_0_total, eq_name_indices_0_delta [SIMPLE JOIN]\n  rw_name <-- rw_name_indices_1_delta, eq_name_indices_0_total+delta [SIMPLE JOIN]\n  rw_name <-- rw_name_indices_1_total, eq_name_indices_0_delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_delta\n  eq_name <-- name_indices_none_delta\n  eq_proc <-- name_indices_none_delta, name_indices_none_total+delta, eq_name_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_delta, eq_name_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_delta [SIMPLE JOIN]\n  eq_name <-- proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_name <-- proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_name <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]\n  eq_name <-- name_indices_none_delta, if let ⋯, if let ⋯, let ⋯\n  eq_proc <-- name_indices_none_delta, name_indices_none_total+delta, eq_name_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_delta, eq_name_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]\n  eq_proc <-- proc_indices_none_delta, if let ⋯, let ⋯\n  eq_proc <-- proc_indices_none_delta, if let ⋯, if let ⋯, let ⋯\n  redex_eq <-- eq_proc_indices_0_delta\n  dynamic relations: eq_name, eq_proc, name, proc, redex_eq, rw_name, rw_proc\nscc 2, is_looping: false:\n  path <-- redex_eq_indices_none_total\n  dynamic relations: path\nscc 3, is_looping: false:\n  path <-- rw_proc_indices_none_total\n  dynamic relations: path\nscc 4, is_looping: false:\n  path <-- for_\n  dynamic relations: path\nscc 5, is_looping: true:\n  path <-- rw_proc_indices_none_total, path_indices_0_delta\n  dynamic relations: path\nscc 6, is_looping: false:\n  is_normal_form <-- proc_indices_none_total, agg rw_proc_indices_0\n  dynamic relations: is_normal_form\nscc 7, is_looping: false:\n  path_full <-- is_normal_form_indices_none_total, path_indices_0_1_total\n  dynamic relations: path_full\n"
                }
                pub fn relation_sizes_summary(&self) -> String {
                    #![allow(clippy::all)]
                    use std::fmt::Write;
                    let mut res = String::new();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "eq_name",
                                self.eq_name.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "eq_proc",
                                self.eq_proc.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "is_normal_form",
                                self.is_normal_form.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!("{0} size: {1}\n", "name", self.name.len()),
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
                                "path_full",
                                self.path_full.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!("{0} size: {1}\n", "proc", self.proc.len()),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "redex_eq",
                                self.redex_eq.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "rw_name",
                                self.rw_name.len(),
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "{0} size: {1}\n",
                                "rw_proc",
                                self.rw_proc.len(),
                            ),
                        )
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
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "6",
                                self.scc_iters[6usize],
                                self.scc_times[6usize],
                            ),
                        )
                        .unwrap();
                    (&mut res)
                        .write_fmt(
                            format_args!(
                                "scc {0}: iterations: {1}, time: {2:?}\n",
                                "7",
                                self.scc_iters[7usize],
                                self.scc_times[7usize],
                            ),
                        )
                        .unwrap();
                    res
                }
            }
            impl Default for AscentProgram {
                fn default() -> Self {
                    let mut _self = AscentProgram {
                        eq_name: Default::default(),
                        __eq_name_ind_common: Default::default(),
                        eq_name_indices_0: Default::default(),
                        eq_name_indices_0_1: Default::default(),
                        eq_proc: Default::default(),
                        __eq_proc_ind_common: Default::default(),
                        eq_proc_indices_0: Default::default(),
                        eq_proc_indices_0_1: Default::default(),
                        is_normal_form: Default::default(),
                        __is_normal_form_ind_common: Default::default(),
                        is_normal_form_indices_0: Default::default(),
                        is_normal_form_indices_none: Default::default(),
                        name: Default::default(),
                        __name_ind_common: Default::default(),
                        name_indices_0: Default::default(),
                        name_indices_none: Default::default(),
                        path: Default::default(),
                        __path_ind_common: Default::default(),
                        path_indices_0: Default::default(),
                        path_indices_0_1: Default::default(),
                        path_full: Default::default(),
                        __path_full_ind_common: Default::default(),
                        path_full_indices_0_1: Default::default(),
                        proc: Default::default(),
                        __proc_ind_common: Default::default(),
                        proc_indices_0: Default::default(),
                        proc_indices_none: Default::default(),
                        redex_eq: Default::default(),
                        __redex_eq_ind_common: Default::default(),
                        redex_eq_indices_0: Default::default(),
                        redex_eq_indices_none: Default::default(),
                        rw_name: Default::default(),
                        __rw_name_ind_common: Default::default(),
                        rw_name_indices_0: Default::default(),
                        rw_name_indices_0_1: Default::default(),
                        rw_name_indices_1: Default::default(),
                        rw_proc: Default::default(),
                        __rw_proc_ind_common: Default::default(),
                        rw_proc_indices_0: Default::default(),
                        rw_proc_indices_0_1: Default::default(),
                        rw_proc_indices_1: Default::default(),
                        rw_proc_indices_none: Default::default(),
                        scc_times: [std::time::Duration::ZERO; 8usize],
                        scc_iters: [0; 8usize],
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
                    let mut __eq_name_ind_common_delta: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Name,
                    > = ::std::mem::take(&mut _self.__eq_name_ind_common);
                    let mut __eq_name_ind_common_total: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Name,
                    > = Default::default();
                    let mut __eq_name_ind_common_new: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Name,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __eq_name_ind_common_new,
                        &mut __eq_name_ind_common_delta,
                        &mut __eq_name_ind_common_total,
                    );
                    let mut eq_name_indices_0_delta: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Name,
                    > = ::std::mem::take(&mut _self.eq_name_indices_0);
                    let mut eq_name_indices_0_total: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Name,
                    > = Default::default();
                    let mut eq_name_indices_0_new: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Name,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_name_indices_0_new
                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                        &mut eq_name_indices_0_delta
                            .to_rel_index_write(&mut __eq_name_ind_common_delta),
                        &mut eq_name_indices_0_total
                            .to_rel_index_write(&mut __eq_name_ind_common_total),
                    );
                    let mut eq_name_indices_0_1_delta: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Name,
                    > = ::std::mem::take(&mut _self.eq_name_indices_0_1);
                    let mut eq_name_indices_0_1_total: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Name,
                    > = Default::default();
                    let mut eq_name_indices_0_1_new: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Name,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_name_indices_0_1_new
                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                        &mut eq_name_indices_0_1_delta
                            .to_rel_index_write(&mut __eq_name_ind_common_delta),
                        &mut eq_name_indices_0_1_total
                            .to_rel_index_write(&mut __eq_name_ind_common_total),
                    );
                    let mut __eq_proc_ind_common_delta: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Proc,
                    > = ::std::mem::take(&mut _self.__eq_proc_ind_common);
                    let mut __eq_proc_ind_common_total: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Proc,
                    > = Default::default();
                    let mut __eq_proc_ind_common_new: ::ascent_byods_rels::eqrel_ind::EqRelIndCommon<
                        Proc,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __eq_proc_ind_common_new,
                        &mut __eq_proc_ind_common_delta,
                        &mut __eq_proc_ind_common_total,
                    );
                    let mut eq_proc_indices_0_delta: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Proc,
                    > = ::std::mem::take(&mut _self.eq_proc_indices_0);
                    let mut eq_proc_indices_0_total: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Proc,
                    > = Default::default();
                    let mut eq_proc_indices_0_new: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0<
                        Proc,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_proc_indices_0_new
                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                        &mut eq_proc_indices_0_delta
                            .to_rel_index_write(&mut __eq_proc_ind_common_delta),
                        &mut eq_proc_indices_0_total
                            .to_rel_index_write(&mut __eq_proc_ind_common_total),
                    );
                    let mut eq_proc_indices_0_1_delta: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Proc,
                    > = ::std::mem::take(&mut _self.eq_proc_indices_0_1);
                    let mut eq_proc_indices_0_1_total: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Proc,
                    > = Default::default();
                    let mut eq_proc_indices_0_1_new: ::ascent_byods_rels::eqrel_ind::ToEqRelInd0_1<
                        Proc,
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut eq_proc_indices_0_1_new
                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                        &mut eq_proc_indices_0_1_delta
                            .to_rel_index_write(&mut __eq_proc_ind_common_delta),
                        &mut eq_proc_indices_0_1_total
                            .to_rel_index_write(&mut __eq_proc_ind_common_total),
                    );
                    let mut __name_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__name_ind_common,
                    );
                    let mut __name_ind_common_total: () = Default::default();
                    let mut __name_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __name_ind_common_new,
                        &mut __name_ind_common_delta,
                        &mut __name_ind_common_total,
                    );
                    let mut name_indices_0_delta: ascent::internal::RelFullIndexType<
                        (Name,),
                        (),
                    > = ::std::mem::take(&mut _self.name_indices_0);
                    let mut name_indices_0_total: ascent::internal::RelFullIndexType<
                        (Name,),
                        (),
                    > = Default::default();
                    let mut name_indices_0_new: ascent::internal::RelFullIndexType<
                        (Name,),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut name_indices_0_new
                            .to_rel_index_write(&mut __name_ind_common_new),
                        &mut name_indices_0_delta
                            .to_rel_index_write(&mut __name_ind_common_delta),
                        &mut name_indices_0_total
                            .to_rel_index_write(&mut __name_ind_common_total),
                    );
                    let mut name_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Name,),
                    > = ::std::mem::take(&mut _self.name_indices_none);
                    let mut name_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Name,),
                    > = Default::default();
                    let mut name_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Name,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut name_indices_none_new
                            .to_rel_index_write(&mut __name_ind_common_new),
                        &mut name_indices_none_delta
                            .to_rel_index_write(&mut __name_ind_common_delta),
                        &mut name_indices_none_total
                            .to_rel_index_write(&mut __name_ind_common_total),
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
                    let mut __redex_eq_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__redex_eq_ind_common,
                    );
                    let mut __redex_eq_ind_common_total: () = Default::default();
                    let mut __redex_eq_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __redex_eq_ind_common_new,
                        &mut __redex_eq_ind_common_delta,
                        &mut __redex_eq_ind_common_total,
                    );
                    let mut redex_eq_indices_0_delta: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = ::std::mem::take(&mut _self.redex_eq_indices_0);
                    let mut redex_eq_indices_0_total: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    let mut redex_eq_indices_0_new: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut redex_eq_indices_0_new
                            .to_rel_index_write(&mut __redex_eq_ind_common_new),
                        &mut redex_eq_indices_0_delta
                            .to_rel_index_write(&mut __redex_eq_ind_common_delta),
                        &mut redex_eq_indices_0_total
                            .to_rel_index_write(&mut __redex_eq_ind_common_total),
                    );
                    let mut redex_eq_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.redex_eq_indices_none);
                    let mut redex_eq_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    let mut redex_eq_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut redex_eq_indices_none_new
                            .to_rel_index_write(&mut __redex_eq_ind_common_new),
                        &mut redex_eq_indices_none_delta
                            .to_rel_index_write(&mut __redex_eq_ind_common_delta),
                        &mut redex_eq_indices_none_total
                            .to_rel_index_write(&mut __redex_eq_ind_common_total),
                    );
                    let mut __rw_name_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__rw_name_ind_common,
                    );
                    let mut __rw_name_ind_common_total: () = Default::default();
                    let mut __rw_name_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __rw_name_ind_common_new,
                        &mut __rw_name_ind_common_delta,
                        &mut __rw_name_ind_common_total,
                    );
                    let mut rw_name_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = ::std::mem::take(&mut _self.rw_name_indices_0);
                    let mut rw_name_indices_0_total: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = Default::default();
                    let mut rw_name_indices_0_new: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_name_indices_0_new
                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                        &mut rw_name_indices_0_delta
                            .to_rel_index_write(&mut __rw_name_ind_common_delta),
                        &mut rw_name_indices_0_total
                            .to_rel_index_write(&mut __rw_name_ind_common_total),
                    );
                    let mut rw_name_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Name, Name),
                        (),
                    > = ::std::mem::take(&mut _self.rw_name_indices_0_1);
                    let mut rw_name_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Name, Name),
                        (),
                    > = Default::default();
                    let mut rw_name_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Name, Name),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_name_indices_0_1_new
                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                        &mut rw_name_indices_0_1_delta
                            .to_rel_index_write(&mut __rw_name_ind_common_delta),
                        &mut rw_name_indices_0_1_total
                            .to_rel_index_write(&mut __rw_name_ind_common_total),
                    );
                    let mut rw_name_indices_1_delta: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = ::std::mem::take(&mut _self.rw_name_indices_1);
                    let mut rw_name_indices_1_total: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = Default::default();
                    let mut rw_name_indices_1_new: ascent::rel::ToRelIndexType<
                        (Name,),
                        (Name,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_name_indices_1_new
                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                        &mut rw_name_indices_1_delta
                            .to_rel_index_write(&mut __rw_name_ind_common_delta),
                        &mut rw_name_indices_1_total
                            .to_rel_index_write(&mut __rw_name_ind_common_total),
                    );
                    let mut __rw_proc_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__rw_proc_ind_common,
                    );
                    let mut __rw_proc_ind_common_total: () = Default::default();
                    let mut __rw_proc_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __rw_proc_ind_common_new,
                        &mut __rw_proc_ind_common_delta,
                        &mut __rw_proc_ind_common_total,
                    );
                    let mut rw_proc_indices_0_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.rw_proc_indices_0);
                    let mut rw_proc_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut rw_proc_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_proc_indices_0_new
                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                        &mut rw_proc_indices_0_delta
                            .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                        &mut rw_proc_indices_0_total
                            .to_rel_index_write(&mut __rw_proc_ind_common_total),
                    );
                    let mut rw_proc_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.rw_proc_indices_0_1);
                    let mut rw_proc_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut rw_proc_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_proc_indices_0_1_new
                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                        &mut rw_proc_indices_0_1_delta
                            .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                        &mut rw_proc_indices_0_1_total
                            .to_rel_index_write(&mut __rw_proc_ind_common_total),
                    );
                    let mut rw_proc_indices_1_delta: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.rw_proc_indices_1);
                    let mut rw_proc_indices_1_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut rw_proc_indices_1_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_proc_indices_1_new
                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                        &mut rw_proc_indices_1_delta
                            .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                        &mut rw_proc_indices_1_total
                            .to_rel_index_write(&mut __rw_proc_ind_common_total),
                    );
                    let mut rw_proc_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = ::std::mem::take(&mut _self.rw_proc_indices_none);
                    let mut rw_proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    let mut rw_proc_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut rw_proc_indices_none_new
                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                        &mut rw_proc_indices_none_delta
                            .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                        &mut rw_proc_indices_none_total
                            .to_rel_index_write(&mut __rw_proc_ind_common_total),
                    );
                    #[allow(unused_assignments, unused_variables)]
                    loop {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "proc <-- proc_indices_0_delta, rw_proc_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &rw_proc_indices_0_total
                                            .to_rel_index(&__rw_proc_ind_common_total),
                                        &rw_proc_indices_0_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                proc_indices_0_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &rw_proc_indices_0_total
                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                &rw_proc_indices_0_delta
                                                    .to_rel_index(&__rw_proc_ind_common_delta),
                                            )
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let c1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(c1),
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
                                        &rw_proc_indices_0_total
                                            .to_rel_index(&__rw_proc_ind_common_total),
                                        &rw_proc_indices_0_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_delta
                                            .to_rel_index(&__proc_ind_common_delta)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let c1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(c1),
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
                            "proc <-- proc_indices_0_total, rw_proc_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if proc_indices_0_total
                                .to_rel_index(&__proc_ind_common_total)
                                .len_estimate()
                                <= rw_proc_indices_0_delta
                                    .to_rel_index(&__rw_proc_ind_common_delta)
                                    .len_estimate()
                            {
                                proc_indices_0_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_proc_indices_0_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let c1: &Proc = __val.0;
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(c1),
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
                                rw_proc_indices_0_delta
                                    .to_rel_index(&__rw_proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = proc_indices_0_total
                                            .to_rel_index(&__proc_ind_common_total)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let c1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Proc,) = (
                                                                ascent::internal::Convert::convert(c1),
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
                            "name, proc <-- proc_indices_none_delta, if let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Proc = __val.0;
                                        if let Proc::POutput(field_0, field_1) = t {
                                            let __new_row: (Name,) = (*field_0.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &name_indices_0_total
                                                    .to_rel_index(&__name_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &name_indices_0_delta
                                                        .to_rel_index(&__name_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut name_indices_0_new
                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.name.len();
                                                    _self.name.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut name_indices_none_new
                                                            .to_rel_index_write(&mut __name_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                            let __new_row: (Proc,) = (*field_1.clone(),);
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
                            "name, proc <-- proc_indices_none_delta, if let ⋯, let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Proc = __val.0;
                                        if let Proc::PInput(field_0, scope_field) = t {
                                            let (binder, body) = scope_field.clone().unbind();
                                            let __new_row: (Name,) = (*field_0.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &name_indices_0_total
                                                    .to_rel_index(&__name_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &name_indices_0_delta
                                                        .to_rel_index(&__name_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut name_indices_0_new
                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.name.len();
                                                    _self.name.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut name_indices_none_new
                                                            .to_rel_index_write(&mut __name_ind_common_new),
                                                        (),
                                                        (__new_row.0.clone(),),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                            let __new_row: (Proc,) = (*body.clone(),);
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
                                        let t: &Proc = __val.0;
                                        if let Proc::PPar(field_0, field_1) = t {
                                            let __new_row: (Proc,) = (*field_0.clone(),);
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
                                            let __new_row: (Proc,) = (*field_1.clone(),);
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
                            "proc <-- name_indices_none_delta, if let ⋯",
                        );
                        {
                            if let Some(__matching) = name_indices_none_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Name = __val.0;
                                        if let Name::NQuote(field_0) = t {
                                            let __new_row: (Proc,) = (*field_0.clone(),);
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
                            "proc <-- redex_eq_indices_none_delta",
                        );
                        {
                            if let Some(__matching) = redex_eq_indices_none_delta
                                .to_rel_index(&__redex_eq_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let q: &Proc = __val.0;
                                        let __new_row: (Proc,) = (
                                            ascent::internal::Convert::convert(q),
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
                            }
                        }
                        ascent::internal::comment(
                            "rw_proc <-- rw_proc_indices_0_delta, eq_proc_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_proc_indices_0_delta
                                .to_rel_index(&__rw_proc_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                rw_proc_indices_0_delta
                                    .to_rel_index(&__rw_proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_proc_indices_0_total
                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                &eq_proc_indices_0_delta
                                                    .to_rel_index(&__eq_proc_ind_common_delta),
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
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                        &eq_proc_indices_0_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_proc_indices_0_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta)
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
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- rw_proc_indices_0_total, eq_proc_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_proc_indices_0_total
                                .to_rel_index(&__rw_proc_ind_common_total)
                                .len_estimate()
                                <= eq_proc_indices_0_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_proc_indices_0_total
                                    .to_rel_index(&__rw_proc_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta)
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
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                eq_proc_indices_0_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_proc_indices_0_total
                                            .to_rel_index(&__rw_proc_ind_common_total)
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
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- rw_proc_indices_1_delta, eq_proc_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_proc_indices_1_delta
                                .to_rel_index(&__rw_proc_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                rw_proc_indices_1_delta
                                    .to_rel_index(&__rw_proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_proc_indices_0_total
                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                &eq_proc_indices_0_delta
                                                    .to_rel_index(&__eq_proc_ind_common_delta),
                                            )
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t1: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                        &eq_proc_indices_0_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_proc_indices_1_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- rw_proc_indices_1_total, eq_proc_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_proc_indices_1_total
                                .to_rel_index(&__rw_proc_ind_common_total)
                                .len_estimate()
                                <= eq_proc_indices_0_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_proc_indices_1_total
                                    .to_rel_index(&__rw_proc_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_proc_indices_0_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t1: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                eq_proc_indices_0_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_proc_indices_1_total
                                            .to_rel_index(&__rw_proc_ind_common_total)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t1: &Proc = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s: &Proc = __val.0;
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- proc_indices_none_delta, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, eq_name_indices_0_1_total+delta, if ⋯, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_1_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_1_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
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
                                            if let Proc::PPar(s_f0, s_f1) = s {
                                                let s_f0_inner = &**s_f0;
                                                if let Proc::PInput(s_f0_inner_f0, s_f0_inner_f1) = s_f0_inner {
                                                    let (binder_0, body_0) = s_f0_inner_f1.clone().unbind();
                                                    let s_f0_inner_f0_val = &**s_f0_inner_f0;
                                                    let s_f1_inner = &**s_f1;
                                                    if let Proc::POutput(s_f1_inner_f0, s_f1_inner_f1) = s_f1_inner {
                                                        let s_f1_inner_f0_val = &**s_f1_inner_f0;
                                                        let s_f1_inner_f1_val = &**s_f1_inner_f1;
                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                &eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                &eq_name_indices_0_1_delta
                                                                    .to_rel_index(&__eq_name_ind_common_delta),
                                                            )
                                                            .index_get(
                                                                &(
                                                                    (s_f0_inner_f0_val.clone()).clone(),
                                                                    (s_f1_inner_f0_val.clone()).clone(),
                                                                ),
                                                            )
                                                        {
                                                            __matching
                                                                .for_each(|__val| {
                                                                    if is_fresh(&binder_0, &s_f1_inner_f1_val.clone()) {
                                                                        let t = ((body_0.clone()).clone())
                                                                            .substitute_name(
                                                                                &binder_0.0,
                                                                                &Name::NQuote(Box::new((s_f1_inner_f1_val.clone()).clone())),
                                                                            );
                                                                        let __new_row: (Proc, Proc) = (
                                                                            ascent::internal::Convert::convert(s),
                                                                            ascent::internal::Convert::convert(t),
                                                                        );
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &rw_proc_indices_0_1_total
                                                                                .to_rel_index(&__rw_proc_ind_common_total),
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &rw_proc_indices_0_1_delta
                                                                                    .to_rel_index(&__rw_proc_ind_common_delta),
                                                                                &__new_row,
                                                                            )
                                                                        {
                                                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut rw_proc_indices_0_1_new
                                                                                    .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                &__new_row,
                                                                                (),
                                                                            ) {
                                                                                let __new_row_ind = _self.rw_proc.len();
                                                                                _self
                                                                                    .rw_proc
                                                                                    .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_0_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                    (__new_row.0.clone(),),
                                                                                    (__new_row.1.clone(),),
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_1_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                    (__new_row.1.clone(),),
                                                                                    (__new_row.0.clone(),),
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_none_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                                }
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "rw_proc <-- proc_indices_none_total, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯, eq_name_indices_0_1_delta, if ⋯, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || eq_name_indices_0_1_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
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
                                            if let Proc::PPar(s_f0, s_f1) = s {
                                                let s_f0_inner = &**s_f0;
                                                if let Proc::PInput(s_f0_inner_f0, s_f0_inner_f1) = s_f0_inner {
                                                    let (binder_0, body_0) = s_f0_inner_f1.clone().unbind();
                                                    let s_f0_inner_f0_val = &**s_f0_inner_f0;
                                                    let s_f1_inner = &**s_f1;
                                                    if let Proc::POutput(s_f1_inner_f0, s_f1_inner_f1) = s_f1_inner {
                                                        let s_f1_inner_f0_val = &**s_f1_inner_f0;
                                                        let s_f1_inner_f1_val = &**s_f1_inner_f1;
                                                        if let Some(__matching) = eq_name_indices_0_1_delta
                                                            .to_rel_index(&__eq_name_ind_common_delta)
                                                            .index_get(
                                                                &(
                                                                    (s_f0_inner_f0_val.clone()).clone(),
                                                                    (s_f1_inner_f0_val.clone()).clone(),
                                                                ),
                                                            )
                                                        {
                                                            __matching
                                                                .for_each(|__val| {
                                                                    if is_fresh(&binder_0, &s_f1_inner_f1_val.clone()) {
                                                                        let t = ((body_0.clone()).clone())
                                                                            .substitute_name(
                                                                                &binder_0.0,
                                                                                &Name::NQuote(Box::new((s_f1_inner_f1_val.clone()).clone())),
                                                                            );
                                                                        let __new_row: (Proc, Proc) = (
                                                                            ascent::internal::Convert::convert(s),
                                                                            ascent::internal::Convert::convert(t),
                                                                        );
                                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                            &rw_proc_indices_0_1_total
                                                                                .to_rel_index(&__rw_proc_ind_common_total),
                                                                            &__new_row,
                                                                        )
                                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &rw_proc_indices_0_1_delta
                                                                                    .to_rel_index(&__rw_proc_ind_common_delta),
                                                                                &__new_row,
                                                                            )
                                                                        {
                                                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                &mut rw_proc_indices_0_1_new
                                                                                    .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                &__new_row,
                                                                                (),
                                                                            ) {
                                                                                let __new_row_ind = _self.rw_proc.len();
                                                                                _self
                                                                                    .rw_proc
                                                                                    .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_0_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                    (__new_row.0.clone(),),
                                                                                    (__new_row.1.clone(),),
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_1_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                                    (__new_row.1.clone(),),
                                                                                    (__new_row.0.clone(),),
                                                                                );
                                                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                                                    &mut rw_proc_indices_none_new
                                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                                                }
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "rw_proc <-- proc_indices_none_delta, if let ⋯, let ⋯, if let ⋯, let ⋯, let ⋯",
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
                                        if let Proc::PDrop(s_f0) = s {
                                            let s_f0_inner = &**s_f0;
                                            if let Name::NQuote(s_f0_inner_f0) = s_f0_inner {
                                                let s_f0_inner_f0_val = &**s_f0_inner_f0;
                                                let t = (s_f0_inner_f0_val.clone()).clone();
                                                let __new_row: (Proc, Proc) = (
                                                    ascent::internal::Convert::convert(s),
                                                    ascent::internal::Convert::convert(t),
                                                );
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &rw_proc_indices_0_1_total
                                                        .to_rel_index(&__rw_proc_ind_common_total),
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &rw_proc_indices_0_1_delta
                                                            .to_rel_index(&__rw_proc_ind_common_delta),
                                                        &__new_row,
                                                    )
                                                {
                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut rw_proc_indices_0_1_new
                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                        &__new_row,
                                                        (),
                                                    ) {
                                                        let __new_row_ind = _self.rw_proc.len();
                                                        _self
                                                            .rw_proc
                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_proc_indices_0_new
                                                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                            (__new_row.0.clone(),),
                                                            (__new_row.1.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_proc_indices_1_new
                                                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                            (__new_row.1.clone(),),
                                                            (__new_row.0.clone(),),
                                                        );
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut rw_proc_indices_none_new
                                                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- proc_indices_none_delta, if let ⋯, rw_proc_indices_0_total+delta, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &rw_proc_indices_0_total
                                            .to_rel_index(&__rw_proc_ind_common_total),
                                        &rw_proc_indices_0_delta
                                            .to_rel_index(&__rw_proc_ind_common_delta),
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
                                            if let Proc::PPar(p, s0) = s {
                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                        &rw_proc_indices_0_total
                                                            .to_rel_index(&__rw_proc_ind_common_total),
                                                        &rw_proc_indices_0_delta
                                                            .to_rel_index(&__rw_proc_ind_common_delta),
                                                    )
                                                    .index_get(&((**s0).clone(),))
                                                {
                                                    __matching
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t0: &Proc = __val.0;
                                                            let t = Proc::PPar(p.clone(), Box::new(t0.clone()));
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "rw_proc <-- proc_indices_none_total, if let ⋯, rw_proc_indices_0_delta, let ⋯",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || rw_proc_indices_0_delta
                                    .to_rel_index(&__rw_proc_ind_common_delta)
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
                                            if let Proc::PPar(p, s0) = s {
                                                if let Some(__matching) = rw_proc_indices_0_delta
                                                    .to_rel_index(&__rw_proc_ind_common_delta)
                                                    .index_get(&((**s0).clone(),))
                                                {
                                                    __matching
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t0: &Proc = __val.0;
                                                            let t = Proc::PPar(p.clone(), Box::new(t0.clone()));
                                                            let __new_row: (Proc, Proc) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_proc_indices_0_1_total
                                                                    .to_rel_index(&__rw_proc_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_proc_indices_0_1_delta
                                                                        .to_rel_index(&__rw_proc_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_proc_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_proc.len();
                                                                    _self
                                                                        .rw_proc
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
                                                                        (__new_row.1.clone(),),
                                                                        (__new_row.0.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_proc_indices_none_new
                                                                            .to_rel_index_write(&mut __rw_proc_ind_common_new),
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
                            "name <-- proc_indices_none_delta, if let ⋯",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Proc = __val.0;
                                        if let Proc::PDrop(field_0) = t {
                                            let __new_row: (Name,) = (*field_0.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &name_indices_0_total
                                                    .to_rel_index(&__name_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &name_indices_0_delta
                                                        .to_rel_index(&__name_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut name_indices_0_new
                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.name.len();
                                                    _self.name.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut name_indices_none_new
                                                            .to_rel_index_write(&mut __name_ind_common_new),
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
                            "name <-- name_indices_0_delta, rw_name_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if name_indices_0_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &rw_name_indices_0_total
                                            .to_rel_index(&__rw_name_ind_common_total),
                                        &rw_name_indices_0_delta
                                            .to_rel_index(&__rw_name_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                name_indices_0_delta
                                    .to_rel_index(&__name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &rw_name_indices_0_total
                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                &rw_name_indices_0_delta
                                                    .to_rel_index(&__rw_name_ind_common_delta),
                                            )
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let c1: &Name = __val.0;
                                                            let __new_row: (Name,) = (
                                                                ascent::internal::Convert::convert(c1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &name_indices_0_total
                                                                    .to_rel_index(&__name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &name_indices_0_delta
                                                                        .to_rel_index(&__name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut name_indices_0_new
                                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.name.len();
                                                                    _self.name.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut name_indices_none_new
                                                                            .to_rel_index_write(&mut __name_ind_common_new),
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
                                        &rw_name_indices_0_total
                                            .to_rel_index(&__rw_name_ind_common_total),
                                        &rw_name_indices_0_delta
                                            .to_rel_index(&__rw_name_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = name_indices_0_delta
                                            .to_rel_index(&__name_ind_common_delta)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let c1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Name,) = (
                                                                ascent::internal::Convert::convert(c1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &name_indices_0_total
                                                                    .to_rel_index(&__name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &name_indices_0_delta
                                                                        .to_rel_index(&__name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut name_indices_0_new
                                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.name.len();
                                                                    _self.name.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut name_indices_none_new
                                                                            .to_rel_index_write(&mut __name_ind_common_new),
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
                            "name <-- name_indices_0_total, rw_name_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if name_indices_0_total
                                .to_rel_index(&__name_ind_common_total)
                                .len_estimate()
                                <= rw_name_indices_0_delta
                                    .to_rel_index(&__rw_name_ind_common_delta)
                                    .len_estimate()
                            {
                                name_indices_0_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_name_indices_0_delta
                                            .to_rel_index(&__rw_name_ind_common_delta)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let c1: &Name = __val.0;
                                                            let __new_row: (Name,) = (
                                                                ascent::internal::Convert::convert(c1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &name_indices_0_total
                                                                    .to_rel_index(&__name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &name_indices_0_delta
                                                                        .to_rel_index(&__name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut name_indices_0_new
                                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.name.len();
                                                                    _self.name.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut name_indices_none_new
                                                                            .to_rel_index_write(&mut __name_ind_common_new),
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
                                rw_name_indices_0_delta
                                    .to_rel_index(&__rw_name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let c0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = name_indices_0_total
                                            .to_rel_index(&__name_ind_common_total)
                                            .index_get(&(c0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let c1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __new_row: (Name,) = (
                                                                ascent::internal::Convert::convert(c1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &name_indices_0_total
                                                                    .to_rel_index(&__name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &name_indices_0_delta
                                                                        .to_rel_index(&__name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut name_indices_0_new
                                                                        .to_rel_index_write(&mut __name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.name.len();
                                                                    _self.name.push((__new_row.0.clone(),));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut name_indices_none_new
                                                                            .to_rel_index_write(&mut __name_ind_common_new),
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
                            "rw_name <-- rw_name_indices_0_delta, eq_name_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_name_indices_0_delta
                                .to_rel_index(&__rw_name_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                rw_name_indices_0_delta
                                    .to_rel_index(&__rw_name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_name_indices_0_total
                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                &eq_name_indices_0_delta
                                                    .to_rel_index(&__eq_name_ind_common_delta),
                                            )
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s1: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                                        &eq_name_indices_0_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_name_indices_0_delta
                                            .to_rel_index(&__rw_name_ind_common_delta)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                            "rw_name <-- rw_name_indices_0_total, eq_name_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_name_indices_0_total
                                .to_rel_index(&__rw_name_ind_common_total)
                                .len_estimate()
                                <= eq_name_indices_0_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_name_indices_0_total
                                    .to_rel_index(&__rw_name_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s1: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                                eq_name_indices_0_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let s0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_name_indices_0_total
                                            .to_rel_index(&__rw_name_ind_common_total)
                                            .index_get(&(s0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s1),
                                                                ascent::internal::Convert::convert(t),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                            "rw_name <-- rw_name_indices_1_delta, eq_name_indices_0_total+delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_name_indices_1_delta
                                .to_rel_index(&__rw_name_ind_common_delta)
                                .len_estimate()
                                <= ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .len_estimate()
                            {
                                rw_name_indices_1_delta
                                    .to_rel_index(&__rw_name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                &eq_name_indices_0_total
                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                &eq_name_indices_0_delta
                                                    .to_rel_index(&__eq_name_ind_common_delta),
                                            )
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t1: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                                        &eq_name_indices_0_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_name_indices_1_delta
                                            .to_rel_index(&__rw_name_ind_common_delta)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                            "rw_name <-- rw_name_indices_1_total, eq_name_indices_0_delta [SIMPLE JOIN]",
                        );
                        {
                            if rw_name_indices_1_total
                                .to_rel_index(&__rw_name_ind_common_total)
                                .len_estimate()
                                <= eq_name_indices_0_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .len_estimate()
                            {
                                rw_name_indices_1_total
                                    .to_rel_index(&__rw_name_ind_common_total)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = eq_name_indices_0_delta
                                            .to_rel_index(&__eq_name_ind_common_delta)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let s: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let t1: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                                eq_name_indices_0_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .iter_all()
                                    .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                        let __cl1_joined_columns = __cl1_joined_columns
                                            .tuple_of_borrowed();
                                        let t0 = __cl1_joined_columns.0;
                                        if let Some(__matching) = rw_name_indices_1_total
                                            .to_rel_index(&__rw_name_ind_common_total)
                                            .index_get(&(t0.clone(),))
                                        {
                                            __cl1_tuple_indices
                                                .for_each(|cl1_val| {
                                                    let cl1_val = cl1_val.tuple_of_borrowed();
                                                    let t1: &Name = cl1_val.0;
                                                    __matching
                                                        .clone()
                                                        .for_each(|__val| {
                                                            let __val = __val.tuple_of_borrowed();
                                                            let s: &Name = __val.0;
                                                            let __new_row: (Name, Name) = (
                                                                ascent::internal::Convert::convert(s),
                                                                ascent::internal::Convert::convert(t1),
                                                            );
                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &rw_name_indices_0_1_total
                                                                    .to_rel_index(&__rw_name_ind_common_total),
                                                                &__new_row,
                                                            )
                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                    &rw_name_indices_0_1_delta
                                                                        .to_rel_index(&__rw_name_ind_common_delta),
                                                                    &__new_row,
                                                                )
                                                            {
                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                    &mut rw_name_indices_0_1_new
                                                                        .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                    &__new_row,
                                                                    (),
                                                                ) {
                                                                    let __new_row_ind = _self.rw_name.len();
                                                                    _self
                                                                        .rw_name
                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_0_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.0.clone(),),
                                                                        (__new_row.1.clone(),),
                                                                    );
                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                        &mut rw_name_indices_1_new
                                                                            .to_rel_index_write(&mut __rw_name_ind_common_new),
                                                                        (__new_row.1.clone(),),
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
                        ascent::internal::comment("eq_proc <-- proc_indices_none_delta");
                        {
                            if let Some(__matching) = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Proc = __val.0;
                                        let __new_row: (Proc, Proc) = (t.clone(), t.clone());
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &eq_proc_indices_0_1_total
                                                .to_rel_index(&__eq_proc_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &eq_proc_indices_0_1_delta
                                                    .to_rel_index(&__eq_proc_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut eq_proc_indices_0_1_new
                                                    .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.eq_proc.len();
                                                _self
                                                    .eq_proc
                                                    .push((__new_row.0.clone(), __new_row.1.clone()));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_proc_indices_0_new
                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                    (__new_row.0.clone(),),
                                                    (__new_row.1.clone(),),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment("eq_name <-- name_indices_none_delta");
                        {
                            if let Some(__matching) = name_indices_none_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Name = __val.0;
                                        let __new_row: (Name, Name) = (t.clone(), t.clone());
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &eq_name_indices_0_1_total
                                                .to_rel_index(&__eq_name_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &eq_name_indices_0_1_delta
                                                    .to_rel_index(&__eq_name_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut eq_name_indices_0_1_new
                                                    .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.eq_name.len();
                                                _self
                                                    .eq_name
                                                    .push((__new_row.0.clone(), __new_row.1.clone()));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut eq_name_indices_0_new
                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                    (__new_row.0.clone(),),
                                                    (__new_row.1.clone(),),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_delta, name_indices_none_total+delta, eq_name_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &name_indices_none_total
                                            .to_rel_index(&__name_ind_common_total),
                                        &name_indices_none_delta
                                            .to_rel_index(&__name_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_1_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_1_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_delta
                                    .to_rel_index(&__name_ind_common_delta)
                                    .len_estimate()
                                    <= ascent::internal::RelIndexCombined::new(
                                            &name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total),
                                            &name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta),
                                        )
                                        .len_estimate()
                                {
                                    name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                    &name_indices_none_total
                                                        .to_rel_index(&__name_ind_common_total),
                                                    &name_indices_none_delta
                                                        .to_rel_index(&__name_ind_common_delta),
                                                )
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    ascent::internal::RelIndexCombined::new(
                                            &name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total),
                                            &name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta),
                                        )
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_delta, eq_name_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_delta
                                    .to_rel_index(&__name_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_1_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_1_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .is_empty()
                                || eq_name_indices_0_1_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_delta
                                                                    .to_rel_index(&__eq_name_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_delta
                                                                    .to_rel_index(&__eq_name_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Proc, Proc) = (
                                                                                Proc::PDrop(Box::new(x0.clone())),
                                                                                Proc::PDrop(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_proc_indices_0_1_total
                                                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_proc_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_proc_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_proc.len();
                                                                                    _self
                                                                                        .eq_proc
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_proc_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_name <-- proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .len_estimate()
                                    <= ascent::internal::RelIndexCombined::new(
                                            &proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total),
                                            &proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta),
                                        )
                                        .len_estimate()
                                {
                                    proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                    &proc_indices_none_total
                                                        .to_rel_index(&__proc_ind_common_total),
                                                    &proc_indices_none_delta
                                                        .to_rel_index(&__proc_ind_common_delta),
                                                )
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    ascent::internal::RelIndexCombined::new(
                                            &proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total),
                                            &proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta),
                                        )
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_name <-- proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_name <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            let __new_row: (Name, Name) = (
                                                                                Name::NQuote(Box::new(x0.clone())),
                                                                                Name::NQuote(Box::new(y0.clone())),
                                                                            );
                                                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                &eq_name_indices_0_1_total
                                                                                    .to_rel_index(&__eq_name_ind_common_total),
                                                                                &__new_row,
                                                                            )
                                                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                    &eq_name_indices_0_1_delta
                                                                                        .to_rel_index(&__eq_name_ind_common_delta),
                                                                                    &__new_row,
                                                                                )
                                                                            {
                                                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                    &mut eq_name_indices_0_1_new
                                                                                        .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                    &__new_row,
                                                                                    (),
                                                                                ) {
                                                                                    let __new_row_ind = _self.eq_name.len();
                                                                                    _self
                                                                                        .eq_name
                                                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                                                        &mut eq_name_indices_0_new
                                                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                                                        (__new_row.0.clone(),),
                                                                                        (__new_row.1.clone(),),
                                                                                    );
                                                                                    __changed = true;
                                                                                }
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_name <-- name_indices_none_delta, if let ⋯, if let ⋯, let ⋯",
                        );
                        {
                            if let Some(__matching) = name_indices_none_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p0: &Name = __val.0;
                                        if let Name::NQuote(field_0) = p0 {
                                            if let Proc::PDrop(n) = &**field_0 {
                                                let p1 = (**n).clone();
                                                let __new_row: (Name, Name) = (
                                                    ascent::internal::Convert::convert(p0),
                                                    ascent::internal::Convert::convert(p1),
                                                );
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &eq_name_indices_0_1_total
                                                        .to_rel_index(&__eq_name_ind_common_total),
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &eq_name_indices_0_1_delta
                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                        &__new_row,
                                                    )
                                                {
                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut eq_name_indices_0_1_new
                                                            .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                        &__new_row,
                                                        (),
                                                    ) {
                                                        let __new_row_ind = _self.eq_name.len();
                                                        _self
                                                            .eq_name
                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut eq_name_indices_0_new
                                                                .to_rel_index_write(&mut __eq_name_ind_common_new),
                                                            (__new_row.0.clone(),),
                                                            (__new_row.1.clone(),),
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
                            "eq_proc <-- name_indices_none_delta, name_indices_none_total+delta, eq_name_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_delta
                                .to_rel_index(&__name_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &name_indices_none_total
                                            .to_rel_index(&__name_ind_common_total),
                                        &name_indices_none_delta
                                            .to_rel_index(&__name_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_1_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_1_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_delta
                                    .to_rel_index(&__name_ind_common_delta)
                                    .len_estimate()
                                    <= ascent::internal::RelIndexCombined::new(
                                            &name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total),
                                            &name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta),
                                        )
                                        .len_estimate()
                                {
                                    name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                    &name_indices_none_total
                                                        .to_rel_index(&__name_ind_common_total),
                                                    &name_indices_none_delta
                                                        .to_rel_index(&__name_ind_common_delta),
                                                )
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    ascent::internal::RelIndexCombined::new(
                                            &name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total),
                                            &name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta),
                                        )
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_delta, eq_name_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_delta
                                    .to_rel_index(&__name_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_name_indices_0_1_total
                                            .to_rel_index(&__eq_name_ind_common_total),
                                        &eq_name_indices_0_1_delta
                                            .to_rel_index(&__eq_name_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_delta
                                                .to_rel_index(&__name_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_delta
                                        .to_rel_index(&__name_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_name_indices_0_1_total
                                                                            .to_rel_index(&__eq_name_ind_common_total),
                                                                        &eq_name_indices_0_1_delta
                                                                            .to_rel_index(&__eq_name_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .is_empty()
                                || eq_name_indices_0_1_delta
                                    .to_rel_index(&__eq_name_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_delta
                                                                    .to_rel_index(&__eq_name_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_delta
                                                                    .to_rel_index(&__eq_name_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .is_empty()
                                || eq_name_indices_0_1_total
                                    .to_rel_index(&__eq_name_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_delta
                                                                                .to_rel_index(&__proc_ind_common_delta)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_delta
                                                                                .to_rel_index(&__proc_ind_common_delta)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .is_empty()
                                || eq_name_indices_0_1_total
                                    .to_rel_index(&__eq_name_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_delta
                                                                                            .to_rel_index(&__proc_ind_common_delta)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_delta
                                                                                            .to_rel_index(&__proc_ind_common_delta)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- name_indices_none_total, name_indices_none_total, eq_name_indices_0_1_total, proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = name_indices_none_total
                                .to_rel_index(&__name_ind_common_total)
                                .is_empty()
                                || name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .is_empty()
                                || eq_name_indices_0_1_total
                                    .to_rel_index(&__eq_name_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if name_indices_none_total
                                    .to_rel_index(&__name_ind_common_total)
                                    .len_estimate()
                                    <= name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .len_estimate()
                                {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_total
                                                                                            .to_rel_index(&__proc_ind_common_total)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                                                        .to_rel_index(&__eq_proc_ind_common_delta)
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    name_indices_none_total
                                        .to_rel_index(&__name_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = name_indices_none_total
                                                .to_rel_index(&__name_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Name = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Name = __val.0;
                                                                if let Some(__matching) = eq_name_indices_0_1_total
                                                                    .to_rel_index(&__eq_name_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_total
                                                                                            .to_rel_index(&__proc_ind_common_total)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                                                        .to_rel_index(&__eq_proc_ind_common_delta)
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::POutput(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::POutput(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_delta
                                .to_rel_index(&__proc_ind_common_delta)
                                .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .len_estimate()
                                    <= ascent::internal::RelIndexCombined::new(
                                            &proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total),
                                            &proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta),
                                        )
                                        .len_estimate()
                                {
                                    proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                    &proc_indices_none_total
                                                        .to_rel_index(&__proc_ind_common_total),
                                                    &proc_indices_none_delta
                                                        .to_rel_index(&__proc_ind_common_delta),
                                                )
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    ascent::internal::RelIndexCombined::new(
                                            &proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total),
                                            &proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta),
                                        )
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_delta
                                                .to_rel_index(&__proc_ind_common_delta)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_delta
                                        .to_rel_index(&__proc_ind_common_delta)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                        &eq_proc_indices_0_1_total
                                                                            .to_rel_index(&__eq_proc_ind_common_total),
                                                                        &eq_proc_indices_0_1_delta
                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                    )
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta, proc_indices_none_total+delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                    &proc_indices_none_total
                                                                                        .to_rel_index(&__proc_ind_common_total),
                                                                                    &proc_indices_none_delta
                                                                                        .to_rel_index(&__proc_ind_common_delta),
                                                                                )
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_delta, proc_indices_none_total+delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_total
                                    .to_rel_index(&__eq_proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &proc_indices_none_total
                                            .to_rel_index(&__proc_ind_common_total),
                                        &proc_indices_none_delta
                                            .to_rel_index(&__proc_ind_common_delta),
                                    )
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_delta
                                                                                .to_rel_index(&__proc_ind_common_delta)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_delta
                                                                                .to_rel_index(&__proc_ind_common_delta)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                &proc_indices_none_total
                                                                                                    .to_rel_index(&__proc_ind_common_total),
                                                                                                &proc_indices_none_delta
                                                                                                    .to_rel_index(&__proc_ind_common_delta),
                                                                                            )
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_total, proc_indices_none_delta, eq_proc_indices_0_1_total+delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_total
                                    .to_rel_index(&__eq_proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_delta
                                    .to_rel_index(&__proc_ind_common_delta)
                                    .is_empty()
                                || ascent::internal::RelIndexCombined::new(
                                        &eq_proc_indices_0_1_total
                                            .to_rel_index(&__eq_proc_ind_common_total),
                                        &eq_proc_indices_0_1_delta
                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                    )
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_delta
                                                                                            .to_rel_index(&__proc_ind_common_delta)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_delta
                                                                                            .to_rel_index(&__proc_ind_common_delta)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = ascent::internal::RelIndexCombined::new(
                                                                                                            &eq_proc_indices_0_1_total
                                                                                                                .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                            &eq_proc_indices_0_1_delta
                                                                                                                .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                        )
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_total, proc_indices_none_total, proc_indices_none_total, eq_proc_indices_0_1_delta [SIMPLE JOIN]",
                        );
                        {
                            let any_rel_empty = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_total
                                    .to_rel_index(&__eq_proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .is_empty()
                                || eq_proc_indices_0_1_delta
                                    .to_rel_index(&__eq_proc_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if proc_indices_none_total
                                    .to_rel_index(&__proc_ind_common_total)
                                    .len_estimate()
                                    <= proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .len_estimate()
                                {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let x0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let y0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_total
                                                                                            .to_rel_index(&__proc_ind_common_total)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                                                        .to_rel_index(&__eq_proc_ind_common_delta)
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                } else {
                                    proc_indices_none_total
                                        .to_rel_index(&__proc_ind_common_total)
                                        .iter_all()
                                        .for_each(|(__cl1_joined_columns, __cl1_tuple_indices)| {
                                            let __cl1_joined_columns = __cl1_joined_columns
                                                .tuple_of_borrowed();
                                            if let Some(__matching) = proc_indices_none_total
                                                .to_rel_index(&__proc_ind_common_total)
                                                .index_get(&())
                                            {
                                                __cl1_tuple_indices
                                                    .for_each(|cl1_val| {
                                                        let cl1_val = cl1_val.tuple_of_borrowed();
                                                        let y0: &Proc = cl1_val.0;
                                                        __matching
                                                            .clone()
                                                            .for_each(|__val| {
                                                                let __val = __val.tuple_of_borrowed();
                                                                let x0: &Proc = __val.0;
                                                                if let Some(__matching) = eq_proc_indices_0_1_total
                                                                    .to_rel_index(&__eq_proc_ind_common_total)
                                                                    .index_get(&((x0.clone()).clone(), (y0.clone()).clone()))
                                                                {
                                                                    __matching
                                                                        .for_each(|__val| {
                                                                            if let Some(__matching) = proc_indices_none_total
                                                                                .to_rel_index(&__proc_ind_common_total)
                                                                                .index_get(&())
                                                                            {
                                                                                __matching
                                                                                    .for_each(|__val| {
                                                                                        let __val = __val.tuple_of_borrowed();
                                                                                        let x1: &Proc = __val.0;
                                                                                        if let Some(__matching) = proc_indices_none_total
                                                                                            .to_rel_index(&__proc_ind_common_total)
                                                                                            .index_get(&())
                                                                                        {
                                                                                            __matching
                                                                                                .for_each(|__val| {
                                                                                                    let __val = __val.tuple_of_borrowed();
                                                                                                    let y1: &Proc = __val.0;
                                                                                                    if let Some(__matching) = eq_proc_indices_0_1_delta
                                                                                                        .to_rel_index(&__eq_proc_ind_common_delta)
                                                                                                        .index_get(&((x1.clone()).clone(), (y1.clone()).clone()))
                                                                                                    {
                                                                                                        __matching
                                                                                                            .for_each(|__val| {
                                                                                                                let __new_row: (Proc, Proc) = (
                                                                                                                    Proc::PPar(Box::new(x0.clone()), Box::new(x1.clone())),
                                                                                                                    Proc::PPar(Box::new(y0.clone()), Box::new(y1.clone())),
                                                                                                                );
                                                                                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                    &eq_proc_indices_0_1_total
                                                                                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                                                                                    &__new_row,
                                                                                                                )
                                                                                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                                                                        &eq_proc_indices_0_1_delta
                                                                                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                                                                                        &__new_row,
                                                                                                                    )
                                                                                                                {
                                                                                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                                                                        &mut eq_proc_indices_0_1_new
                                                                                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                        &__new_row,
                                                                                                                        (),
                                                                                                                    ) {
                                                                                                                        let __new_row_ind = _self.eq_proc.len();
                                                                                                                        _self
                                                                                                                            .eq_proc
                                                                                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                                                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                                                                                            &mut eq_proc_indices_0_new
                                                                                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                                                                                            (__new_row.0.clone(),),
                                                                                                                            (__new_row.1.clone(),),
                                                                                                                        );
                                                                                                                        __changed = true;
                                                                                                                    }
                                                                                                                }
                                                                                                            });
                                                                                                    }
                                                                                                });
                                                                                        }
                                                                                    });
                                                                            }
                                                                        });
                                                                }
                                                            });
                                                    });
                                            }
                                        });
                                }
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_delta, if let ⋯, let ⋯",
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
                                                &eq_proc_indices_0_1_total
                                                    .to_rel_index(&__eq_proc_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &eq_proc_indices_0_1_delta
                                                        .to_rel_index(&__eq_proc_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut eq_proc_indices_0_1_new
                                                        .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.eq_proc.len();
                                                    _self
                                                        .eq_proc
                                                        .push((__new_row.0.clone(), __new_row.1.clone()));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut eq_proc_indices_0_new
                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                        (__new_row.0.clone(),),
                                                        (__new_row.1.clone(),),
                                                    );
                                                    __changed = true;
                                                }
                                            }
                                        }
                                    });
                            }
                        }
                        ascent::internal::comment(
                            "eq_proc <-- proc_indices_none_delta, if let ⋯, if let ⋯, let ⋯",
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
                                        if let Proc::PPar(p, field_1) = p0 {
                                            if let Proc::PPar(q, r) = &**field_1 {
                                                let p1 = Proc::PPar(
                                                    Box::new(Proc::PPar(p.clone(), q.clone())),
                                                    r.clone(),
                                                );
                                                let __new_row: (Proc, Proc) = (
                                                    ascent::internal::Convert::convert(p0),
                                                    ascent::internal::Convert::convert(p1),
                                                );
                                                if !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &eq_proc_indices_0_1_total
                                                        .to_rel_index(&__eq_proc_ind_common_total),
                                                    &__new_row,
                                                )
                                                    && !::ascent::internal::RelFullIndexRead::contains_key(
                                                        &eq_proc_indices_0_1_delta
                                                            .to_rel_index(&__eq_proc_ind_common_delta),
                                                        &__new_row,
                                                    )
                                                {
                                                    if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                        &mut eq_proc_indices_0_1_new
                                                            .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                        &__new_row,
                                                        (),
                                                    ) {
                                                        let __new_row_ind = _self.eq_proc.len();
                                                        _self
                                                            .eq_proc
                                                            .push((__new_row.0.clone(), __new_row.1.clone()));
                                                        ::ascent::internal::RelIndexWrite::index_insert(
                                                            &mut eq_proc_indices_0_new
                                                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                                                            (__new_row.0.clone(),),
                                                            (__new_row.1.clone(),),
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
                            "redex_eq <-- eq_proc_indices_0_delta",
                        );
                        {
                            if let Some(__matching) = eq_proc_indices_0_delta
                                .to_rel_index(&__eq_proc_ind_common_delta)
                                .index_get(&((redex.clone()).clone(),))
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let q: &Proc = __val.0;
                                        let __new_row: (Proc,) = (q.clone(),);
                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                            &redex_eq_indices_0_total
                                                .to_rel_index(&__redex_eq_ind_common_total),
                                            &__new_row,
                                        )
                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                &redex_eq_indices_0_delta
                                                    .to_rel_index(&__redex_eq_ind_common_delta),
                                                &__new_row,
                                            )
                                        {
                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                &mut redex_eq_indices_0_new
                                                    .to_rel_index_write(&mut __redex_eq_ind_common_new),
                                                &__new_row,
                                                (),
                                            ) {
                                                let __new_row_ind = _self.redex_eq.len();
                                                _self.redex_eq.push((__new_row.0.clone(),));
                                                ::ascent::internal::RelIndexWrite::index_insert(
                                                    &mut redex_eq_indices_none_new
                                                        .to_rel_index_write(&mut __redex_eq_ind_common_new),
                                                    (),
                                                    (__new_row.0.clone(),),
                                                );
                                                __changed = true;
                                            }
                                        }
                                    });
                            }
                        }
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __eq_name_ind_common_new,
                            &mut __eq_name_ind_common_delta,
                            &mut __eq_name_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_name_indices_0_new
                                .to_rel_index_write(&mut __eq_name_ind_common_new),
                            &mut eq_name_indices_0_delta
                                .to_rel_index_write(&mut __eq_name_ind_common_delta),
                            &mut eq_name_indices_0_total
                                .to_rel_index_write(&mut __eq_name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_name_indices_0_1_new
                                .to_rel_index_write(&mut __eq_name_ind_common_new),
                            &mut eq_name_indices_0_1_delta
                                .to_rel_index_write(&mut __eq_name_ind_common_delta),
                            &mut eq_name_indices_0_1_total
                                .to_rel_index_write(&mut __eq_name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __eq_proc_ind_common_new,
                            &mut __eq_proc_ind_common_delta,
                            &mut __eq_proc_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_proc_indices_0_new
                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                            &mut eq_proc_indices_0_delta
                                .to_rel_index_write(&mut __eq_proc_ind_common_delta),
                            &mut eq_proc_indices_0_total
                                .to_rel_index_write(&mut __eq_proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut eq_proc_indices_0_1_new
                                .to_rel_index_write(&mut __eq_proc_ind_common_new),
                            &mut eq_proc_indices_0_1_delta
                                .to_rel_index_write(&mut __eq_proc_ind_common_delta),
                            &mut eq_proc_indices_0_1_total
                                .to_rel_index_write(&mut __eq_proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __name_ind_common_new,
                            &mut __name_ind_common_delta,
                            &mut __name_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut name_indices_0_new
                                .to_rel_index_write(&mut __name_ind_common_new),
                            &mut name_indices_0_delta
                                .to_rel_index_write(&mut __name_ind_common_delta),
                            &mut name_indices_0_total
                                .to_rel_index_write(&mut __name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut name_indices_none_new
                                .to_rel_index_write(&mut __name_ind_common_new),
                            &mut name_indices_none_delta
                                .to_rel_index_write(&mut __name_ind_common_delta),
                            &mut name_indices_none_total
                                .to_rel_index_write(&mut __name_ind_common_total),
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
                            &mut __redex_eq_ind_common_new,
                            &mut __redex_eq_ind_common_delta,
                            &mut __redex_eq_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut redex_eq_indices_0_new
                                .to_rel_index_write(&mut __redex_eq_ind_common_new),
                            &mut redex_eq_indices_0_delta
                                .to_rel_index_write(&mut __redex_eq_ind_common_delta),
                            &mut redex_eq_indices_0_total
                                .to_rel_index_write(&mut __redex_eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut redex_eq_indices_none_new
                                .to_rel_index_write(&mut __redex_eq_ind_common_new),
                            &mut redex_eq_indices_none_delta
                                .to_rel_index_write(&mut __redex_eq_ind_common_delta),
                            &mut redex_eq_indices_none_total
                                .to_rel_index_write(&mut __redex_eq_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __rw_name_ind_common_new,
                            &mut __rw_name_ind_common_delta,
                            &mut __rw_name_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_name_indices_0_new
                                .to_rel_index_write(&mut __rw_name_ind_common_new),
                            &mut rw_name_indices_0_delta
                                .to_rel_index_write(&mut __rw_name_ind_common_delta),
                            &mut rw_name_indices_0_total
                                .to_rel_index_write(&mut __rw_name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_name_indices_0_1_new
                                .to_rel_index_write(&mut __rw_name_ind_common_new),
                            &mut rw_name_indices_0_1_delta
                                .to_rel_index_write(&mut __rw_name_ind_common_delta),
                            &mut rw_name_indices_0_1_total
                                .to_rel_index_write(&mut __rw_name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_name_indices_1_new
                                .to_rel_index_write(&mut __rw_name_ind_common_new),
                            &mut rw_name_indices_1_delta
                                .to_rel_index_write(&mut __rw_name_ind_common_delta),
                            &mut rw_name_indices_1_total
                                .to_rel_index_write(&mut __rw_name_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __rw_proc_ind_common_new,
                            &mut __rw_proc_ind_common_delta,
                            &mut __rw_proc_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_proc_indices_0_new
                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                            &mut rw_proc_indices_0_delta
                                .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                            &mut rw_proc_indices_0_total
                                .to_rel_index_write(&mut __rw_proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_proc_indices_0_1_new
                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                            &mut rw_proc_indices_0_1_delta
                                .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                            &mut rw_proc_indices_0_1_total
                                .to_rel_index_write(&mut __rw_proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_proc_indices_1_new
                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                            &mut rw_proc_indices_1_delta
                                .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                            &mut rw_proc_indices_1_total
                                .to_rel_index_write(&mut __rw_proc_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut rw_proc_indices_none_new
                                .to_rel_index_write(&mut __rw_proc_ind_common_new),
                            &mut rw_proc_indices_none_delta
                                .to_rel_index_write(&mut __rw_proc_ind_common_delta),
                            &mut rw_proc_indices_none_total
                                .to_rel_index_write(&mut __rw_proc_ind_common_total),
                        );
                        _self.scc_iters[1usize] += 1;
                        if !__changed {
                            break;
                        }
                    }
                    _self.__eq_name_ind_common = __eq_name_ind_common_total;
                    _self.eq_name_indices_0 = eq_name_indices_0_total;
                    _self.eq_name_indices_0_1 = eq_name_indices_0_1_total;
                    _self.__eq_proc_ind_common = __eq_proc_ind_common_total;
                    _self.eq_proc_indices_0 = eq_proc_indices_0_total;
                    _self.eq_proc_indices_0_1 = eq_proc_indices_0_1_total;
                    _self.__name_ind_common = __name_ind_common_total;
                    _self.name_indices_0 = name_indices_0_total;
                    _self.name_indices_none = name_indices_none_total;
                    _self.__proc_ind_common = __proc_ind_common_total;
                    _self.proc_indices_0 = proc_indices_0_total;
                    _self.proc_indices_none = proc_indices_none_total;
                    _self.__redex_eq_ind_common = __redex_eq_ind_common_total;
                    _self.redex_eq_indices_0 = redex_eq_indices_0_total;
                    _self.redex_eq_indices_none = redex_eq_indices_none_total;
                    _self.__rw_name_ind_common = __rw_name_ind_common_total;
                    _self.rw_name_indices_0 = rw_name_indices_0_total;
                    _self.rw_name_indices_0_1 = rw_name_indices_0_1_total;
                    _self.rw_name_indices_1 = rw_name_indices_1_total;
                    _self.__rw_proc_ind_common = __rw_proc_ind_common_total;
                    _self.rw_proc_indices_0 = rw_proc_indices_0_total;
                    _self.rw_proc_indices_0_1 = rw_proc_indices_0_1_total;
                    _self.rw_proc_indices_1 = rw_proc_indices_1_total;
                    _self.rw_proc_indices_none = rw_proc_indices_none_total;
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
                        (Proc,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
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
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
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
                    let __redex_eq_ind_common_total: () = std::mem::take(
                        &mut _self.__redex_eq_ind_common,
                    );
                    let redex_eq_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = std::mem::take(&mut _self.redex_eq_indices_none);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "path <-- redex_eq_indices_none_total",
                        );
                        {
                            if let Some(__matching) = redex_eq_indices_none_total
                                .to_rel_index(&__redex_eq_ind_common_total)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let q: &Proc = __val.0;
                                        let __new_row: (Proc, Proc) = (redex.clone(), q.clone());
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
                        _self.scc_iters[2usize] += 1;
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.__redex_eq_ind_common = __redex_eq_ind_common_total;
                    _self.redex_eq_indices_none = redex_eq_indices_none_total;
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
                        (Proc,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
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
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
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
                    let __rw_proc_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_proc_ind_common,
                    );
                    let rw_proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = std::mem::take(&mut _self.rw_proc_indices_none);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment("path <-- rw_proc_indices_none_total");
                        {
                            if let Some(__matching) = rw_proc_indices_none_total
                                .to_rel_index(&__rw_proc_ind_common_total)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let p: &Proc = __val.0;
                                        let q: &Proc = __val.1;
                                        let __new_row: (Proc, Proc) = (p.clone(), q.clone());
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
                        _self.scc_iters[3usize] += 1;
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.__rw_proc_ind_common = __rw_proc_ind_common_total;
                    _self.rw_proc_indices_none = rw_proc_indices_none_total;
                    _self.scc_times[3usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 4");
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
                        (Proc,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
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
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
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
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment("path <-- for_");
                        {
                            for _ in [()] {
                                let __new_row: (Proc, Proc) = (
                                    redex.clone(),
                                    redex.clone(),
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
                                        __changed = true;
                                    }
                                }
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
                        _self.scc_iters[4usize] += 1;
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.scc_times[4usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 5");
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
                        (Proc,),
                    > = ::std::mem::take(&mut _self.path_indices_0);
                    let mut path_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = Default::default();
                    let mut path_indices_0_new: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
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
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.path_indices_0_1);
                    let mut path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut path_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
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
                    let __rw_proc_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_proc_ind_common,
                    );
                    let rw_proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc, Proc),
                    > = std::mem::take(&mut _self.rw_proc_indices_none);
                    #[allow(unused_assignments, unused_variables)]
                    loop {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "path <-- rw_proc_indices_none_total, path_indices_0_delta",
                        );
                        {
                            let any_rel_empty = rw_proc_indices_none_total
                                .to_rel_index(&__rw_proc_ind_common_total)
                                .is_empty()
                                || path_indices_0_delta
                                    .to_rel_index(&__path_ind_common_delta)
                                    .is_empty();
                            if !any_rel_empty {
                                if let Some(__matching) = rw_proc_indices_none_total
                                    .to_rel_index(&__rw_proc_ind_common_total)
                                    .index_get(&())
                                {
                                    __matching
                                        .for_each(|__val| {
                                            let __val = __val.tuple_of_borrowed();
                                            let p: &Proc = __val.0;
                                            let q: &Proc = __val.1;
                                            if let Some(__matching) = path_indices_0_delta
                                                .to_rel_index(&__path_ind_common_delta)
                                                .index_get(&((q.clone()).clone(),))
                                            {
                                                __matching
                                                    .for_each(|__val| {
                                                        let __val = __val.tuple_of_borrowed();
                                                        let r: &Proc = __val.0;
                                                        let __new_row: (Proc, Proc) = (p.clone(), r.clone());
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
                        _self.scc_iters[5usize] += 1;
                        if !__changed {
                            break;
                        }
                    }
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0 = path_indices_0_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.__rw_proc_ind_common = __rw_proc_ind_common_total;
                    _self.rw_proc_indices_none = rw_proc_indices_none_total;
                    _self.scc_times[5usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 6");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __is_normal_form_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__is_normal_form_ind_common,
                    );
                    let mut __is_normal_form_ind_common_total: () = Default::default();
                    let mut __is_normal_form_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __is_normal_form_ind_common_new,
                        &mut __is_normal_form_ind_common_delta,
                        &mut __is_normal_form_ind_common_total,
                    );
                    let mut is_normal_form_indices_0_delta: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = ::std::mem::take(&mut _self.is_normal_form_indices_0);
                    let mut is_normal_form_indices_0_total: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    let mut is_normal_form_indices_0_new: ascent::internal::RelFullIndexType<
                        (Proc,),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut is_normal_form_indices_0_new
                            .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                        &mut is_normal_form_indices_0_delta
                            .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                        &mut is_normal_form_indices_0_total
                            .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                    );
                    let mut is_normal_form_indices_none_delta: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = ::std::mem::take(&mut _self.is_normal_form_indices_none);
                    let mut is_normal_form_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    let mut is_normal_form_indices_none_new: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut is_normal_form_indices_none_new
                            .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                        &mut is_normal_form_indices_none_delta
                            .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                        &mut is_normal_form_indices_none_total
                            .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                    );
                    let __proc_ind_common_total: () = std::mem::take(
                        &mut _self.__proc_ind_common,
                    );
                    let proc_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = std::mem::take(&mut _self.proc_indices_none);
                    let __rw_proc_ind_common_total: () = std::mem::take(
                        &mut _self.__rw_proc_ind_common,
                    );
                    let rw_proc_indices_0_total: ascent::rel::ToRelIndexType<
                        (Proc,),
                        (Proc,),
                    > = std::mem::take(&mut _self.rw_proc_indices_0);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "is_normal_form <-- proc_indices_none_total, agg rw_proc_indices_0",
                        );
                        {
                            if let Some(__matching) = proc_indices_none_total
                                .to_rel_index(&__proc_ind_common_total)
                                .index_get(&())
                            {
                                __matching
                                    .for_each(|__val| {
                                        let __val = __val.tuple_of_borrowed();
                                        let t: &Proc = __val.0;
                                        let __aggregated_rel = rw_proc_indices_0_total
                                            .to_rel_index(&__rw_proc_ind_common_total);
                                        let __matching = __aggregated_rel
                                            .index_get(&((t.clone()).clone(),));
                                        let __agg_args = __matching
                                            .into_iter()
                                            .flatten()
                                            .map(|__val| { () });
                                        for () in ::ascent::aggregators::not(__agg_args) {
                                            let __new_row: (Proc,) = (t.clone(),);
                                            if !::ascent::internal::RelFullIndexRead::contains_key(
                                                &is_normal_form_indices_0_total
                                                    .to_rel_index(&__is_normal_form_ind_common_total),
                                                &__new_row,
                                            )
                                                && !::ascent::internal::RelFullIndexRead::contains_key(
                                                    &is_normal_form_indices_0_delta
                                                        .to_rel_index(&__is_normal_form_ind_common_delta),
                                                    &__new_row,
                                                )
                                            {
                                                if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                    &mut is_normal_form_indices_0_new
                                                        .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                                                    &__new_row,
                                                    (),
                                                ) {
                                                    let __new_row_ind = _self.is_normal_form.len();
                                                    _self.is_normal_form.push((__new_row.0.clone(),));
                                                    ::ascent::internal::RelIndexWrite::index_insert(
                                                        &mut is_normal_form_indices_none_new
                                                            .to_rel_index_write(&mut __is_normal_form_ind_common_new),
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
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __is_normal_form_ind_common_new,
                            &mut __is_normal_form_ind_common_delta,
                            &mut __is_normal_form_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut is_normal_form_indices_0_new
                                .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                            &mut is_normal_form_indices_0_delta
                                .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                            &mut is_normal_form_indices_0_total
                                .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut is_normal_form_indices_none_new
                                .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                            &mut is_normal_form_indices_none_delta
                                .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                            &mut is_normal_form_indices_none_total
                                .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __is_normal_form_ind_common_new,
                            &mut __is_normal_form_ind_common_delta,
                            &mut __is_normal_form_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut is_normal_form_indices_0_new
                                .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                            &mut is_normal_form_indices_0_delta
                                .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                            &mut is_normal_form_indices_0_total
                                .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut is_normal_form_indices_none_new
                                .to_rel_index_write(&mut __is_normal_form_ind_common_new),
                            &mut is_normal_form_indices_none_delta
                                .to_rel_index_write(&mut __is_normal_form_ind_common_delta),
                            &mut is_normal_form_indices_none_total
                                .to_rel_index_write(&mut __is_normal_form_ind_common_total),
                        );
                        _self.scc_iters[6usize] += 1;
                    }
                    _self.__is_normal_form_ind_common = __is_normal_form_ind_common_total;
                    _self.is_normal_form_indices_0 = is_normal_form_indices_0_total;
                    _self.is_normal_form_indices_none = is_normal_form_indices_none_total;
                    _self.__proc_ind_common = __proc_ind_common_total;
                    _self.proc_indices_none = proc_indices_none_total;
                    _self.__rw_proc_ind_common = __rw_proc_ind_common_total;
                    _self.rw_proc_indices_0 = rw_proc_indices_0_total;
                    _self.scc_times[6usize] += _scc_start_time.elapsed();
                }
                ascent::internal::comment("scc 7");
                {
                    let _scc_start_time = ::ascent::internal::Instant::now();
                    let mut __path_full_ind_common_delta: () = ::std::mem::take(
                        &mut _self.__path_full_ind_common,
                    );
                    let mut __path_full_ind_common_total: () = Default::default();
                    let mut __path_full_ind_common_new: () = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut __path_full_ind_common_new,
                        &mut __path_full_ind_common_delta,
                        &mut __path_full_ind_common_total,
                    );
                    let mut path_full_indices_0_1_delta: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = ::std::mem::take(&mut _self.path_full_indices_0_1);
                    let mut path_full_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    let mut path_full_indices_0_1_new: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = Default::default();
                    ::ascent::internal::RelIndexMerge::init(
                        &mut path_full_indices_0_1_new
                            .to_rel_index_write(&mut __path_full_ind_common_new),
                        &mut path_full_indices_0_1_delta
                            .to_rel_index_write(&mut __path_full_ind_common_delta),
                        &mut path_full_indices_0_1_total
                            .to_rel_index_write(&mut __path_full_ind_common_total),
                    );
                    let __is_normal_form_ind_common_total: () = std::mem::take(
                        &mut _self.__is_normal_form_ind_common,
                    );
                    let is_normal_form_indices_none_total: ascent::rel::ToRelIndexType<
                        (),
                        (Proc,),
                    > = std::mem::take(&mut _self.is_normal_form_indices_none);
                    let __path_ind_common_total: () = std::mem::take(
                        &mut _self.__path_ind_common,
                    );
                    let path_indices_0_1_total: ascent::internal::RelFullIndexType<
                        (Proc, Proc),
                        (),
                    > = std::mem::take(&mut _self.path_indices_0_1);
                    #[allow(unused_assignments, unused_variables)]
                    {
                        let mut __changed = false;
                        ascent::internal::comment(
                            "path_full <-- is_normal_form_indices_none_total, path_indices_0_1_total",
                        );
                        {
                            let any_rel_empty = is_normal_form_indices_none_total
                                .to_rel_index(&__is_normal_form_ind_common_total)
                                .is_empty()
                                || path_indices_0_1_total
                                    .to_rel_index(&__path_ind_common_total)
                                    .is_empty();
                            if !any_rel_empty {
                                if let Some(__matching) = is_normal_form_indices_none_total
                                    .to_rel_index(&__is_normal_form_ind_common_total)
                                    .index_get(&())
                                {
                                    __matching
                                        .for_each(|__val| {
                                            let __val = __val.tuple_of_borrowed();
                                            let z: &Proc = __val.0;
                                            if let Some(__matching) = path_indices_0_1_total
                                                .to_rel_index(&__path_ind_common_total)
                                                .index_get(&((redex.clone()).clone(), z.clone()))
                                            {
                                                __matching
                                                    .for_each(|__val| {
                                                        let __new_row: (Proc, Proc) = (redex.clone(), z.clone());
                                                        if !::ascent::internal::RelFullIndexRead::contains_key(
                                                            &path_full_indices_0_1_total
                                                                .to_rel_index(&__path_full_ind_common_total),
                                                            &__new_row,
                                                        )
                                                            && !::ascent::internal::RelFullIndexRead::contains_key(
                                                                &path_full_indices_0_1_delta
                                                                    .to_rel_index(&__path_full_ind_common_delta),
                                                                &__new_row,
                                                            )
                                                        {
                                                            if ::ascent::internal::RelFullIndexWrite::insert_if_not_present(
                                                                &mut path_full_indices_0_1_new
                                                                    .to_rel_index_write(&mut __path_full_ind_common_new),
                                                                &__new_row,
                                                                (),
                                                            ) {
                                                                let __new_row_ind = _self.path_full.len();
                                                                _self.path_full.push((__new_row.0, __new_row.1));
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
                            &mut __path_full_ind_common_new,
                            &mut __path_full_ind_common_delta,
                            &mut __path_full_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_full_indices_0_1_new
                                .to_rel_index_write(&mut __path_full_ind_common_new),
                            &mut path_full_indices_0_1_delta
                                .to_rel_index_write(&mut __path_full_ind_common_delta),
                            &mut path_full_indices_0_1_total
                                .to_rel_index_write(&mut __path_full_ind_common_total),
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut __path_full_ind_common_new,
                            &mut __path_full_ind_common_delta,
                            &mut __path_full_ind_common_total,
                        );
                        ::ascent::internal::RelIndexMerge::merge_delta_to_total_new_to_delta(
                            &mut path_full_indices_0_1_new
                                .to_rel_index_write(&mut __path_full_ind_common_new),
                            &mut path_full_indices_0_1_delta
                                .to_rel_index_write(&mut __path_full_ind_common_delta),
                            &mut path_full_indices_0_1_total
                                .to_rel_index_write(&mut __path_full_ind_common_total),
                        );
                        _self.scc_iters[7usize] += 1;
                    }
                    _self.__path_full_ind_common = __path_full_ind_common_total;
                    _self.path_full_indices_0_1 = path_full_indices_0_1_total;
                    _self.__is_normal_form_ind_common = __is_normal_form_ind_common_total;
                    _self.is_normal_form_indices_none = is_normal_form_indices_none_total;
                    _self.__path_ind_common = __path_ind_common_total;
                    _self.path_indices_0_1 = path_indices_0_1_total;
                    _self.scc_times[7usize] += _scc_start_time.elapsed();
                }
            }
            __run_res
        }
    };
    let mut procs = prog.proc;
    procs.sort_by(|a, b| a.0.cmp(&b.0));
    {
        ::std::io::_print(format_args!("Terms: {0}\n", procs.len()));
    };
    {
        ::std::io::_print(format_args!("Rewrites: {0}\n", prog.rw_proc.len()));
    };
    {
        ::std::io::_print(
            format_args!("Normal forms: {0}\n", prog.is_normal_form.len()),
        );
    };
    {
        ::std::io::_print(format_args!("\n=== proc facts ===\n"));
    };
    {
        ::std::io::_print(format_args!("Count: {0}\n", procs.len()));
    };
    for p in procs.iter().take(10) {
        {
            ::std::io::_print(format_args!("  {0}\n", p.0));
        };
    }
    {
        ::std::io::_print(format_args!("\n=== name facts ===\n"));
    };
    let mut names: Vec<_> = prog.name.iter().collect();
    names
        .sort_by(|a, b| {
            ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", a))
                })
                .cmp(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0:?}", b))
                    }),
                )
        });
    {
        ::std::io::_print(format_args!("Count: {0}\n", names.len()));
    };
    for n in names.iter() {
        {
            ::std::io::_print(format_args!("  {0}\n", n.0));
        };
    }
    {
        ::std::io::_print(format_args!("\n=== eq_name facts ===\n"));
    };
    let mut eq_names: Vec<_> = prog.eq_name.iter().collect();
    eq_names
        .sort_by(|a, b| {
            ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", a))
                })
                .cmp(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0:?}", b))
                    }),
                )
        });
    for (n1, n2) in eq_names.iter().take(20) {
        {
            ::std::io::_print(format_args!("  {0} == {1}\n", n1, n2));
        };
    }
    let mut path_full = prog.path_full.clone();
    path_full.sort_by(|a, b| a.0.cmp(&b.0));
    {
        ::std::io::_print(format_args!("\n=== Paths to normal forms ===\n"));
    };
    {
        ::std::io::_print(format_args!("Count: {0}\n", path_full.len()));
    };
    for (s, t) in path_full {
        {
            ::std::io::_print(format_args!("  {0} ~> {1}\n", s, t));
        };
    }
    let elapsed = Instant::now().duration_since(start_time);
    {
        ::std::io::_print(format_args!("Time: {0:?}\n", elapsed));
    };
}
