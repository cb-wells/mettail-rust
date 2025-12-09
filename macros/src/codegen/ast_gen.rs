#![allow(clippy::cmp_owned, clippy::single_match)]

use super::{display, is_var_rule, subst, termgen};
use crate::ast::{GrammarItem, GrammarRule, TheoryDef};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn generate_ast(theory: &TheoryDef) -> TokenStream {
    let ast_enums = generate_ast_enums(theory);
    let flatten_helpers = generate_flatten_helpers(theory);
    let normalize_impl = generate_normalize_functions(theory);
    let subst_impl = subst::generate_substitution(theory);
    let display_impl = display::generate_display(theory);
    let generation_impl = termgen::generate_term_generation(theory);
    let random_gen_impl = termgen::generate_random_generation(theory);

    // Generate LALRPOP module reference
    let theory_name = &theory.name;
    let theory_name_lower = theory_name.to_string().to_lowercase();
    let theory_mod = syn::Ident::new(&theory_name_lower, proc_macro2::Span::call_site());

    quote! {
        use lalrpop_util::lalrpop_mod;

        #ast_enums

        #flatten_helpers

        #normalize_impl

        #subst_impl

        #display_impl

        #generation_impl

        #random_gen_impl

        #[cfg(not(test))]
        lalrpop_util::lalrpop_mod!(pub #theory_mod);

        #[cfg(test)]
        lalrpop_util::lalrpop_mod!(#theory_mod);
    }
}

/// Generate just the AST enums (without parser)
fn generate_ast_enums(theory: &TheoryDef) -> TokenStream {
    // Group rules by category
    let mut rules_by_cat: HashMap<String, Vec<&GrammarRule>> = HashMap::new();

    for rule in &theory.terms {
        let cat_name = rule.category.to_string();
        rules_by_cat.entry(cat_name).or_default().push(rule);
    }

    // Generate enum for each exported category
    let enums: Vec<TokenStream> = theory.exports.iter().map(|export| {
        let cat_name = &export.name;

        let rules = rules_by_cat
            .get(&cat_name.to_string())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        // Check if there's already a Var rule
        let has_var_rule = rules.iter().any(|rule| is_var_rule(rule));

        let mut variants: Vec<TokenStream> = rules.iter().map(|rule| {
            generate_variant(rule)
        }).collect();

        // Automatically add Var variant if it doesn't exist
        if !has_var_rule {
            use quote::format_ident;
            // Generate label: first letter of category + "Var"
            let cat_str = cat_name.to_string();
            let first_letter = cat_str.chars().next().unwrap_or('V').to_uppercase().collect::<String>();
            let var_label = format_ident!("{}Var", first_letter);

            variants.push(quote! {
                #var_label(mettail_runtime::OrdVar)
            });
        }

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, mettail_runtime::BoundTerm)]
            pub enum #cat_name {
                #(#variants),*
            }
        }
    }).collect();

    quote! {
        #(#enums)*
    }
}

fn generate_variant(rule: &GrammarRule) -> TokenStream {
    let label = &rule.label;

    // Check if this rule has bindings
    if !rule.bindings.is_empty() {
        // This constructor has binders - generate Scope type
        return generate_binder_variant(rule);
    }

    // Count non-terminal and collection items (these become fields)
    #[derive(Clone)]
    enum FieldType {
        NonTerminal(syn::Ident),
        Collection {
            coll_type: crate::ast::CollectionType,
            element_type: syn::Ident,
        },
    }

    let fields: Vec<FieldType> = rule
        .items
        .iter()
        .filter_map(|item| match item {
            GrammarItem::NonTerminal(ident) => Some(FieldType::NonTerminal(ident.clone())),
            GrammarItem::Collection { coll_type, element_type, .. } => {
                Some(FieldType::Collection {
                    coll_type: coll_type.clone(),
                    element_type: element_type.clone(),
                })
            },
            GrammarItem::Binder { .. } => None, // Handled above
            _ => None,
        })
        .collect();

    if fields.is_empty() {
        // Unit variant
        quote! { #label }
    } else if fields.len() == 1 {
        #[allow(clippy::cmp_owned)]
        match &fields[0] {
            FieldType::NonTerminal(ident) if ident.to_string() == "Var" => {
                // Special case: Var field -> generate OrdVar directly (not boxed)
                quote! { #label(mettail_runtime::OrdVar) }
            },
            FieldType::NonTerminal(ident) => {
                // Single non-terminal field
                quote! { #label(Box<#ident>) }
            },
            FieldType::Collection { coll_type, element_type } => {
                // Single collection field
                let coll_type_ident = match coll_type {
                    crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                    crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                    crate::ast::CollectionType::Vec => quote! { Vec },
                };
                quote! { #label(#coll_type_ident<#element_type>) }
            },
        }
    } else {
        // Multiple fields - tuple variant
        let field_types: Vec<TokenStream> = fields
            .iter()
            .map(|f| match f {
                FieldType::NonTerminal(ident) if ident.to_string() == "Var" => {
                    quote! { mettail_runtime::OrdVar }
                },
                FieldType::NonTerminal(ident) => {
                    quote! { Box<#ident> }
                },
                FieldType::Collection { coll_type, element_type } => {
                    let coll_type_ident = match coll_type {
                        crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                        crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                        crate::ast::CollectionType::Vec => quote! { Vec },
                    };
                    quote! { #coll_type_ident<#element_type> }
                },
            })
            .collect();

        quote! { #label(#(#field_types),*) }
    }
}

fn generate_binder_variant(rule: &GrammarRule) -> TokenStream {
    let label = &rule.label;

    // For now, support single binder binding in single body
    // Future: support multiple binders and bodies
    let (binder_idx, body_indices) = &rule.bindings[0];
    let body_idx = body_indices[0];

    // Get the binder and body categories
    let _binder_cat = match &rule.items[*binder_idx] {
        GrammarItem::Binder { category } => category,
        _ => panic!("Binding index doesn't point to a Binder"),
    };

    let body_cat = match &rule.items[body_idx] {
        GrammarItem::NonTerminal(cat) => cat,
        _ => panic!("Body index doesn't point to a NonTerminal"),
    };

    let mut fields = Vec::new();

    for (i, item) in rule.items.iter().enumerate() {
        if i == *binder_idx {
            // Skip the binder - it's part of the Scope
            continue;
        }

        if i == body_idx {
            // This is the body - generate Scope
            fields.push(quote! {
                mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<#body_cat>>
            });
        } else {
            // Regular field (comes before or after, but not binder or body)
            match item {
                GrammarItem::NonTerminal(cat) => {
                    if cat.to_string() == "Var" {
                        fields.push(quote! { mettail_runtime::Var<String> });
                    } else {
                        fields.push(quote! { Box<#cat> });
                    }
                },
                GrammarItem::Collection { coll_type, element_type, .. } => {
                    // Collection becomes a field with the appropriate collection type
                    let coll_type_ident = match coll_type {
                        crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                        crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                        crate::ast::CollectionType::Vec => quote! { Vec },
                    };
                    fields.push(quote! { #coll_type_ident<#element_type> });
                },
                GrammarItem::Binder { .. } => {
                    // Should have been skipped above
                    panic!("Unexpected binder at position {}", i);
                },
                GrammarItem::Terminal(_) => {
                    // Terminals don't become fields
                },
            }
        }
    }

    // Generate the variant
    quote! {
        #label(#(#fields),*)
    }
}

/// Generate automatic flattening helpers for collection constructors
///
/// For each constructor with a collection field, generates a helper function
/// that automatically flattens nested collections of the same type.
///
/// Example generated code:
/// ```ignore
/// impl Proc {
///     fn insert_into_ppar(bag: &mut mettail_runtime::HashBag<Proc>, elem: Proc) {
///         match elem {
///             Proc::PPar(inner) => {
///                 // Recursively flatten nested PPar
///                 for (e, count) in inner.iter() {
///                     for _ in 0..*count {
///                         Self::insert_into_ppar(bag, e.clone());
///                     }
///                 }
///             }
///             _ => bag.insert(elem),
///         }
///     }
/// }
/// ```
fn generate_flatten_helpers(theory: &TheoryDef) -> TokenStream {
    use quote::format_ident;

    // Group rules by category
    let mut helpers_by_cat: HashMap<String, Vec<TokenStream>> = HashMap::new();

    for rule in &theory.terms {
        // Check if this rule has a collection field
        let has_collection = rule
            .items
            .iter()
            .any(|item| matches!(item, GrammarItem::Collection { .. }));

        if !has_collection {
            continue;
        }

        let category = &rule.category;
        let label = &rule.label;
        let helper_name = format_ident!("insert_into_{}", label.to_string().to_lowercase());

        let helper = quote! {
            /// Auto-flattening insert for #label
            ///
            /// If elem is itself a #label, recursively merges its contents instead of nesting.
            /// This ensures that collection constructors are always flat, never nested.
            pub fn #helper_name(
                bag: &mut mettail_runtime::HashBag<#category>,
                elem: #category
            ) {
                match elem {
                    #category::#label(inner) => {
                        // Flatten: recursively merge inner bag contents
                        for (e, count) in inner.iter() {
                            for _ in 0..count {
                                // Recursive call handles multi-level nesting
                                Self::#helper_name(bag, e.clone());
                            }
                        }
                    }
                    _ => {
                        // Normal insert - not a nested collection
                        bag.insert(elem);
                    }
                }
            }
        };

        helpers_by_cat
            .entry(category.to_string())
            .or_default()
            .push(helper);
    }

    // Generate impl blocks for each category
    let impls: Vec<TokenStream> = theory
        .exports
        .iter()
        .filter_map(|export| {
            let cat_name = &export.name;
            let helpers = helpers_by_cat.get(&cat_name.to_string())?;

            if helpers.is_empty() {
                return None;
            }

            Some(quote! {
                impl #cat_name {
                    #(#helpers)*
                }
            })
        })
        .collect();

    quote! {
        #(#impls)*
    }
}

/// Generate normalize functions that recursively flatten nested collections
fn generate_normalize_functions(theory: &TheoryDef) -> TokenStream {
    use quote::format_ident;

    let mut impls = Vec::new();

    for export in &theory.exports {
        let category = &export.name;

        // Find all rules for this category
        let rules_for_category: Vec<_> = theory
            .terms
            .iter()
            .filter(|rule| rule.category == *category)
            .collect();

        // Find collection constructors
        let has_collections = rules_for_category.iter().any(|rule| {
            rule.items
                .iter()
                .any(|item| matches!(item, GrammarItem::Collection { .. }))
        });

        // Only generate normalize if this category has collections
        if !has_collections {
            continue;
        }

        // Generate match arms for each constructor
        let match_arms: Vec<TokenStream> = rules_for_category
            .iter()
            .filter_map(|rule| {
                let label = &rule.label;

                // Check if this is a collection constructor
                let is_collection = rule
                    .items
                    .iter()
                    .any(|item| matches!(item, GrammarItem::Collection { .. }));

                if is_collection {
                    // For collection constructors, rebuild using the flattening helper
                    let helper_name =
                        format_ident!("insert_into_{}", label.to_string().to_lowercase());

                    Some(quote! {
                        #category::#label(bag) => {
                            // Rebuild the bag using the flattening insert helper
                            let mut new_bag = mettail_runtime::HashBag::new();
                            for (elem, count) in bag.iter() {
                                for _ in 0..count {
                                    // Recursively normalize the element before inserting
                                    let normalized_elem = elem.normalize();
                                    Self::#helper_name(&mut new_bag, normalized_elem);
                                }
                            }
                            #category::#label(new_bag)
                        }
                    })
                } else if rule.bindings.is_empty() {
                    // For non-collection, non-binder constructors
                    // Get fields (excluding Terminals)
                    let fields: Vec<_> = rule
                        .items
                        .iter()
                        .filter(|item| {
                            matches!(
                                item,
                                GrammarItem::NonTerminal(_) | GrammarItem::Collection { .. }
                            )
                        })
                        .collect();

                    if fields.is_empty() {
                        // Nullary - no changes needed
                        Some(quote! {
                            #category::#label => self.clone()
                        })
                    } else if fields.len() == 1 {
                        // Single field constructor
                        match fields[0] {
                            GrammarItem::NonTerminal(field_cat) if field_cat == category => {
                                // Recursive case - normalize the field
                                Some(quote! {
                                    #category::#label(f0) => {
                                        #category::#label(Box::new(f0.as_ref().normalize()))
                                    }
                                })
                            },
                            GrammarItem::NonTerminal(field_cat)
                                if field_cat.to_string() == "Var" =>
                            {
                                // Var field - just clone (no Box)
                                Some(quote! {
                                    #category::#label(v) => {
                                        #category::#label(v.clone())
                                    }
                                })
                            },
                            _ => {
                                // Different category or unsupported - just clone
                                Some(quote! {
                                    #category::#label(f0) => {
                                        #category::#label(f0.clone())
                                    }
                                })
                            },
                        }
                    } else {
                        // Multiple fields - skip for now (too complex)
                        None
                    }
                } else {
                    // Binder constructor
                    // Count total AST fields (non-terminal, non-binder)
                    let (_binder_idx, body_indices) = &rule.bindings[0];
                    let body_idx = body_indices[0];

                    let mut field_names = Vec::new();
                    let mut scope_field_idx = None;
                    for (i, item) in rule.items.iter().enumerate() {
                        if i == *_binder_idx {
                            continue; // Skip binder
                        }
                        match item {
                            GrammarItem::NonTerminal(_) => {
                                if i == body_idx {
                                    scope_field_idx = Some(field_names.len());
                                    field_names.push(format_ident!("scope"));
                                } else {
                                    field_names.push(format_ident!("f{}", field_names.len()));
                                }
                            },
                            _ => {},
                        }
                    }

                    let scope_idx = scope_field_idx.expect("Should have scope");

                    // Generate field reconstruction
                    let reconstructed_fields: Vec<_> = field_names
                        .iter()
                        .enumerate()
                        .map(|(i, name)| {
                            if i == scope_idx {
                                quote! {
                                    mettail_runtime::Scope::from_parts_unsafe(
                                        #name.inner().unsafe_pattern.clone(),
                                        Box::new(#name.inner().unsafe_body.as_ref().normalize())
                                    )
                                }
                            } else {
                                quote! { #name.clone() }
                            }
                        })
                        .collect();

                    Some(quote! {
                        #category::#label(#(#field_names),*) => {
                            #category::#label(#(#reconstructed_fields),*)
                        }
                    })
                }
            })
            .collect();

        // Add a fallback for any unhandled patterns
        let fallback = quote! {
            _ => self.clone()
        };

        let impl_block = quote! {
            impl #category {
                /// Recursively normalize this term by flattening any nested collections.
                ///
                /// For example, `PPar({PPar({a, b}), c})` becomes `PPar({a, b, c})`.
                /// This ensures that collection constructors are always in canonical flat form.
                pub fn normalize(&self) -> Self {
                    match self {
                        #(#match_arms,)*
                        #fallback
                    }
                }
            }
        };

        impls.push(impl_block);
    }

    quote! {
        #(#impls)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use syn::parse_quote;

    #[test]
    fn test_generate_simple_enum() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![Export { name: parse_quote!(Elem) }],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(Zero),
                    category: parse_quote!(Elem),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(Plus),
                    category: parse_quote!(Elem),
                    items: vec![
                        GrammarItem::NonTerminal(parse_quote!(Elem)),
                        GrammarItem::Terminal("+".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Elem)),
                    ],
                    bindings: vec![],
                },
            ],
            equations: vec![],
            rewrites: vec![],
        };

        let output = generate_ast(&theory);

        // Check that it generates valid Rust code
        println!("Generated: {}", output);
        assert!(output.to_string().contains("enum Elem"));
        assert!(output.to_string().contains("Zero"));
        assert!(output.to_string().contains("Plus"));
    }

    #[test]
    fn test_generate_multiple_categories() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![Export { name: parse_quote!(Proc) }, Export { name: parse_quote!(Name) }],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(PZero),
                    category: parse_quote!(Proc),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(NQuote),
                    category: parse_quote!(Name),
                    items: vec![
                        GrammarItem::Terminal("@".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Proc)),
                    ],
                    bindings: vec![],
                },
            ],
            equations: vec![],
            rewrites: vec![],
        };

        let output = generate_ast(&theory);

        println!("Generated: {}", output);
        assert!(output.to_string().contains("enum Proc"));
        assert!(output.to_string().contains("enum Name"));
    }

    #[test]
    fn test_automatic_var_generation() {
        // Tests theory without Var rules - they should be automatically generated
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Proc) },
                Export { name: parse_quote!(Name) },
                Export { name: parse_quote!(Term) },
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(PZero),
                    category: parse_quote!(Proc),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(NQuote),
                    category: parse_quote!(Name),
                    items: vec![
                        GrammarItem::Terminal("@".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Proc)),
                    ],
                    bindings: vec![],
                },
                // No Var rules explicitly defined
            ],
            equations: vec![],
            rewrites: vec![],
        };

        let output = generate_ast(&theory);
        let output_str = output.to_string();

        println!("Generated AST:\n{}", output_str);

        // Checks that Var variants are automatically generated for each exported category
        // Looks for the enum definitions and verify they contain Var variants
        // Proc -> PVar
        let proc_enum_start = output_str.find("pub enum Proc").unwrap_or(0);
        let proc_enum_end = output_str[proc_enum_start..]
            .find("impl")
            .unwrap_or(output_str.len() - proc_enum_start);
        let proc_enum_section = &output_str[proc_enum_start..proc_enum_start + proc_enum_end];
        assert!(
            proc_enum_section.contains("PVar") && proc_enum_section.contains("OrdVar"),
            "Expected PVar variant for Proc category in enum definition"
        );

        // Name -> NVar
        let name_enum_start = output_str.find("pub enum Name").unwrap_or(0);
        let name_enum_end = output_str[name_enum_start..]
            .find("impl")
            .unwrap_or(output_str.len() - name_enum_start);
        let name_enum_section = &output_str[name_enum_start..name_enum_start + name_enum_end];
        assert!(
            name_enum_section.contains("NVar") && name_enum_section.contains("OrdVar"),
            "Expected NVar variant for Name category in enum definition"
        );

        // Term -> TVar
        let term_enum_start = output_str.find("pub enum Term").unwrap_or(0);
        let term_enum_end = output_str[term_enum_start..]
            .find("impl")
            .unwrap_or(output_str.len() - term_enum_start);
        let term_enum_section = &output_str[term_enum_start..term_enum_start + term_enum_end];
        assert!(
            term_enum_section.contains("TVar") && term_enum_section.contains("OrdVar"),
            "Expected TVar variant for Term category in enum definition"
        );

        // Verify the enum structure exists
        assert!(output_str.contains("enum Proc"));
        assert!(output_str.contains("enum Name"));
        assert!(output_str.contains("enum Term"));
    }

    #[test]
    fn test_automatic_var_generation_with_existing_var() {
        // Tests that if a Var rule already exists, we don't generate a duplicate
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![Export { name: parse_quote!(Proc) }],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(PZero),
                    category: parse_quote!(Proc),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(PVar),
                    category: parse_quote!(Proc),
                    items: vec![GrammarItem::NonTerminal(parse_quote!(Var))],
                    bindings: vec![],
                },
                // Var rule explicitly defined
            ],
            equations: vec![],
            rewrites: vec![],
        };

        let output = generate_ast(&theory);
        let output_str = output.to_string();

        println!("Generated AST:\n{}", output_str);

        // Should have exactly one PVar variant in the enum definition (the explicitly defined one)
        // Finds the enum definition section
        let proc_enum_start = output_str.find("pub enum Proc").unwrap_or(0);
        let proc_enum_end = output_str[proc_enum_start..]
            .find("impl")
            .unwrap_or(output_str.len() - proc_enum_start);
        let proc_enum_section = &output_str[proc_enum_start..proc_enum_start + proc_enum_end];

        // Counts PVar in the enum definition only
        let pvar_in_enum = proc_enum_section.matches("PVar").count();
        assert_eq!(
            pvar_in_enum, 1,
            "Expected exactly one PVar variant in enum definition, found {}",
            pvar_in_enum
        );
        assert!(
            proc_enum_section.contains("PVar") && proc_enum_section.contains("OrdVar"),
            "Expected PVar variant in enum definition"
        );
    }
}
