use crate::ast::{TheoryDef, RewriteRule, Expr, FreshnessCondition};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use std::collections::HashMap;

pub fn generate_rewrite_engine(theory: &TheoryDef) -> TokenStream {
    if theory.rewrites.is_empty() {
        return quote! {};
    }
    
    // Only generate matchers for base rules (not congruences)
    // Congruences (rules with premise) are handled by Ascent generation
    let matchers: Vec<TokenStream> = theory.rewrites
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.premise.is_none()) // Skip congruence rules
        .map(|(idx, rule)| generate_rule_matcher(idx, rule, theory))
        .collect();
    
    let freshness_fns = generate_freshness_functions(theory);
    
    quote! {
        #freshness_fns
        
        #(#matchers)*
    }
}

fn generate_rule_matcher(
    idx: usize, 
    rule: &RewriteRule,
    theory: &TheoryDef
) -> TokenStream {
    let fn_name = syn::Ident::new(
        &format!("try_rewrite_rule_{}", idx),
        proc_macro2::Span::call_site()
    );

    let category = extract_category(&rule.left);
    
    let mut bindings = HashMap::new();
    
    let pattern_body = generate_pattern_with_body(
        &rule.left,
        "term",
        theory,
        &mut bindings,
        rule
    );
    
    let result = quote! {
        pub fn #fn_name(term: &#category) -> Option<#category> {
            #pattern_body
            
            None
        }
    };
    
    result
}

fn generate_pattern_with_body(
    expr: &Expr,
    term_name: &str,
    theory: &TheoryDef,
    bindings: &mut HashMap<String, TokenStream>,
    rule: &RewriteRule
) -> TokenStream {
    let term_ident = syn::Ident::new(term_name, proc_macro2::Span::call_site());
    
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            bindings.insert(var_name, quote! { #term_ident.clone() });
            
            let equality_checks = quote! {}; // No equality checks at top level variable
            let freshness_checks = generate_freshness_checks(&rule.conditions, bindings);
            let rhs = generate_rhs(&rule.right, bindings);
            
            quote! {
                #equality_checks
                #freshness_checks
                return Some(#rhs);
            }
        }
        
        Expr::Apply { constructor, args } => {
            let category = extract_category(expr);
            let mut equality_checks = Vec::new();
            generate_constructor_pattern_with_body(
                &category,
                constructor,
                args,
                &term_ident,
                bindings,
                &mut equality_checks,
                theory,
                rule
            )
        }
        
        Expr::Subst { .. } => {
            panic!("Substitution should not appear in LHS of rewrite rule")
        }
    }
}

fn generate_constructor_pattern_with_body(
    category: &Ident,
    constructor: &Ident,
    args: &[Expr],
    term: &Ident,
    bindings: &mut HashMap<String, TokenStream>,
    equality_checks: &mut Vec<(String, TokenStream)>,
    theory: &TheoryDef,
    rule: &RewriteRule
) -> TokenStream {
    let grammar_rule = theory.terms.iter()
        .find(|r| r.label == *constructor)
        .expect(&format!("Constructor {} not found", constructor));
    
    if !grammar_rule.bindings.is_empty() {
        generate_binder_pattern_with_body(
            category, constructor, args, term, bindings,
            equality_checks, grammar_rule, theory, rule
        )
    } else {
        generate_regular_pattern_with_body(
            category, constructor, args, term, bindings,
            equality_checks, grammar_rule, theory, rule
        )
    }
}

fn generate_binder_pattern_with_body(
    category: &Ident,
    constructor: &Ident,
    args: &[Expr],
    term: &Ident,
    bindings: &mut HashMap<String, TokenStream>,
    equality_checks: &mut Vec<(String, TokenStream)>,
    grammar_rule: &crate::ast::GrammarRule,
    _theory: &TheoryDef,
    rule: &RewriteRule
) -> TokenStream {
    let (binder_idx, body_indices) = &grammar_rule.bindings[0];
    let body_idx = body_indices[0];
    
    let mut ast_field_names = Vec::new();
    for (i, item) in grammar_rule.items.iter().enumerate() {
        if i == *binder_idx {
            continue; // Inside scope
        } else if i == body_idx {
            ast_field_names.push(syn::Ident::new("scope_field", proc_macro2::Span::call_site()));
        } else if matches!(item, crate::ast::GrammarItem::NonTerminal(_)) {
            let field_num = ast_field_names.len();
            ast_field_names.push(syn::Ident::new(&format!("field_{}", field_num), proc_macro2::Span::call_site()));
        }
    }
    
    // Bind pattern variables to AST fields
    let mut pattern_arg_idx = 0;
    for (i, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::Binder { .. } => {
                if pattern_arg_idx < args.len() {
                    if let Expr::Var(var) = &args[pattern_arg_idx] {
                        let binder_key = format!("{}_binder", var);
                        let binding = quote! { binder.clone() };
                        
                        // Check for duplicate binder variable
                        if bindings.contains_key(&binder_key) {
                            equality_checks.push((binder_key, binding));
                        } else {
                            bindings.insert(binder_key, binding);
                        }
                    }
                    pattern_arg_idx += 1;
                }
            }
            crate::ast::GrammarItem::NonTerminal(_) => {
                if pattern_arg_idx < args.len() {
                    if i == body_idx {
                        if let Expr::Var(var) = &args[pattern_arg_idx] {
                            let var_name = var.to_string();
                            let binding = quote! { (*body).clone() };
                            
                            // Check for duplicate variable
                            if bindings.contains_key(&var_name) {
                                equality_checks.push((var_name, binding));
                            } else {
                                bindings.insert(var_name, binding);
                            }
                        }
                    } else {
                        // Find corresponding AST field
                        let field_count_before = grammar_rule.items[0..i]
                            .iter()
                            .filter(|it| matches!(it, crate::ast::GrammarItem::NonTerminal(_)) && grammar_rule.items.iter().position(|x| std::ptr::eq(*it, x)) != Some(*binder_idx))
                            .count();
                        
                        if field_count_before < ast_field_names.len() {
                            let field = &ast_field_names[field_count_before];
                            if let Expr::Var(var) = &args[pattern_arg_idx] {
                                let var_name = var.to_string();
                                let binding = quote! { (*#field).clone() };
                                
                                // Check for duplicate variable
                                if bindings.contains_key(&var_name) {
                                    equality_checks.push((var_name, binding));
                                } else {
                                    bindings.insert(var_name, binding);
                                }
                            }
                        }
                    }
                    pattern_arg_idx += 1;
                }
            }
            _ => {}
        }
    }
    
    // Generate final body (equality + freshness + RHS)
    let is_dummy = matches!(rule.right, Expr::Var(ref v) if v == "dummy");
    
    if is_dummy {
        // Dummy rule - return ONLY the inner statements (unbinding), no if-let wrapper
        quote! {
            let (binder, body) = scope_field.clone().unbind();
        }
    } else {
        // Real rule - generate complete if-let with equality + freshness checks and RHS
        let eq_checks = generate_equality_checks(equality_checks, bindings);
        let freshness_checks = generate_freshness_checks(&rule.conditions, bindings);
        let rhs = generate_rhs(&rule.right, bindings);
        quote! {
            if let #category::#constructor(#(#ast_field_names),*) = #term {
                let (binder, body) = scope_field.clone().unbind();
                #eq_checks
                #freshness_checks
                return Some(#rhs);
            }
        }
    }
}

/// Information about a nested pattern at any depth
#[derive(Clone)]
struct NestedPatternInfo {
    /// Index in parent's args
    field_idx: usize,
    /// Unique identifier for this pattern (e.g., "field_0_inner", "field_0_inner_1_inner")
    field_term: Ident,
    /// The expression representing this pattern
    expr: Expr,
    /// Nested patterns within this one (recursive)
    children: Vec<NestedPatternInfo>,
}

/// Recursively extract all variables from an expression tree
fn extract_variables_recursive(
    expr: &Expr,
    field_prefix: &str,
    theory: &TheoryDef,
    bindings: &mut HashMap<String, TokenStream>,
    equality_checks: &mut Vec<(String, TokenStream)>,
) -> Option<NestedPatternInfo> {
    match expr {
        Expr::Var(var) => {
            // Base case: this is a variable, extract it
            let var_name = var.to_string();
            
            // Special handling for variables inside binder bodies
            let binding = if field_prefix == "body" {
                // Body of a binder - 'body' variable comes from unbind(), already unwrapped
                quote! { (*body).clone() }
            } else {
                // Regular field - need double deref from Box
                let field_ident = syn::Ident::new(field_prefix, proc_macro2::Span::call_site());
                quote! { (**#field_ident).clone() }
            };
            
            if bindings.contains_key(&var_name) {
                equality_checks.push((var_name, binding));
            } else {
                bindings.insert(var_name, binding);
            }
            None // Variables don't have nested patterns
        }
        
        Expr::Apply { constructor, args } => {
            // Recursive case: this is a constructor with args
            let grammar = theory.terms.iter()
                .find(|r| r.label == *constructor)
                .expect("Constructor not found");
            
            // Get field count (how many AST fields this constructor has)
            let field_count = grammar.items.iter()
                .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
                .count();
            
            let mut children = Vec::new();
            
            // Handle binder constructors specially
            if !grammar.bindings.is_empty() {
                let (_binder_idx, body_indices) = &grammar.bindings[0];
                let body_idx = body_indices[0];
                
                let mut pattern_arg_idx = 0;
                let mut ast_field_idx = 0;
                
                for (i, item) in grammar.items.iter().enumerate() {
                    if pattern_arg_idx >= args.len() {
                        break;
                    }
                    
                    match item {
                        crate::ast::GrammarItem::Binder { .. } => {
                            if let Expr::Var(var) = &args[pattern_arg_idx] {
                                let binder_key = format!("{}_binder", var);
                                let binding = quote! { binder.clone() };
                                
                                if bindings.contains_key(&binder_key) {
                                    equality_checks.push((binder_key, binding));
                                } else {
                                    bindings.insert(binder_key, binding);
                                }
                            }
                            pattern_arg_idx += 1;
                        }
                        crate::ast::GrammarItem::NonTerminal(_) => {
                            if i == body_idx {
                                // Body - part of Scope, use "body" as field name for variables inside
                                if let Some(mut child_info) = extract_variables_recursive(
                                    &args[pattern_arg_idx],
                                    "body",
                                    theory,
                                    bindings,
                                    equality_checks
                                ) {
                                    child_info.field_idx = ast_field_idx; // Body is still an AST field (the Scope)
                                    children.push(child_info);
                                }
                            } else {
                                // Regular field (not binder, not body)
                                let child_prefix = format!("{}_inner_{}", field_prefix, ast_field_idx);
                                if let Some(mut child_info) = extract_variables_recursive(
                                    &args[pattern_arg_idx],
                                    &child_prefix,
                                    theory,
                                    bindings,
                                    equality_checks
                                ) {
                                    child_info.field_idx = ast_field_idx;
                                    children.push(child_info);
                                }
                            }
                            // Increment ast_field_idx for ALL NonTerminals (both body and regular fields)
                            // because they all appear in the AST (body is wrapped in Scope)
                            ast_field_idx += 1;
                            pattern_arg_idx += 1;
                        }
                        _ => {}
                    }
                }
            } else {
                // Regular constructor - recurse into all args
                for (i, arg) in args.iter().enumerate() {
                    if i >= field_count {
                        break;
                    }
                    
                    // Use consistent naming: field_prefix_inner_{i}
                    let child_prefix = format!("{}_inner_{}", field_prefix, i);
                    if let Some(mut child_info) = extract_variables_recursive(
                        arg,
                        &child_prefix,
                        theory,
                        bindings,
                        equality_checks
                    ) {
                        child_info.field_idx = i;
                        children.push(child_info);
                    }
                }
            }
            
            // Return info about this nested pattern
            Some(NestedPatternInfo {
                field_idx: 0, // Will be set by caller
                field_term: syn::Ident::new(&format!("{}_inner", field_prefix), proc_macro2::Span::call_site()),
                expr: expr.clone(),
                children,
            })
        }
        
        Expr::Subst { .. } => {
            panic!("Substitution should not appear in LHS");
        }
    }
}

/// Generate pattern for regular constructor with complete body
fn generate_regular_pattern_with_body(
    category: &Ident,
    constructor: &Ident,
    args: &[Expr],
    term: &Ident,
    bindings: &mut HashMap<String, TokenStream>,
    equality_checks: &mut Vec<(String, TokenStream)>,
    grammar_rule: &crate::ast::GrammarRule,
    theory: &TheoryDef,
    rule: &RewriteRule
) -> TokenStream {
    // Count AST fields
    let ast_field_count = grammar_rule.items.iter()
        .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
        .count();
    
    let field_names: Vec<Ident> = (0..ast_field_count)
        .map(|i| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()))
        .collect();
    
    // First bind all top-level simple variables
    for (i, arg) in args.iter().enumerate() {
        if i >= field_names.len() {
            break;
        }
        
        let field = &field_names[i];
        if let Expr::Var(var) = arg {
            let var_name = var.to_string();
            // Top-level fields are &Box<T>, so (**field) gives T
            let binding = quote! { (**#field).clone() };
            
            if bindings.contains_key(&var_name) {
                equality_checks.push((var_name, binding));
            } else {
                bindings.insert(var_name, binding);
            }
        }
    }
    
    // Recursively extract all variables from nested patterns (ANY DEPTH)
    let mut nested_patterns: Vec<NestedPatternInfo> = Vec::new();
    
    for (i, arg) in args.iter().enumerate() {
        if i >= field_names.len() {
            break;
        }
        
        if let Expr::Apply { .. } = arg {
            let field_prefix = format!("field_{}", i);
            if let Some(mut pattern_info) = extract_variables_recursive(
                arg,
                &field_prefix,
                theory,
                bindings,
                equality_checks
            ) {
                pattern_info.field_idx = i;
                nested_patterns.push(pattern_info);
            }
        }
    }
    
    // Generate final body (equality + freshness + RHS)
    let is_dummy = matches!(rule.right, Expr::Var(ref v) if v == "dummy");
    
    if is_dummy {
        // Dummy rule - return ONLY the binding statements, no if-let wrapper
        quote! {}
    } else {
        // Real rule - nest all patterns and add final body with equality + freshness + RHS
        let eq_checks = generate_equality_checks(equality_checks, bindings);
        let freshness_checks = generate_freshness_checks(&rule.conditions, bindings);
        let rhs = generate_rhs(&rule.right, bindings);
        
        // Build nested structure from inside out
        let mut body = quote! {
            #eq_checks
            #freshness_checks
            return Some(#rhs);
        };
        
        // Wrap with nested patterns recursively (handles arbitrary depth)
        body = wrap_nested_patterns_recursive(&nested_patterns, &field_names, body, theory);
        
        quote! {
            if let #category::#constructor(#(#field_names),*) = #term {
                #body
            }
        }
    }
}

/// Recursively wrap body with nested pattern matches to arbitrary depth
fn wrap_nested_patterns_recursive(
    patterns: &[NestedPatternInfo],
    parent_field_names: &[Ident],
    mut body: TokenStream,
    theory: &TheoryDef,
) -> TokenStream {
    // Process patterns in reverse order (inside-out)
    for pattern in patterns.iter().rev() {
        body = wrap_single_pattern(&pattern, parent_field_names, body, theory);
    }
    body
}

/// Wrap body with a single pattern match, recursively handling its children
fn wrap_single_pattern(
    pattern: &NestedPatternInfo,
    parent_field_names: &[Ident],
    mut body: TokenStream,
    theory: &TheoryDef,
) -> TokenStream {
    // First, recursively wrap any children of this pattern
    if !pattern.children.is_empty() {
        // We need to compute the field names that will be bound by this pattern
        let field_term = &pattern.field_term;
        let grammar = match &pattern.expr {
            Expr::Apply { constructor, .. } => {
                theory.terms.iter()
                    .find(|r| r.label == *constructor)
                    .expect("Constructor not found")
            }
            _ => unreachable!("Pattern should be Apply"),
        };
        
        let field_count = grammar.items.iter()
            .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
            .count();
        
        // Use consistent naming: {field_term}_{i} for child fields
        let child_field_names: Vec<Ident> = (0..field_count)
            .map(|i| syn::Ident::new(&format!("{}_{}", field_term, i), proc_macro2::Span::call_site()))
            .collect();
        
        body = wrap_nested_patterns_recursive(&pattern.children, &child_field_names, body, theory);
    }
    
    // Now wrap this pattern itself
    let field_name = &parent_field_names[pattern.field_idx];
    let field_term = &pattern.field_term;
    
    match &pattern.expr {
        Expr::Apply { constructor, .. } => {
            let grammar = theory.terms.iter()
                .find(|r| r.label == *constructor)
                .expect("Constructor not found");
            
            let category = extract_category(&pattern.expr);
            
            if !grammar.bindings.is_empty() {
                // Binder constructor
                let (_binder_idx, body_indices) = &grammar.bindings[0];
                let body_idx = body_indices[0];
                
                let mut inner_ast_fields = Vec::new();
                let mut ast_field_idx = 0;
                for (i, item) in grammar.items.iter().enumerate() {
                    if i == *_binder_idx {
                        continue;
                    } else if i == body_idx {
                        inner_ast_fields.push(syn::Ident::new("scope_field", proc_macro2::Span::call_site()));
                    } else if matches!(item, crate::ast::GrammarItem::NonTerminal(_)) {
                        // Use consistent naming: {field_term}_{ast_field_idx}
                        inner_ast_fields.push(syn::Ident::new(&format!("{}_{}", field_term, ast_field_idx), proc_macro2::Span::call_site()));
                        ast_field_idx += 1;
                    }
                }
                
                quote! {
                    let #field_term = &(**#field_name);
                    if let #category::#constructor(#(#inner_ast_fields),*) = #field_term {
                        let (binder, body) = scope_field.clone().unbind();
                        #body
                    }
                }
            } else {
                // Regular constructor
                let field_count = grammar.items.iter()
                    .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
                    .count();
                
                // Use consistent naming: {field_term}_{i}
                let inner_fields: Vec<Ident> = (0..field_count)
                    .map(|i| syn::Ident::new(&format!("{}_{}", field_term, i), proc_macro2::Span::call_site()))
                    .collect();
                
                quote! {
                    let #field_term = &(**#field_name);
                    if let #category::#constructor(#(#inner_fields),*) = #field_term {
                        #body
                    }
                }
            }
        }
        _ => unreachable!("Pattern should be Apply"),
    }
}

/// Extract category from expression
fn extract_category(expr: &Expr) -> Ident {
    match expr {
        Expr::Apply { constructor, .. } => {
            let name = constructor.to_string();
            if name.starts_with('P') {
                syn::Ident::new("Proc", constructor.span())
            } else if name.starts_with('N') {
                syn::Ident::new("Name", constructor.span())
            } else {
                constructor.clone()
            }
        }
        Expr::Var(ident) => ident.clone(),
        Expr::Subst { term, .. } => extract_category(term),
    }
}

/// Generate equality checks for duplicate variables
fn generate_equality_checks(
    checks: &[(String, TokenStream)],
    bindings: &HashMap<String, TokenStream>
) -> TokenStream {
    if checks.is_empty() {
        return quote! {};
    }
    
    let check_code: Vec<TokenStream> = checks.iter().map(|(var_name, field_access)| {
        let first_binding = bindings.get(var_name)
            .expect(&format!("First binding for {} not found", var_name));
        
        quote! {
            if !(#first_binding == #field_access) {
                return None;
            }
        }
    }).collect();
    
    quote! {
        #(#check_code)*
    }
}

/// Generate freshness condition checks
fn generate_freshness_checks(
    conditions: &[FreshnessCondition],
    bindings: &HashMap<String, TokenStream>
) -> TokenStream {
    if conditions.is_empty() {
        return quote! {};
    }
    
    let checks: Vec<TokenStream> = conditions.iter().map(|cond| {
        let var = &cond.var;
        let term = &cond.term;
        
        let var_key = format!("{}_binder", var);
        let var_binding = bindings.get(&var_key)
            .or_else(|| bindings.get(&var.to_string()))
            .expect(&format!("Variable {} not found in bindings", var));
        
        let term_binding = bindings.get(&term.to_string())
            .expect(&format!("Term {} not found in bindings", term));
        
        quote! {
            if !is_fresh(&#var_binding, &#term_binding) {
                return None;
            }
        }
    }).collect();
    
    quote! {
        #(#checks)*
    }
}

/// Generate RHS construction
fn generate_rhs(expr: &Expr, bindings: &HashMap<String, TokenStream>) -> TokenStream {
    match expr {
        Expr::Var(var) => {
            bindings.get(&var.to_string())
                .cloned()
                .unwrap_or_else(|| quote! { #var })
        }
        
        Expr::Apply { constructor, args } => {
            let category = extract_category(expr);
            let rhs_args: Vec<TokenStream> = args.iter()
                .map(|arg| {
                    let arg_code = generate_rhs(arg, bindings);
                    quote! { Box::new(#arg_code) }
                })
                .collect();
            
            quote! {
                #category::#constructor(#(#rhs_args),*)
            }
        }
        
        Expr::Subst { term, var, replacement } => {
            let term_code = generate_rhs(term, bindings);
            let replacement_code = generate_rhs(replacement, bindings);
            
            let binder_key = format!("{}_binder", var);
            let binder = bindings.get(&binder_key)
                .expect(&format!("Binder for {} not found", var));
            
            let term_category = extract_category(term);
            let repl_category = extract_category(replacement);
            
            let subst_method = if term_category == repl_category {
                syn::Ident::new("substitute", proc_macro2::Span::call_site())
            } else {
                let repl_cat_lower = repl_category.to_string().to_lowercase();
                syn::Ident::new(
                    &format!("substitute_{}", repl_cat_lower),
                    proc_macro2::Span::call_site()
                )
            };
            
            // binder.0 is the FreeVar that was bound - pass it directly without cloning first
            quote! {
                (#term_code).#subst_method(&(#binder).0, &#replacement_code)
            }
        }
    }
}

/// Generate freshness checking functions
fn generate_freshness_functions(_theory: &TheoryDef) -> TokenStream {
    quote! {
        fn is_fresh<T>(binder: &mettail_runtime::Binder<String>, term: &T) -> bool
        where
            T: mettail_runtime::BoundTerm<String>
        {
            use mettail_runtime::BoundTerm;
            
            let mut is_fresh = true;
            term.visit_vars(&mut |v| {
                if let mettail_runtime::Var::Free(fv) = v {
                    if fv == &binder.0 {
                        is_fresh = false;
                    }
                }
            });
            
            is_fresh
        }
    }
}

