pub mod rhocalc {
    #![allow(
        non_local_definitions,
        clippy::crate_in_macro_def,
        clippy::empty_line_after_outer_attr
    )]
    use mettail_macros::theory;
    use lalrpop_util::lalrpop_mod;
    pub enum Proc {
        PZero,
        PDrop(Box<Name>),
        POutput(Box<Name>, Box<Proc>),
        PInput(
            Box<Name>,
            mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>,
        ),
        PPar(mettail_runtime::HashBag<Proc>),
        PVar(mettail_runtime::OrdVar),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Proc {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Proc::PZero => ::core::fmt::Formatter::write_str(f, "PZero"),
                Proc::PDrop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PDrop",
                        &__self_0,
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
                Proc::PInput(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "PInput",
                        __self_0,
                        &__self_1,
                    )
                }
                Proc::PPar(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PPar",
                        &__self_0,
                    )
                }
                Proc::PVar(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PVar",
                        &__self_0,
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
                Proc::PDrop(__self_0) => {
                    Proc::PDrop(::core::clone::Clone::clone(__self_0))
                }
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
                Proc::PPar(__self_0) => Proc::PPar(::core::clone::Clone::clone(__self_0)),
                Proc::PVar(__self_0) => Proc::PVar(::core::clone::Clone::clone(__self_0)),
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
                    (Proc::PDrop(__self_0), Proc::PDrop(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        Proc::POutput(__self_0, __self_1),
                        Proc::POutput(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (
                        Proc::PInput(__self_0, __self_1),
                        Proc::PInput(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (Proc::PPar(__self_0), Proc::PPar(__arg1_0)) => __self_0 == __arg1_0,
                    (Proc::PVar(__self_0), Proc::PVar(__arg1_0)) => __self_0 == __arg1_0,
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
            let _: ::core::cmp::AssertParamIsEq<mettail_runtime::HashBag<Proc>>;
            let _: ::core::cmp::AssertParamIsEq<mettail_runtime::OrdVar>;
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
                (
                    Proc::POutput(__self_0, __self_1),
                    Proc::POutput(__arg1_0, __arg1_1),
                ) => {
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
                (Proc::PPar(__self_0), Proc::PPar(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (Proc::PVar(__self_0), Proc::PVar(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
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
                        (Proc::PPar(__self_0), Proc::PPar(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (Proc::PVar(__self_0), Proc::PVar(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
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
                Proc::PPar(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Proc::PVar(__self_0) => ::core::hash::Hash::hash(__self_0, state),
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
                        &Proc::PPar(ref __binding_lhs_0),
                        &Proc::PPar(ref __binding_rhs_0),
                    ) => {
                        true
                            && moniker::BoundTerm::<
                                String,
                            >::term_eq(__binding_lhs_0, __binding_rhs_0)
                    }
                    (
                        &Proc::PVar(ref __binding_lhs_0),
                        &Proc::PVar(ref __binding_rhs_0),
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
                    Proc::PPar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::close_term(__binding_0, __state, __on_free);
                    }
                    Proc::PVar(ref mut __binding_0) => {
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
                    Proc::PPar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_0, __state, __on_bound);
                    }
                    Proc::PVar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::open_term(__binding_0, __state, __on_bound);
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
                            moniker::BoundTerm::<
                                String,
                            >::visit_vars(__binding_0, __on_var);
                        }
                        {
                            moniker::BoundTerm::<
                                String,
                            >::visit_vars(__binding_1, __on_var);
                        }
                    }
                    Proc::PInput(ref __binding_0, ref __binding_1) => {
                        {
                            moniker::BoundTerm::<
                                String,
                            >::visit_vars(__binding_0, __on_var);
                        }
                        {
                            moniker::BoundTerm::<
                                String,
                            >::visit_vars(__binding_1, __on_var);
                        }
                    }
                    Proc::PPar(ref __binding_0) => {
                        moniker::BoundTerm::<String>::visit_vars(__binding_0, __on_var);
                    }
                    Proc::PVar(ref __binding_0) => {
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
                    Proc::PDrop(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
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
                    Proc::PPar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                    Proc::PVar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
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
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NQuote",
                        &__self_0,
                    )
                }
                Name::NVar(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NVar",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Name {
        #[inline]
        fn clone(&self) -> Name {
            match self {
                Name::NQuote(__self_0) => {
                    Name::NQuote(::core::clone::Clone::clone(__self_0))
                }
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
                    (Name::NQuote(__self_0), Name::NQuote(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
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
                    (
                        &Name::NVar(ref __binding_lhs_0),
                        &Name::NVar(ref __binding_rhs_0),
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
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                    Name::NVar(ref mut __binding_0) => {
                        moniker::BoundTerm::<
                            String,
                        >::visit_mut_vars(__binding_0, __on_var);
                    }
                }
            }
        }
    };
    impl Proc {
        /// Auto-flattening insert for #label
        ///
        /// If elem is itself a #label, recursively merges its contents instead of nesting.
        /// This ensures that collection constructors are always flat, never nested.
        pub fn insert_into_ppar(bag: &mut mettail_runtime::HashBag<Proc>, elem: Proc) {
            match elem {
                Proc::PPar(inner) => {
                    for (e, count) in inner.iter() {
                        for _ in 0..count {
                            Self::insert_into_ppar(bag, e.clone());
                        }
                    }
                }
                _ => {
                    bag.insert(elem);
                }
            }
        }
    }
    impl Proc {
        /// Recursively normalize this term by flattening any nested collections.
        ///
        /// For example, `PPar({PPar({a, b}), c})` becomes `PPar({a, b, c})`.
        /// This ensures that collection constructors are always in canonical flat form.
        pub fn normalize(&self) -> Self {
            match self {
                Proc::PZero => self.clone(),
                Proc::PDrop(f0) => Proc::PDrop(f0.clone()),
                Proc::PInput(f0, scope) => {
                    Proc::PInput(
                        f0.clone(),
                        mettail_runtime::Scope::from_parts_unsafe(
                            scope.inner().unsafe_pattern.clone(),
                            Box::new(scope.inner().unsafe_body.as_ref().normalize()),
                        ),
                    )
                }
                Proc::PPar(bag) => {
                    let mut new_bag = mettail_runtime::HashBag::new();
                    for (elem, count) in bag.iter() {
                        for _ in 0..count {
                            let normalized_elem = elem.normalize();
                            Self::insert_into_ppar(&mut new_bag, normalized_elem);
                        }
                    }
                    Proc::PPar(new_bag)
                }
                Proc::PVar(v) => Proc::PVar(v.clone()),
                _ => self.clone(),
            }
        }
    }
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
                Proc::PPar(field_0) => {
                    Proc::PPar({
                        let mut bag = mettail_runtime::HashBag::new();
                        for (elem, count) in field_0.iter() {
                            let subst_elem = elem.substitute(var, replacement);
                            for _ in 0..count {
                                Proc::insert_into_ppar(&mut bag, subst_elem.clone());
                            }
                        }
                        bag
                    })
                }
                Proc::PVar(
                    mettail_runtime::OrdVar(mettail_runtime::Var::Free(v)),
                ) if v == var => replacement.clone(),
                Proc::PVar(_) => self.clone(),
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
                    let binder = &scope.inner().unsafe_pattern;
                    let body = &scope.inner().unsafe_body;
                    if binder.0 == *var {
                        self.clone()
                    } else {
                        let subst_body = (**body).substitute_name(var, replacement);
                        let new_scope = mettail_runtime::Scope::new(
                            binder.clone(),
                            Box::new(subst_body),
                        );
                        Proc::PInput(
                            Box::new((**field_0).substitute(var, replacement)),
                            new_scope.clone(),
                        )
                    }
                }
                Proc::PPar(field_0) => {
                    Proc::PPar({
                        let mut bag = mettail_runtime::HashBag::new();
                        for (elem, count) in field_0.iter() {
                            let subst_elem = elem.substitute_name(var, replacement);
                            for _ in 0..count {
                                Proc::insert_into_ppar(&mut bag, subst_elem.clone());
                            }
                        }
                        bag
                    })
                }
                Proc::PVar(_) => self.clone(),
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
                Proc::PDrop(f2) => f.write_fmt(format_args!("*({0})", f2)),
                Proc::POutput(f0, f3) => f.write_fmt(format_args!("{0}!({1})", f0, f3)),
                Proc::PInput(f0, scope) => {
                    let (binder, body) = scope.clone().unbind();
                    let binder_name = binder
                        .0
                        .pretty_name
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("_");
                    f.write_fmt(
                        format_args!("for({0}->{1}){{{2}}}", f0, binder_name, body),
                    )
                }
                Proc::PPar(f0) => {
                    f.write_fmt(
                        format_args!(
                            "{0}",
                            {
                                let mut s = String::from("{");
                                let items: Vec<String> = f0
                                    .iter()
                                    .map(|(elem, count)| {
                                        (0..count)
                                            .map(|_| elem.to_string())
                                            .collect::<Vec<_>>()
                                            .join(
                                                &::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!(" {0} ", "|"))
                                                }),
                                            )
                                    })
                                    .collect();
                                if !items.is_empty() {
                                    s.push_str(
                                        &items
                                            .join(
                                                &::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!(" {0} ", "|"))
                                                }),
                                            ),
                                    );
                                }
                                s.push_str("}");
                                s
                            },
                        ),
                    )
                }
                Proc::PVar(f0) => {
                    f.write_fmt(
                        format_args!(
                            "{0}",
                            match &(f0).0 {
                                mettail_runtime::Var::Free(fv) => {
                                    fv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_")
                                }
                                mettail_runtime::Var::Bound(bv) => {
                                    bv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_")
                                }
                            },
                        ),
                    )
                }
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
                                mettail_runtime::Var::Bound(bv) => {
                                    bv.pretty_name.as_ref().map(|s| s.as_str()).unwrap_or("_")
                                }
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
        max_collection_width: usize,
        proc_by_depth: std::collections::HashMap<usize, Vec<Proc>>,
        name_by_depth: std::collections::HashMap<usize, Vec<Name>>,
    }
    impl GenerationContext {
        fn new(
            vars: Vec<String>,
            max_depth: usize,
            max_collection_width: usize,
        ) -> Self {
            let initial_var_count = vars.len();
            Self {
                vars,
                initial_var_count,
                max_depth,
                max_collection_width,
                proc_by_depth: std::collections::HashMap::new(),
                name_by_depth: std::collections::HashMap::new(),
            }
        }
        fn new_with_extended_vars(
            vars: Vec<String>,
            initial_var_count: usize,
            max_depth: usize,
            max_collection_width: usize,
        ) -> Self {
            Self {
                vars,
                initial_var_count,
                max_depth,
                max_collection_width,
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
                for var_name in &self.vars {
                    terms
                        .push(
                            Proc::PVar(
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
                let mut temp_ctx = GenerationContext::new_with_extended_vars(
                    extended_vars,
                    self.initial_var_count,
                    depth - 1,
                    self.max_collection_width,
                );
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
                for size in 0..=self.max_collection_width {
                    if size == 0 {
                        let bag = mettail_runtime::HashBag::new();
                        terms.push(Proc::PPar(bag));
                    } else if size == 1 {
                        for d in 0..depth {
                            if let Some(elems) = self.proc_by_depth.get(&d) {
                                for elem in elems {
                                    let mut bag = mettail_runtime::HashBag::new();
                                    bag.insert(elem.clone());
                                    terms.push(Proc::PPar(bag));
                                }
                            }
                        }
                    } else if size == 2 {
                        for d1 in 0..depth {
                            for d2 in 0..depth {
                                if let Some(elems1) = self.proc_by_depth.get(&d1) {
                                    if let Some(elems2) = self.proc_by_depth.get(&d2) {
                                        for elem1 in elems1 {
                                            for elem2 in elems2 {
                                                let mut bag = mettail_runtime::HashBag::new();
                                                bag.insert(elem1.clone());
                                                bag.insert(elem2.clone());
                                                terms.push(Proc::PPar(bag));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if size == 3 {
                        for d1 in 0..depth {
                            for d2 in 0..depth {
                                for d3 in 0..depth {
                                    if let Some(elems1) = self.proc_by_depth.get(&d1) {
                                        if let Some(elems2) = self.proc_by_depth.get(&d2) {
                                            if let Some(elems3) = self.proc_by_depth.get(&d3) {
                                                for elem1 in elems1 {
                                                    for elem2 in elems2 {
                                                        for elem3 in elems3 {
                                                            let mut bag = mettail_runtime::HashBag::new();
                                                            bag.insert(elem1.clone());
                                                            bag.insert(elem2.clone());
                                                            bag.insert(elem3.clone());
                                                            terms.push(Proc::PPar(bag));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {}
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
        /// * `max_collection_width` - Maximum number of elements in any collection
        ///
        /// # Returns
        /// Sorted, deduplicated vector of terms
        ///
        /// # Warning
        /// Number of terms grows exponentially with depth and collection width!
        /// Recommend max_depth <= 3 and max_collection_width <= 2 for exhaustive generation.
        pub fn generate_terms(
            vars: &[String],
            max_depth: usize,
            max_collection_width: usize,
        ) -> Vec<Proc> {
            let ctx = GenerationContext::new(
                vars.to_vec(),
                max_depth,
                max_collection_width,
            );
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
        /// * `max_collection_width` - Maximum number of elements in any collection
        ///
        /// # Returns
        /// Sorted, deduplicated vector of terms
        ///
        /// # Warning
        /// Number of terms grows exponentially with depth and collection width!
        /// Recommend max_depth <= 3 and max_collection_width <= 2 for exhaustive generation.
        pub fn generate_terms(
            vars: &[String],
            max_depth: usize,
            max_collection_width: usize,
        ) -> Vec<Name> {
            let ctx = GenerationContext::new(
                vars.to_vec(),
                max_depth,
                max_collection_width,
            );
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
        /// * `max_collection_width` - Maximum number of elements in any collection
        ///
        /// # Example
        /// ```ignore
        /// let term = Proc::generate_random_at_depth(&["a".into(), "b".into()], 25, 3);
        /// ```
        pub fn generate_random_at_depth(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
        ) -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            Self::generate_random_at_depth_internal(
                vars,
                depth,
                max_collection_width,
                &mut rng,
                0,
            )
        }
        /// Generate a random term at exactly the given depth with a seed
        ///
        /// This is deterministic - same seed produces same term.
        ///
        /// # Arguments
        /// * `vars` - Pool of variable names for free variables
        /// * `depth` - Target depth (operator nesting level)
        /// * `max_collection_width` - Maximum number of elements in any collection
        /// * `seed` - Random seed for reproducibility
        pub fn generate_random_at_depth_with_seed(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
            seed: u64,
        ) -> Self {
            use rand::{SeedableRng, Rng};
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            Self::generate_random_at_depth_internal(
                vars,
                depth,
                max_collection_width,
                &mut rng,
                0,
            )
        }
        fn generate_random_at_depth_internal<R: rand::Rng>(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
            rng: &mut R,
            binding_depth: usize,
        ) -> Self {
            if depth == 0 {
                {
                    let choice = rng.gen_range(0..2usize);
                    match choice {
                        0usize => Proc::PZero,
                        1usize => {
                            if !vars.is_empty() {
                                let idx = rng.gen_range(0..vars.len());
                                Proc::PVar(
                                    mettail_runtime::OrdVar(
                                        mettail_runtime::Var::Free(
                                            mettail_runtime::get_or_create_var(&vars[idx]),
                                        ),
                                    ),
                                )
                            } else {
                                Proc::PVar(
                                    mettail_runtime::OrdVar(
                                        mettail_runtime::Var::Free(
                                            mettail_runtime::get_or_create_var("_"),
                                        ),
                                    ),
                                )
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            } else {
                {
                    let choice = rng.gen_range(0..4usize);
                    match choice {
                        0usize => {
                            let arg = Name::generate_random_at_depth_internal(
                                vars,
                                depth - 1,
                                max_collection_width,
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
                                max_collection_width,
                                rng,
                                binding_depth,
                            );
                            let arg2 = Proc::generate_random_at_depth_internal(
                                vars,
                                d2,
                                max_collection_width,
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
                                max_collection_width,
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
                                max_collection_width,
                                rng,
                                binding_depth + 1,
                            );
                            let binder_var = mettail_runtime::get_or_create_var(
                                &binder_name,
                            );
                            let binder = mettail_runtime::Binder(binder_var);
                            let scope = mettail_runtime::Scope::new(
                                binder,
                                Box::new(body),
                            );
                            Proc::PInput(Box::new(arg1), scope)
                        }
                        3usize => {
                            let size = rng.gen_range(0..=max_collection_width);
                            let mut bag = mettail_runtime::HashBag::new();
                            for _ in 0..size {
                                let elem_depth = if depth > 0 {
                                    rng.gen_range(0..depth)
                                } else {
                                    0
                                };
                                let elem = Proc::generate_random_at_depth_internal(
                                    vars,
                                    elem_depth,
                                    max_collection_width,
                                    rng,
                                    binding_depth,
                                );
                                bag.insert(elem);
                            }
                            Proc::PPar(bag)
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
        /// * `max_collection_width` - Maximum number of elements in any collection
        ///
        /// # Example
        /// ```ignore
        /// let term = Proc::generate_random_at_depth(&["a".into(), "b".into()], 25, 3);
        /// ```
        pub fn generate_random_at_depth(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
        ) -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            Self::generate_random_at_depth_internal(
                vars,
                depth,
                max_collection_width,
                &mut rng,
                0,
            )
        }
        /// Generate a random term at exactly the given depth with a seed
        ///
        /// This is deterministic - same seed produces same term.
        ///
        /// # Arguments
        /// * `vars` - Pool of variable names for free variables
        /// * `depth` - Target depth (operator nesting level)
        /// * `max_collection_width` - Maximum number of elements in any collection
        /// * `seed` - Random seed for reproducibility
        pub fn generate_random_at_depth_with_seed(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
            seed: u64,
        ) -> Self {
            use rand::{SeedableRng, Rng};
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            Self::generate_random_at_depth_internal(
                vars,
                depth,
                max_collection_width,
                &mut rng,
                0,
            )
        }
        fn generate_random_at_depth_internal<R: rand::Rng>(
            vars: &[String],
            depth: usize,
            max_collection_width: usize,
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
                                max_collection_width,
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
        use mettail_runtime::{Var, Binder, Scope};
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
            use mettail_runtime::{Var, Binder, Scope};
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
                Variant1(Proc),
                Variant2(alloc::vec::Vec<Proc>),
                Variant3(String),
                Variant4(Name),
                Variant5(core::option::Option<Proc>),
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
                13,
                0,
                0,
                0,
                0,
                17,
                0,
                18,
                12,
                19,
                3,
                0,
                13,
                0,
                0,
                0,
                0,
                17,
                0,
                18,
                12,
                19,
                3,
                23,
                13,
                0,
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
                13,
                0,
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
                13,
                0,
                0,
                0,
                0,
                17,
                0,
                18,
                12,
                19,
                3,
                27,
                13,
                0,
                0,
                0,
                0,
                17,
                0,
                18,
                12,
                19,
                3,
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
                0,
                0,
                0,
                17,
                0,
                18,
                12,
                19,
                3,
                0,
                13,
                0,
                0,
                0,
                -8,
                0,
                -8,
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
                -6,
                0,
                -6,
                0,
                -6,
                0,
                0,
                0,
                0,
                -6,
                0,
                -6,
                -8,
                0,
                -17,
                0,
                0,
                0,
                0,
                0,
                0,
                -17,
                0,
                -17,
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
                4,
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
                -9,
                0,
                0,
                0,
                0,
                0,
                0,
                -9,
                0,
                -9,
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
                -7,
                0,
                -7,
                0,
                -7,
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
                28,
                0,
                29,
                0,
                0,
                -14,
                0,
                0,
                0,
                0,
                0,
                0,
                -14,
                0,
                -14,
                0,
                0,
                31,
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
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                32,
                0,
                33,
                0,
                0,
                -16,
                0,
                0,
                0,
                0,
                0,
                0,
                -16,
                0,
                -16,
                0,
                0,
                -13,
                0,
                0,
                0,
                0,
                0,
                0,
                -13,
                0,
                -13,
                0,
                0,
                0,
                -4,
                0,
                -4,
                -4,
                -4,
                -4,
                -4,
                -4,
                0,
                0,
                0,
                34,
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
                -10,
                0,
                0,
                0,
                0,
                0,
                0,
                -10,
                0,
                -10,
                0,
                0,
                -15,
                0,
                0,
                0,
                0,
                0,
                0,
                -15,
                0,
                -15,
                0,
                0,
                0,
                -5,
                0,
                -5,
                -5,
                -5,
                -5,
                -5,
                -5,
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
                0,
                -11,
                0,
                0,
                36,
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
                38,
                0,
                0,
                0,
                0,
                -12,
                0,
                0,
                0,
                0,
                0,
                0,
                -12,
                0,
                -12,
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
                -8,
                -20,
                0,
                -6,
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
                    2 => 5,
                    3 => {
                        match state {
                            0 | 3..=4 => 9,
                            7 => 34,
                            _ => 13,
                        }
                    }
                    4 => {
                        match state {
                            0 => 10,
                            3 => 23,
                            4 => 24,
                            _ => 14,
                        }
                    }
                    5 => {
                        match state {
                            2 => 21,
                            5 => 25,
                            6 => 29,
                            8 => 36,
                            _ => 15,
                        }
                    }
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
                r###""}""###,
                r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
                r###"r#"\\|"#"###,
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
                    __token_to_symbol(
                        token_index,
                        token,
                        core::marker::PhantomData::<(&())>,
                    )
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
                    __expected_tokens_from_states(
                        states,
                        core::marker::PhantomData::<(&())>,
                    )
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
                    Token(2, _) if true => Some(0),
                    Token(3, _) if true => Some(1),
                    Token(4, _) if true => Some(2),
                    Token(5, _) if true => Some(3),
                    Token(6, _) if true => Some(4),
                    Token(7, _) if true => Some(5),
                    Token(8, _) if true => Some(6),
                    Token(9, _) if true => Some(7),
                    Token(10, _) if true => Some(8),
                    Token(11, _) if true => Some(9),
                    Token(0, _) if true => Some(10),
                    Token(1, _) if true => Some(11),
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
                            Token(2, __tok0)
                            | Token(3, __tok0)
                            | Token(4, __tok0)
                            | Token(5, __tok0)
                            | Token(6, __tok0)
                            | Token(7, __tok0)
                            | Token(8, __tok0)
                            | Token(9, __tok0)
                            | Token(10, __tok0)
                            | Token(11, __tok0)
                            | Token(0, __tok0)
                            | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                            _ => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn __simulate_reduce<'input>(
                __reduce_index: i8,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> __state_machine::SimulatedReduce<__StateMachine<'input>> {
                match __reduce_index {
                    0 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 2,
                            nonterminal_produced: 0,
                        }
                    }
                    1 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 0,
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
                            states_to_pop: 2,
                            nonterminal_produced: 2,
                        }
                    }
                    4 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 2,
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
                            states_to_pop: 4,
                            nonterminal_produced: 4,
                        }
                    }
                    7 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 4,
                        }
                    }
                    8 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 5,
                        }
                    }
                    9 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 4,
                            nonterminal_produced: 5,
                        }
                    }
                    10 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 5,
                            nonterminal_produced: 5,
                        }
                    }
                    11 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 9,
                            nonterminal_produced: 5,
                        }
                    }
                    12 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 5,
                        }
                    }
                    13 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 2,
                            nonterminal_produced: 5,
                        }
                    }
                    14 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 4,
                            nonterminal_produced: 5,
                        }
                    }
                    15 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 5,
                        }
                    }
                    16 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 5,
                        }
                    }
                    17 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 6,
                        }
                    }
                    18 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 0,
                            nonterminal_produced: 6,
                        }
                    }
                    19 => __state_machine::SimulatedReduce::Accept,
                    20 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 8,
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
                Result<
                    Name,
                    __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
                >,
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
                        __reduce12(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    13 => {
                        __reduce13(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    14 => {
                        __reduce14(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    15 => {
                        __reduce15(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    16 => {
                        __reduce16(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    17 => {
                        __reduce17(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    18 => {
                        __reduce18(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    19 => {
                        let __sym0 = __pop_Variant4(__symbols);
                        let __start = __sym0.0;
                        let __end = __sym0.2;
                        let __nt = super::__action1(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    20 => {
                        __reduce20(
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
            fn __pop_Variant4<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Name, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant1<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Proc, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant3<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, String, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant2<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, alloc::vec::Vec<Proc>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant5<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, core::option::Option<Proc>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
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
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action15(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            fn __reduce1<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __start = __lookahead_start
                    .cloned()
                    .or_else(|| __symbols.last().map(|s| s.2))
                    .unwrap_or_default();
                let __end = __start;
                let __nt = super::__action13(input, &__start, &__end);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (0, 1)
            }
            fn __reduce2<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action14(input, __sym0);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            fn __reduce3<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action18(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (2, 2)
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
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action19(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (3, 2)
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
                let __nt = super::__action2(input, __sym0);
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 3)
            }
            fn __reduce6<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 4)
            }
            fn __reduce7<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action10(input, __sym0);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            fn __reduce8<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action3(input, __sym0);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            fn __reduce9<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (4, 5)
            }
            fn __reduce10<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 5) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
                }
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant1(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0;
                let __end = __sym4.2;
                let __nt = super::__action5(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                );
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (5, 5)
            }
            fn __reduce11<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 9) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 9")
                }
                let __sym8 = __pop_Variant0(__symbols);
                let __sym7 = __pop_Variant1(__symbols);
                let __sym6 = __pop_Variant0(__symbols);
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant3(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym8.2;
                let __nt = super::__action6(
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
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (9, 5)
            }
            fn __reduce12<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 5)
            }
            fn __reduce13<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action23(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 5)
            }
            fn __reduce14<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action24(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (4, 5)
            }
            fn __reduce15<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 5)
            }
            fn __reduce16<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action8(input, __sym0);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            fn __reduce17<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action11(input, __sym0);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (1, 6)
            }
            fn __reduce18<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __start = __lookahead_start
                    .cloned()
                    .or_else(|| __symbols.last().map(|s| s.2))
                    .unwrap_or_default();
                let __end = __start;
                let __nt = super::__action12(input, &__start, &__end);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (0, 6)
            }
            fn __reduce20<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0(input, __sym0);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 8)
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
            use mettail_runtime::{Var, Binder, Scope};
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
                Variant1(Proc),
                Variant2(alloc::vec::Vec<Proc>),
                Variant3(String),
                Variant4(Name),
                Variant5(core::option::Option<Proc>),
            }
            const __ACTION: &[i8] = &[
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                0,
                17,
                0,
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                20,
                17,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                15,
                0,
                0,
                0,
                17,
                0,
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                0,
                17,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                15,
                0,
                0,
                0,
                17,
                0,
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                26,
                17,
                0,
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                0,
                17,
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
                17,
                0,
                0,
                0,
                0,
                13,
                0,
                14,
                15,
                16,
                2,
                0,
                17,
                0,
                -8,
                0,
                -17,
                0,
                0,
                0,
                0,
                0,
                0,
                -17,
                0,
                -17,
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
                3,
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
                -9,
                0,
                0,
                0,
                0,
                0,
                0,
                -9,
                0,
                -9,
                0,
                4,
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
                -6,
                0,
                -6,
                0,
                -6,
                0,
                0,
                0,
                0,
                -6,
                0,
                -6,
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
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                27,
                0,
                28,
                0,
                0,
                -14,
                0,
                0,
                0,
                0,
                0,
                0,
                -14,
                0,
                -14,
                0,
                0,
                -8,
                0,
                -8,
                0,
                0,
                0,
                0,
                0,
                0,
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
                31,
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
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                32,
                0,
                33,
                0,
                0,
                -16,
                0,
                0,
                0,
                0,
                0,
                0,
                -16,
                0,
                -16,
                0,
                0,
                -13,
                0,
                0,
                0,
                0,
                0,
                0,
                -13,
                0,
                -13,
                0,
                0,
                0,
                -4,
                0,
                -4,
                -4,
                -4,
                -4,
                -4,
                -4,
                0,
                0,
                0,
                34,
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
                -10,
                0,
                0,
                0,
                0,
                0,
                0,
                -10,
                0,
                -10,
                -7,
                0,
                -7,
                0,
                -7,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                -15,
                0,
                0,
                0,
                0,
                0,
                0,
                -15,
                0,
                -15,
                0,
                0,
                0,
                -5,
                0,
                -5,
                -5,
                -5,
                -5,
                -5,
                -5,
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
                0,
                -11,
                0,
                0,
                36,
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
                38,
                0,
                0,
                0,
                0,
                -12,
                0,
                0,
                0,
                0,
                0,
                0,
                -12,
                0,
                -12,
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
                -17,
                0,
                -21,
                0,
                -9,
                0,
                0,
                -6,
                0,
                0,
                -14,
                0,
                0,
                0,
                0,
                0,
                -16,
                -13,
                0,
                0,
                -10,
                0,
                -15,
                0,
                -11,
                0,
                0,
                0,
                -12,
            ];
            fn __goto(state: i8, nt: usize) -> i8 {
                match nt {
                    2 => 5,
                    3 => {
                        match state {
                            2 | 4 => 20,
                            7 => 34,
                            _ => 9,
                        }
                    }
                    4 => {
                        match state {
                            2 => 21,
                            4 => 23,
                            _ => 10,
                        }
                    }
                    5 => {
                        match state {
                            1 => 18,
                            3 => 22,
                            5 => 24,
                            6 => 28,
                            8 => 36,
                            _ => 11,
                        }
                    }
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
                r###""}""###,
                r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
                r###"r#"\\|"#"###,
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
                    __token_to_symbol(
                        token_index,
                        token,
                        core::marker::PhantomData::<(&())>,
                    )
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
                    __expected_tokens_from_states(
                        states,
                        core::marker::PhantomData::<(&())>,
                    )
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
                    Token(2, _) if true => Some(0),
                    Token(3, _) if true => Some(1),
                    Token(4, _) if true => Some(2),
                    Token(5, _) if true => Some(3),
                    Token(6, _) if true => Some(4),
                    Token(7, _) if true => Some(5),
                    Token(8, _) if true => Some(6),
                    Token(9, _) if true => Some(7),
                    Token(10, _) if true => Some(8),
                    Token(11, _) if true => Some(9),
                    Token(0, _) if true => Some(10),
                    Token(1, _) if true => Some(11),
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
                            Token(2, __tok0)
                            | Token(3, __tok0)
                            | Token(4, __tok0)
                            | Token(5, __tok0)
                            | Token(6, __tok0)
                            | Token(7, __tok0)
                            | Token(8, __tok0)
                            | Token(9, __tok0)
                            | Token(10, __tok0)
                            | Token(11, __tok0)
                            | Token(0, __tok0)
                            | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                            _ => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn __simulate_reduce<'input>(
                __reduce_index: i8,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> __state_machine::SimulatedReduce<__StateMachine<'input>> {
                match __reduce_index {
                    0 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 2,
                            nonterminal_produced: 0,
                        }
                    }
                    1 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 0,
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
                            states_to_pop: 2,
                            nonterminal_produced: 2,
                        }
                    }
                    4 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 2,
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
                            states_to_pop: 4,
                            nonterminal_produced: 4,
                        }
                    }
                    7 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 4,
                        }
                    }
                    8 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 5,
                        }
                    }
                    9 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 4,
                            nonterminal_produced: 5,
                        }
                    }
                    10 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 5,
                            nonterminal_produced: 5,
                        }
                    }
                    11 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 9,
                            nonterminal_produced: 5,
                        }
                    }
                    12 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 5,
                        }
                    }
                    13 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 2,
                            nonterminal_produced: 5,
                        }
                    }
                    14 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 4,
                            nonterminal_produced: 5,
                        }
                    }
                    15 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 3,
                            nonterminal_produced: 5,
                        }
                    }
                    16 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 5,
                        }
                    }
                    17 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 6,
                        }
                    }
                    18 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 0,
                            nonterminal_produced: 6,
                        }
                    }
                    19 => {
                        __state_machine::SimulatedReduce::Reduce {
                            states_to_pop: 1,
                            nonterminal_produced: 7,
                        }
                    }
                    20 => __state_machine::SimulatedReduce::Accept,
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
                Result<
                    Proc,
                    __lalrpop_util::ParseError<usize, Token<'input>, &'static str>,
                >,
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
                        __reduce12(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    13 => {
                        __reduce13(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    14 => {
                        __reduce14(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    15 => {
                        __reduce15(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    16 => {
                        __reduce16(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    17 => {
                        __reduce17(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    18 => {
                        __reduce18(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    19 => {
                        __reduce19(
                            input,
                            __lookahead_start,
                            __symbols,
                            core::marker::PhantomData::<(&())>,
                        )
                    }
                    20 => {
                        let __sym0 = __pop_Variant1(__symbols);
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
            fn __pop_Variant4<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Name, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant1<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, Proc, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant3<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, String, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant2<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, alloc::vec::Vec<Proc>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
                    _ => __symbol_type_mismatch(),
                }
            }
            fn __pop_Variant5<'input>(
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
            ) -> (usize, core::option::Option<Proc>, usize) {
                match __symbols.pop() {
                    Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
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
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action15(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 0)
            }
            fn __reduce1<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __start = __lookahead_start
                    .cloned()
                    .or_else(|| __symbols.last().map(|s| s.2))
                    .unwrap_or_default();
                let __end = __start;
                let __nt = super::__action13(input, &__start, &__end);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (0, 1)
            }
            fn __reduce2<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action14(input, __sym0);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 1)
            }
            fn __reduce3<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action18(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (2, 2)
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
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action19(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (3, 2)
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
                let __nt = super::__action2(input, __sym0);
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 3)
            }
            fn __reduce6<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action9(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 4)
            }
            fn __reduce7<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action10(input, __sym0);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 4)
            }
            fn __reduce8<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action3(input, __sym0);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            fn __reduce9<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (4, 5)
            }
            fn __reduce10<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 5) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 5")
                }
                let __sym4 = __pop_Variant0(__symbols);
                let __sym3 = __pop_Variant1(__symbols);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0;
                let __end = __sym4.2;
                let __nt = super::__action5(
                    input,
                    __sym0,
                    __sym1,
                    __sym2,
                    __sym3,
                    __sym4,
                );
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (5, 5)
            }
            fn __reduce11<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 9) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 9")
                }
                let __sym8 = __pop_Variant0(__symbols);
                let __sym7 = __pop_Variant1(__symbols);
                let __sym6 = __pop_Variant0(__symbols);
                let __sym5 = __pop_Variant0(__symbols);
                let __sym4 = __pop_Variant3(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant4(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym8.2;
                let __nt = super::__action6(
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
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (9, 5)
            }
            fn __reduce12<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action22(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 5)
            }
            fn __reduce13<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 2) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 2")
                }
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym1.2;
                let __nt = super::__action23(input, __sym0, __sym1);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 5)
            }
            fn __reduce14<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 4) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 4")
                }
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym3.2;
                let __nt = super::__action24(input, __sym0, __sym1, __sym2, __sym3);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (4, 5)
            }
            fn __reduce15<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                if !(__symbols.len() >= 3) {
                    ::core::panicking::panic("assertion failed: __symbols.len() >= 3")
                }
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0;
                let __end = __sym2.2;
                let __nt = super::__action25(input, __sym0, __sym1, __sym2);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (3, 5)
            }
            fn __reduce16<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action8(input, __sym0);
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            fn __reduce17<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action11(input, __sym0);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (1, 6)
            }
            fn __reduce18<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __start = __lookahead_start
                    .cloned()
                    .or_else(|| __symbols.last().map(|s| s.2))
                    .unwrap_or_default();
                let __end = __start;
                let __nt = super::__action12(input, &__start, &__end);
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (0, 6)
            }
            fn __reduce19<'input>(
                input: &'input str,
                __lookahead_start: Option<&usize>,
                __symbols: &mut alloc::vec::Vec<(usize, __Symbol<'input>, usize)>,
                _: core::marker::PhantomData<(&'input ())>,
            ) -> (usize, usize) {
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1(input, __sym0);
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 7)
            }
        }
        #[allow(unused_imports)]
        pub use self::__parse__Proc::ProcParser;
        #[rustfmt::skip]
        mod __intern_token {
            #![allow(unused_imports)]
            use mettail_runtime::{Var, Binder, Scope};
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
                    ("\\|", false),
                    ("!", false),
                    ("\\(", false),
                    ("\\)", false),
                    ("\\*", false),
                    ("(?:\\->)", false),
                    ("0", false),
                    ("@", false),
                    ("(?:for)", false),
                    ("\\{", false),
                    ("\\}", false),
                    (r"\s+", true),
                ];
                __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied())
                    .unwrap()
            }
        }
        pub(crate) use self::__lalrpop_util::lexer::Token;
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action0<'input>(
            input: &'input str,
            (_, __0, _): (usize, Proc, usize),
        ) -> Proc {
            __0
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action1<'input>(
            input: &'input str,
            (_, __0, _): (usize, Name, usize),
        ) -> Name {
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
        fn __action3<'input>(
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
        fn __action4<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, _, _): (usize, &'input str, usize),
            (_, f0, _): (usize, Name, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Proc {
            Proc::PDrop(Box::new(f0))
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action5<'input>(
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
        fn __action6<'input>(
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
        fn __action7<'input>(
            input: &'input str,
            (_, _, _): (usize, &'input str, usize),
            (_, elems, _): (usize, alloc::vec::Vec<Proc>, usize),
            (_, last, _): (usize, core::option::Option<Proc>, usize),
            (_, _, _): (usize, &'input str, usize),
        ) -> Proc {
            {
                let mut coll = mettail_runtime::HashBag::new();
                for e in elems {
                    coll.insert(e);
                }
                if let Some(e) = last {
                    coll.insert(e);
                }
                Proc::PPar(coll)
            }
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action8<'input>(
            input: &'input str,
            (_, v, _): (usize, String, usize),
        ) -> Proc {
            Proc::PVar(
                mettail_runtime::OrdVar(Var::Free(mettail_runtime::get_or_create_var(v))),
            )
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action9<'input>(
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
        fn __action10<'input>(
            input: &'input str,
            (_, v, _): (usize, String, usize),
        ) -> Name {
            Name::NVar(
                mettail_runtime::OrdVar(Var::Free(mettail_runtime::get_or_create_var(v))),
            )
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action11<'input>(
            input: &'input str,
            (_, __0, _): (usize, Proc, usize),
        ) -> core::option::Option<Proc> {
            Some(__0)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action12<'input>(
            input: &'input str,
            __lookbehind: &usize,
            __lookahead: &usize,
        ) -> core::option::Option<Proc> {
            None
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action13<'input>(
            input: &'input str,
            __lookbehind: &usize,
            __lookahead: &usize,
        ) -> alloc::vec::Vec<Proc> {
            ::alloc::vec::Vec::new()
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action14<'input>(
            input: &'input str,
            (_, v, _): (usize, alloc::vec::Vec<Proc>, usize),
        ) -> alloc::vec::Vec<Proc> {
            v
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action15<'input>(
            input: &'input str,
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
        fn __action16<'input>(
            input: &'input str,
            (_, __0, _): (usize, Proc, usize),
        ) -> alloc::vec::Vec<Proc> {
            <[_]>::into_vec(::alloc::boxed::box_new([__0]))
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action17<'input>(
            input: &'input str,
            (_, v, _): (usize, alloc::vec::Vec<Proc>, usize),
            (_, e, _): (usize, Proc, usize),
        ) -> alloc::vec::Vec<Proc> {
            {
                let mut v = v;
                v.push(e);
                v
            }
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action18<'input>(
            input: &'input str,
            __0: (usize, Proc, usize),
            __1: (usize, &'input str, usize),
        ) -> alloc::vec::Vec<Proc> {
            let __start0 = __0.0;
            let __end0 = __1.2;
            let __temp0 = __action15(input, __0, __1);
            let __temp0 = (__start0, __temp0, __end0);
            __action16(input, __temp0)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action19<'input>(
            input: &'input str,
            __0: (usize, alloc::vec::Vec<Proc>, usize),
            __1: (usize, Proc, usize),
            __2: (usize, &'input str, usize),
        ) -> alloc::vec::Vec<Proc> {
            let __start0 = __1.0;
            let __end0 = __2.2;
            let __temp0 = __action15(input, __1, __2);
            let __temp0 = (__start0, __temp0, __end0);
            __action17(input, __0, __temp0)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action20<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, core::option::Option<Proc>, usize),
            __2: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __0.2;
            let __end0 = __1.0;
            let __temp0 = __action13(input, &__start0, &__end0);
            let __temp0 = (__start0, __temp0, __end0);
            __action7(input, __0, __temp0, __1, __2)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action21<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, alloc::vec::Vec<Proc>, usize),
            __2: (usize, core::option::Option<Proc>, usize),
            __3: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __1.0;
            let __end0 = __1.2;
            let __temp0 = __action14(input, __1);
            let __temp0 = (__start0, __temp0, __end0);
            __action7(input, __0, __temp0, __2, __3)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action22<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, Proc, usize),
            __2: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __1.0;
            let __end0 = __1.2;
            let __temp0 = __action11(input, __1);
            let __temp0 = (__start0, __temp0, __end0);
            __action20(input, __0, __temp0, __2)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action23<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __0.2;
            let __end0 = __1.0;
            let __temp0 = __action12(input, &__start0, &__end0);
            let __temp0 = (__start0, __temp0, __end0);
            __action20(input, __0, __temp0, __1)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action24<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, alloc::vec::Vec<Proc>, usize),
            __2: (usize, Proc, usize),
            __3: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __2.0;
            let __end0 = __2.2;
            let __temp0 = __action11(input, __2);
            let __temp0 = (__start0, __temp0, __end0);
            __action21(input, __0, __1, __temp0, __3)
        }
        #[allow(unused_variables)]
        #[allow(
            clippy::too_many_arguments,
            clippy::needless_lifetimes,
            clippy::just_underscores_and_digits
        )]
        fn __action25<'input>(
            input: &'input str,
            __0: (usize, &'input str, usize),
            __1: (usize, alloc::vec::Vec<Proc>, usize),
            __2: (usize, &'input str, usize),
        ) -> Proc {
            let __start0 = __1.2;
            let __end0 = __2.0;
            let __temp0 = __action12(input, &__start0, &__end0);
            let __temp0 = (__start0, __temp0, __end0);
            __action21(input, __0, __1, __temp0, __2)
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
    pub fn is_fresh<T>(binder: &mettail_runtime::Binder<String>, term: &T) -> bool
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
}
