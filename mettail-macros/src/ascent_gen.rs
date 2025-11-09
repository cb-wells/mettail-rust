#![allow(dead_code, unused_imports, unused_variables)]

use crate::ast::{TheoryDef, GrammarRule, Equation, Expr};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;
use std::collections::HashMap;
use ascent_byods_rels::eqrel;


/// Generate complete ascent_source! block for a theory
pub fn generate_ascent_source(theory: &TheoryDef) -> TokenStream {
    let theory_name = theory.name.to_string().to_lowercase();
    let source_name = format_ident!("{}_source", theory_name);

    let relations = generate_relations(theory);
    let category_rules = generate_category_rules(theory);
    let equation_rules = generate_equation_rules(theory);
    let rewrite_rules = generate_rewrite_rules(theory);

    let result = quote! {
        ::ascent::ascent_source! {
            #source_name:

            #relations

            #category_rules

            #equation_rules

            #rewrite_rules
        }
    };
    
    // Debug: print the full generated ascent source with formatting
    eprintln!("\n========== FULL GENERATED ASCENT SOURCE ==========");
    eprintln!("ascent_source! {{");
    eprintln!("    {}:\n", source_name);
    eprintln!("    // Relations");
    for line in relations.to_string().split(';') {
        if !line.trim().is_empty() {
            eprintln!("    {};", line.trim());
        }
    }
    eprintln!("\n    // Category rules");
    for line in category_rules.to_string().split(';') {
        if !line.trim().is_empty() {
            eprintln!("    {};", line.trim());
        }
    }
    eprintln!("\n    // Equation rules");
    for line in equation_rules.to_string().split(';') {
        if !line.trim().is_empty() {
            eprintln!("    {};", line.trim());
        }
    }
    eprintln!("\n    // Rewrite rules");
    for line in rewrite_rules.to_string().split(';') {
        if !line.trim().is_empty() {
            eprintln!("    {};", line.trim());
        }
    }
    eprintln!("}}");
    eprintln!("==================================================\n");
    
    result
}

/// Generate relation declarations
fn generate_relations(theory: &TheoryDef) -> TokenStream {
    let mut relations = Vec::new();
    
    // Category exploration relations (unadorned)
    for export in &theory.exports {
        let cat = &export.name;
        let cat_lower = format_ident!("{}", cat.to_string().to_lowercase());
        relations.push(quote! { 
            relation #cat_lower(#cat);
        });
    }
    
    // Equality relations (per-category, typed)
    for export in &theory.exports {
        let cat = &export.name;
        let eq_rel = format_ident!("eq_{}", cat.to_string().to_lowercase());
        relations.push(quote! { 
            #[ds(crate::eqrel)]
            relation #eq_rel(#cat, #cat);
        });
    }
    
    // Rewrite relations (per-category, typed)
    for export in &theory.exports {
        let cat = &export.name;
        let rw_rel = format_ident!("rw_{}", cat.to_string().to_lowercase());
        relations.push(quote! { 
            relation #rw_rel(#cat, #cat);
        });
    }
    
    quote! { 
        #(#relations)*
    }
}

/// Generate category exploration rules
fn generate_category_rules(theory: &TheoryDef) -> TokenStream {
    let mut rules = Vec::new();
    
    for export in &theory.exports {
        let cat = &export.name;
        let cat_lower = format_ident!("{}", cat.to_string().to_lowercase());
        let rw_rel = format_ident!("rw_{}", cat.to_string().to_lowercase());
        
        // Expand via rewrites ONLY (not via equality)
        // This prevents exponential term explosion from eq + exploration feedback loop
        rules.push(quote! {
            #cat_lower(c1) <-- #cat_lower(c0), #rw_rel(c0, c1);
        });
        
        // REMOVED: Expand via equality
        // The old rule `cat(c1) <-- cat(c0), eq_cat(c0, c1)` caused:
        // 1. Every discovered term added to eq via reflexivity
        // 2. Transitivity creates O(n²) equality facts
        // 3. Category exploration uses those to generate more terms
        // 4. Those terms get added to eq via reflexivity
        // 5. EXPONENTIAL BLOWUP
        //
        // Instead: eq relations are computed separately and used only for
        // explicit queries, not for driving exploration.
        
        // Generate deconstruction rules for this category
        let deconstruct_rules = generate_deconstruction_rules(cat, theory);
        rules.extend(deconstruct_rules);
    }
    
    quote! { 
        #(#rules)*
    }
}

/// Generate deconstruction rules for a category
fn generate_deconstruction_rules(category: &Ident, theory: &TheoryDef) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    
    // Find all constructors for this category
    let constructors: Vec<&GrammarRule> = theory.terms
        .iter()
        .filter(|r| r.category == *category)
        .collect();
    
    for constructor in constructors {
        if let Some(rule) = generate_deconstruction_for_constructor(category, constructor, theory) {
            rules.push(rule);
        }
    }
    
    rules
}

/// Generate deconstruction rule for a single constructor
fn generate_deconstruction_for_constructor(
    category: &Ident,
    constructor: &GrammarRule,
    _theory: &TheoryDef
) -> Option<TokenStream> {
    let _cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let _label = &constructor.label;
    
    // Count non-terminal fields
    let non_terminals: Vec<_> = constructor.items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if let crate::ast::GrammarItem::NonTerminal(ident) = item {
                Some((i, ident))
            } else {
                None
            }
        })
        .collect();
    
    if non_terminals.is_empty() {
        // No fields to deconstruct (e.g., PZero)
        return None;
    }
    
    // Check if this is a binding constructor
    if !constructor.bindings.is_empty() {
        // Binding constructor - need to unbind
        generate_binding_deconstruction(category, constructor)
    } else {
        // Regular constructor
        generate_regular_deconstruction(category, constructor, &non_terminals)
    }
}

/// Generate deconstruction for regular (non-binding) constructor
fn generate_regular_deconstruction(
    category: &Ident,
    constructor: &GrammarRule,
    non_terminals: &[(usize, &Ident)]
) -> Option<TokenStream> {
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let label = &constructor.label;
    
    // Generate field names
    let field_names: Vec<_> = (0..non_terminals.len())
        .map(|i| format_ident!("field_{}", i))
        .collect();
    
    // Generate subterm facts for each non-terminal field
    // Skip 'Var' fields as Var is a runtime type, not an exported category
    let subterm_facts: Vec<TokenStream> = non_terminals
        .iter()
        .zip(&field_names)
        .filter_map(|((_, field_type), field_name)| {
            // Skip Var - it's a special runtime type, not a category
            if field_type.to_string() == "Var" {
                return None;
            }
            let field_type_lower = format_ident!("{}", field_type.to_string().to_lowercase());
            // In Ascent pattern matching, fields are &Box<T>
            // Use *field.clone() which becomes (*field).clone() and auto-derefs to T
            Some(quote! { 
                #field_type_lower(*#field_name.clone())
            })
        })
        .collect();
    
    // If all fields are Var, skip this constructor entirely
    if subterm_facts.is_empty() {
        return None;
    }
    
    Some(quote! {
        #(#subterm_facts),* <--
            #cat_lower(t),
            if let #category::#label(#(#field_names),*) = t;
    })
}

/// Generate deconstruction for binding constructor
fn generate_binding_deconstruction(
    category: &Ident,
    constructor: &GrammarRule,
) -> Option<TokenStream> {
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let label = &constructor.label;
    
    // For now, handle single binder binding in single body
    let (_binder_idx, body_indices) = &constructor.bindings[0];
    let body_idx = body_indices[0];
    
    // Get the body category
    let body_cat = match &constructor.items[body_idx] {
        crate::ast::GrammarItem::NonTerminal(cat) => cat,
        _ => return None,
    };
    let body_cat_lower = format_ident!("{}", body_cat.to_string().to_lowercase());
    
    // Count fields (for pattern matching)
    let field_count = constructor.items
        .iter()
        .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
        .count();
    
    if field_count == 1 {
        // Only the scope field (body)
        Some(quote! {
            #body_cat_lower(*body.clone()) <--
                #cat_lower(t),
                if let #category::#label(scope) = t,
                let (binder, body) = scope.clone().unbind();
        })
    } else {
        // Has other fields besides the scope
        // Generate field names and collect their categories
        let mut field_names = Vec::new();
        let mut field_cats = Vec::new();
        let mut ast_field_idx = 0usize;
        
        for (i, item) in constructor.items.iter().enumerate() {
            if i == *_binder_idx {
                continue; // Skip binder
            } else if i == body_idx {
                field_names.push(format_ident!("scope_field"));
            } else if let crate::ast::GrammarItem::NonTerminal(cat) = item {
                let field_name = format!("field_{}", ast_field_idx);
                field_names.push(format_ident!("{}", field_name));
                field_cats.push((ast_field_idx, cat.clone()));
                ast_field_idx += 1;
            }
        }
        
        // Generate category facts for all non-body fields, then the body
        // Maintain grammar order: non-body fields first, then body
        let mut subterm_facts = Vec::new();
        for (idx, cat) in &field_cats {
            let cat_lower = format_ident!("{}", cat.to_string().to_lowercase());
            let field_name = format_ident!("field_{}", idx);
            subterm_facts.push(quote! { #cat_lower(*#field_name.clone()) });
        }
        subterm_facts.push(quote! { #body_cat_lower(*body.clone()) });
        
        Some(quote! {
            #(#subterm_facts),* <--
                #cat_lower(t),
                if let #category::#label(#(#field_names),*) = t,
                let (binder, body) = scope_field.clone().unbind();
        })
    }
}

/// Generate congruence rules for equality
/// For each constructor, generate: if args are equal, then constructed terms are equal
fn generate_congruence_rules(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    
    for grammar_rule in &theory.terms {
        let category = &grammar_rule.category;
        let eq_rel = format_ident!("eq_{}", category.to_string().to_lowercase());
        let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
        
        // Check if this constructor has binders
        let has_binders = !grammar_rule.bindings.is_empty();
        
        if has_binders {
            // Skip binders for now - congruence for binders is more complex
            // (requires alpha-equivalence reasoning)
            continue;
        }
        
        // Collect non-terminal arguments and their categories
        let mut args = Vec::new();
        let mut arg_categories = Vec::new();
        
        for item in &grammar_rule.items {
            if let crate::ast::GrammarItem::NonTerminal(cat) = item {
                args.push(cat);
                arg_categories.push(cat);
            }
        }
        
        if args.is_empty() {
            // No arguments - nullary constructor, no congruence needed
            continue;
        }
        
        // Skip constructors with Var arguments - Var is not a user-defined category
        if args.iter().any(|cat| cat.to_string() == "Var") {
            continue;
        }
        
        // Generate variable names for LHS and RHS
        let lhs_vars: Vec<Ident> = (0..args.len())
            .map(|i| format_ident!("x{}", i))
            .collect();
        let rhs_vars: Vec<Ident> = (0..args.len())
            .map(|i| format_ident!("y{}", i))
            .collect();
        
        // Generate category bindings and equality checks for each argument
        // For each arg: cat(x), cat(y), eq_cat(x, y)
        // Note: First occurrences bind the variables (no .clone()), subsequent uses need .clone()
        let mut body_clauses = Vec::new();
        
        for (cat, (lhs, rhs)) in args.iter().zip(lhs_vars.iter().zip(rhs_vars.iter())) {
            let cat_rel = format_ident!("{}", cat.to_string().to_lowercase());
            let eq_arg_rel = format_ident!("eq_{}", cat.to_string().to_lowercase());
            
            // Bind the variables (no .clone())
            body_clauses.push(quote! { #cat_rel(#lhs) });
            body_clauses.push(quote! { #cat_rel(#rhs) });
            // Use the bound variables (.clone() needed here)
            body_clauses.push(quote! { #eq_arg_rel(#lhs.clone(), #rhs.clone()) });
        }
        
        // Generate LHS and RHS constructor applications for the head
        let lhs_boxed: Vec<TokenStream> = lhs_vars.iter()
            .map(|v| quote! { Box::new(#v.clone()) })
            .collect();
        let rhs_boxed: Vec<TokenStream> = rhs_vars.iter()
            .map(|v| quote! { Box::new(#v.clone()) })
            .collect();
        
        let label = grammar_rule.label.clone();
        // Generate the congruence rule
        // eq_cat(Constructor(x0, x1, ...), Constructor(y0, y1, ...)) <--
        //   cat0(x0), cat0(y0), eq_cat0(x0, y0),
        //   cat1(x1), cat1(y1), eq_cat1(x1, y1), ...
        rules.push(quote! {
            #eq_rel(
                #category::#label(#(#lhs_boxed),*),
                #category::#label(#(#rhs_boxed),*)
            ) <-- #(#body_clauses),*;
        });
    }
    
    rules
}

/// Generate equation rules
fn generate_equation_rules(theory: &TheoryDef) -> TokenStream {
    let mut rules = Vec::new();
    
    // Add reflexivity for eq relations
    // This is needed for rewrites that check eq_cat(x, y) where x == y syntactically
    // The eqrel data structure requires explicit seeding even for reflexivity
    for export in &theory.exports {
        let cat = &export.name;
        let cat_lower = format_ident!("{}", cat.to_string().to_lowercase());
        let eq_rel = format_ident!("eq_{}", cat.to_string().to_lowercase());
        
        rules.push(quote! {
            #eq_rel(t.clone(), t.clone()) <-- #cat_lower(t);
        });
    }
    
    // Add congruence rules for all constructors
    // If arg1 == arg2, then Constructor(arg1) == Constructor(arg2)
    let congruence_rules = generate_congruence_rules(theory);
    rules.extend(congruence_rules);
    
    // Generate clauses for each equation declaration
    // These add the BASE equalities specified by the theory
    for equation in &theory.equations {
        if let Some(rule) = generate_equation_clause(equation, theory) {
            rules.push(rule);
        }
    }
    
    quote! {
        #(#rules)*
    }
}

/// Generate rewrite rules
fn generate_rewrite_rules(theory: &TheoryDef) -> TokenStream {
    let mut rules = Vec::new();
    
    // For each category, generate:
    // 1. Extension along eq: rw_cat(s1, t) <-- rw_cat(s0, t), eq_cat(s0, s1);
    for export in &theory.exports {
        let cat = &export.name;
        let eq_rel = format_ident!("eq_{}", cat.to_string().to_lowercase());
        let rw_rel = format_ident!("rw_{}", cat.to_string().to_lowercase());
        
        // Extension along eq
        rules.push(quote! {
            #rw_rel(s1, t) <-- #rw_rel(s0, t), #eq_rel(s0, s1);
        });
        rules.push(quote! {
            #rw_rel(s, t1) <-- #rw_rel(s, t0), #eq_rel(t0, t1);
        });
    }
    
    // 2. Base rewrites: generate Ascent clauses with equational matching
    // Only for rules without a premise (congruences are handled separately)
    let base_rewrite_clauses = crate::rewrite_gen::generate_rewrite_clauses(theory);
    rules.extend(base_rewrite_clauses);
    
    // 3. Congruence rules: explicitly declared as "if S => T then ..."
    for (idx, rewrite) in theory.rewrites.iter().enumerate() {
        if rewrite.premise.is_some() {
            // Congruence rewrite - generate inline pattern matching
            if let Some(rule) = generate_congruence_rewrite(idx, rewrite, theory) {
                rules.push(rule);
            }
        }
    }
    
    quote! {
        #(#rules)*
    }
}

/// Generate congruence rewrite rules
/// These are declared as: if S => T then (Constructor P S Q) => (Constructor P T Q)
fn generate_congruence_rewrite(idx: usize, rewrite: &crate::ast::RewriteRule, theory: &TheoryDef) -> Option<TokenStream> {
    // Only process rules with a congruence premise
    let (source_var, target_var) = rewrite.premise.as_ref()?;
    
    // Extract category from LHS
    let category = extract_category_from_expr(&rewrite.left, theory)?;
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let rw_rel = format_ident!("rw_{}", category.to_string().to_lowercase());
    
    // Parse LHS to determine constructor and which field contains source_var
    let (constructor, field_idx, bindings) = extract_congruence_info(&rewrite.left, source_var, theory)?;
    
    // Check if this is a binding constructor
    let rule = theory.terms.iter().find(|r| r.label == constructor)?;
    let is_binding = !rule.bindings.is_empty();
    
    let result = if is_binding {
        generate_binding_congruence(&category, &cat_lower, &rw_rel, constructor, field_idx, &bindings, rule)
    } else {
        generate_regular_congruence(&category, &cat_lower, &rw_rel, constructor, field_idx, &bindings)
    };
    
    result
}

/// Extract congruence information: (constructor, field_index, all_fields)
/// From: (PPar P S) where S is the source_var, returns ("PPar", 1, ["P", "S"])
fn extract_congruence_info(expr: &Expr, source_var: &Ident, theory: &TheoryDef) -> Option<(Ident, usize, Vec<Ident>)> {
    match expr {
        Expr::Apply { constructor, args } => {
            let mut bindings = Vec::new();
            let mut field_idx = None;
            
            for (i, arg) in args.iter().enumerate() {
                match arg {
                    Expr::Var(var) => {
                        if var == source_var {
                            field_idx = Some(i);
                        }
                        bindings.push(var.clone());
                    }
                    _ => return None, // Nested constructors not supported in congruence LHS
                }
            }
            
            Some((constructor.clone(), field_idx?, bindings))
        }
        _ => None,
    }
}

/// Generate congruence for regular (non-binding) constructors
/// Example: if S => T then (PPar P S) => (PPar P T)
/// Generates:
/// rw_proc(s, t) <-- 
///     proc(s),
///     if let Proc::PPar(p, s0) = s,
///     rw_proc(**s0, t0),
///     let t = Proc::PPar(p.clone(), Box::new(t0.clone()));
fn generate_regular_congruence(
    category: &Ident,
    cat_lower: &Ident,
    rw_rel: &Ident,
    constructor: Ident,
    field_idx: usize,
    bindings: &[Ident],
) -> Option<TokenStream> {
    // Generate field patterns
    let field_patterns: Vec<_> = bindings.iter().enumerate().map(|(i, var)| {
        let var_lower = format_ident!("{}", var.to_string().to_lowercase());
        if i == field_idx {
            // This is the field being rewritten - name it s0
            format_ident!("s0")
        } else {
            var_lower
        }
    }).collect();
    
    // Generate the recursive rewrite clause
    let rewritten_field = format_ident!("t0");
    
    // Generate reconstruction arguments
    let recon_args: Vec<_> = bindings.iter().enumerate().map(|(i, _)| {
        if i == field_idx {
            quote! { Box::new(#rewritten_field.clone()) }
        } else {
            let field_name = &field_patterns[i];
            quote! { #field_name.clone() }
        }
    }).collect();
    
    Some(quote! {
        #rw_rel(s, t) <-- 
            #cat_lower(s),
            if let #category::#constructor(#(#field_patterns),*) = s,
            #rw_rel(**s0, #rewritten_field),
            let t = #category::#constructor(#(#recon_args),*);
    })
}

/// Generate congruence for binding constructors
/// Example: if S => T then (PNew x S) => (PNew x T)
/// Generates:
/// rw_proc(s, t) <-- 
///     proc(s),
///     if let Proc::PNew(scope) = s,
///     let (x, body) = scope.clone().unbind(),
///     rw_proc(*body, t0),
///     let new_scope = mettail_runtime::Scope::new(x.clone(), Box::new(t0.clone())),
///     let t = Proc::PNew(new_scope);
fn generate_binding_congruence(
    category: &Ident,
    cat_lower: &Ident,
    rw_rel: &Ident,
    constructor: Ident,
    field_idx: usize,
    bindings: &[Ident],
    rule: &crate::ast::GrammarRule,
) -> Option<TokenStream> {
    // For binding constructors, generate unbind + rewrite + rebind logic
    let rewritten_field = format_ident!("t0");
    
    // Determine which fields are binders and which are being rewritten
    let binder_vars: Vec<_> = rule.bindings.iter().map(|(binder_idx, _)| {
        let var_name = bindings.get(*binder_idx)?;
        Some(format_ident!("{}", var_name.to_string().to_lowercase()))
    }).collect::<Option<Vec<_>>>()?;
    
    // For now, assume single binder and single body (common case)
    if binder_vars.len() != 1 {
        return None; // Complex binding patterns not yet supported
    }
    
    let binder_var = &binder_vars[0];
    
    // Generate field patterns for non-binder, non-rewritten fields
    let mut other_fields = Vec::new();
    let mut recon_args = Vec::new();
    
    for (i, _var) in bindings.iter().enumerate() {
        let is_binder = rule.bindings.iter().any(|(b_idx, _)| *b_idx == i);
        
        if is_binder {
            // Skip - handled by unbind
            continue;
        } else if i == field_idx {
            // This is the field being rewritten
            recon_args.push(quote! { Box::new(#rewritten_field.clone()) });
        } else {
            // Other field - extract and clone
            let field_name = format_ident!("field_{}", i);
            other_fields.push((i, field_name.clone()));
            recon_args.push(quote! { #field_name.clone() });
        }
    }
    
    Some(quote! {
        #rw_rel(s, t) <-- 
            #cat_lower(s),
            if let #category::#constructor(scope) = s,
            let (#binder_var, body) = scope.clone().unbind(),
            #rw_rel(*body, #rewritten_field),
            let new_scope = mettail_runtime::Scope::new(#binder_var.clone(), Box::new(#rewritten_field.clone())),
            let t = #category::#constructor(new_scope);
    })
}

/// Generate a single equation clause
/// Example: (PPar P Q) == (PPar Q P) generates:
/// eq_proc(p0, p1) <-- proc(p0), if let Proc::PPar(p, q) = p0, let p1 = Proc::PPar(q.clone(), p.clone());
fn generate_equation_clause(equation: &Equation, theory: &TheoryDef) -> Option<TokenStream> {
    // Determine the category from the LHS
    let category = extract_category_from_expr(&equation.left, theory)?;
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let eq_rel = format_ident!("eq_{}", category.to_string().to_lowercase());
    
    // Generate pattern matching for LHS
    let mut bindings: HashMap<String, Ident> = HashMap::new();
    let mut nested_patterns = Vec::new();
    let lhs_pattern = generate_equation_pattern(&equation.left, "p0", &mut bindings, theory, &mut nested_patterns)?;
    
    // Generate RHS construction
    let rhs_construction = generate_equation_rhs(&equation.right, &bindings, theory, false);
    
    // Generate freshness checks if any
    let freshness_checks = generate_equation_freshness(&equation.conditions, &bindings);
    
    Some(quote! {
        #eq_rel(p0, p1) <--
            #cat_lower(p0),
            #lhs_pattern
            #(#nested_patterns)*
            #freshness_checks
            let p1 = #rhs_construction;
    })
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
    }
}

/// Check if an identifier is a constructor in the theory
fn is_constructor(ident: &Ident, theory: &TheoryDef) -> bool {
    theory.terms.iter().any(|rule| rule.label == *ident)
}

/// Check if a constructor is nullary (has no non-terminal arguments)
fn is_nullary_constructor(ident: &Ident, theory: &TheoryDef) -> bool {
    theory.terms.iter()
        .find(|rule| rule.label == *ident)
        .map(|rule| rule.items.iter().all(|item| matches!(item, crate::ast::GrammarItem::Terminal(_))))
        .unwrap_or(false)
}

/// Generate pattern matching code for equation LHS
/// Convert a variable name to snake_case for use in generated code
/// Examples: P -> p, Chan -> chan, MyVar -> my_var
fn to_snake_case(name: &str) -> String {
    if name.is_empty() {
        return name.to_string();
    }
    
    // If it's a single character, just lowercase it
    if name.len() == 1 {
        return name.to_lowercase();
    }
    
    // If it's already all lowercase, return as-is
    if name.chars().all(|c| !c.is_uppercase()) {
        return name.to_string();
    }
    
    // Convert CamelCase or UPPERCASE to snake_case
    let mut result = String::new();
    let mut prev_was_uppercase = false;
    
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase if not at start and prev was lowercase
            if i > 0 && !prev_was_uppercase {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
            prev_was_uppercase = true;
        } else {
            result.push(ch);
            prev_was_uppercase = false;
        }
    }
    
    result
}

/// Returns the "if let" pattern match
fn generate_equation_pattern(
    expr: &Expr,
    term_name: &str,
    bindings: &mut HashMap<String, Ident>,
    theory: &TheoryDef,
    nested_patterns: &mut Vec<TokenStream>,
) -> Option<TokenStream> {
    match expr {
        Expr::Var(var) => {
            // Check if this is actually a constructor (nullary or otherwise)
            if is_constructor(var, theory) {
                // This is a constructor, not a variable - shouldn't happen at top level
                None
            } else {
                // Just bind the variable
                let var_name = var.to_string();
                // Convert to snake_case for Rust naming conventions
                let var_snake = to_snake_case(&var_name);
                let var_ident = format_ident!("{}", var_snake);
                bindings.insert(var_name, var_ident.clone());
                None // No pattern match needed, just use the term directly
            }
        }
        Expr::Apply { constructor, args } => {
            // Generate if let Constructor(field_0, field_1, ...) = term_name
            let term_ident = format_ident!("{}", term_name);
            
            // Look up the category in the theory
            let category = extract_category_from_expr(expr, theory)?;
            
            let mut field_patterns = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                match arg {
                    Expr::Var(var) => {
                        // Check if this is actually a constructor
                        if is_constructor(var, theory) {
                            // It's a constructor
                            if is_nullary_constructor(var, theory) {
                                // Nullary constructor - bind to temp and match
                                let field_ident = format_ident!("field_{}", i);
                                field_patterns.push(quote! { #field_ident });
                                
                                // Get the constructor's category
                                let constructor_category = theory.terms.iter()
                                    .find(|r| r.label == *var)
                                    .map(|r| &r.category)?;
                                
                                // Generate nested pattern: if let Cat::Constructor = **field_i
                                // Note: **field_i because field_i is &Box<T>, and we need T
                                nested_patterns.push(quote! {
                                    if let #constructor_category::#var = **#field_ident,
                                });
                            } else {
                                // Non-nullary constructor - bind to temp and match (shouldn't happen for Var)
                                let field_ident = format_ident!("field_{}", i);
                                field_patterns.push(quote! { #field_ident });
                                
                                // Get the constructor's category
                                let constructor_category = theory.terms.iter()
                                    .find(|r| r.label == *var)
                                    .map(|r| &r.category)?;
                                
                                // Generate nested pattern: if let Cat::Constructor(...) = &**field_i
                                nested_patterns.push(quote! {
                                    if let #constructor_category::#var = &**#field_ident,
                                });
                            }
                        } else {
                            // It's a real variable
                            let var_name = var.to_string();
                            // Convert to snake_case for Rust naming conventions
                            let var_snake = to_snake_case(&var_name);
                            let var_ident = format_ident!("{}", var_snake);
                            bindings.insert(var_name, var_ident.clone());
                            field_patterns.push(quote! { #var_ident });
                        }
                    }
                    Expr::Apply { constructor: nested_constructor, args: nested_args } => {
                        // Nested pattern - bind to a temp variable and generate a nested if-let
                        let field_ident = format_ident!("field_{}", i);
                        field_patterns.push(quote! { #field_ident });
                        
                        // Generate nested pattern match: if let Cat::Constructor(p, q) = &**field_i
                        let nested_category = extract_category_from_expr(arg, theory)?;
                        let mut nested_field_patterns = Vec::new();
                        
                        for nested_arg in nested_args {
                            match nested_arg {
                                Expr::Var(var) => {
                                    // Check if it's a constructor
                                    if is_constructor(var, theory) {
                                        // Nested nullary constructor - we'd need another level
                                        // For now, skip deeply nested constructors
                                        return None;
                                    } else {
                                        let var_name = var.to_string();
                                        // Convert to snake_case for Rust naming conventions
                                        let var_snake = to_snake_case(&var_name);
                                        let var_ident = format_ident!("{}", var_snake);
                                        bindings.insert(var_name, var_ident.clone());
                                        nested_field_patterns.push(var_ident);
                                    }
                                }
                                _ => {
                                    // Deeply nested patterns - TODO: handle recursively
                                    return None;
                                }
                            }
                        }
                        
                        nested_patterns.push(quote! {
                            if let #nested_category::#nested_constructor(#(#nested_field_patterns),*) = &**#field_ident,
                        });
                    }
                    Expr::Subst { .. } => {
                        // Substitution in LHS pattern - shouldn't happen
                        return None;
                    }
                }
            }
            
            Some(quote! {
                if let #category::#constructor(#(#field_patterns),*) = #term_ident,
            })
        }
        Expr::Subst { .. } => {
            // Substitution shouldn't appear in equation LHS
            None
        }
    }
}

/// Generate RHS construction code
/// `in_constructor` flag indicates if we're inside a constructor argument (affects Box wrapping)
fn generate_equation_rhs(expr: &Expr, bindings: &HashMap<String, Ident>, theory: &TheoryDef, in_constructor: bool) -> TokenStream {
    match expr {
        Expr::Var(var) => {
            // Check if this is a constructor or a variable
            if is_constructor(var, theory) {
                // It's a nullary constructor - generate the constructor
                let constructor_category = theory.terms.iter()
                    .find(|r| r.label == *var)
                    .map(|r| &r.category)
                    .expect("Constructor not found in theory");
                
                if in_constructor {
                    quote! { Box::new(#constructor_category::#var) }
                } else {
                    quote! { #constructor_category::#var }
                }
            } else {
                // It's a variable
                let var_name = var.to_string();
                if let Some(var_ident) = bindings.get(&var_name) {
                    // Variables are bound as &Box<T> from pattern matching
                    if in_constructor {
                        // Inside constructor: just clone (keeps it as Box<T>)
                        quote! { #var_ident.clone() }
                    } else {
                        // Top-level: need to dereference to get the inner value
                        // Use **var to get from &Box<T> to T
                        quote! { (**#var_ident).clone() }
                    }
                } else {
                    // Unbound variable - shouldn't happen
                    quote! { #var }
                }
            }
        }
        Expr::Apply { constructor, args } => {
            let category = extract_category_from_expr(expr, theory)
                .unwrap_or_else(|| constructor.clone());
            let arg_constructions: Vec<TokenStream> = args
                .iter()
                .map(|arg| {
                    let inner = generate_equation_rhs(arg, bindings, theory, true);
                    // The recursive call with in_constructor=true handles Box wrapping
                    inner
                })
                .collect();
            
            if in_constructor {
                // We're being called as an argument to another constructor
                // We need to wrap ourselves in Box::new
                quote! {
                    Box::new(#category::#constructor(#(#arg_constructions),*))
                }
            } else {
                // Top-level constructor - no wrapping needed
                quote! {
                    #category::#constructor(#(#arg_constructions),*)
                }
            }
        }
        Expr::Subst { term, var, replacement } => {
            let term_code = generate_equation_rhs(term, bindings, theory, in_constructor);
            let var_name = var.to_string();
            let replacement_code = generate_equation_rhs(replacement, bindings, theory, in_constructor);
            
            quote! {
                mettail_runtime::substitute(
                    &#term_code,
                    &mettail_runtime::Var::new(#var_name.to_string()),
                    &#replacement_code
                )
            }
        }
    }
}

/// Generate freshness checks for equation
fn generate_equation_freshness(
    _conditions: &[crate::ast::FreshnessCondition],
    _bindings: &HashMap<String, Ident>,
) -> TokenStream {
    // TODO: Implement freshness checks
    quote! {}
}

