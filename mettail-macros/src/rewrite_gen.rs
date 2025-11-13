use crate::ast::{TheoryDef, RewriteRule, Expr};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;
use std::collections::HashMap;


/// Generate Ascent clauses for rewrite rules (for equational matching)
/// This is the new approach that allows duplicate variables to use eq_cat() relations
pub fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    use crate::congruence_analysis;
    
    let mut all_clauses = Vec::new();
    
    // Find which categories are covered by collection congruences
    let collection_cong_categories = congruence_analysis::find_collection_congruence_element_categories(theory);
    
    for (rule_idx, rule) in theory.rewrites.iter().enumerate() {
        // Skip congruence rules (handled elsewhere)
        if rule.premise.is_some() {
            continue;
        }
        
        // Check if this base rewrite's LHS involves a collection pattern for a category
        // that's covered by a collection congruence
        let is_covered_by_congruence = if congruence_analysis::contains_collection_pattern(&rule.left) {
            // This rule has a collection pattern in its LHS
            // Get the element category from within the collection
            if let Some(elem_cat) = get_collection_element_category(&rule.left, theory) {
                // Check if this element category is covered by a collection congruence
                collection_cong_categories.contains(&elem_cat)
            } else {
                false
            }
        } else {
            false
        };
        
        if is_covered_by_congruence {
            // Skip this rule - it will be handled by congruence-driven projection
            continue;
        }
        
        // OLD PATH: For non-collection patterns or uncovered categories
        all_clauses.push(generate_rewrite_clause(rule, theory));
    }
    
    all_clauses
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
    let rhs = generate_ascent_rhs(&rule.right, &bindings, theory);
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
        Expr::CollectionPattern { elements, rest, .. } => {
            for elem in elements {
                collect_variable_occurrences(elem, occurrences, idx);
            }
            if let Some(rest_var) = rest {
                let var_name = rest_var.to_string();
                occurrences.entry(var_name).or_insert_with(Vec::new).push(*idx);
                *idx += 1;
            }
        }
    }
}

/// ========== END PHASE 6.2 ==========

/// Generate Ascent pattern matching clauses for LHS
/// Handles nested patterns and tracks bindings for equational checks
/// `expected_category` is the category this expression should have based on its context
/// Made public for use in congruence projection generation
pub fn generate_ascent_pattern(
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
        
        Expr::CollectionPattern { constructor, elements, rest } => {
            // Collection patterns in LHS need special handling
            generate_ascent_collection_pattern(
                constructor,
                elements,
                rest,
                term_name,
                expected_category,
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

/// Generate Ascent pattern matching for collection patterns with rest variables
/// Handles patterns like `{P, Q, ...rest}` by:
/// 1. Matching the constructor that contains a collection
/// 2. Checking minimum size
/// 3. Extracting specific elements
/// 4. Binding the rest to remaining elements
#[allow(clippy::too_many_arguments)]
fn generate_ascent_collection_pattern(
    constructor: &Option<Ident>,
    elements: &[Expr],
    rest: &Option<Ident>,
    term_name: &Ident,
    expected_category: &Ident,
    theory: &TheoryDef,
    bindings: &mut HashMap<String, TokenStream>,
    variable_categories: &mut HashMap<String, Ident>,
    clauses: &mut Vec<TokenStream>,
    duplicate_vars: &std::collections::HashSet<String>,
    equational_checks: &mut Vec<TokenStream>,
) {
    // Find the constructor that contains a collection field
    let constructor_ident = if let Some(cons) = constructor {
        cons.clone()
                            } else {
        // No explicit constructor - need to infer from context
        // For now, require explicit constructor
        panic!("Collection patterns must specify explicit constructor (e.g., PPar {{P, ...rest}})");
    };
    
    // Find the grammar rule
    let grammar_rule = theory.terms.iter()
        .find(|r| r.label == constructor_ident && r.category == *expected_category)
        .unwrap_or_else(|| panic!(
            "Constructor {} (category: {}) not found in theory",
            constructor_ident,
            expected_category
        ));
    
    // Find the collection field in this constructor
    let collection_info = grammar_rule.items.iter()
        .enumerate()
        .find_map(|(idx, item)| {
            if let crate::ast::GrammarItem::Collection { element_type, .. } = item {
                Some((idx, element_type.clone()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!(
            "Constructor {} does not have a collection field",
            constructor_ident
        ));
    
    let (_field_idx, element_category) = collection_info;
    
    // Generate pattern match for the constructor
    // For now, assume the collection is the only field (like PPar(HashBag<Proc>))
    let bag_var = quote::format_ident!("{}_bag", term_name);
    
    clauses.push(quote! {
        if let #expected_category::#constructor_ident(#bag_var) = #term_name
    });
    
    // Generate loop-based matching for each element (order-independent!)
    let mut elem_vars = Vec::new();
    
    for (elem_idx, elem_pattern) in elements.iter().enumerate() {
        let elem_var = quote::format_ident!("{}_elem_{}", term_name, elem_idx);
        let count_var = quote::format_ident!("_count_{}_{}", term_name, elem_idx);
        elem_vars.push(elem_var.clone());
        
        // Generate: for (elem_var, _count_NAME_N) in bag_field.iter()
        clauses.push(quote! {
            for (#elem_var, #count_var) in #bag_var.iter()
        });
        
        // Add distinctness checks (ensure we don't match the same element twice)
        for prev_elem_var in &elem_vars[..elem_idx] {
            clauses.push(quote! {
                if &#elem_var != &#prev_elem_var
            });
        }
        
        // Recursively generate pattern for this element
        // This handles Var, Apply, and any nested structures
        generate_ascent_pattern(
            elem_pattern,
            &elem_var,
            &element_category,
            theory,
            bindings,
            variable_categories,
            clauses,
            duplicate_vars,
            equational_checks,
        );
    }
    
    // Bind rest variable if present
    if let Some(rest_var) = rest {
        let rest_var_name = rest_var.to_string();
        let rest_ident = quote::format_ident!("{}_rest", term_name);
        
        if !elem_vars.is_empty() {
            // Build rest by removing matched elements
            clauses.push(quote! {
                let #rest_ident = {
                    let mut bag = #bag_var.clone();
                    #(bag.remove(&#elem_vars);)*
                    bag
                }
            });
        } else {
            // No specific elements - rest is the whole bag
            clauses.push(quote! {
                let #rest_ident = #bag_var.clone()
            });
        }
        
        // Bind the rest variable
        // Rest has type HashBag<ElementCategory>, not ElementCategory
        bindings.insert(rest_var_name, quote! { #rest_ident });
        // Don't add to variable_categories since it's a different type (collection vs element)
    }
}

/// Get the element category from within a collection pattern
/// For example, (PPar {(PAmb ...), ...}) returns Proc (the element type)
fn get_collection_element_category(expr: &Expr, theory: &TheoryDef) -> Option<Ident> {
    use crate::ast::GrammarItem;
    
    match expr {
        Expr::Apply { constructor, args } => {
            // Check if this constructor has a collection field
            if let Some(grammar_rule) = theory.terms.iter().find(|r| r.label == *constructor) {
                for item in &grammar_rule.items {
                    if let GrammarItem::Collection { element_type, .. } = item {
                        return Some(element_type.clone());
                    }
                }
            }
            
            // Recursively check args
            for arg in args {
                if let Some(cat) = get_collection_element_category(arg, theory) {
                    return Some(cat);
                }
            }
            None
        }
        _ => None,
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
        Expr::CollectionPattern { constructor, .. } => {
            if let Some(cons) = constructor {
                // Use the provided constructor to infer category
                let name = cons.to_string();
                if name.starts_with('P') {
                    syn::Ident::new("Proc", cons.span())
                } else if name.starts_with('N') {
                    syn::Ident::new("Name", cons.span())
                } else {
                    cons.clone()
                }
            } else {
                // No constructor - default to a placeholder
                syn::Ident::new("Unknown", proc_macro2::Span::call_site())
            }
        }
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
    let (_binder_idx, body_indices) = &grammar_rule.bindings[0];
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
            crate::ast::GrammarItem::Collection { .. } => {
                // Collection counts as a field
                field_count += 1;
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
            crate::ast::GrammarItem::Collection { .. } => {
                // Collection gets its own field
                grammar_idx_to_field[grammar_idx] = Some(field_idx);
                field_idx += 1;
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
            crate::ast::GrammarItem::Collection { .. } => {
                // Collection gets an arg slot
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
            
            // For body variables, create a dereferenced binding
            // body_0 is Box<Proc>, so we want to access *body_0
            let body_deref = quote::format_ident!("{}_deref", body_var);
            clauses.push(quote! {
                let #body_deref = body_0.as_ref()
            });
            
            generate_ascent_pattern(
                arg,
                &body_deref,
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
                            let #inner_var = #field_name.as_ref()
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
    // Count ALL fields (NonTerminal AND Collection)
    let field_count = grammar_rule.items.iter()
        .filter(|item| matches!(
            item,
            crate::ast::GrammarItem::NonTerminal(_) | crate::ast::GrammarItem::Collection { .. }
        ))
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
        
        // Get the category/type from the grammar for this field
        let (field_info, is_collection) = grammar_rule.items.iter()
            .filter(|item| matches!(
                item,
                crate::ast::GrammarItem::NonTerminal(_) | crate::ast::GrammarItem::Collection { .. }
            ))
            .nth(i)
            .map(|item| match item {
                crate::ast::GrammarItem::NonTerminal(cat) => (cat.clone(), false),
                crate::ast::GrammarItem::Collection { element_type, .. } => (element_type.clone(), true),
                _ => unreachable!(),
            })
            .unwrap_or_else(|| panic!("Field {} not found in grammar rule", i));
        
        let field_category = field_info;
        let field_name = &field_names[i];
        
        match arg {
            Expr::CollectionPattern { elements, rest, .. } if is_collection => {
                // This argument is a collection pattern matching a collection field
                // The field_name already points to the HashBag from the constructor match
                // Generate loop-based matching for order independence!
                
                let mut elem_vars = Vec::new();
                
                // Generate loop-based matching for each element
                for (elem_idx, elem_pattern) in elements.iter().enumerate() {
                    let elem_var = quote::format_ident!("{}_elem_{}", field_name, elem_idx);
                    let count_var = quote::format_ident!("_count_{}_{}", field_name, elem_idx);
                    elem_vars.push(elem_var.clone());
                    
                    // Generate: for (elem_var, _count_FIELD_N) in field.iter()
                    clauses.push(quote! {
                        for (#elem_var, #count_var) in #field_name.iter()
                    });
                    
                    // Add distinctness checks (ensure we don't match the same element twice)
                    for prev_elem_var in &elem_vars[..elem_idx] {
                        clauses.push(quote! {
                            if &#elem_var != &#prev_elem_var
                        });
                    }
                    
                    // Recursively generate pattern for this element
                    // This handles Var, Apply, and any nested structures
                    generate_ascent_pattern(
                        elem_pattern,
                        &elem_var,
                        &field_category,
                        theory,
                        bindings,
                        variable_categories,
                        clauses,
                        duplicate_vars,
                        equational_checks,
                    );
                }
                
                // Bind rest variable if present
                if let Some(rest_var) = rest {
                    let rest_var_name = rest_var.to_string();
                    let rest_ident = quote::format_ident!("{}_rest", field_name);
                    
                    if !elem_vars.is_empty() {
                        // Build rest by removing matched elements
                        clauses.push(quote! {
                            let #rest_ident = {
                                let mut bag = #field_name.clone();
                                #(bag.remove(&#elem_vars);)*
                                bag
                            }
                        });
                    } else {
                        clauses.push(quote! {
                            let #rest_ident = #field_name.clone()
                        });
                    }
                    
                    bindings.insert(rest_var_name, quote! { #rest_ident });
                }
            }
            Expr::Var(_) => {
                // Variable - need to dereference the Box
                let inner_var = quote::format_ident!("{}_val", field_name);
                clauses.push(quote! {
                    let #inner_var = #field_name.as_ref()
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
                    let #inner_var = #field_name.as_ref()
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
            Expr::CollectionPattern { .. } => {
                panic!("Collection pattern in LHS - not yet implemented");
            }
        }
    }
}

/// Generate RHS construction for Ascent clause
fn generate_ascent_rhs(expr: &Expr, bindings: &HashMap<String, TokenStream>, theory: &TheoryDef) -> TokenStream {
    match expr {
        Expr::CollectionPattern { .. } => {
            // If we reach here, it's a bare collection pattern (not inside Apply)
            // Fall back to no-flatten version
            generate_ascent_collection_rhs(expr, bindings, theory, None)
        }
        _ => generate_ascent_rhs_inner(expr, bindings, theory)
    }
}

/// Generate RHS for collection patterns, optionally using flatten helper
/// 
/// If `constructor_context` is Some((category, label)), uses the flatten helper.
/// Otherwise, uses plain `bag.insert`.
fn generate_ascent_collection_rhs(
    expr: &Expr,
    bindings: &HashMap<String, TokenStream>,
    theory: &TheoryDef,
    constructor_context: Option<(syn::Ident, syn::Ident)>
) -> TokenStream {
    if let Expr::CollectionPattern { constructor: _, elements, rest } = expr {
        let elem_constructions: Vec<TokenStream> = elements.iter()
            .map(|e| generate_ascent_rhs_inner(e, bindings, theory))
            .collect();
        
        let coll_type = quote! { mettail_runtime::HashBag };
        
        if let Some((category, label)) = constructor_context {
            // Use flatten helper
            let helper_name = quote::format_ident!("insert_into_{}", label.to_string().to_lowercase());
            
            if let Some(rest_var) = rest {
                // Merge rest with new elements using flatten helper
                let rest_var_name = rest_var.to_string();
                let rest_binding = bindings.get(&rest_var_name)
                    .unwrap_or_else(|| panic!(
                        "Rest variable '{}' not bound. Available bindings: {:?}",
                        rest_var_name,
                        bindings.keys().collect::<Vec<_>>()
                    ));
        
        quote! {
                    {
                        let mut bag = (#rest_binding).clone();
                        #(#category::#helper_name(&mut bag, #elem_constructions);)*
                        bag
                    }
                }
            } else {
                // Build from elements using flatten helper
                quote! {
                    {
                        let mut bag = #coll_type::new();
                        #(#category::#helper_name(&mut bag, #elem_constructions);)*
                        bag
                    }
                }
            }
        } else {
            // No constructor context - use plain insert (shouldn't flatten)
            if let Some(rest_var) = rest {
                let rest_var_name = rest_var.to_string();
                let rest_binding = bindings.get(&rest_var_name)
                    .unwrap_or_else(|| panic!(
                        "Rest variable '{}' not bound. Available bindings: {:?}",
                        rest_var_name,
                        bindings.keys().collect::<Vec<_>>()
                    ));
    
    quote! {
                    {
                        let mut bag = (#rest_binding).clone();
                        #(bag.insert(#elem_constructions);)*
                        bag
                    }
                }
            } else {
                quote! {
                    {
                        let mut bag = #coll_type::new();
                        #(bag.insert(#elem_constructions);)*
                        bag
                    }
                }
            }
        }
    } else {
        panic!("generate_ascent_collection_rhs called on non-CollectionPattern");
    }
}

/// Internal RHS generation - does not handle top-level CollectionPattern
/// Made public for use in congruence generation
pub fn generate_ascent_rhs_inner(expr: &Expr, bindings: &HashMap<String, TokenStream>, theory: &TheoryDef) -> TokenStream {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            if let Some(binding) = bindings.get(&var_name) {
                // The binding already includes .clone() if needed
                // If it's a Box type, we need to get the contents
                // For now, just use the binding as-is - the binding should already produce the right type
                quote! { #binding }
            } else {
                // Unbound variable - check if it's a constructor
                if let Some(rule) = theory.terms.iter().find(|r| r.label == *var) {
                    // It's a nullary constructor - qualify it
                    let category = &rule.category;
                    quote! { #category::#var }
                } else {
                    // Unknown identifier - this shouldn't happen
                    panic!("Unbound variable '{}' in RHS", var_name);
                }
            }
        }
        
        Expr::Apply { constructor, args } => {
            let category = extract_category(expr);
            
            // Check if this constructor has collection fields
            let grammar_rule = theory.terms.iter()
                .find(|r| r.label == *constructor && r.category == category);
            
            let rhs_args: Vec<TokenStream> = args.iter().enumerate()
                .map(|(i, arg)| {
                    // Check if this argument position corresponds to a Collection field
                    let is_collection_field = grammar_rule
                        .and_then(|rule| {
                            rule.items.iter()
                                .filter(|item| matches!(
                                    item,
                                    crate::ast::GrammarItem::NonTerminal(_) | 
                                    crate::ast::GrammarItem::Collection { .. }
                                ))
                                .nth(i)
                        })
                        .map(|item| matches!(item, crate::ast::GrammarItem::Collection { .. }))
                        .unwrap_or(false);
                    
                    // For collection fields, pass the constructor label so flatten helper can be used
                    let inner = if is_collection_field && matches!(arg, Expr::CollectionPattern { .. }) {
                        generate_ascent_collection_rhs(arg, bindings, theory, Some((category.clone(), constructor.clone())))
                    } else {
                        generate_ascent_rhs(arg, bindings, theory)
                    };
                    
                    // Don't wrap collection fields in Box::new
                    if is_collection_field {
                        inner
                    } else {
                        quote! { Box::new(#inner) }
                    }
                })
                .collect();
            
            quote! {
                #category::#constructor(#(#rhs_args),*)
            }
        }
        
        Expr::Subst { term, var, replacement } => {
            let term_rhs = generate_ascent_rhs(term, bindings, theory);
            let var_name = var.to_string();
            let var_binding = bindings.get(&var_name)
                .unwrap_or_else(|| panic!(
                    "Substitution variable '{}' not bound. Available bindings: {:?}",
                    var_name,
                    bindings.keys().collect::<Vec<_>>()
                ));
            let replacement_rhs = generate_ascent_rhs(replacement, bindings, theory);
            
            // Determine the category of the replacement to call the right substitute method
            let replacement_category = extract_category(replacement).to_string().to_lowercase();
            let subst_method = quote::format_ident!("substitute_{}", replacement_category);
            
            quote! {
                (#term_rhs).#subst_method(&#var_binding.0, &#replacement_rhs)
            }
        }
        
        Expr::CollectionPattern { .. } => {
            // This should have been handled at the top level by generate_ascent_rhs
            panic!("CollectionPattern should be handled by generate_ascent_rhs/generate_ascent_collection_rhs");
        }
    }
}
