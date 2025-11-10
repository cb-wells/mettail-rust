use crate::ast::{TheoryDef, GrammarItem, GrammarRule};
use crate::{subst_gen, display_gen, termgen_gen, random_generation};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub fn generate_ast(theory: &TheoryDef) -> TokenStream {
    let ast_enums = generate_ast_enums(theory);
    let flatten_helpers = generate_flatten_helpers(theory);
    let subst_impl = subst_gen::generate_substitution(theory);
    let display_impl = display_gen::generate_display(theory);
    let generation_impl = termgen_gen::generate_term_generation(theory);
    let random_gen_impl = random_generation::generate_random_generation(theory);
    
    // Generate LALRPOP module reference
    let theory_name = &theory.name;
    let theory_name_lower = theory_name.to_string().to_lowercase();
    let theory_mod = syn::Ident::new(&theory_name_lower, proc_macro2::Span::call_site());
    
    quote! {
        #ast_enums
        
        #flatten_helpers
        
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
        
        let variants: Vec<TokenStream> = rules.iter().map(|rule| {
            generate_variant(rule)
        }).collect();
        
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
    
    let fields: Vec<FieldType> = rule.items
        .iter()
        .filter_map(|item| match item {
            GrammarItem::NonTerminal(ident) => Some(FieldType::NonTerminal(ident.clone())),
            GrammarItem::Collection { coll_type, element_type, .. } => 
                Some(FieldType::Collection { 
                    coll_type: coll_type.clone(), 
                    element_type: element_type.clone() 
                }),
            GrammarItem::Binder { .. } => None, // Handled above
            _ => None,
        })
        .collect();
    
    if fields.is_empty() {
        // Unit variant
        quote! { #label }
    } else if fields.len() == 1 {
        match &fields[0] {
            FieldType::NonTerminal(ident) if ident.to_string() == "Var" => {
                // Special case: Var field -> generate OrdVar directly (not boxed)
                quote! { #label(mettail_runtime::OrdVar) }
            }
            FieldType::NonTerminal(ident) => {
                // Single non-terminal field
                quote! { #label(Box<#ident>) }
            }
            FieldType::Collection { coll_type, element_type } => {
                // Single collection field
                let coll_type_ident = match coll_type {
                    crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                    crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                    crate::ast::CollectionType::Vec => quote! { Vec },
                };
                quote! { #label(#coll_type_ident<#element_type>) }
            }
        }
    } else {
        // Multiple fields - tuple variant
        let field_types: Vec<TokenStream> = fields.iter().map(|f| {
            match f {
                FieldType::NonTerminal(ident) if ident.to_string() == "Var" => {
                    quote! { mettail_runtime::OrdVar }
                }
                FieldType::NonTerminal(ident) => {
                    quote! { Box<#ident> }
                }
                FieldType::Collection { coll_type, element_type } => {
                    let coll_type_ident = match coll_type {
                        crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                        crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                        crate::ast::CollectionType::Vec => quote! { Vec },
                    };
                    quote! { #coll_type_ident<#element_type> }
                }
            }
        }).collect();
        
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
                }
                GrammarItem::Collection { coll_type, element_type, .. } => {
                    // Collection becomes a field with the appropriate collection type
                    let coll_type_ident = match coll_type {
                        crate::ast::CollectionType::HashBag => quote! { mettail_runtime::HashBag },
                        crate::ast::CollectionType::HashSet => quote! { std::collections::HashSet },
                        crate::ast::CollectionType::Vec => quote! { Vec },
                    };
                    fields.push(quote! { #coll_type_ident<#element_type> });
                }
                GrammarItem::Binder { .. } => {
                    // Should have been skipped above
                    panic!("Unexpected binder at position {}", i);
                }
                GrammarItem::Terminal(_) => {
                    // Terminals don't become fields
                }
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
/// ```
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
        let has_collection = rule.items.iter().any(|item| {
            matches!(item, GrammarItem::Collection { .. })
        });
        
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
    let impls: Vec<TokenStream> = theory.exports.iter().filter_map(|export| {
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
    }).collect();
    
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
            exports: vec![
                Export { name: parse_quote!(Elem) }
            ],
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
            exports: vec![
                Export { name: parse_quote!(Proc) },
                Export { name: parse_quote!(Name) },
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
            ],
            equations: vec![],
            rewrites: vec![],
        };
        
        let output = generate_ast(&theory);
        
        println!("Generated: {}", output);
        assert!(output.to_string().contains("enum Proc"));
        assert!(output.to_string().contains("enum Name"));
    }
}

