use crate::ast::{TheoryDef, GrammarItem, GrammarRule};
use crate::{substitution, display_gen};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Generate complete code for a theory: AST enums + substitution + display
/// Note: Parser is generated separately by LALRPOP (see grammar_writer.rs)
pub fn generate_ast(theory: &TheoryDef) -> TokenStream {
    let ast_enums = generate_ast_enums(theory);
    let subst_impl = substitution::generate_substitution(theory);
    let display_impl = display_gen::generate_display(theory);
    
    // Generate LALRPOP module reference
    let theory_name = &theory.name;
    let theory_name_lower = theory_name.to_string().to_lowercase();
    let theory_mod = syn::Ident::new(&theory_name_lower, proc_macro2::Span::call_site());
    
    quote! {
        #ast_enums
        
        #subst_impl
        
        #display_impl

        // #theory_mod;
        
        // Include the LALRPOP-generated parser
        // The .lalrpop file is generated at compile time and compiled by build.rs
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
            #[derive(Debug, Clone, PartialEq, Eq, Hash, mettail_runtime::BoundTerm)]
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
    
    // Count non-terminal items (these become fields)
    let fields: Vec<_> = rule.items
        .iter()
        .filter_map(|item| match item {
            GrammarItem::NonTerminal(ident) => Some(ident),
            GrammarItem::Binder { .. } => None, // Handled above
            _ => None,
        })
        .collect();
    
    if fields.is_empty() {
        // Unit variant
        quote! { #label }
    } else if fields.len() == 1 && fields[0].to_string() == "Var" {
        // Special case: Var field -> generate Var<String> directly (not boxed)
        quote! { #label(mettail_runtime::Var<String>) }
    } else {
        // Tuple variant - wrap in Box to avoid recursive type
        // Check each field to see if it's Var
        let boxed_fields: Vec<TokenStream> = fields.iter().map(|f| {
            if f.to_string() == "Var" {
                // Var is not boxed
                quote! { mettail_runtime::Var<String> }
            } else {
                quote! { Box<#f> }
            }
        }).collect();
        
        quote! { #label(#(#boxed_fields),*) }
    }
}

/// Generate a variant for a constructor with binders
/// Example: PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
/// Generates: PInput(Box<Name>, mettail_runtime::Scope<mettail_runtime::Binder<String>, Box<Proc>>)
/// Fields are generated in the ORDER they appear in the grammar
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
    
    // Generate fields in GRAMMAR ORDER
    // For each item:
    //   - If it's the binder: skip (it's part of the Scope)
    //   - If it's the body: generate Scope<Binder<String>, Box<Body>>
    //   - Otherwise: generate Box<Type> or Var<String>
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

