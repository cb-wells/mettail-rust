/// Congruence-driven projection analysis and generation
/// This module contains the AST analysis and code generation for the new congruence-driven approach

use crate::ast::{TheoryDef, RewriteRule, Expr, GrammarItem, GrammarRule};
use syn::Ident;
use std::collections::{HashSet, HashMap};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

/// Information about a collection congruence rule
#[derive(Debug, Clone)]
pub struct CollectionCongruenceInfo {
    pub constructor: Ident,           // PPar
    pub parent_category: Ident,       // Proc
    pub element_category: Ident,      // Proc (for PPar)
    pub source_var: Ident,            // S in "if S => T"
    pub target_var: Ident,            // T in "if S => T"
    pub rest_var: Option<Ident>,      // rest in "{S, ...rest}"
}

/// Information about a regular (non-collection) congruence rule
#[derive(Debug, Clone)]
pub struct RegularCongruencePattern {
    pub constructor: Ident,           // PNew
    pub category: Ident,              // Proc
    pub rewrite_field_idx: usize,     // Which field rewrites (the one matching source_var)
    pub is_binding: bool,             // Whether this is a binding constructor
    pub source_var: Ident,            // S in "if S => T"
    pub target_var: Ident,            // T in "if S => T"
}

/// Extract collection congruence information from a congruence rule
/// Returns Some if this is a collection congruence (has CollectionPattern in LHS)
pub fn extract_collection_congruence_info(
    lhs: &Expr,
    source_var: &Ident,
    target_var: &Ident,
    theory: &TheoryDef,
) -> Option<CollectionCongruenceInfo> {
    // Parse: (Constructor {source_var, ...rest})
    if let Expr::Apply { constructor, args } = lhs {
        for arg in args {
            if let Expr::CollectionPattern { elements, rest, .. } = arg {
                // Check if elements contains just source_var
                if elements.len() == 1 {
                    if let Expr::Var(v) = &elements[0] {
                        if v == source_var {
                            // Found it! Get category info
                            let parent_category = theory.terms.iter()
                                .find(|r| r.label == *constructor)
                                .map(|r| r.category.clone())?;
                            
                            // Get element category from the collection field
                            let element_category = get_collection_element_category(constructor, theory)?;
                            
                            return Some(CollectionCongruenceInfo {
                                constructor: constructor.clone(),
                                parent_category,
                                element_category,
                                source_var: source_var.clone(),
                                target_var: target_var.clone(),
                                rest_var: rest.as_ref().cloned(),
                            });
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract regular congruence pattern from a congruence rule
/// Returns Some if this is a regular (non-collection) congruence
pub fn extract_regular_congruence_pattern(
    rule: &RewriteRule,
    theory: &TheoryDef,
) -> Option<RegularCongruencePattern> {
    // Check that this has a premise but no collection pattern
    let (source_var, target_var) = rule.premise.as_ref()?;
    
    // LHS must not contain collection pattern
    if contains_collection_pattern(&rule.left) {
        return None;
    }
    
    // LHS must be Apply
    if let Expr::Apply { constructor, args } = &rule.left {
        let category = theory.terms.iter()
            .find(|r| r.label == *constructor)
            .map(|r| r.category.clone())?;
        
        // Find which field contains source_var
        let mut rewrite_field_idx = None;
        for (idx, arg) in args.iter().enumerate() {
            if let Expr::Var(v) = arg {
                if v == source_var {
                    rewrite_field_idx = Some(idx);
                    break;
                }
            }
        }
        
        let rewrite_field_idx = rewrite_field_idx?;
        
        // Check if this is a binding constructor
        let grammar_rule = theory.terms.iter()
            .find(|r| r.label == *constructor)?;
        let is_binding = !grammar_rule.bindings.is_empty();
        
        return Some(RegularCongruencePattern {
            constructor: constructor.clone(),
            category,
            rewrite_field_idx,
            is_binding,
            source_var: source_var.clone(),
            target_var: target_var.clone(),
        });
    }
    
    None
}

/// Find all base rewrites that involve a category
/// This includes:
/// - Root constructor of that category: (PDrop ...)
/// - Collection containing that category: (PPar {(PInput ...), ...})
pub fn find_base_rewrites_for_category<'a>(
    category: &Ident,
    theory: &'a TheoryDef,
) -> Vec<&'a RewriteRule> {
    theory.rewrites.iter()
        .filter(|rule| {
            // Base rewrites only (no premise)
            rule.premise.is_none() &&
            // Rule involves the target category (either as root or in collection)
            rule_involves_category(&rule.left, category, theory)
        })
        .collect()
}

/// Find all regular (non-collection) congruence rules for a category
pub fn find_regular_congruences_for_category<'a>(
    category: &Ident,
    theory: &'a TheoryDef,
) -> Vec<&'a RewriteRule> {
    theory.rewrites.iter()
        .filter(|rule| {
            // Has premise (is a congruence)
            if rule.premise.is_none() {
                return false;
            }
            
            // Is NOT a collection congruence
            if is_collection_congruence(rule, theory) {
                return false;
            }
            
            // Operates on the target category
            extract_category_from_expr(&rule.left, theory)
                .map(|cat| cat == *category)
                .unwrap_or(false)
        })
        .collect()
}

/// Check if a rewrite rule is a collection congruence
pub fn is_collection_congruence(rule: &RewriteRule, _theory: &TheoryDef) -> bool {
    // Check if LHS contains CollectionPattern
    contains_collection_pattern(&rule.left)
}

/// Check if an expression contains a collection pattern
pub fn contains_collection_pattern(expr: &Expr) -> bool {
    match expr {
        Expr::CollectionPattern { .. } => true,
        Expr::Apply { args, .. } => args.iter().any(contains_collection_pattern),
        Expr::Subst { term, replacement, .. } => {
            contains_collection_pattern(term) || contains_collection_pattern(replacement)
        }
        Expr::Var(_) => false,
    }
}

/// Check if a rewrite LHS involves a category
/// This includes:
/// - Root constructor of that category: (PDrop ...)
/// - Collection containing that category: (PPar {(PInput ...), ...})
fn rule_involves_category(expr: &Expr, category: &Ident, theory: &TheoryDef) -> bool {
    // Check root category
    if let Some(root_cat) = extract_category_from_expr(expr, theory) {
        if root_cat == *category {
            return true;
        }
    }
    
    // Check collection element categories
    if let Expr::Apply { constructor, args } = expr {
        for arg in args {
            if let Expr::CollectionPattern { .. } = arg {
                // Get collection element type from theory
                if let Some(elem_cat) = get_collection_element_category(constructor, theory) {
                    if elem_cat == *category {
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

/// Get the element category of a collection constructor
fn get_collection_element_category(constructor: &Ident, theory: &TheoryDef) -> Option<Ident> {
    let grammar_rule = theory.terms.iter()
        .find(|r| r.label == *constructor)?;
    
    for item in &grammar_rule.items {
        if let GrammarItem::Collection { element_type, .. } = item {
            return Some(element_type.clone());
        }
    }
    
    None
}

/// Extract the category from an expression by looking up the constructor in the theory
fn extract_category_from_expr(expr: &Expr, theory: &TheoryDef) -> Option<Ident> {
    match expr {
        Expr::Apply { constructor, .. } => {
            // Look up this constructor in the theory to find its category
            for rule in &theory.terms {
                if rule.label == *constructor {
                    return Some(rule.category.clone());
                }
            }
            None
        }
        Expr::Var(_) => None,
        Expr::Subst { term, .. } => extract_category_from_expr(term, theory),
        Expr::CollectionPattern { constructor, .. } => {
            // If constructor is specified, look it up
            if let Some(cons) = constructor {
                for rule in &theory.terms {
                    if rule.label == *cons {
                        return Some(rule.category.clone());
                    }
                }
            }
            None
        }
    }
}

/// Find all collection congruence element categories
/// Returns the set of categories that are subjects of collection congruences
pub fn find_collection_congruence_element_categories(theory: &TheoryDef) -> HashSet<Ident> {
    let mut categories = HashSet::new();
    
    for rule in &theory.rewrites {
        if let Some((source_var, target_var)) = &rule.premise {
            if let Some(info) = extract_collection_congruence_info(&rule.left, source_var, target_var, theory) {
                categories.insert(info.element_category);
            }
        }
    }
    
    categories
}

//=============================================================================
// CODE GENERATION: Projection Relations
//=============================================================================

/// Information about an element pattern extracted from a base rewrite
#[derive(Debug, Clone)]
pub struct ElementPatternInfo {
    pub constructor: Ident,      // PInput, POutput, PDrop, PAmb
    pub category: Ident,          // Proc
    pub captures: Vec<CaptureInfo>,
    pub is_nested: bool,         // True if pattern has nested structure (like PDrop with NQuote inside)
    pub expr: Option<Expr>,      // The full element expression for nested pattern matching
}

/// Information about a captured variable
#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub var_name: String,
    pub category: Ident,
    pub field_idx: usize,
    pub is_binder: bool,
}

/// Generate all projection relations for a collection congruence
/// This is the main entry point for congruence-driven projection generation
/// Returns (projection_decls, base_patterns) where base_patterns[base_idx][pat_idx] are the updated patterns
pub fn generate_congruence_projections(
    cong_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    theory: &TheoryDef,
) -> (Vec<TokenStream>, Vec<Vec<ElementPatternInfo>>) {
    let mut projections = Vec::new();
    
    // Find all base rewrites that involve this element category
    let base_rewrites = find_base_rewrites_for_category(&cong_info.element_category, theory);
    
    // Find all regular congruences on this element category
    let regular_congruences = find_regular_congruences_for_category(&cong_info.element_category, theory);
    
    // Generate projections for base rewrites
    let mut updated_base_patterns: Vec<Vec<ElementPatternInfo>> = Vec::new();
    
    for (base_idx, base_rule) in base_rewrites.iter().enumerate() {
        // Extract element patterns from the base rewrite's LHS
        let element_patterns = extract_element_patterns_from_base_rewrite(
            &base_rule.left,
            cong_info,
            theory
        );
        
        let mut updated_patterns_for_base = Vec::new();
        
        // Generate projection for each element pattern
        // Pass the base_rule so we can access the full LHS for nested pattern matching
        for (pat_idx, pattern) in element_patterns.iter().enumerate() {
            let (proj, updated_pattern) = generate_base_rewrite_projection(
                cong_idx,
                base_idx,
                pat_idx,
                cong_info,
                pattern,
                &base_rule.left,  // Pass the full LHS for nested pattern extraction
                theory,
            );
            projections.extend(proj);
            updated_patterns_for_base.push(updated_pattern);
        }
        updated_base_patterns.push(updated_patterns_for_base);
    }
    
    // Generate projections for regular congruences
    for (reg_idx, reg_cong) in regular_congruences.iter().enumerate() {
        if let Some(pattern) = extract_regular_congruence_pattern(reg_cong, theory) {
            let proj = generate_regular_congruence_projection(
                cong_idx,
                reg_idx,
                cong_info,
                &pattern,
                theory,
            );
            projections.extend(proj);
        }
    }
    
    (projections, updated_base_patterns)
}

/// Extract element patterns from a base rewrite LHS
/// For `(PDrop (NQuote P)) => P`: returns [(PDrop, ...)]
/// For `(PPar {(PInput ...), (POutput ...)}) => ...`: returns [(PInput, ...), (POutput, ...)]
pub fn extract_element_patterns_from_base_rewrite(
    lhs: &Expr,
    cong_info: &CollectionCongruenceInfo,
    theory: &TheoryDef,
) -> Vec<ElementPatternInfo> {
    let mut patterns = Vec::new();
    
    // Only extract patterns from collection LHS that match the congruence's collection constructor
    // Case 1: Direct constructor patterns (e.g., (PDrop ...) for PDrop base rewrite)
    // Case 2: Collection with element patterns (e.g., (PPar {(PInput ...), (POutput ...)})
    
    if let Expr::Apply { constructor, args } = lhs {
        if *constructor == cong_info.constructor {
            // This is the collection constructor we're interested in
            // Extract element patterns from inside the collection
            for arg in args {
                if let Expr::CollectionPattern { elements, .. } = arg {
                    // Extract each element pattern
                    for elem in elements {
                        if let Some(mut pattern_info) = analyze_constructor_pattern(elem, theory) {
                            // Store the element expression for nested pattern matching
                            if pattern_info.is_nested {
                                pattern_info.expr = Some(elem.clone());
                            }
                            patterns.push(pattern_info);
                        }
                    }
                }
            }
        } else {
            // This is a direct pattern (not a collection)
            // Check if it's of the element category
            if let Some(root_cat) = extract_category_from_expr(lhs, theory) {
                if root_cat == cong_info.element_category {
                    // This is a direct pattern - the congruence should lift it into collections
                    // Extract captures from the pattern for the congruence clause
                    let var_categories = extract_variable_categories(lhs, theory);
                    let captures: Vec<CaptureInfo> = var_categories.iter().map(|(var_name, cat)| {
                        CaptureInfo {
                            var_name: var_name.clone(),
                            category: cat.clone(),
                            field_idx: 0,  // Not used for nested patterns
                            is_binder: false,  // Simplified for now
                        }
                    }).collect();
                    
                    patterns.push(ElementPatternInfo {
                        constructor: constructor.clone(),
                        category: cong_info.element_category.clone(),
                        captures,
                        is_nested: true,  // Direct patterns use full pattern matching
                        expr: Some(lhs.clone()),  // Store full LHS for pattern matching
                    });
                }
            }
        }
    }
    
    patterns
}

/// Analyze a constructor pattern to extract captures
fn analyze_constructor_pattern(
    expr: &Expr,
    theory: &TheoryDef,
) -> Option<ElementPatternInfo> {
    if let Expr::Apply { constructor, args } = expr {
        let grammar_rule = theory.terms.iter()
            .find(|r| r.label == *constructor)?;
        
        let category = grammar_rule.category.clone();
        
        // Pattern is nested if it has nested Apply nodes in args (not just Vars)
        let is_nested = args.iter().any(|arg| matches!(arg, Expr::Apply { .. }));
        
        // For nested patterns, we'll use full pattern matching via generate_ascent_pattern
        // Don't extract captures here - they'll be extracted during projection generation
        let captures = if is_nested {
            Vec::new()  // Will be populated during projection generation
        } else {
            // For flat patterns, extract captures normally
            extract_captures(args, grammar_rule, theory)
        };
        
        return Some(ElementPatternInfo {
            constructor: constructor.clone(),
            category,
            captures,
            is_nested,
            expr: None,  // Will be set by caller if needed
        });
    }
    None
}

/// Extract captures from constructor arguments
fn extract_captures(
    args: &[Expr],
    grammar_rule: &GrammarRule,
    _theory: &TheoryDef,
) -> Vec<CaptureInfo> {
    let mut captures = Vec::new();
    
    // Get non-terminal positions in grammar
    let non_term_positions: Vec<(usize, &GrammarItem)> = grammar_rule.items.iter()
        .enumerate()
        .filter(|(_, item)| matches!(item, 
            GrammarItem::NonTerminal(_) | 
            GrammarItem::Binder { .. } |
            GrammarItem::Collection { .. }))
        .collect();
    
    // Build a map of which indices are bound together
    // For bindings like (binder_idx, [body_idx]), both should map to the same field
    let mut bound_indices: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (binder_idx, body_indices) in &grammar_rule.bindings {
        for body_idx in body_indices {
            bound_indices.insert(*body_idx, *binder_idx);
        }
    }
    
    // Map grammar indices to field indices, accounting for bindings
    let mut field_idx = 0;
    let mut grammar_to_field = vec![0; grammar_rule.items.len()];
    let mut processed_as_binding = std::collections::HashSet::new();
    
    for (idx, item) in grammar_rule.items.iter().enumerate() {
        if matches!(item, GrammarItem::NonTerminal(_) | GrammarItem::Binder { .. } | GrammarItem::Collection { .. }) {
            // Check if this index is a body that's bound to a binder
            if let Some(&binder_idx) = bound_indices.get(&idx) {
                // This is a body bound to a binder - use the binder's field index
                if let Some(&binder_field) = grammar_to_field.get(binder_idx) {
                    grammar_to_field[idx] = binder_field;
                }
                processed_as_binding.insert(idx);
                continue;
            }
            
            // Check if this is a binder
            let is_binder = grammar_rule.bindings.iter().any(|(b_idx, _)| *b_idx == idx);
            if is_binder {
                // This is a binder - assign it a field and mark that its bodies will share this field
                grammar_to_field[idx] = field_idx;
                field_idx += 1;
                processed_as_binding.insert(idx);
                continue;
            }
            
            // Regular non-terminal
            grammar_to_field[idx] = field_idx;
            field_idx += 1;
        }
    }
    
    // Process each argument
    for (arg_idx, arg) in args.iter().enumerate() {
        if arg_idx >= non_term_positions.len() {
            continue;
        }
        
        let (grammar_idx, grammar_item) = non_term_positions[arg_idx];
        let field_idx = grammar_to_field[grammar_idx];
        
        let category = match grammar_item {
            GrammarItem::NonTerminal(cat) => cat.clone(),
            GrammarItem::Binder { category: cat } => cat.clone(),
            GrammarItem::Collection { element_type, .. } => element_type.clone(),
            _ => continue,
        };
        
        let is_binder = grammar_rule.bindings.iter()
            .any(|(binder_idx, _)| *binder_idx == grammar_idx);
        
        // Extract all variables from this argument (handles nested patterns)
        extract_vars_from_expr(arg, &category, field_idx, is_binder, &mut captures);
    }
    
    captures
}

/// Recursively extract all variables from an expression
/// This handles nested patterns like (NQuote P) within (PDrop (NQuote P))
fn extract_vars_from_expr(
    expr: &Expr,
    category: &Ident,
    field_idx: usize,
    is_binder: bool,
    captures: &mut Vec<CaptureInfo>,
) {
    match expr {
        Expr::Var(var) => {
            captures.push(CaptureInfo {
                var_name: var.to_string(),
                category: category.clone(),
                field_idx,
                is_binder,
            });
        }
        Expr::Apply { args, .. } => {
            // Recurse into nested constructors
            // All nested variables share the same field_idx (they're accessed via dereferencing)
            for arg in args {
                extract_vars_from_expr(arg, category, field_idx, is_binder, captures);
            }
        }
        Expr::Subst { term, .. } => {
            // Recurse into substitution term
            extract_vars_from_expr(term, category, field_idx, is_binder, captures);
        }
        Expr::CollectionPattern { elements, .. } => {
            // Recurse into collection elements
            for elem in elements {
                extract_vars_from_expr(elem, category, field_idx, is_binder, captures);
            }
        }
    }
}

/// Extract all variables and their categories from an expression
fn extract_variable_categories(expr: &Expr, theory: &TheoryDef) -> HashMap<String, Ident> {
    let mut categories = HashMap::new();
    extract_variable_categories_recursive(expr, theory, &mut categories);
    categories
}

/// Recursively extract variable categories from an expression
fn extract_variable_categories_recursive(
    expr: &Expr,
    theory: &TheoryDef,
    categories: &mut HashMap<String, Ident>,
) {
    match expr {
        Expr::Var(var_name) => {
            // We don't know the category from just the variable, need context
            // This will be filled in from the grammar context
        }
        Expr::Apply { constructor, args } => {
            // Look up the constructor in the grammar to find field categories
            if let Some(grammar_rule) = theory.terms.iter().find(|r| r.label == *constructor) {
                let mut non_term_idx = 0;
                for (item_idx, item) in grammar_rule.items.iter().enumerate() {
                    match item {
                        GrammarItem::NonTerminal(cat) => {
                            if non_term_idx < args.len() {
                                if let Expr::Var(var_name) = &args[non_term_idx] {
                                    categories.insert(var_name.to_string(), cat.clone());
                                } else {
                                    // Recurse into nested expressions
                                    extract_variable_categories_recursive(&args[non_term_idx], theory, categories);
                                }
                            }
                            non_term_idx += 1;
                        }
                        GrammarItem::Binder { category } => {
                            if non_term_idx < args.len() {
                                if let Expr::Var(var_name) = &args[non_term_idx] {
                                    categories.insert(var_name.to_string(), category.clone());
                                }
                            }
                            non_term_idx += 1;
                            
                            // The body is the next non-terminal (handled in bindings)
                            if let Some(&body_idx) = grammar_rule.bindings.iter()
                                .find(|(binder_idx, _)| *binder_idx == item_idx)
                                .and_then(|(_, bodies)| bodies.first())
                            {
                                if let Some(GrammarItem::NonTerminal(body_cat)) = grammar_rule.items.get(body_idx) {
                                    if non_term_idx < args.len() {
                                        if let Expr::Var(var_name) = &args[non_term_idx] {
                                            categories.insert(var_name.to_string(), body_cat.clone());
                                        } else {
                                            extract_variable_categories_recursive(&args[non_term_idx], theory, categories);
                                        }
                                    }
                                    non_term_idx += 1;
                                }
                            }
                        }
                        GrammarItem::Collection { element_type, .. } => {
                            if non_term_idx < args.len() {
                                if let Expr::CollectionPattern { elements, .. } = &args[non_term_idx] {
                                    for elem in elements {
                                        // Bare variables in collections should have the element type
                                        if let Expr::Var(var_name) = elem {
                                            categories.insert(var_name.to_string(), element_type.clone());
                                        } else {
                                            extract_variable_categories_recursive(elem, theory, categories);
                                        }
                                    }
                                }
                            }
                            non_term_idx += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Expr::Subst { term, .. } => {
            extract_variable_categories_recursive(term, theory, categories);
        }
        Expr::CollectionPattern { elements, .. } => {
            for elem in elements {
                extract_variable_categories_recursive(elem, theory, categories);
            }
        }
    }
}

/// Generate projection relation and population rule for a base rewrite pattern  
/// Returns (generated code, updated pattern with captures)
fn generate_base_rewrite_projection(
    cong_idx: usize,
    base_idx: usize,
    pat_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    pattern: &ElementPatternInfo,
    base_lhs: &Expr,  // Full LHS of the base rewrite for nested pattern matching
    theory: &TheoryDef,
) -> (Vec<TokenStream>, ElementPatternInfo) {
    use std::collections::HashMap;
    use crate::rewrite_gen;
    
    let mut result = Vec::new();
    
    // Generate relation name: constructor_proj_c{cong}_b{base}_p{pattern}
    let rel_name = format_ident!(
        "{}_proj_c{}_b{}_p{}",
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        base_idx,
        pat_idx
    );
    
    let parent_cat = &cong_info.parent_category;
    let parent_cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
    let elem_cat = &cong_info.element_category;
    let collection_constructor = &cong_info.constructor;
    
    // For nested patterns, generate full pattern matching using existing logic
    if pattern.is_nested {
        // Get the expression to match - either from the pattern itself or from base_lhs
        let pattern_expr = pattern.expr.as_ref().unwrap_or(base_lhs);
        
        // Use the rewrite_gen pattern matching to extract variables from nested structure
        let mut bindings: HashMap<String, TokenStream> = HashMap::new();
        let mut variable_categories: HashMap<String, Ident> = HashMap::new();
        let mut clauses = Vec::new();
        let mut equational_checks = Vec::new();
        let duplicate_vars = std::collections::HashSet::new();  // No duplicates in single pattern
        
        // Generate pattern matching clauses for the element expression
        let elem_ident = format_ident!("elem");
        rewrite_gen::generate_ascent_pattern(
            pattern_expr,
            &elem_ident,
            elem_cat,
            theory,
            &mut bindings,
            &mut variable_categories,
            &mut clauses,
            &duplicate_vars,
            &mut equational_checks,
        );
        
        // Build relation signature from bindings
        let mut field_types = vec![quote! { #parent_cat }];
        let mut rel_fields = vec![quote! { parent.clone() }];
        let mut binding_vars = Vec::new();
        
        // We need to infer categories from the pattern expression
        // Extract variables and their categories from the expression
        let var_categories = extract_variable_categories(pattern_expr, theory);
        
        // Also build a map for RHS reconstruction: original var name -> lowercase ident
        let mut rhs_bindings = HashMap::new();
        
        for (var_name, binding_ts) in &bindings {
            let var_ident = format_ident!("{}", var_name.to_lowercase());
            // Try to get category from variable_categories (for duplicates) or infer from var_categories
            let cat = variable_categories.get(var_name)
                .or_else(|| var_categories.get(var_name))
                .expect(&format!("Variable category not found for '{}'", var_name));
            field_types.push(quote! { #cat });
            rel_fields.push(quote! { #var_ident.clone() });
            binding_vars.push((var_ident.clone(), binding_ts.clone()));
            
            // For RHS reconstruction, map original name to the lowercase ident
            rhs_bindings.insert(var_name.clone(), quote! { #var_ident.clone() });
        }
        field_types.push(quote! { #elem_cat });
        rel_fields.push(quote! { elem.clone() });
        
        let rel_decl = quote! {
            relation #rel_name(#(#field_types),*);
        };
        
        // Generate let bindings for captured variables
        let capture_bindings: Vec<_> = binding_vars.iter().map(|(var_ident, binding)| {
            quote! { let #var_ident = #binding }
        }).collect();
        
        let population_rule = quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                #(#clauses),*,
                #(#capture_bindings),*;
        };
        
        // Build updated pattern with extracted captures
        // Use the order from bindings to ensure consistent ordering with projection signature
        let updated_captures: Vec<CaptureInfo> = bindings.keys().map(|var_name| {
            let cat = variable_categories.get(var_name)
                .or_else(|| var_categories.get(var_name))
                .expect(&format!("Variable category not found for '{}'", var_name));
            CaptureInfo {
                var_name: var_name.clone(),
                category: cat.clone(),
                field_idx: 0,  // Not used for nested patterns
                is_binder: false,  // TODO: detect binders properly
            }
        }).collect();
        
        let mut updated_pattern = pattern.clone();
        updated_pattern.captures = updated_captures;
        
        result.push(rel_decl);
        result.push(population_rule);
        return (result, updated_pattern);
    }
    
    // Build relation signature: (Parent, Capture1, Capture2, ..., Element)
    let mut field_types = vec![quote! { #parent_cat }];
    for capture in &pattern.captures {
        let cat = &capture.category;
        if capture.is_binder {
            field_types.push(quote! { mettail_runtime::Binder<String> });
        } else {
            field_types.push(quote! { #cat });
        }
    }
    field_types.push(quote! { #elem_cat });
    
    // Generate relation declaration
    let rel_decl = quote! {
        relation #rel_name(#(#field_types),*);
    };
    
    // Generate population rule
    let parent_cat_lower = format_ident!("{}", cong_info.parent_category.to_string().to_lowercase());
    let collection_constructor = &cong_info.constructor;
    let parent_cat = &cong_info.parent_category;
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;
    
    // Generate field pattern matching and capture extraction
    let (field_patterns, capture_bindings) = generate_field_extraction(pattern);
    
    // Generate relation tuple
    let mut rel_fields = vec![quote! { parent.clone() }];
    for capture in &pattern.captures {
        let cap_name = format_ident!("cap_{}", capture.var_name.to_lowercase());
        rel_fields.push(quote! { #cap_name.clone() });
    }
    rel_fields.push(quote! { elem.clone() });
    
    // Generate the population rule with conditional capture bindings
    let population_rule = if pattern.captures.is_empty() {
        // No captures - simpler pattern without bindings
        quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                if let #elem_cat::#elem_constructor(#field_patterns) = elem;
        }
    } else {
        // Has captures - include capture bindings
        quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                if let #elem_cat::#elem_constructor(#field_patterns) = elem,
                #capture_bindings;
        }
    };
    
    result.push(rel_decl);
    result.push(population_rule);
    (result, pattern.clone())
}

/// Generate field patterns and capture extraction for an element pattern
fn generate_field_extraction(pattern: &ElementPatternInfo) -> (TokenStream, TokenStream) {
    let mut field_patterns = Vec::new();
    let mut capture_bindings = Vec::new();
    
    // Group captures by field index
    let mut field_to_captures: std::collections::HashMap<usize, Vec<&CaptureInfo>> = 
        std::collections::HashMap::new();
    
    for capture in &pattern.captures {
        field_to_captures.entry(capture.field_idx)
            .or_insert_with(Vec::new)
            .push(capture);
    }
    
    // Get all unique field indices and sort
    let mut field_indices: Vec<usize> = field_to_captures.keys().copied().collect();
    field_indices.sort();
    
    // Generate patterns and bindings for each field
    for (pattern_idx, &field_idx) in field_indices.iter().enumerate() {
        let field_name = format_ident!("f{}", pattern_idx);
        field_patterns.push(quote! { ref #field_name });
        
        let captures_for_field = &field_to_captures[&field_idx];
        
        // Check if this is a binder field (has a binder capture)
        if let Some(binder_capture) = captures_for_field.iter().find(|c| c.is_binder) {
            // This is a scope field - unbind it to get binder and body
            let binder_name = format_ident!("cap_{}", binder_capture.var_name.to_lowercase());
            
            // Find the body capture (should be at the same field index, non-binder)
            if let Some(body_capture) = captures_for_field.iter().find(|c| !c.is_binder) {
                let body_name = format_ident!("cap_{}", body_capture.var_name.to_lowercase());
                capture_bindings.push(quote! {
                    let (#binder_name, body_box) = (* #field_name).clone().unbind(),
                    let #body_name = (* body_box).clone()
                });
            } else {
                // Only binder, no body capture
                capture_bindings.push(quote! {
                    let (#binder_name, _body) = (* #field_name).clone().unbind()
                });
            }
        } else {
            // Regular field(s) - just dereference
            for capture in captures_for_field {
                let cap_name = format_ident!("cap_{}", capture.var_name.to_lowercase());
                capture_bindings.push(quote! {
                    let #cap_name = (** #field_name).clone()
                });
            }
        }
    }
    
    let field_pattern = quote! { #(#field_patterns),* };
    let bindings = quote! { #(#capture_bindings),* };
    
    (field_pattern, bindings)
}

/// Generate projection for a regular congruence constructor
/// e.g., for "if S => T then (PNew x S) => (PNew x T)", generate:
/// pnew_proj_c{cong}_r{reg}(parent, x, body, elem)
fn generate_regular_congruence_projection(
    cong_idx: usize,
    reg_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    pattern: &RegularCongruencePattern,
    theory: &TheoryDef,
) -> Vec<TokenStream> {
    let mut result = Vec::new();
    
    let rel_name = format_ident!(
        "{}_proj_c{}_r{}",
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        reg_idx
    );
    
    let parent_cat = &cong_info.parent_category;
    let parent_cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
    let collection_constructor = &cong_info.constructor;
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;
    
    // Build relation signature based on whether it's a binding constructor
    let mut field_types = vec![quote! { #parent_cat }];
    
    let grammar_rule = theory.terms.iter()
        .find(|r| r.label == pattern.constructor)
        .expect("Constructor not found");
    
    // Add binder field if this is a binding constructor
    if pattern.is_binding {
        field_types.push(quote! { mettail_runtime::Binder<String> });
    }
    
    // Add the rewrite field (the body that can be rewritten)
    field_types.push(quote! { #elem_cat });
    
    // Add the original element
    field_types.push(quote! { #elem_cat });
    
    let rel_decl = quote! {
        relation #rel_name(#(#field_types),*);
    };
    
    // Generate extraction based on whether it's a binding constructor
    let extraction = if pattern.is_binding {
        // Binding constructor: extract scope and unbind
        quote! {
            if let #elem_cat::#elem_constructor(ref scope) = elem,
            let (binder_var, body_box) = scope.clone().unbind(),
            let rewrite_field = (*body_box).clone()
        }
    } else {
        // Non-binding constructor: directly extract the rewrite field
        let field_idx = pattern.rewrite_field_idx;
        // Generate field pattern with the rewrite field
        let mut field_pats = Vec::new();
        for i in 0..grammar_rule.items.iter().filter(|item| !matches!(item, GrammarItem::Terminal(_))).count() {
            if i == field_idx {
                field_pats.push(quote! { ref rewrite_field_box });
            } else {
                let other_field = format_ident!("_field{}", i);
                field_pats.push(quote! { #other_field });
            }
        }
        
        quote! {
            if let #elem_cat::#elem_constructor(#(#field_pats),*) = elem,
            let rewrite_field = (**rewrite_field_box).clone()
        }
    };
    
    let rel_fields = if pattern.is_binding {
        vec![
            quote! { parent.clone() },
            quote! { binder_var.clone() },
            quote! { rewrite_field.clone() },
            quote! { elem.clone() }
        ]
    } else {
        vec![
            quote! { parent.clone() },
            quote! { rewrite_field.clone() },
            quote! { elem.clone() }
        ]
    };
    
    let population_rule = quote! {
        #rel_name(#(#rel_fields),*) <--
            #parent_cat_lower(parent),
            if let #parent_cat::#collection_constructor(ref bag_field) = parent,
            for (elem, _count) in bag_field.iter(),
            #extraction;
    };
    
    result.push(rel_decl);
    result.push(population_rule);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: Add unit tests for the analysis and generation functions
}

