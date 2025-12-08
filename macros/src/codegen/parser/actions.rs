//! Parser generation for MeTTaIL theories
//!
//! Generates parser combinator code from grammar rules

use crate::ast::{GrammarItem, GrammarRule, TheoryDef};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Generate parser code for a theory
///
/// For each exported category, generates a parser function that can
/// parse terms of that category from strings.
#[allow(dead_code)]
pub fn generate_parsers(theory: &TheoryDef) -> TokenStream {
    let theory_name = &theory.name;

    // Generate parser for each exported category
    let parser_fns: Vec<TokenStream> = theory
        .exports
        .iter()
        .map(|export| {
            let category = &export.name;

            // Find all rules for this category
            let rules: Vec<&GrammarRule> = theory
                .terms
                .iter()
                .filter(|r| r.category == *category)
                .collect();

            generate_category_parser(category, &rules)
        })
        .collect();

    // Generate parser trait implementation
    quote! {
        /// Parser for #theory_name theory
        pub struct #theory_name;

        impl #theory_name {
            #(#parser_fns)*
        }
    }
}

/// Generate a parser function for a specific category
#[allow(dead_code)]
fn generate_category_parser(category: &Ident, rules: &[&GrammarRule]) -> TokenStream {
    let fn_name = quote::format_ident!("parse_{}", category.to_string().to_lowercase());

    // Generate parser branches for each rule
    let branches: Vec<TokenStream> = rules
        .iter()
        .map(|rule| generate_rule_parser(rule))
        .collect();

    quote! {
        /// Parse a #category term
        pub fn #fn_name(input: &str) -> Result<#category, mettail_runtime::ParseError> {
            let input = input.trim();

            #(#branches)*

            Err(mettail_runtime::ParseError {
                message: format!("Failed to parse {} from: {}", stringify!(#category), input),
                position: 0,
            })
        }
    }
}

/// Generate parser code for a single grammar rule
#[allow(dead_code)]
fn generate_rule_parser(rule: &GrammarRule) -> TokenStream {
    let label = &rule.label;
    let category = &rule.category;

    // Check if this rule has bindings
    if !rule.bindings.is_empty() {
        return generate_binder_rule_parser(rule);
    }

    // Check if rule has at least one terminal (for now, we only generate parsers for rules with terminals)
    let has_terminal = rule
        .items
        .iter()
        .any(|item| matches!(item, GrammarItem::Terminal(_)));
    if !has_terminal {
        return quote! {
            // TODO: Parser for rule #label (no terminals) not yet implemented
        };
    }

    // Build the pattern matching code
    let (pattern_checks, field_parsers, field_names) = build_rule_pattern(rule);

    // If pattern is empty, skip this rule (e.g., for Var rules)
    if pattern_checks.is_empty() && field_parsers.is_empty() && field_names.is_empty() {
        return quote! {
            // TODO: Parser for rule #label not yet implemented
        };
    }

    if field_names.is_empty() {
        // Unit constructor - just check terminals
        quote! {
            if #(#pattern_checks)&&* {
                return Ok(#category::#label);
            }
        }
    } else {
        // Constructor with fields
        quote! {
            if #(#pattern_checks)&&* {
                // Parse fields
                #(#field_parsers)*

                return Ok(#category::#label(#(Box::new(#field_names)),*));
            }
        }
    }
}

/// Generate parser for a rule with binders
/// Example: ELam . Expr ::= "\\" <Var> "." Expr ;
/// Should generate code that creates: Expr::ELam(Scope::new(Binder(var), body))
#[allow(dead_code)]
fn generate_binder_rule_parser(_rule: &GrammarRule) -> TokenStream {
    // For now, we don't generate parsers for rules with binders
    // This is a placeholder that will be implemented later
    // We generate an empty token stream so the parser function compiles
    // but this rule will never match
    quote! {
        // TODO: Parser for binder rule #(#rule.label) not yet implemented
    }
}

/// Build pattern matching and field parsing code for a rule WITHOUT bindings
#[allow(dead_code)]
fn build_rule_pattern(rule: &GrammarRule) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<Ident>) {
    let mut pattern_checks = Vec::new();
    let mut field_parsers = Vec::new();
    let mut field_names = Vec::new();
    let mut pos = 0;
    let mut field_idx = 0usize;

    for item in &rule.items {
        match item {
            GrammarItem::Terminal(lit) => {
                // Check if input starts with this literal
                let check = quote! {
                    input[#pos..].starts_with(#lit)
                };
                pattern_checks.push(check);
                pos += lit.len();
            },
            GrammarItem::NonTerminal(cat) => {
                let field_name = quote::format_ident!("field_{}", field_idx);

                // Special handling for built-in Var type
                if cat.to_string() == "Var" {
                    // For rules that use Var, we skip parser generation for now
                    // This will be implemented later when we have full variable support
                    return (vec![], vec![], vec![]);
                } else {
                    // Parse a field of this category
                    let parse_fn = quote::format_ident!("parse_{}", cat.to_string().to_lowercase());

                    let parser = quote! {
                        let #field_name = Self::#parse_fn(&input[#pos..])?;
                    };
                    field_parsers.push(parser);
                    field_names.push(field_name);
                }
                field_idx += 1;
            },
            GrammarItem::Collection { .. } => {
                // Collections will be handled in Phase 4 (Parser Integration)
                // For now, skip parser generation for collection constructors
                return (vec![], vec![], vec![]);
            },
            GrammarItem::Binder { .. } => {
                // Binders should be handled by generate_binder_rule_parser
                // If we get here, something went wrong
                panic!("Binder encountered in non-binding rule pattern building");
            },
        }
    }

    (pattern_checks, field_parsers, field_names)
}
