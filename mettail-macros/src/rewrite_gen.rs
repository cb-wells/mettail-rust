use crate::ast::{TheoryDef, RewriteRule, Expr, FreshnessCondition};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use std::collections::HashMap;

/// Generate Ascent clauses for rewrite rules (for equational matching)
/// This is the new approach that allows duplicate variables to use eq_cat() relations
pub fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    theory.rewrites
        .iter()
        .filter(|rule| rule.premise.is_none()) // Skip congruence rules (handled elsewhere)
        .map(|rule| generate_rewrite_clause(rule, theory))
        .collect()
}

/// Generate a single rewrite rule as an Ascent clause
/// Example output:
/// rw_proc(s, t) <--
///     proc(s),
///     if let Proc::PPar(p_in, p_out) = s,
///     if let Proc::PInput(chan1, scope) = &**p_in,
///     if let Proc::POutput(chan2, q) = &**p_out,
///     eq_name((**chan1).clone(), (**chan2).clone()),
///     let (x, p) = scope.clone().unbind(),
///     if !p.contains_free(&x),
///     let t = p.substitute(&x, &Name::NQuote((**q).clone()));
fn generate_rewrite_clause(rule: &RewriteRule, theory: &TheoryDef) -> TokenStream {
    let category = extract_category(&rule.left);
    let cat_lower = quote::format_ident!("{}", category.to_string().to_lowercase());
    let rw_rel = quote::format_ident!("rw_{}", category.to_string().to_lowercase());
    
    // Track variable occurrences for duplicate detection
    let mut var_occurrences: HashMap<String, Vec<usize>> = HashMap::new();
    let mut occurrence_idx = 0;
    collect_variable_occurrences(&rule.left, &mut var_occurrences, &mut occurrence_idx);
    
    // Identify which variables appear multiple times (need equational matching)
    let duplicate_vars: std::collections::HashSet<String> = var_occurrences
        .into_iter()
        .filter(|(_, occurrences)| occurrences.len() > 1)
        .map(|(var_name, _)| var_name)
        .collect();
    
    // Generate pattern matching clauses
    let mut bindings: HashMap<String, TokenStream> = HashMap::new();
    let mut variable_categories: HashMap<String, Ident> = HashMap::new();
    let mut equational_checks: Vec<TokenStream> = Vec::new();
    let mut clauses = Vec::new();
    
    // Start with proc(s) clause
    clauses.push(quote! { #cat_lower(s) });
    
    // Generate pattern matching with category tracking
    let lhs_category = extract_category(&rule.left);
    generate_ascent_pattern(
        &rule.left,
        &quote::format_ident!("s"),
        &lhs_category,
        theory,
        &mut bindings,
        &mut variable_categories,
        &mut clauses,
        &duplicate_vars,
        &mut equational_checks,
    );
    
    // Add equational checks for duplicate variables
    clauses.extend(equational_checks);
    
    // Add freshness checks
    for condition in &rule.conditions {
        let var_name = condition.var.to_string();
        let term_name = condition.term.to_string();
        
        let var_binding = bindings.get(&var_name)
            .unwrap_or_else(|| panic!("Freshness variable '{}' not bound. Available bindings: {:?}", var_name, bindings.keys().collect::<Vec<_>>()));
        let term_binding = bindings.get(&term_name)
            .unwrap_or_else(|| panic!("Freshness term '{}' not bound. Available bindings: {:?}", term_name, bindings.keys().collect::<Vec<_>>()));
        
        clauses.push(quote! {
            if is_fresh(&#var_binding, &#term_binding)
        });
    }
    
    // Generate RHS
    let rhs = generate_ascent_rhs(&rule.right, &bindings);
    clauses.push(quote! { let t = #rhs });
    
    quote! {
        #rw_rel(s, t) <--
            #(#clauses),*;
    }
}

/// Collect all variable occurrences in an expression (for duplicate detection)
fn collect_variable_occurrences(
    expr: &Expr,
    occurrences: &mut HashMap<String, Vec<usize>>,
    idx: &mut usize,
) {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            occurrences.entry(var_name).or_insert_with(Vec::new).push(*idx);
            *idx += 1;
        }
        Expr::Apply { args, .. } => {
            for arg in args {
                collect_variable_occurrences(arg, occurrences, idx);
            }
        }
        Expr::Subst { term, .. } => {
            collect_variable_occurrences(term, occurrences, idx);
        }
    }
}

/// Generate Ascent pattern matching clauses for LHS
/// Handles nested patterns and tracks bindings for equational checks
/// `expected_category` is the category this expression should have based on its context
fn generate_ascent_pattern(
    expr: &Expr,
    term_name: &Ident,
    expected_category: &Ident,
    theory: &TheoryDef,
    bindings: &mut HashMap<String, TokenStream>,
    variable_categories: &mut HashMap<String, Ident>,
    clauses: &mut Vec<TokenStream>,
    duplicate_vars: &std::collections::HashSet<String>,
    equational_checks: &mut Vec<TokenStream>,
) {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            let binding = quote! { #term_name.clone() };
            
            // Check if this is a duplicate variable
            if duplicate_vars.contains(&var_name) {
                // Check if we've seen this variable before
                if let Some(first_binding) = bindings.get(&var_name) {
                    // Duplicate occurrence - use the stored category
                    let category = variable_categories.get(&var_name)
                        .expect(&format!("Variable {} should have been tracked", var_name));
                    let eq_rel = quote::format_ident!("eq_{}", category.to_string().to_lowercase());
                    
                    equational_checks.push(quote! {
                        #eq_rel(#first_binding, #binding)
                    });
                } else {
                    // First occurrence of duplicate variable - bind it and track its category
                    bindings.insert(var_name.clone(), binding);
                    variable_categories.insert(var_name, expected_category.clone());
                }
            } else {
                // Single occurrence - just bind (no need to track category)
                bindings.insert(var_name, binding);
            }
        }
        
        Expr::Apply { constructor, args } => {
            let category = extract_category(expr);
            
            // Find the grammar rule for this constructor
            let grammar_rule = theory.terms.iter()
                .find(|r| r.label == *constructor && r.category == category)
                .unwrap_or_else(|| panic!(
                    "Constructor {} (category: {}) not found in theory. Available: {:?}",
                    constructor,
                    category,
                    theory.terms.iter().map(|r| (&r.label, &r.category)).collect::<Vec<_>>()
                ));
            
            // Check if this is a binder constructor
            if !grammar_rule.bindings.is_empty() {
                generate_ascent_binder_pattern(
                    &category,
                    constructor,
                    args,
                    term_name,
                    theory,
                    grammar_rule,
                    bindings,
                    variable_categories,
                    clauses,
                    duplicate_vars,
                    equational_checks,
                );
            } else {
                generate_ascent_regular_pattern(
                    &category,
                    constructor,
                    args,
                    term_name,
                    theory,
                    grammar_rule,
                    bindings,
                    variable_categories,
                    clauses,
                    duplicate_vars,
                    equational_checks,
                );
            }
        }
        
        Expr::Subst { .. } => {
            panic!("Substitution should not appear in LHS of rewrite rule")
        }
    }
}

/// Extract category from expression
fn extract_category(expr: &Expr) -> Ident {
    match expr {
        Expr::Apply { constructor, .. } => {
            // For backward compatibility, keep the heuristic for common patterns
            let name = constructor.to_string();
            if name.starts_with('P') {
                syn::Ident::new("Proc", constructor.span())
            } else if name.starts_with('N') {
                syn::Ident::new("Name", constructor.span())
            } else if name.starts_with('T') {
                syn::Ident::new("Term", constructor.span())
            } else {
                constructor.clone()
            }
        }
        Expr::Var(ident) => ident.clone(),
        Expr::Subst { term, .. } => extract_category(term),
    }
}

/// Generate freshness checking functions
pub fn generate_freshness_functions(_theory: &TheoryDef) -> TokenStream {
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

/// Generate pattern for binder constructor in Ascent
fn generate_ascent_binder_pattern(
    category: &Ident,
    constructor: &Ident,
    args: &[Expr],
    term_name: &Ident,
    theory: &TheoryDef,
    grammar_rule: &crate::ast::GrammarRule,
    bindings: &mut HashMap<String, TokenStream>,
    variable_categories: &mut HashMap<String, Ident>,
    clauses: &mut Vec<TokenStream>,
    duplicate_vars: &std::collections::HashSet<String>,
    equational_checks: &mut Vec<TokenStream>,
) {
    // Count all AST fields
    // Note: A binder + its body count as ONE field (the Scope)
    let (binder_idx, body_indices) = &grammar_rule.bindings[0];
    let body_idx = body_indices[0];
    
    let mut field_count = 0;
    for (idx, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::NonTerminal(_) => {
                // Regular non-terminal counts as a field, unless it's the body (which is part of the Scope)
                if idx != body_idx {
                    field_count += 1;
                }
            }
            crate::ast::GrammarItem::Binder { .. } => {
                // Binder + body together form one Scope field
                field_count += 1;
            }
            crate::ast::GrammarItem::Terminal(_) => {}
        }
    }
    
    // Generate field names for ALL fields using term_name as prefix for uniqueness
    let term_name_str = term_name.to_string();
    let field_names: Vec<Ident> = (0..field_count)
        .map(|i| quote::format_ident!("{}_f{}", term_name_str, i as u32))
        .collect();
    
    // Generate pattern: if let Category::Constructor(field_0, field_1, ...) = term_name
    clauses.push(quote! {
        if let #category::#constructor(#(#field_names),*) = #term_name
    });
    
    // Find which field is the scope (combining binder and body)
    let (binder_idx, body_indices) = &grammar_rule.bindings[0];
    let body_idx = body_indices[0];
    
    // Map grammar indices to field/arg indices
    // AST fields: non-terminals (except body which is part of Scope) + binder (as Scope)
    let mut grammar_idx_to_field: Vec<Option<usize>> = vec![None; grammar_rule.items.len()];
    let mut field_idx = 0;
    for (grammar_idx, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::NonTerminal(_) => {
                if grammar_idx != body_idx {
                    // Regular non-terminal gets its own field
                    grammar_idx_to_field[grammar_idx] = Some(field_idx);
                    field_idx += 1;
                }
                // Body doesn't get a separate field - it's part of the Scope
            }
            crate::ast::GrammarItem::Binder { .. } => {
                // Binder itself points to the Scope field
                grammar_idx_to_field[grammar_idx] = Some(field_idx);
                // Body also points to the same Scope field
                grammar_idx_to_field[body_idx] = Some(field_idx);
                field_idx += 1;
            }
            crate::ast::GrammarItem::Terminal(_) => {}
        }
    }
    
    // Map grammar indices to arg indices  
    // Args in rewrite rule correspond to non-terminals AND binders (but binder appears twice: once for name, once for body)
    let mut grammar_idx_to_arg: Vec<Option<usize>> = vec![None; grammar_rule.items.len()];
    let mut arg_idx = 0;
    for (grammar_idx, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::NonTerminal(_) => {
                if arg_idx < args.len() {
                    grammar_idx_to_arg[grammar_idx] = Some(arg_idx);
                    arg_idx += 1;
                }
            }
            crate::ast::GrammarItem::Binder { .. } => {
                // Binder gets TWO arg slots: one for the binder name, one for the body
                // The binder name comes first
                if arg_idx < args.len() {
                    grammar_idx_to_arg[grammar_idx] = Some(arg_idx);
                    arg_idx += 1;
                }
                // Body comes next - map it too
                if body_idx < grammar_rule.items.len() && arg_idx < args.len() {
                    grammar_idx_to_arg[body_idx] = Some(arg_idx);
                    arg_idx += 1;
                }
            }
            crate::ast::GrammarItem::Terminal(_) => {}
        }
    }
    
    let scope_field_idx = grammar_idx_to_field[*binder_idx].expect("Binder should have field index");
    let scope_field = &field_names[scope_field_idx];
    
    // Unbind the scope
    let binder_var = quote::format_ident!("binder_{}", bindings.len());
    let body_var = quote::format_ident!("body_{}", bindings.len());
    
    clauses.push(quote! {
        let (#binder_var, #body_var) = #scope_field.clone().unbind()
    });
    
    // Bind the binder variable name if present in args
    if let Some(binder_arg_idx) = grammar_idx_to_arg[*binder_idx] {
        if binder_arg_idx < args.len() {
            if let Expr::Var(binder_name_var) = &args[binder_arg_idx] {
                let binder_name = binder_name_var.to_string();
                bindings.insert(binder_name, quote! { #binder_var });
            }
        }
    }
    
    // Process all arguments
    for (arg_idx, arg) in args.iter().enumerate() {
        // Check if this arg is the binder variable (skip it, already bound above)
        if grammar_idx_to_arg[*binder_idx] == Some(arg_idx) {
            continue;
        }
        
        // Check if this arg is the body
        if grammar_idx_to_arg[body_idx] == Some(arg_idx) {
            // This is the body - get its category from the grammar
            let body_category = match &grammar_rule.items[body_idx] {
                crate::ast::GrammarItem::NonTerminal(cat) => cat.clone(),
                _ => panic!("Body should be NonTerminal"),
            };
            generate_ascent_pattern(
                arg,
                &body_var,
                &body_category,
                theory,
                bindings,
                variable_categories,
                clauses,
                duplicate_vars,
                equational_checks,
            );
        } else {
            // Regular field - find which field it corresponds to
            // Find the grammar index for this arg
            if let Some((grammar_idx, item)) = grammar_rule.items.iter().enumerate()
                .find(|(gi, _)| grammar_idx_to_arg[*gi] == Some(arg_idx))
            {
                if let crate::ast::GrammarItem::NonTerminal(field_category) = item {
                    if let Some(field_idx) = grammar_idx_to_field[grammar_idx] {
                        let field_name = &field_names[field_idx];
                        let inner_var = quote::format_ident!("{}_val", field_name);
                        clauses.push(quote! {
                            let #inner_var = &**#field_name
                        });
                        
                        generate_ascent_pattern(
                            arg,
                            &inner_var,
                            field_category,
                            theory,
                            bindings,
                            variable_categories,
                            clauses,
                            duplicate_vars,
                            equational_checks,
                        );
                    }
                }
            }
        }
    }
    
    // Store binder binding (for legacy compatibility)
    bindings.insert(format!("binder_{}", binder_idx), quote! { #binder_var });
}

/// Generate pattern for regular constructor in Ascent
fn generate_ascent_regular_pattern(
    category: &Ident,
    constructor: &Ident,
    args: &[Expr],
    term_name: &Ident,
    theory: &TheoryDef,
    grammar_rule: &crate::ast::GrammarRule,
    bindings: &mut HashMap<String, TokenStream>,
    variable_categories: &mut HashMap<String, Ident>,
    clauses: &mut Vec<TokenStream>,
    duplicate_vars: &std::collections::HashSet<String>,
    equational_checks: &mut Vec<TokenStream>,
) {
    // Count non-terminal fields
    let field_count = grammar_rule.items.iter()
        .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
        .count();
    
    // Generate field names using term_name as prefix for uniqueness
    let term_name_str = term_name.to_string();
    let field_names: Vec<Ident> = (0..field_count)
        .map(|i| quote::format_ident!("{}_f{}", term_name_str, i))
        .collect();
    
    // Generate pattern: if let Category::Constructor(field_0, field_1, ...) = term_name
    clauses.push(quote! {
        if let #category::#constructor(#(#field_names),*) = #term_name
    });
    
    // Process each argument
    for (i, arg) in args.iter().enumerate() {
        if i >= field_names.len() {
            break;
        }
        
        // Get the category from the grammar for this field
        let field_category = if let Some(crate::ast::GrammarItem::NonTerminal(cat)) = 
            grammar_rule.items.iter()
                .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
                .nth(i) 
        {
            cat.clone()
        } else {
            panic!("Field {} should be NonTerminal", i);
        };
        
        let field_name = &field_names[i];
        
        match arg {
            Expr::Var(_) => {
                // Variable - need to dereference the Box
                let inner_var = quote::format_ident!("{}_val", field_name);
                clauses.push(quote! {
                    let #inner_var = &**#field_name
                });
                
                generate_ascent_pattern(
                    arg,
                    &inner_var,
                    &field_category,
                    theory,
                    bindings,
                    variable_categories,
                    clauses,
                    duplicate_vars,
                    equational_checks,
                );
            }
            Expr::Apply { .. } => {
                // Nested constructor - create inner term and recurse
                let inner_var = quote::format_ident!("{}_inner", field_name);
                clauses.push(quote! {
                    let #inner_var = &**#field_name
                });
                
                generate_ascent_pattern(
                    arg,
                    &inner_var,
                    &field_category,
                    theory,
                    bindings,
                    variable_categories,
                    clauses,
                    duplicate_vars,
                    equational_checks,
                );
            }
            Expr::Subst { .. } => {
                panic!("Substitution in LHS");
            }
        }
    }
}

/// Generate RHS construction for Ascent clause
fn generate_ascent_rhs(expr: &Expr, bindings: &HashMap<String, TokenStream>) -> TokenStream {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            if let Some(binding) = bindings.get(&var_name) {
                quote! { (#binding).clone() }
            } else {
                // Unbound variable - shouldn't happen
                quote! { #var }
            }
        }
        
        Expr::Apply { constructor, args } => {
            let category = extract_category(expr);
            let rhs_args: Vec<TokenStream> = args.iter()
                .map(|arg| {
                    let inner = generate_ascent_rhs(arg, bindings);
                    quote! { Box::new(#inner) }
                })
                .collect();
            
            quote! {
                #category::#constructor(#(#rhs_args),*)
            }
        }
        
        Expr::Subst { term, var, replacement } => {
            let term_rhs = generate_ascent_rhs(term, bindings);
            let var_binding = bindings.get(&var.to_string())
                .expect("Substitution variable not bound");
            let replacement_rhs = generate_ascent_rhs(replacement, bindings);
            
            // Determine the category of the replacement to call the right substitute method
            let replacement_category = extract_category(replacement).to_string().to_lowercase();
            let subst_method = quote::format_ident!("substitute_{}", replacement_category);
            
            quote! {
                (#term_rhs).#subst_method(&#var_binding.0, &#replacement_rhs)
            }
        }
    }
}
