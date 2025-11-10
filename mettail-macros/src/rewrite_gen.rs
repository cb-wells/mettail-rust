use crate::ast::{TheoryDef, RewriteRule, Expr};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;
use std::collections::HashMap;


/// Metadata for automatic indexed projection generation
#[derive(Debug, Clone)]
pub struct ProjectionSpec {
    /// The collection field being matched
    pub collection_field_idx: usize,
    
    /// Nested patterns within the collection
    pub element_patterns: Vec<ElementPattern>,
    
    /// Variables shared across patterns (join keys)
    pub shared_variables: Vec<String>,
    
    /// Rest variable binding, if present
    pub rest_variable: Option<String>,
    
    /// The constructor containing the collection
    pub parent_constructor: Ident,
    
    /// The category of the parent constructor
    pub parent_category: Ident,
}

/// A single element pattern within a collection
#[derive(Debug, Clone)]
pub struct ElementPattern {
    /// The nested constructor (e.g., PInput, POutput)
    pub constructor: Ident,
    
    /// Category of this constructor
    pub category: Ident,
    
    /// Position in the collection pattern
    pub pattern_idx: usize,
    
    /// Variables to capture (var_name, category, field_index)
    pub captures: Vec<CaptureInfo>,
    
    /// Which captures are join keys (indices into captures vec)
    pub join_key_indices: Vec<usize>,
}

/// Information about a captured variable
#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub var_name: String,
    pub category: Ident,
    pub field_idx: usize,
    pub is_binder: bool,
}

/// Generate Ascent clauses for rewrite rules (for equational matching)
/// This is the new approach that allows duplicate variables to use eq_cat() relations
pub fn generate_rewrite_clauses(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut all_clauses = Vec::new();
    
    for (rule_idx, rule) in theory.rewrites.iter().enumerate() {
        // Skip congruence rules (handled elsewhere)
        if rule.premise.is_some() {
            continue;
        }
        
        // Check if this rule requires indexed projection approach
        if requires_indexed_projection(rule, theory) {
            // NEW PATH: Generate indexed projection-based rewrite
            if let Some(spec) = analyze_collection_pattern(&rule.left, theory) {
                // Generate projection relations
                let relations = generate_projection_relations(rule_idx, &spec);
                all_clauses.extend(relations);
                
                // Generate extraction rules
                let extractions = generate_extraction_rules(rule_idx, &spec, theory);
                all_clauses.extend(extractions);
                
                // Generate join-based rewrite
                let rewrite = generate_join_rewrite(rule_idx, &spec, rule, theory);
                all_clauses.push(rewrite);
            } else {
                // Fallback to old approach if analysis fails
                all_clauses.push(generate_rewrite_clause(rule, theory));
            }
        } else {
            // OLD PATH: Use existing generation logic
            all_clauses.push(generate_rewrite_clause(rule, theory));
        }
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

/// Check if a rewrite rule requires indexed projection approach
/// Returns true if the LHS has a collection pattern with:
/// 1. Nested Apply patterns (not just variables)
/// 2. Shared variables across those nested patterns
pub fn requires_indexed_projection(rule: &RewriteRule, _theory: &TheoryDef) -> bool {
    // Check if LHS is an Apply with collection pattern arguments
    if let Expr::Apply { args, .. } = &rule.left {
        for arg in args {
            if let Expr::CollectionPattern { elements, .. } = arg {
                // Check if any elements are Apply (nested constructors)
                let has_nested_apply = elements.iter().any(|e| matches!(e, Expr::Apply { .. }));
                
                if !has_nested_apply {
                    continue; // Just variables, no need for projections
                }
                
                // Check for shared variables across elements
                let shared_vars = find_shared_variables_in_collection(elements);
                
                if !shared_vars.is_empty() {
                    return true; // Need indexed projection!
                }
            }
        }
    }
    false
}

/// Find variables that appear in multiple elements of a collection pattern
fn find_shared_variables_in_collection(elements: &[Expr]) -> Vec<String> {
    let mut var_to_elements: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (elem_idx, elem) in elements.iter().enumerate() {
        let vars = collect_vars_in_expr(elem);
        for var in vars {
            var_to_elements.entry(var).or_insert_with(Vec::new).push(elem_idx);
        }
    }
    
    // Return variables that appear in multiple elements
    var_to_elements
        .into_iter()
        .filter(|(_, indices)| indices.len() > 1)
        .map(|(var, _)| var)
        .collect()
}

/// Collect all variable names in an expression
fn collect_vars_in_expr(expr: &Expr) -> Vec<String> {
    let mut vars = Vec::new();
    collect_vars_recursive(expr, &mut vars);
    vars
}

fn collect_vars_recursive(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Var(v) => {
            vars.push(v.to_string());
        }
        Expr::Apply { args, .. } => {
            for arg in args {
                collect_vars_recursive(arg, vars);
            }
        }
        Expr::Subst { term, .. } => {
            collect_vars_recursive(term, vars);
        }
        Expr::CollectionPattern { elements, rest, .. } => {
            for elem in elements {
                collect_vars_recursive(elem, vars);
            }
            if let Some(rest_var) = rest {
                vars.push(rest_var.to_string());
            }
        }
    }
}

/// Analyze a collection pattern and extract projection specification
pub fn analyze_collection_pattern(
    lhs: &Expr,
    theory: &TheoryDef,
) -> Option<ProjectionSpec> {
    // LHS must be an Apply (the parent constructor)
    let (parent_constructor, parent_args) = match lhs {
        Expr::Apply { constructor, args } => (constructor.clone(), args),
        _ => return None,
    };
    
    // Find the category of the parent
    let parent_category = theory.terms.iter()
        .find(|r| r.label == parent_constructor)
        .map(|r| r.category.clone())?;
    
    // Find which argument is the collection pattern
    let mut collection_field_idx = None;
    let mut collection_pattern = None;
    
    for (idx, arg) in parent_args.iter().enumerate() {
        if let Expr::CollectionPattern { .. } = arg {
            collection_field_idx = Some(idx);
            collection_pattern = Some(arg);
            break;
        }
    }
    
    let collection_field_idx = collection_field_idx?;
    let collection_pattern = collection_pattern?;
    
    // Extract element patterns and captures
    if let Expr::CollectionPattern { elements, rest, .. } = collection_pattern {
        let mut element_patterns = Vec::new();
        let mut all_var_occurrences: HashMap<String, Vec<usize>> = HashMap::new();
        
        // Analyze each element pattern
        for (pattern_idx, elem) in elements.iter().enumerate() {
            if let Expr::Apply { constructor, args } = elem {
                // Find the grammar rule for this constructor
                let grammar_rule = theory.terms.iter()
                    .find(|r| r.label == *constructor)?;
                
                let category = grammar_rule.category.clone();
                
                // Extract captures from the arguments
                let captures = extract_captures_from_args(args, grammar_rule, theory);
                
                // Track variable occurrences across patterns
                for capture in &captures {
                    all_var_occurrences
                        .entry(capture.var_name.clone())
                        .or_insert_with(Vec::new)
                        .push(pattern_idx);
                }
                
                element_patterns.push(ElementPattern {
                    constructor: constructor.clone(),
                    category,
                    pattern_idx,
                    captures,
                    join_key_indices: vec![], // Filled in next step
                });
            } else {
                // Not an Apply pattern - skip for now
                // (Could be just Var, which doesn't need projection)
                return None;
            }
        }
        
        // Identify shared variables (appear in multiple patterns)
        let shared_variables: Vec<String> = all_var_occurrences
            .iter()
            .filter(|(_, indices)| indices.len() > 1)
            .map(|(var, _)| var.clone())
            .collect();
        
        if shared_variables.is_empty() {
            // No shared variables - no need for indexed projection
            return None;
        }
        
        // Mark which captures are join keys
        for pattern in &mut element_patterns {
            for (capture_idx, capture) in pattern.captures.iter().enumerate() {
                if shared_variables.contains(&capture.var_name) {
                    pattern.join_key_indices.push(capture_idx);
                }
            }
        }
        
        Some(ProjectionSpec {
            collection_field_idx,
            element_patterns,
            shared_variables,
            rest_variable: rest.as_ref().map(|r| r.to_string()),
            parent_constructor,
            parent_category,
        })
    } else {
        None
    }
}

/// Extract capture information from constructor arguments
fn extract_captures_from_args(
    args: &[Expr],
    grammar_rule: &crate::ast::GrammarRule,
    theory: &TheoryDef,
) -> Vec<CaptureInfo> {
    let mut captures = Vec::new();
    
    // Get the list of all argument positions (non-terminals AND binders, in order)
    let non_term_indices: Vec<usize> = grammar_rule.items.iter()
        .enumerate()
        .filter(|(_, item)| matches!(item, 
            crate::ast::GrammarItem::NonTerminal(_) | 
            crate::ast::GrammarItem::Binder { .. } |
            crate::ast::GrammarItem::Collection { .. }))
        .map(|(idx, _)| idx)
        .collect();
    
    // Map grammar indices to field indices
    let grammar_to_field = build_grammar_to_field_map(grammar_rule);
    
    // Process each argument
    for (arg_idx, arg) in args.iter().enumerate() {
        match arg {
            Expr::Var(var) => {
                let var_name = var.to_string();
                
                // Get the grammar index for this argument
                if arg_idx >= non_term_indices.len() {
                    panic!(
                        "Argument {} out of range for constructor {}. \
                         Pattern has {} args but grammar has {} non-terminals",
                        arg_idx, grammar_rule.label, args.len(), non_term_indices.len()
                    );
                }
                let grammar_idx = non_term_indices[arg_idx];
                
                // Get the field index
                let field_idx = grammar_to_field[grammar_idx];
                
                let category = get_field_category(grammar_idx, grammar_rule, theory);
                
                // Check if this is a binder
                let is_binder = grammar_rule.bindings.iter()
                    .any(|(binder_idx, _)| *binder_idx == grammar_idx);
                
                captures.push(CaptureInfo {
                    var_name,
                    category,
                    field_idx,
                    is_binder,
                });
            }
            Expr::Apply { .. } => {
                // Nested constructor - not handled yet
            }
            _ => {}
        }
    }
    
    captures
}

/// Build a map from argument positions to grammar indices
/// This skips over terminals and maps to NonTerminal/Collection positions
fn build_arg_to_grammar_map(grammar_rule: &crate::ast::GrammarRule) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    let mut arg_idx = 0;
    
    for (grammar_idx, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::NonTerminal(_) | 
            crate::ast::GrammarItem::Collection { .. } => {
                map.insert(arg_idx, grammar_idx);
                arg_idx += 1;
            }
            _ => {} // Terminals - skip
        }
    }
    
    map
}

/// Build a map from grammar indices to field indices
/// This accounts for binders collapsing into scope fields
fn build_grammar_to_field_map(grammar_rule: &crate::ast::GrammarRule) -> Vec<usize> {
    let mut field_idx_map = vec![0; grammar_rule.items.len()];
    let mut current_field = 0;
    
    // Find binder positions
    let binder_positions: std::collections::HashSet<usize> = grammar_rule.bindings.iter()
        .map(|(binder_idx, _)| *binder_idx)
        .collect();
    
    let body_positions: std::collections::HashSet<usize> = grammar_rule.bindings.iter()
        .flat_map(|(_, body_indices)| body_indices.iter().copied())
        .collect();
    
    for (grammar_idx, item) in grammar_rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::Binder { .. } => {
                // Binder starts a new scope field
                field_idx_map[grammar_idx] = current_field;
                // Don't increment - the body will share this field
            }
            crate::ast::GrammarItem::NonTerminal(_) | 
            crate::ast::GrammarItem::Collection { .. } => {
                if body_positions.contains(&grammar_idx) {
                    // This is a body - it shares the field with its binder
                    // Find the binder for this body
                    let binder_idx = grammar_rule.bindings.iter()
                        .find(|(_, bodies)| bodies.contains(&grammar_idx))
                        .map(|(b, _)| *b)
                        .expect("Body without binder");
                    
                    // Use the same field index as the binder
                    field_idx_map[grammar_idx] = field_idx_map[binder_idx];
                    // Increment for next field after processing the scope
                    current_field += 1;
                } else {
                    // Regular field
                    field_idx_map[grammar_idx] = current_field;
                    current_field += 1;
                }
            }
            _ => {} // Terminals
        }
    }
    
    field_idx_map
}

/// Find the grammar index for a given argument index
fn find_grammar_index_for_arg(arg_idx: usize, grammar_rule: &crate::ast::GrammarRule) -> usize {
    // Map from argument position to grammar item position
    // Skip over terminals
    let mut current_arg = 0;
    for (grammar_idx, item) in grammar_rule.items.iter().enumerate() {
        if matches!(item, crate::ast::GrammarItem::NonTerminal(_) | 
                         crate::ast::GrammarItem::Collection { .. }) {
            if current_arg == arg_idx {
                return grammar_idx;
            }
            current_arg += 1;
        }
    }
    panic!("Argument index {} not found in grammar rule", arg_idx);
}

/// Get the category of a grammar field
fn get_field_category(
    grammar_idx: usize,
    grammar_rule: &crate::ast::GrammarRule,
    _theory: &TheoryDef,
) -> Ident {
    match &grammar_rule.items[grammar_idx] {
        crate::ast::GrammarItem::NonTerminal(cat) => cat.clone(),
        crate::ast::GrammarItem::Binder { category } => category.clone(),
        crate::ast::GrammarItem::Collection { element_type, .. } => element_type.clone(),
        _ => panic!("Expected NonTerminal, Binder, or Collection at grammar index {}", grammar_idx),
    }
}

/// ========== PHASE 6.2: CODE GENERATION ==========

/// Generate projection relations for a rewrite rule
/// Returns: Vec of relation declarations
pub fn generate_projection_relations(
    rule_idx: usize,
    spec: &ProjectionSpec,
) -> Vec<TokenStream> {
    let mut relations = Vec::new();
    
    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_r{}_p{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );
        
        // Build relation signature:
        // (Parent, JoinKey1, JoinKey2, ..., OtherCapture1, OtherCapture2, ..., OriginalElement)
        let mut field_types: Vec<TokenStream> = vec![
            {
                let cat = &spec.parent_category;
                quote! { #cat }
            },
        ];
        
        // Add join key types (shared variables) in order
        for shared_var in &spec.shared_variables {
            // Find this variable in the captures
            if let Some(capture) = elem_pattern.captures.iter()
                .find(|c| c.var_name == *shared_var) 
            {
                // Use the actual captured type
                if capture.is_binder {
                    // Binders are captured as Binder<String>
                    field_types.push(quote! { mettail_runtime::Binder<String> });
                } else {
                    let cat = &capture.category;
                    field_types.push(quote! { #cat });
                }
            }
        }
        
        // Add non-join capture types
        for capture in &elem_pattern.captures {
            if !spec.shared_variables.contains(&capture.var_name) {
                if capture.is_binder {
                    // Binders store Binder<String>
                    field_types.push(quote! { mettail_runtime::Binder<String> });
                } else {
                    let cat = &capture.category;
                    field_types.push(quote! { #cat });
                }
            }
        }
        
        // Add original element type
        let elem_cat = &elem_pattern.category;
        field_types.push(quote! { #elem_cat });
        
        relations.push(quote! {
            relation #rel_name(#(#field_types),*);
        });
    }
    
    relations
}

/// Generate extraction rules that populate projection relations
pub fn generate_extraction_rules(
    rule_idx: usize,
    spec: &ProjectionSpec,
    theory: &TheoryDef,
) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    
    let parent_cat_lower = format_ident!("{}", spec.parent_category.to_string().to_lowercase());
    let parent_constructor = &spec.parent_constructor;
    let parent_category = &spec.parent_category;
    
    // Get the grammar rule for the parent constructor
    let _parent_grammar = theory.terms.iter()
        .find(|r| r.label == *parent_constructor)
        .expect("Parent constructor not found in theory");
    
    // Find the collection field
    let collection_field_name = format_ident!("bag_field");
    
    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_r{}_p{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );
        
        let elem_var = format_ident!("elem");
        
        // Generate the pattern match for the nested constructor
        let nested_grammar = theory.terms.iter()
            .find(|r| r.label == elem_pattern.constructor)
            .expect("Element constructor not found in theory");
        
        // Build field pattern for the nested constructor
        let mut field_patterns = Vec::new();
        let mut let_clauses = Vec::new();
        
        // Map grammar indices to field indices for the nested constructor
        let nested_grammar_to_field = build_grammar_to_field_map(nested_grammar);
        
        // Find body positions
        let body_positions: std::collections::HashSet<usize> = nested_grammar.bindings.iter()
            .flat_map(|(_, bodies)| bodies.iter().copied())
            .collect();
        
        // Count actual fields and generate patterns
        let mut field_idx = 0;
        for (grammar_idx, item) in nested_grammar.items.iter().enumerate() {
            match item {
                crate::ast::GrammarItem::Binder { .. } => {
                    // Binder creates a scope field
                    let field_name = format_ident!("f{}", field_idx as u32);
                    field_patterns.push(quote! { ref #field_name });
                    field_idx += 1;
                }
                crate::ast::GrammarItem::NonTerminal(_) | 
                crate::ast::GrammarItem::Collection { .. } => {
                    // Only create a pattern if this isn't a body (bodies are part of scope)
                    if !body_positions.contains(&grammar_idx) {
                        let field_name = format_ident!("f{}", field_idx as u32);
                        field_patterns.push(quote! { ref #field_name });
                        field_idx += 1;
                    }
                }
                _ => {} // Terminals - skip
            }
        }
        
        // Generate extraction for each capture
        // Group captures by field (binder+body share the same field)
        let mut field_captures: HashMap<usize, Vec<&CaptureInfo>> = HashMap::new();
        for capture in &elem_pattern.captures {
            field_captures.entry(capture.field_idx)
                .or_insert_with(Vec::new)
                .push(capture);
        }
        
        let mut capture_vars: HashMap<String, Ident> = HashMap::new();
        
        for (field_idx, captures) in field_captures {
            let field_name = format_ident!("f{}", field_idx);
            
            // Check if this field contains a binder
            let has_binder = captures.iter().any(|c| c.is_binder);
            
            if has_binder {
                // This is a scope field - unbind it
                let binder_capture = captures.iter().find(|c| c.is_binder)
                    .expect("Binder should exist");
                let body_capture = captures.iter().find(|c| !c.is_binder);
                
                let binder_var = format_ident!("binder_{}", binder_capture.var_name.to_lowercase());
                let body_var = format_ident!("body_{}", binder_capture.var_name.to_lowercase());
                
                let_clauses.push(quote! {
                    let (#binder_var, #body_var) = (*#field_name).clone().unbind()
                });
                
                // Store the binder (keep as Binder, not extracted)
                capture_vars.insert(binder_capture.var_name.clone(), binder_var.clone());
                
                // Store the body variable (dereference Box)
                if let Some(body_cap) = body_capture {
                    let body_deref_var = format_ident!("deref_{}", body_cap.var_name.to_lowercase());
                    let_clauses.push(quote! {
                        let #body_deref_var = (*#body_var).clone()
                    });
                    capture_vars.insert(body_cap.var_name.clone(), body_deref_var);
                }
            } else {
                // Regular field(s) - just dereference Box
                for capture in captures {
                    let capture_var = format_ident!("cap_{}", capture.var_name.to_lowercase());
                    let_clauses.push(quote! {
                        let #capture_var = (**#field_name).clone()
                    });
                    capture_vars.insert(capture.var_name.clone(), capture_var);
                }
            }
        }
        
        // Build the relation fact arguments in the right order
        let mut fact_args = vec![
            quote! { parent.clone() },
        ];
        
        // Add join keys first (in order of shared_variables)
        for shared_var in &spec.shared_variables {
            if let Some(var_ident) = capture_vars.get(shared_var) {
                fact_args.push(quote! { #var_ident.clone() });
            }
        }
        
        // Add non-join captures (in the original order from elem_pattern.captures)
        for capture in &elem_pattern.captures {
            if !spec.shared_variables.contains(&capture.var_name) {
                if let Some(var_ident) = capture_vars.get(&capture.var_name) {
                    fact_args.push(quote! { #var_ident.clone() });
                }
            }
        }
        
        // Add original element
        fact_args.push(quote! { #elem_var.clone() });
        
        let elem_constructor = &elem_pattern.constructor;
        let elem_category = &elem_pattern.category;
        
        rules.push(quote! {
            #rel_name(#(#fact_args),*) <--
                #parent_cat_lower(parent),
                if let #parent_category::#parent_constructor(ref #collection_field_name) = parent,
                for (#elem_var, _count) in #collection_field_name.iter(),
                if let #elem_category::#elem_constructor(#(#field_patterns),*) = #elem_var,
                #(#let_clauses),*;
        });
    }
    
    rules
}

/// Generate the join-based rewrite rule
pub fn generate_join_rewrite(
    rule_idx: usize,
    spec: &ProjectionSpec,
    rule: &RewriteRule,
    theory: &TheoryDef,
) -> TokenStream {
    let _parent_cat_lower = format_ident!("{}", spec.parent_category.to_string().to_lowercase());
    let rw_rel = format_ident!("rw_{}", spec.parent_category.to_string().to_lowercase());
    let parent_var = format_ident!("parent");
    
    // Generate join clauses for each projection
    let mut join_clauses = Vec::new();
    
    // Variables for join keys (use lowercase for Rust naming conventions)
    let join_key_vars: Vec<_> = spec.shared_variables.iter()
        .map(|v| format_ident!("{}", v.to_lowercase()))
        .collect();
    
    // Track all bindings for RHS generation
    let mut bindings: HashMap<String, TokenStream> = HashMap::new();
    
    // Generate join clause for each element pattern
    for elem_pattern in &spec.element_patterns {
        let rel_name = format_ident!(
            "{}_proj_r{}_p{}",
            elem_pattern.constructor.to_string().to_lowercase(),
            rule_idx,
            elem_pattern.pattern_idx
        );
        
        let mut join_args = vec![parent_var.clone()];
        
        // Add join key variables (shared across all patterns)
        join_args.extend(join_key_vars.iter().cloned());
        
        // Add variables for non-join captures
        for capture in &elem_pattern.captures {
            if !spec.shared_variables.contains(&capture.var_name) {
                let var_ident = format_ident!("{}", capture.var_name.to_lowercase());
                join_args.push(var_ident.clone());
                // Clone to ensure owned values
                bindings.insert(capture.var_name.clone(), quote! { #var_ident.clone() });
            }
        }
        
        // Add element variable
        let elem_var = format_ident!("elem_{}", elem_pattern.pattern_idx);
        join_args.push(elem_var);
        
        join_clauses.push(quote! {
            #rel_name(#(#join_args),*)
        });
    }
    
    // Add join key variables to bindings
    for (idx, shared_var) in spec.shared_variables.iter().enumerate() {
        let var_ident = &join_key_vars[idx];
        // Clone to ensure owned values
        bindings.insert(shared_var.clone(), quote! { #var_ident.clone() });
    }
    
    // Generate rest construction if needed
    let rest_construction = if let Some(rest_var) = &spec.rest_variable {
        let rest_ident = format_ident!("{}", rest_var);
        let parent_constructor = &spec.parent_constructor;
        let parent_category = &spec.parent_category;
        let collection_field = format_ident!("bag");
        
        let elem_vars: Vec<_> = (0..spec.element_patterns.len())
            .map(|i| format_ident!("elem_{}", i))
            .collect();
        
        bindings.insert(rest_var.clone(), quote! { #rest_ident });
        
        quote! {
            if let #parent_category::#parent_constructor(ref #collection_field) = #parent_var,
            let #rest_ident = {
                let mut b = #collection_field.clone();
                #(b.remove(&#elem_vars);)*
                b
            },
        }
    } else {
        quote! {}
    };
    
    // Generate RHS construction
    let rhs_code = generate_ascent_rhs(&rule.right, &bindings, theory);
    
    quote! {
        #rw_rel(#parent_var.clone(), result) <--
            #(#join_clauses),*,
            #rest_construction
            let result = #rhs_code;
    }
}

/// ========== END PHASE 6.2 ==========

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
    
    // Check minimum size if we have specific elements
    let min_size = elements.len();
    if min_size > 0 {
        clauses.push(quote! {
            if #bag_var.len() >= #min_size
        });
    }
    
    // Extract specific elements
    // Strategy: iterate through the bag and bind variables
    for (i, elem_pattern) in elements.iter().enumerate() {
        match elem_pattern {
            Expr::Var(var) => {
                let var_name = var.to_string();
                let elem_var = quote::format_ident!("{}_elem_{}", term_name, i);
                
                // Generate code to extract the i-th element
                if i == 0 {
                    // First element - take any one
                    clauses.push(quote! {
                        let #elem_var = #bag_var.iter().next().unwrap().0.clone()
                    });
                } else {
                    // Skip previously bound elements
                    clauses.push(quote! {
                        let #elem_var = #bag_var.iter().nth(#i).unwrap().0.clone()
                    });
                }
                
                let binding = quote! { #elem_var.clone() };
                
                // Check if this is a duplicate variable
                if duplicate_vars.contains(&var_name) {
                    if let Some(first_binding) = bindings.get(&var_name) {
                        // Duplicate occurrence - add equational check
                        let eq_rel = quote::format_ident!("eq_{}", element_category.to_string().to_lowercase());
                        equational_checks.push(quote! {
                            #eq_rel(#first_binding, #binding)
                        });
                    } else {
                        // First occurrence - bind and track
                        bindings.insert(var_name.clone(), binding);
                        variable_categories.insert(var_name, element_category.clone());
                    }
                } else {
                    // Single occurrence - just bind
                    bindings.insert(var_name, binding);
                }
            }
            Expr::Apply { .. } => {
                // Nested constructor pattern in collection
                // This requires more sophisticated matching
                // For MVP, we'll defer this
                panic!("Nested constructor patterns in collections not yet supported. Use variables only.");
            }
            Expr::CollectionPattern { .. } => {
                panic!("Nested collection patterns not supported");
            }
            Expr::Subst { .. } => {
                panic!("Substitution in collection pattern not allowed");
            }
        }
    }
    
    // Bind rest variable if present
    if let Some(rest_var) = rest {
        let rest_var_name = rest_var.to_string();
        let rest_ident = quote::format_ident!("{}_rest", term_name);
        
        if min_size > 0 {
            // Build rest by filtering out the specific elements
            let elem_vars: Vec<_> = (0..min_size)
                .map(|i| quote::format_ident!("{}_elem_{}", term_name, i))
                .collect();
            
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
                // We just need to decompose it according to the pattern
                
                // Check minimum size
                let min_size = elements.len();
                if min_size > 0 {
                    clauses.push(quote! {
                        if #field_name.len() >= #min_size
                    });
                }
                
                // Extract specific elements
                for (elem_idx, elem_pattern) in elements.iter().enumerate() {
                    match elem_pattern {
                        Expr::Var(var) => {
                            let var_name = var.to_string();
                            let elem_var = quote::format_ident!("{}_elem_{}", field_name, elem_idx);
                            
                            // Extract element from bag
                            if elem_idx == 0 {
                                clauses.push(quote! {
                                    let #elem_var = #field_name.iter().next().unwrap().0.clone()
                                });
                            } else {
                                clauses.push(quote! {
                                    let #elem_var = #field_name.iter().nth(#elem_idx).unwrap().0.clone()
                                });
                            }
                            
                            let binding = quote! { #elem_var.clone() };
                            
                            // Check for duplicate variables
                            if duplicate_vars.contains(&var_name) {
                                if let Some(first_binding) = bindings.get(&var_name) {
                                    let eq_rel = quote::format_ident!("eq_{}", field_category.to_string().to_lowercase());
                                    equational_checks.push(quote! {
                                        #eq_rel(#first_binding, #binding)
                                    });
                                } else {
                                    bindings.insert(var_name.clone(), binding);
                                    variable_categories.insert(var_name, field_category.clone());
                                }
                            } else {
                                bindings.insert(var_name, binding);
                            }
                        }
                        Expr::Apply { .. } => {
                            // Nested constructor pattern in collection
                            // Extract the element and recursively match it
                            let elem_var = quote::format_ident!("{}_elem_{}", field_name, elem_idx);
                            
                            // Extract element from bag
                            if elem_idx == 0 {
                                clauses.push(quote! {
                                    let #elem_var = #field_name.iter().next().unwrap().0.clone()
                                });
                            } else {
                                clauses.push(quote! {
                                    let #elem_var = #field_name.iter().nth(#elem_idx).unwrap().0.clone()
                                });
                            }
                            
                            // Create a reference to avoid moving the value
                            // We need to keep elem_var intact so we can use it in bag.remove()
                            let elem_ref = quote::format_ident!("{}_ref", elem_var);
                            clauses.push(quote! {
                                let #elem_ref = &#elem_var
                            });
                            
                            // Now recursively generate pattern matching for this element
                            generate_ascent_pattern(
                                elem_pattern,
                                &elem_ref,
                                &field_category,
                                theory,
                                bindings,
                                variable_categories,
                                clauses,
                                duplicate_vars,
                                equational_checks,
                            );
                        }
                        _ => {
                            panic!("Unsupported pattern type in collection: {:?}", elem_pattern);
                        }
                    }
                }
                
                // Bind rest variable if present
                if let Some(rest_var) = rest {
                    let rest_var_name = rest_var.to_string();
                    let rest_ident = quote::format_ident!("{}_rest", field_name);
                    
                    if min_size > 0 {
                        // Build rest by filtering out the specific elements
                        let elem_vars: Vec<_> = (0..min_size)
                            .map(|i| quote::format_ident!("{}_elem_{}", field_name, i))
                            .collect();
                        
                        // Create the rest by building a new HashBag with removed elements
                        // We do this by cloning and then calling remove for each element
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
fn generate_ascent_rhs_inner(expr: &Expr, bindings: &HashMap<String, TokenStream>, theory: &TheoryDef) -> TokenStream {
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
