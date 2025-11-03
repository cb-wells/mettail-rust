use crate::ast::{TheoryDef, RewriteRule, Expr, FreshnessCondition};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use std::collections::HashMap;

/// Generate rewrite execution code from theory definition
pub fn generate_rewrite_engine(theory: &TheoryDef) -> TokenStream {
    if theory.rewrites.is_empty() {
        return quote! {};
    }
    
    // Generate one matcher per rewrite rule
    let matchers: Vec<TokenStream> = theory.rewrites
        .iter()
        .enumerate()
        .map(|(idx, rule)| generate_rule_matcher(idx, rule, theory))
        .collect();
    
    // Generate freshness checkers
    let freshness_fns = generate_freshness_functions(theory);
    
    quote! {
        #freshness_fns
        
        #(#matchers)*
    }
}

/// Generate a pattern matcher for one rewrite rule
fn generate_rule_matcher(
    idx: usize, 
    rule: &RewriteRule,
    theory: &TheoryDef
) -> TokenStream {
    let fn_name = syn::Ident::new(
        &format!("try_rewrite_rule_{}", idx),
        proc_macro2::Span::call_site()
    );
    
    // Determine which category this rule applies to (from LHS)
    let category = extract_category(&rule.left);
    
    // Build bindings as we generate
    let mut bindings = HashMap::new();
    
    // Generate the complete pattern match with body
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
    
    eprintln!("\n=== Generated rewrite function ===\n{}\n================================\n", result);
    
    result
}

/// Generate complete pattern match with body
/// This builds the entire if-let structure with proper nesting and matched braces
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
            // Variable matches anything - bind it and generate body
            let var_name = var.to_string();
            bindings.insert(var_name, quote! { #term_ident.clone() });
            
            // This is the innermost level - generate equality + freshness checks + RHS
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

/// Generate constructor pattern with complete body
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
    // Look up grammar rule
    let grammar_rule = theory.terms.iter()
        .find(|r| r.label == *constructor)
        .expect(&format!("Constructor {} not found", constructor));
    
    // Check if this is a binder constructor
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

/// Generate pattern for binder constructor with complete body
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
    
    // Generate AST field names (non-binder fields + scope)
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
    
    // First bind all simple variables, checking for duplicates
    for (i, arg) in args.iter().enumerate() {
        if i >= field_names.len() {
            break;
        }
        
        let field = &field_names[i];
        if let Expr::Var(var) = arg {
            let var_name = var.to_string();
            let binding = quote! { (*#field).clone() };
            
            // Check if this variable was already bound
            if bindings.contains_key(&var_name) {
                // Duplicate variable - add equality check instead of rebinding
                equality_checks.push((var_name, binding));
            } else {
                // First occurrence - bind it
                bindings.insert(var_name, binding);
            }
        }
    }
    
    // Now generate nested patterns for complex args
    // Don't recursively generate - we'll manually bind variables with correct unique names
    let mut nested_pattern_info: Vec<(usize, Ident)> = Vec::new();
    
    for (i, arg) in args.iter().enumerate() {
        if i >= field_names.len() {
            break;
        }
        
        if let Expr::Apply { .. } = arg {
            let field_term = syn::Ident::new(&format!("field_{}_inner", i), proc_macro2::Span::call_site());
            nested_pattern_info.push((i, field_term));
        }
    }
    
    // IMPORTANT: Add bindings for nested patterns BEFORE generating freshness checks/RHS
    for (field_idx, field_term) in &nested_pattern_info {
        let field_arg = &args[*field_idx];
        if let Expr::Apply { constructor: inner_constr, args: inner_args, .. } = field_arg {
            let inner_grammar = theory.terms.iter()
                .find(|r| r.label == *inner_constr)
                .expect("Constructor not found");
            
            if !inner_grammar.bindings.is_empty() {
                // Binder constructor - add bindings for ALL fields (regular, binder, and body), checking for duplicates
                let (binder_idx, body_indices) = &inner_grammar.bindings[0];
                let body_idx = body_indices[0];
                
                let mut pattern_arg_idx = 0;
                let mut ast_field_idx = 0;
                
                for (i, item) in inner_grammar.items.iter().enumerate() {
                    if pattern_arg_idx >= inner_args.len() {
                        break;
                    }
                    
                    match item {
                        crate::ast::GrammarItem::Binder { .. } => {
                            if let Expr::Var(var) = &inner_args[pattern_arg_idx] {
                                let binder_key = format!("{}_binder", var);
                                let binding = quote! { binder.clone() };
                                
                                // Check for duplicate
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
                                // This is the body
                                if let Expr::Var(var) = &inner_args[pattern_arg_idx] {
                                    let var_name = var.to_string();
                                    let binding = quote! { (*body).clone() };
                                    
                                    // Check for duplicate
                                    if bindings.contains_key(&var_name) {
                                        equality_checks.push((var_name, binding));
                                    } else {
                                        bindings.insert(var_name, binding);
                                    }
                                }
                            } else {
                                // Regular field (not binder, not body) - use unique nested field name
                                let field_name = syn::Ident::new(&format!("{}_{}", field_term, ast_field_idx), proc_macro2::Span::call_site());
                                if let Expr::Var(var) = &inner_args[pattern_arg_idx] {
                                    let var_name = var.to_string();
                                    // field_name is &Box<T> from the pattern match, so **field_name gets T
                                    let binding = quote! { (**#field_name).clone() };
                                    
                                    // Check for duplicate
                                    if bindings.contains_key(&var_name) {
                                        equality_checks.push((var_name, binding));
                                    } else {
                                        bindings.insert(var_name, binding);
                                    }
                                }
                                ast_field_idx += 1;
                            }
                            pattern_arg_idx += 1;
                        }
                        _ => {}
                    }
                }
            } else {
                // Regular constructor - add bindings with unique field names, checking for duplicates
                let inner_field_count = inner_grammar.items.iter()
                    .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
                    .count();
                
                let inner_fields: Vec<Ident> = (0..inner_field_count)
                    .map(|i| syn::Ident::new(&format!("{}_{}", field_term, i), proc_macro2::Span::call_site()))
                    .collect();
                
                for (i, arg) in inner_args.iter().enumerate() {
                    if i >= inner_fields.len() {
                        break;
                    }
                    
                    let field = &inner_fields[i];
                    if let Expr::Var(var) = arg {
                        let var_name = var.to_string();
                        let binding = quote! { (**#field).clone() };
                        
                        // Check for duplicate
                        if bindings.contains_key(&var_name) {
                            equality_checks.push((var_name, binding));
                        } else {
                            bindings.insert(var_name, binding);
                        }
                    }
                }
            }
        }
    }
    
    // Generate final body (equality + freshness + RHS)
    // NOW all bindings from nested patterns are available
    // Only generate if this is not a dummy rule
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
        
        // Wrap with nested patterns and manually create bindings
        for (field_idx, field_term) in nested_pattern_info.iter().rev() {
            let field_name = &field_names[*field_idx];
            
            // Get the grammar rule for this nested pattern to know if it's a binder
            let field_arg = &args[*field_idx];
            if let Expr::Apply { constructor: inner_constr, .. } = field_arg {
                let inner_grammar = theory.terms.iter()
                    .find(|r| r.label == *inner_constr)
                    .expect("Constructor not found");
                
                if !inner_grammar.bindings.is_empty() {
                    // Binder constructor - generate if-let with unbinding
                    let inner_category = extract_category(field_arg);
                    
                    // Generate AST field names for this binder constructor with unique names
                    let (_binder_idx, body_indices) = &inner_grammar.bindings[0];
                    let body_idx = body_indices[0];
                    
                    let mut inner_ast_fields = Vec::new();
                    let mut ast_field_idx = 0;
                    for (i, item) in inner_grammar.items.iter().enumerate() {
                        if i == *_binder_idx {
                            continue;
                        } else if i == body_idx {
                            inner_ast_fields.push(syn::Ident::new("scope_field", proc_macro2::Span::call_site()));
                        } else if matches!(item, crate::ast::GrammarItem::NonTerminal(_)) {
                            // Use unique nested field names matching those in binding phase
                            inner_ast_fields.push(syn::Ident::new(&format!("{}_{}", field_term, ast_field_idx), proc_macro2::Span::call_site()));
                            ast_field_idx += 1;
                        }
                    }
                    
                    // Bindings were already added earlier - don't duplicate them
                    
                    body = quote! {
                        let #field_term = &(**#field_name);
                        if let #inner_category::#inner_constr(#(#inner_ast_fields),*) = #field_term {
                            let (binder, body) = scope_field.clone().unbind();
                            #body
                        }
                    };
                } else {
                    // Regular constructor
                    let inner_category = extract_category(field_arg);
                    let inner_field_count = inner_grammar.items.iter()
                        .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
                        .count();
                    
                    // Use unique field names for nested patterns to avoid shadowing
                    let inner_fields: Vec<Ident> = (0..inner_field_count)
                        .map(|i| syn::Ident::new(&format!("{}_{}", field_term, i), proc_macro2::Span::call_site()))
                        .collect();
                    
                    // Bindings were already added earlier - don't duplicate them
                    
                    body = quote! {
                        let #field_term = &(**#field_name);
                        if let #inner_category::#inner_constr(#(#inner_fields),*) = #field_term {
                            #body
                        }
                    };
                }
            }
        }
        
        quote! {
            if let #category::#constructor(#(#field_names),*) = #term {
                #body
            }
        }
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
