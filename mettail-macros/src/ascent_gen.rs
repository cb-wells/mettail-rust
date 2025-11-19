use crate::ast::{TheoryDef, GrammarRule, Equation, Expr, RewriteRule};
use crate::utils::{print_rule};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;
use std::collections::{HashMap, HashSet};


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
    
    eprintln!("\n========== FULL GENERATED ASCENT SOURCE ==========");
    eprintln!("ascent_source! {{");
    eprintln!("    {}:\n", source_name);
    eprintln!("    // Relations");
    for line in relations.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Category rules");
    for line in category_rules.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Equation rules");
    for line in equation_rules.to_string().split(';') {
        print_rule(line);
    }
    eprintln!("\n    // Rewrite rules");
    for line in rewrite_rules.to_string().split(';') {
        print_rule(line);
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
    
    // Collection projection relations (automatic)
    // For each constructor with a collection field, generate a "contains" relation
    // Example: PPar(HashBag<Proc>) generates: relation ppar_contains(Proc, Proc);
    let projection_relations = generate_collection_projection_relations(theory);
    relations.extend(projection_relations);
    
    quote! { 
        #(#relations)*
    }
}

/// Generate collection projection relations
/// For each constructor with a collection field, automatically generate a "contains" relation
/// that relates the parent term to each element in the collection.
/// 
/// Example: For PPar(HashBag<Proc>), generates:
/// ```
/// relation ppar_contains(Proc, Proc);
/// ```
/// 
/// These relations are populated by rules in generate_category_rules.
fn generate_collection_projection_relations(theory: &TheoryDef) -> Vec<TokenStream> {
    let mut relations = Vec::new();
    
    for rule in &theory.terms {
        // Check if this constructor has a collection field
        for (field_idx, item) in rule.items.iter().enumerate() {
            if let crate::ast::GrammarItem::Collection { element_type, .. } = item {
                // Found a collection field!
                let parent_cat = &rule.category;
                let constructor = &rule.label;
                let elem_cat = element_type;
                
                // Generate relation name: <constructor_lowercase>_contains
                let rel_name = format_ident!("{}_contains", 
                                             constructor.to_string().to_lowercase());
                
                relations.push(quote! {
                    relation #rel_name(#parent_cat, #elem_cat);
                });
                
                // Note: Only one collection per constructor for now
                // If we need multiple, we'd generate: constructor_contains_field0, etc.
                break;
            }
        }
    }
    
    relations
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
        
        // Generate deconstruction rules for this category
        let deconstruct_rules = generate_deconstruction_rules(cat, theory);
        rules.extend(deconstruct_rules);
        
        // Generate collection projection population rules for this category
        let projection_rules = generate_collection_projection_population(cat, theory);
        rules.extend(projection_rules);
        
        // Generate projection seeding rules for this category
        // This adds collection elements to their category relations
        let seeding_rules = generate_projection_seeding_rules(cat, theory);
        rules.extend(seeding_rules);
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
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let label = &constructor.label;
    
    // Check if this constructor has collection fields
    let has_collections = constructor.items.iter().any(|item| {
        matches!(item, crate::ast::GrammarItem::Collection { .. })
    });
    
    if has_collections {
        // Generate deconstruction for collection fields
        return generate_collection_deconstruction(category, constructor);
    }
    
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

/// Generate deconstruction for constructors with collection fields
/// 
/// PERFORMANCE NOTE: This eagerly extracts ALL elements from collections as separate facts,
/// which causes exponential fact explosion (O(N*M) where N=terms, M=bag size).
/// 
/// This is DISABLED because:
/// 1. Collection congruence works via projection relations, not deconstruction
/// 2. Base rewrites are seeded directly from projection relations (see generate_category_rules)
/// 3. Eager deconstruction creates 100s-1000s of redundant facts
/// 4. Results in 50x+ slowdown on moderately complex terms
/// 
/// Instead: Elements are accessed on-demand via `ppar_contains` projection relation.
fn generate_collection_deconstruction(
    category: &Ident,
    constructor: &GrammarRule,
) -> Option<TokenStream> {
    // DISABLED: Use projection relations instead
    None
}

/// Generate collection projection population rules
/// For each constructor with a collection field, generate rules that populate
/// the corresponding "contains" relation.
/// 
/// Example: For PPar(HashBag<Proc>), generates:
/// ```
/// ppar_contains(parent.clone(), elem.clone()) <--
///     proc(parent),
///     if let Proc::PPar(ref bag_field) = parent,
///     for (elem, _count) in bag_field.iter();
/// ```
/// 
/// This creates a database of all collection-element relationships that can be
/// efficiently queried and joined by Ascent.
fn generate_collection_projection_population(category: &Ident, theory: &TheoryDef) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    
    // Find all constructors for this category
    let constructors: Vec<&GrammarRule> = theory.terms
        .iter()
        .filter(|r| r.category == *category)
        .collect();
    
    for constructor in constructors {
        // Check if this constructor has a collection field
        for item in &constructor.items {
            if let crate::ast::GrammarItem::Collection { element_type, .. } = item {
                // Found a collection field - generate projection rule
                let parent_cat = &constructor.category;
                let parent_cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
                let constructor_label = &constructor.label;
                let elem_cat = element_type;
                
                // Generate relation name: <constructor_lowercase>_contains
                let rel_name = format_ident!("{}_contains", 
                                             constructor_label.to_string().to_lowercase());
                
                rules.push(quote! {
                    #rel_name(parent.clone(), elem.clone()) <--
                        #parent_cat_lower(parent),
                        if let #parent_cat::#constructor_label(ref bag_field) = parent,
                        for (elem, _count) in bag_field.iter();
                });
                
                // Only handle one collection per constructor for now
                break;
            }
        }
    }
    
    rules
}

/// Generate rules to seed category relations from projection relations
/// This allows base rewrites to match on collection elements without eager deconstruction.
/// 
/// Example: For PPar(HashBag<Proc>) with projection relation ppar_contains(Proc, Proc),
/// generates:
/// ```
/// proc(elem) <-- ppar_contains(_parent, elem);
/// ```
/// 
/// This is much more efficient than eager deconstruction because:
/// 1. Elements are only added to proc when they're actually in a ppar_contains fact
/// 2. No redundant facts for elements that appear in multiple collections
/// 3. Lazy evaluation: only computes what's needed
fn generate_projection_seeding_rules(category: &Ident, theory: &TheoryDef) -> Vec<TokenStream> {
    let mut rules = Vec::new();
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    
    // Find all constructors for this category that have collections
    let constructors: Vec<&GrammarRule> = theory.terms
        .iter()
        .filter(|r| r.category == *category)
        .collect();
    
    for constructor in constructors {
        // Check if this constructor has a collection field
        for item in &constructor.items {
            if let crate::ast::GrammarItem::Collection { element_type, .. } = item {
                // Found a collection field
                let elem_cat = element_type;
                let elem_cat_lower = format_ident!("{}", elem_cat.to_string().to_lowercase());
                let constructor_label = &constructor.label;
                
                // Generate relation name: <constructor_lowercase>_contains
                let rel_name = format_ident!("{}_contains", 
                                             constructor_label.to_string().to_lowercase());
                
                // Generate seeding rule: elem_cat(elem) <-- contains_rel(_parent, elem);
                rules.push(quote! {
                    #elem_cat_lower(elem) <-- #rel_name(_parent, elem);
                });
                
                // Only handle one collection per constructor
                break;
            }
        }
    }
    
    rules
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
            // Clone the Box to get Box<T>, then use as_ref() to get &T, then clone to get T
            Some(quote! { 
                #field_type_lower(#field_name.as_ref().clone())
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
        // IMPORTANT: Access unsafe_body field directly to avoid fresh IDs from unbind()
        // The inner moniker Scope has public unsafe_body and unsafe_pattern fields
        // We access via .inner() to get the moniker Scope, then access the field directly
        Some(quote! {
            #body_cat_lower(body_value) <--
                #cat_lower(t),
                if let #category::#label(scope) = t,
                let body_value = scope.inner().unsafe_body.as_ref().clone();
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
            subterm_facts.push(quote! { #cat_lower(#field_name.as_ref().clone()) });
        }
        subterm_facts.push(quote! { #body_cat_lower(body.as_ref().clone()) });
        
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
        
        // Check if this constructor has collections - skip if so
        let has_collections = grammar_rule.items.iter().any(|item| {
            matches!(item, crate::ast::GrammarItem::Collection { .. })
        });
        
        if has_collections {
            // Skip collections - they have built-in equality
            // (HashBag/HashSet equality is order-independent)
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
    
    // 3. Congruence rules: NEW APPROACH - congruence-driven projection generation
    // For each collection congruence, generate projections and clauses
    for (cong_idx, rewrite) in theory.rewrites.iter().enumerate() {
        if let Some((source_var, target_var)) = &rewrite.premise {
            // Check if this is a collection congruence
            if let Some(cong_info) = crate::congruence_analysis::extract_collection_congruence_info(
                &rewrite.left, source_var, target_var, theory
            ) {
                // Generate all projections for this congruence
                let (projections, base_patterns) = crate::congruence_analysis::generate_congruence_projections(
                    cong_idx, &cong_info, theory
                );
                rules.extend(projections);
                
                // Generate congruence clauses using those projections and the updated patterns
                let congruence_clauses = generate_new_collection_congruence_clauses(
                    cong_idx, &cong_info, &base_patterns, theory
                );
                rules.extend(congruence_clauses);
            } else {
                // Regular (non-collection) congruence - use existing generation
                if let Some(rule) = generate_congruence_rewrite(cong_idx, rewrite, theory) {
                    rules.push(rule);
                }
            }
        }
    }
    
    quote! {
        #(#rules)*
    }
}

/// Generate congruence rewrite rules
/// These are declared as: if S => T then (Constructor P S Q) => (Constructor P T Q)
/// Also handles collection congruence: if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})
fn generate_congruence_rewrite(idx: usize, rewrite: &crate::ast::RewriteRule, theory: &TheoryDef) -> Option<TokenStream> {
    // Only process rules with a congruence premise
    let (source_var, target_var) = rewrite.premise.as_ref()?;
    
    // Extract category from LHS
    let category = extract_category_from_expr(&rewrite.left, theory)?;
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let rw_rel = format_ident!("rw_{}", category.to_string().to_lowercase());
    
    // Check if this is a collection congruence
    // Pattern: (Constructor {S, ...rest}) where S is the source_var
    if let Expr::Apply { constructor, args } = &rewrite.left {
        for arg in args {
            if let Expr::CollectionPattern { elements, rest, .. } = arg {
                // Check if source_var appears in elements
                for elem in elements {
                    if let Expr::Var(v) = elem {
                        if v == source_var {
                            // This is a collection congruence!
                            return generate_collection_congruence(
                                &category,
                                &cat_lower,
                                &rw_rel,
                                constructor,
                                source_var,
                                target_var,
                                rest.as_ref(),
                                theory,
                            );
                        }
                    }
                }
            }
        }
    }
    
    // Regular (non-collection) congruence
    // Parse LHS to determine constructor and which field contains source_var
    let (constructor, field_idx, bindings) = extract_congruence_info(&rewrite.left, source_var, theory)?;
    
    // Check if this is a binding constructor
    let rule = theory.terms.iter().find(|r| r.label == constructor)?;
    let is_binding = !rule.bindings.is_empty();
    
    let result = if is_binding {
        // NEW: Use projection-based binding congruence
        eprintln!("DEBUG: Generating projection-based binding congruence for {} (idx={})", constructor, idx);
        let result = generate_projection_based_binding_congruence(idx, &category, constructor.clone(), field_idx, rule, theory);
        if result.is_none() {
            eprintln!("DEBUG: generate_projection_based_binding_congruence returned None for {}", constructor);
        }
        result
    } else {
        generate_regular_congruence(&category, &cat_lower, &rw_rel, constructor, field_idx, &bindings)
    };
    
    result
}

/// Extract congruence information: (constructor, field_index, all_fields)
/// From: (PPar P S) where S is the source_var, returns ("PPar", 1, ["P", "S"])
/// Also handles collection patterns: (PPar {S, ...rest}) returns constructor info
fn extract_congruence_info(expr: &Expr, source_var: &Ident, theory: &TheoryDef) -> Option<(Ident, usize, Vec<Ident>)> {
    match expr {
        Expr::Apply { constructor, args } => {
            // Check if any arg is a CollectionPattern
            for (i, arg) in args.iter().enumerate() {
                if let Expr::CollectionPattern { elements, rest, .. } = arg {
                    // Collection pattern case
                    // Check if source_var appears in the elements
                    for elem in elements {
                        if let Expr::Var(v) = elem {
                            if v == source_var {
                                // Found it! This is a collection congruence
                                // Return constructor and a sentinel field_idx
                                // We'll use field_idx = 0 to indicate "collection field"
                                return Some((constructor.clone(), 0, vec![source_var.clone()]));
                            }
                        }
                    }
                }
            }
            
            // Regular (non-collection) case
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
                    Expr::CollectionPattern { .. } => {
                        // Skip collection patterns in regular case
                        continue;
                    }
                    _ => return None, // Nested constructors not supported in congruence LHS
                }
            }
            
            Some((constructor.clone(), field_idx?, bindings))
        }
        _ => None,
    }
}

/// Generate congruence for collection constructors
/// Example: if S => T then (PPar {S, ...rest}) => (PPar {T, ...rest})
/// Generates:
/// ```
/// rw_proc(parent, result) <--
///     ppar_contains(parent, elem),
///     rw_proc(elem, elem_rewritten),
///     if let Proc::PPar(ref bag) = parent,
///     let rest = { let mut b = bag.clone(); b.remove(&elem); b },
///     let result = Proc::PPar({
///         let mut bag = rest;
///         Proc::insert_into_ppar(&mut bag, elem_rewritten);
///         bag
///     });
/// ```
fn generate_collection_congruence(
    category: &Ident,
    cat_lower: &Ident,
    rw_rel: &Ident,
    constructor: &Ident,
    source_var: &Ident,
    target_var: &Ident,
    rest_var: Option<&Ident>,
    theory: &TheoryDef,
) -> Option<TokenStream> {
    // Get the element category by finding this constructor's collection field
    let rule = theory.terms.iter().find(|r| r.label == *constructor)?;
    
    let elem_cat = rule.items.iter().find_map(|item| {
        if let crate::ast::GrammarItem::Collection { element_type, .. } = item {
            Some(element_type.clone())
        } else {
            None
        }
    })?;
    
    // Generate relation name for projection
    let contains_rel = format_ident!("{}_contains", 
                                     constructor.to_string().to_lowercase());
    
    // Generate the element rewrite relation
    let elem_rw_rel = format_ident!("rw_{}", elem_cat.to_string().to_lowercase());
    
    // Generate flatten helper name
    let constructor_lower = format_ident!("{}", constructor.to_string().to_lowercase());
    let insert_helper = format_ident!("insert_into_{}", constructor_lower);
    
    Some(quote! {
        #rw_rel(parent, result) <--
            #contains_rel(parent, elem),
            #elem_rw_rel(*elem, elem_rewritten),
            if let #category::#constructor(ref bag) = parent,
            let rest = {
                let mut b = bag.clone();
                b.remove(elem);
                b
            },
            let result = #category::#constructor({
                let mut bag = rest;
                #category::#insert_helper(&mut bag, elem_rewritten.clone());
                bag
            });
    })
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
    let binder_vars: Vec<_> = rule.bindings.iter().enumerate().map(|(idx, (binder_idx, _))| {
        let var_name = bindings.get(*binder_idx)?;
        // Use a unique name to avoid shadowing the input parameter 's'
        Some(format_ident!("binder_{}", idx))
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
            let new_scope_tmp = mettail_runtime::Scope::new(#binder_var.clone(), Box::new(#rewritten_field.clone())),
            let t = #category::#constructor(new_scope_tmp);
    })
}

/// Generate projection-based binding congruence
/// For: if S => T then (PNew x S) => (PNew x T)
/// Generates:
///   1. Projection relation: pnew_direct_congruence_proj(parent, binder, body)
///   2. Population rule: extracts parent, unbinds, projects body
///   3. Congruence clause: joins projection with rw_proc(body, _)
fn generate_projection_based_binding_congruence(
    _congruence_idx: usize,
    category: &Ident,
    constructor: Ident,
    rewrite_field_idx: usize,
    rule: &crate::ast::GrammarRule,
    _theory: &TheoryDef,
) -> Option<TokenStream> {
    // Validate: only single binder supported for now
    if rule.bindings.len() != 1 {
        eprintln!(
            "Warning: Multiple binders not yet supported for projection-based congruence: {}",
            constructor
        );
        return None;
    }
    
    let (binder_idx, body_indices) = &rule.bindings[0];
    
    // Validate: binder must bind in exactly one body
    if body_indices.len() != 1 {
        eprintln!(
            "Warning: Binder binding in multiple bodies not supported: {}",
            constructor
        );
        return None;
    }
    
    let body_idx = body_indices[0];
    
    // Get the body category
    let body_cat = match &rule.items[body_idx] {
        crate::ast::GrammarItem::NonTerminal(cat) => cat,
        _ => {
            eprintln!("Warning: Body field is not a non-terminal: {}", constructor);
            return None;
        }
    };
    
    // Map field_idx (from args) to grammar item index
    // field_idx counts only Var arguments in the congruence LHS
    // We need to find which grammar item position this corresponds to
    let mut non_terminal_count = 0;
    let mut rewrite_grammar_idx = None;
    for (grammar_idx, item) in rule.items.iter().enumerate() {
        match item {
            crate::ast::GrammarItem::NonTerminal(_) => {
                if non_terminal_count == rewrite_field_idx {
                    rewrite_grammar_idx = Some(grammar_idx);
                    break;
                }
                non_terminal_count += 1;
            }
            crate::ast::GrammarItem::Binder { .. } => {
                // Binders are also counted as fields in the congruence args
                if non_terminal_count == rewrite_field_idx {
                    rewrite_grammar_idx = Some(grammar_idx);
                    break;
                }
                non_terminal_count += 1;
            }
            _ => {}
        }
    }
    
    let rewrite_grammar_idx = match rewrite_grammar_idx {
        Some(idx) => idx,
        None => {
            eprintln!(
                "Warning: Could not map field_idx {} to grammar index for {}",
                rewrite_field_idx, constructor
            );
            return None;
        }
    };
    
    // Validate: the rewrite field must be the body
    if rewrite_grammar_idx != body_idx {
        eprintln!(
            "Warning: Rewrite field is not the bound body for {}: field={}, body={}, mapped_grammar_idx={}",
            constructor, rewrite_field_idx, body_idx, rewrite_grammar_idx
        );
        return None;
    }
    
    // Generate projection relation name
    let constructor_lower = format_ident!("{}", constructor.to_string().to_lowercase());
    let proj_rel = format_ident!("{}_direct_congruence_proj", constructor_lower);
    
    // Generate components
    let proj_decl = generate_binding_proj_declaration(
        &proj_rel, category, body_cat
    );
    
    let proj_population = generate_binding_proj_population(
        &proj_rel, category, &constructor, body_cat, rule, *binder_idx
    );
    
    let rw_rel = format_ident!("rw_{}", category.to_string().to_lowercase());
    let congruence_clause = generate_binding_congruence_clause(
        &rw_rel, &proj_rel, category, &constructor, body_cat
    );
    
    Some(quote! {
        #proj_decl
        #proj_population
        #congruence_clause
    })
}

/// Generate projection relation declaration
/// pnew_direct_congruence_proj(Proc, Binder<String>, Proc)
fn generate_binding_proj_declaration(
    proj_rel: &Ident,
    parent_cat: &Ident,
    body_cat: &Ident,
) -> TokenStream {
    quote! {
        relation #proj_rel(#parent_cat, mettail_runtime::Binder<String>, #body_cat);
    }
}

/// Generate projection population rule
/// Projects parent term, unbinds scope, extracts body
fn generate_binding_proj_population(
    proj_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    _body_cat: &Ident,
    rule: &crate::ast::GrammarRule,
    _binder_idx: usize,
) -> TokenStream {
    let cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());
    
    // Count non-terminal fields
    let field_count = rule.items
        .iter()
        .filter(|item| matches!(item, crate::ast::GrammarItem::NonTerminal(_)))
        .count();
    
    if field_count == 1 {
        // Simple case: only the scope field (common for PNew)
        // IMPORTANT: Access unsafe_body field directly instead of unbind() to avoid fresh IDs
        // This preserves the Bound variables which have stable structure
        quote! {
            #proj_rel(parent, binder_var, body) <--
                #cat_lower(parent),
                if let #parent_cat::#constructor(ref scope) = parent,
                let binder_var = scope.inner().unsafe_pattern.clone(),
                let body = scope.inner().unsafe_body.as_ref().clone();
        }
    } else {
        // Complex case: has other fields besides scope
        // For now, we don't support this (would need to handle all fields)
        eprintln!(
            "Warning: Binding constructor with multiple fields not yet supported: {}",
            constructor
        );
        quote! {
            // TODO: Support binding constructors with multiple fields
            // Projection not generated for {}
        }
    }
}

/// Generate congruence clause using projection
/// Joins projection with rw_proc, reconstructs term
/// IMPORTANT: body still has Bound variables (we didn't unbind), so we reconstruct
/// the Scope directly without rebinding. We also re-box the body.
fn generate_binding_congruence_clause(
    rw_rel: &Ident,
    proj_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    body_cat: &Ident,
) -> TokenStream {
    let body_rw_rel = format_ident!("rw_{}", body_cat.to_string().to_lowercase());
    
    quote! {
        #rw_rel(parent, result) <--
            #proj_rel(parent, binder_var, body),
            #body_rw_rel(body, body_rewritten),
            let scope_tmp = mettail_runtime::Scope::from_parts_unsafe(
                binder_var.clone(), 
                Box::new(body_rewritten.clone())
            ),
            let result = #parent_cat::#constructor(scope_tmp);
    }
}

/// Generate a single equation clause
/// Example: (PPar P Q) == (PPar Q P) generates:
/// eq_proc(p0, p1) <-- proc(p0), if let Proc::PPar(p, q) = p0, let p1 = Proc::PPar(q.clone(), p.clone());
fn generate_equation_clause(equation: &Equation, theory: &TheoryDef) -> Option<TokenStream> {
    // NORMALIZE: If LHS is Apply(Constructor, [CollectionPattern{constructor: None}]),
    // transform to CollectionPattern{constructor: Some(Constructor)}
    let normalized_left = normalize_collection_apply(&equation.left);
    
    // Determine the category from the LHS
    let category = extract_category_from_expr(&normalized_left, theory)?;
    let cat_lower = format_ident!("{}", category.to_string().to_lowercase());
    let eq_rel = format_ident!("eq_{}", category.to_string().to_lowercase());
    
    // Generate pattern matching for LHS
    let mut bindings: HashMap<String, Ident> = HashMap::new();
    let mut nested_patterns = Vec::new();
    let lhs_pattern = generate_equation_pattern(&normalized_left, "p0", &mut bindings, theory, &mut nested_patterns)?;
    
    // Generate RHS construction  
    // For collection patterns, variables are bound as T, not &Box<T>
    // So we need to use a modified RHS generator
    let is_collection_lhs = matches!(normalized_left, Expr::CollectionPattern { .. });
    let rhs_construction = if is_collection_lhs {
        generate_collection_equation_rhs(&equation.right, &bindings, theory)
    } else {
        generate_equation_rhs(&equation.right, &bindings, theory, false)
    };
    
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

/// Normalize Apply(Constructor, [CollectionPattern]) to CollectionPattern{constructor}
/// This handles cases like (PPar {P}) where the collection is wrapped in Apply
fn normalize_collection_apply(expr: &Expr) -> Expr {
    match expr {
        Expr::Apply { constructor, args } if args.len() == 1 => {
            // Check if the single argument is a CollectionPattern with no constructor
            if let Expr::CollectionPattern { constructor: None, elements, rest } = &args[0] {
                // Transform to CollectionPattern with the Apply's constructor
                return Expr::CollectionPattern {
                    constructor: Some(constructor.clone()),
                    elements: elements.clone(),
                    rest: rest.clone(),
                };
            }
            // Not a collection, return as-is
            expr.clone()
        }
        _ => expr.clone()
    }
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
                                
                                // Generate nested pattern: if let Cat::Constructor = *field_i.as_ref()
                                // Note: field_i is &Box<T>, as_ref() gives &T, then * dereferences to T
                                nested_patterns.push(quote! {
                                    if let #constructor_category::#var = *#field_ident.as_ref(),
                                });
                            } else {
                                // Non-nullary constructor - bind to temp and match (shouldn't happen for Var)
                                let field_ident = format_ident!("field_{}", i);
                                field_patterns.push(quote! { #field_ident });
                                
                                // Get the constructor's category
                                let constructor_category = theory.terms.iter()
                                    .find(|r| r.label == *var)
                                    .map(|r| &r.category)?;
                                
                                // Generate nested pattern: if let Cat::Constructor(...) = field_i.as_ref()
                                nested_patterns.push(quote! {
                                    if let #constructor_category::#var = #field_ident.as_ref(),
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
                            if let #nested_category::#nested_constructor(#(#nested_field_patterns),*) = #field_ident.as_ref(),
                        });
                    }
                    Expr::Subst { .. } => {
                        // Substitution in LHS pattern - shouldn't happen
                        return None;
                    }
                    Expr::CollectionPattern { .. } => {
                        // Collection pattern in nested position - not yet supported
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
        Expr::CollectionPattern { constructor, elements, rest } => {
            // Collection pattern: PPar {P, Q, ...rest}
            // Generate pattern matching for collection
            
            if rest.is_some() {
                // Rest patterns in equations are complex - skip for now
                // TODO: Implement rest pattern support for equations
                return None;
            }
            
            let constructor = constructor.as_ref()?;
            let category = extract_category_from_expr(expr, theory)?;
            let term_ident = format_ident!("{}", term_name);
            
            // Generate pattern: if let Cat::Constructor(ref bag) = term
            // Then use loop-based matching for order independence
            let bag_var = format_ident!("bag");
            let size_check = elements.len();
            
            // Generate loop-based matching for each element
            let mut extract_clauses = Vec::new();
            let mut elem_vars = Vec::new();
            
            for (elem_idx, elem) in elements.iter().enumerate() {
                match elem {
                    Expr::Var(var) if !is_constructor(var, theory) => {
                        let var_name = var.to_string();
                        let var_snake = to_snake_case(&var_name);
                        let var_ident = format_ident!("{}", var_snake);
                        let elem_var = format_ident!("elem_{}", elem_idx);
                        let count_var = format_ident!("_count_bag_{}", elem_idx);
                        
                        elem_vars.push(elem_var.clone());
                        
                        // Generate: for (elem_var, _count_bag_N) in bag.iter()
                        extract_clauses.push(quote! {
                            for (#elem_var, #count_var) in #bag_var.iter(),
                        });
                        
                        // Add distinctness checks
                        for prev_elem_var in &elem_vars[..elem_idx] {
                            extract_clauses.push(quote! {
                                if &#elem_var != &#prev_elem_var,
                            });
                        }
                        
                        // Bind the variable
                        extract_clauses.push(quote! {
                            let #var_ident = #elem_var.clone(),
                        });
                        
                        bindings.insert(var_name, var_ident.clone());
                    }
                    _ => {
                        // Complex pattern - not supported yet
                        return None;
                    }
                }
            }
            
            Some(quote! {
                if let #category::#constructor(ref #bag_var) = #term_ident,
                if #bag_var.len() == #size_check,
                #(#extract_clauses)*
            })
        }
    }
}

/// Generate RHS construction code for collection pattern equations
/// Collection variables are bound as T (cloned from iterator), not &Box<T>
fn generate_collection_equation_rhs(expr: &Expr, bindings: &HashMap<String, Ident>, theory: &TheoryDef) -> TokenStream {
    match expr {
        Expr::Var(var) => {
            // Check if this is a constructor or a variable
            if is_constructor(var, theory) {
                // It's a nullary constructor
                let constructor_category = theory.terms.iter()
                    .find(|r| r.label == *var)
                    .map(|r| &r.category)
                    .expect("Constructor not found in theory");
                quote! { #constructor_category::#var }
            } else {
                // It's a variable - just clone it (it's already a T value)
                let var_name = var.to_string();
                if let Some(var_ident) = bindings.get(&var_name) {
                    quote! { #var_ident.clone() }
                } else {
                    // Unbound variable
                    quote! { #var }
                }
            }
        }
        _ => {
            // For other expressions, use the regular generator
            // (shouldn't happen for simple collection equations like (PPar {P}) == P)
            generate_equation_rhs(expr, bindings, theory, false)
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
                    // Variables are bound as &Box<T> from Apply pattern matching
                    if in_constructor {
                        // Inside constructor: just clone (keeps it as Box<T>)
                        quote! { #var_ident.clone() }
                    } else {
                        // Top-level: need to dereference to get the inner value
                        // Use .as_ref().clone() to go from &Box<T> to T
                        quote! { #var_ident.as_ref().clone() }
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
        Expr::CollectionPattern { elements, rest, .. } => {
            // Build a collection in RHS
            let elem_constructions: Vec<TokenStream> = elements.iter()
                .map(|e| generate_equation_rhs(e, bindings, theory, false))
                .collect();
            
            let coll_type = quote! { mettail_runtime::HashBag };
            
            if let Some(rest_var) = rest {
                // Merge rest with new elements
                if let Some(rest_binding) = bindings.get(&rest_var.to_string()) {
                    quote! {
                        {
                            let mut bag = (#rest_binding).clone();
                            #(bag.insert(#elem_constructions);)*
                            bag
                        }
                    }
                } else {
                    // Rest variable not bound - shouldn't happen
                    quote! {
                        {
                            let mut bag = #coll_type::new();
                            #(bag.insert(#elem_constructions);)*
                            bag
                        }
                    }
                }
            } else {
                // Just build from elements
                if in_constructor {
                    quote! {
                        Box::new({
                            let mut bag = #coll_type::new();
                            #(bag.insert(#elem_constructions);)*
                            bag
                        })
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

//=============================================================================
// NEW: Congruence-Driven Collection Rewriting
//=============================================================================

/// Generate congruence clauses for a collection congruence using projections
/// This is the new approach that generates clauses for both base rewrites and regular congruences
fn generate_new_collection_congruence_clauses(
    cong_idx: usize,
    cong_info: &crate::congruence_analysis::CollectionCongruenceInfo,
    base_patterns: &[Vec<crate::congruence_analysis::ElementPatternInfo>],
    theory: &TheoryDef,
) -> Vec<TokenStream> {
    let mut clauses = Vec::new();
    
    let rw_rel = format_ident!("rw_{}", cong_info.parent_category.to_string().to_lowercase());
    let parent_cat = &cong_info.parent_category;
    let constructor = &cong_info.constructor;
    let constructor_lower = format_ident!("{}", constructor.to_string().to_lowercase());
    let insert_helper = format_ident!("insert_into_{}", constructor_lower);
    
    // Find all base rewrites that involve this element category
    let base_rewrites = crate::congruence_analysis::find_base_rewrites_for_category(
        &cong_info.element_category, theory
    );
    
    // Find all regular congruences on this element category
    let regular_congruences = crate::congruence_analysis::find_regular_congruences_for_category(
        &cong_info.element_category, theory
    );
    
    // Generate clauses for base rewrites
    // Each base rewrite gets ONE clause that may join multiple projections
    for (base_idx, base_rule) in base_rewrites.iter().enumerate() {
        // Use the updated patterns with captures extracted during projection generation
        let element_patterns = if base_idx < base_patterns.len() {
            &base_patterns[base_idx]
        } else {
            continue;  // Skip if patterns weren't generated for this base rewrite
        };
        
        if element_patterns.is_empty() {
            continue;
        }
        
        // Generate ONE clause for this base rewrite, joining all its projections
        let clause = generate_joined_base_rewrite_clause(
            cong_idx,
            base_idx,
            cong_info,
            element_patterns,
            &base_rule.right,
            &rw_rel,
            parent_cat,
            constructor,
            &insert_helper,
            theory,
        );
        clauses.push(clause);
    }
    
    // Generate clauses for regular congruences
    for (reg_idx, reg_cong) in regular_congruences.iter().enumerate() {
        if let Some(pattern) = crate::congruence_analysis::extract_regular_congruence_pattern(reg_cong, theory) {
            let clause = generate_regular_congruence_clause(
                cong_idx,
                reg_idx,
                cong_info,
                &pattern,
                &rw_rel,
                parent_cat,
                constructor,
                &insert_helper,
            );
            clauses.push(clause);
        }
    }
    
    clauses
}

/// Generate congruence clause for a base rewrite with possibly multiple element patterns
/// For multi-element patterns (like communication), joins all projections
/// For single-element patterns (like drop-quote), uses one projection
fn generate_joined_base_rewrite_clause(
    cong_idx: usize,
    base_idx: usize,
    cong_info: &crate::congruence_analysis::CollectionCongruenceInfo,
    patterns: &[crate::congruence_analysis::ElementPatternInfo],
    rhs: &Expr,
    rw_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    insert_helper: &Ident,
    theory: &TheoryDef,
) -> TokenStream {
    use std::collections::{HashMap, HashSet};
    
    // First pass: identify shared variables (appear in multiple patterns)
    let mut var_pattern_counts: HashMap<String, Vec<usize>> = HashMap::new();
    for (pat_idx, pattern) in patterns.iter().enumerate() {
        for capture in &pattern.captures {
            var_pattern_counts.entry(capture.var_name.clone())
                .or_insert_with(Vec::new)
                .push(pat_idx);
        }
    }
    
    let shared_vars: HashSet<String> = var_pattern_counts.iter()
        .filter(|(_, patterns)| patterns.len() > 1)
        .map(|(name, _)| name.clone())
        .collect();
    
    // Generate projection joins for all patterns
    let mut projection_calls = Vec::new();
    let mut elem_vars = Vec::new();
    let mut all_capture_vars = Vec::new();
    let mut rest_vars = Vec::new();  // Track rest variables
    
    for (pat_idx, pattern) in patterns.iter().enumerate() {
        let rel_name = format_ident!(
            "{}_proj_c{}_b{}_p{}",
            pattern.constructor.to_string().to_lowercase(),
            cong_idx,
            base_idx,
            pat_idx
        );
        
        // Generate projection call arguments
        let mut proj_args = vec![quote! { parent }];
        let mut pattern_capture_vars = Vec::new();
        
        // Extract captures for all patterns (including nested ones)
        for capture in &pattern.captures {
            // Check if this is a rest variable (marked by field_idx == usize::MAX)
            let is_rest = capture.field_idx == usize::MAX;
            
            // For shared variables, use the same name across all patterns (for join)
            // For non-shared variables, use pattern-specific names
            let cap_name = if shared_vars.contains(&capture.var_name) {
                format_ident!("cap_{}", capture.var_name.to_lowercase())
            } else {
                format_ident!("cap_{}_p{}", capture.var_name.to_lowercase(), pat_idx)
            };
            proj_args.push(quote! { #cap_name });
            
            // Only add to capture_vars if it's not a rest variable
            // Rest variables are tracked separately for RHS reconstruction
            if !is_rest {
                pattern_capture_vars.push((cap_name.clone(), capture.clone()));
            } else {
                // Track rest variables separately
                let rest_var_name = syn::Ident::new(&capture.var_name, proc_macro2::Span::call_site());
                rest_vars.push((cap_name.clone(), rest_var_name));
            }
        }
        
        let elem_var = format_ident!("elem_{}", pat_idx);
        proj_args.push(quote! { #elem_var });
        elem_vars.push(elem_var);
        all_capture_vars.extend(pattern_capture_vars);
        
        // Add rest variable from pattern-level rest (if present and not already in captures)
        // This handles rest at the pattern's collection level (e.g., {A, B, ...rest})
        if let Some(rest_var) = &pattern.rest_var {
            // Check if this rest was already added via captures (nested rest)
            let already_added = rest_vars.iter().any(|(_, rv)| rv.to_string() == rest_var.to_string());
            if !already_added {
                let rest_ident = format_ident!("rest_{}_p{}", rest_var.to_string().to_lowercase(), pat_idx);
                proj_args.push(quote! { #rest_ident });
                rest_vars.push((rest_ident, rest_var.clone()));
            }
        }
        
        projection_calls.push(quote! {
            #rel_name(#(#proj_args),*)
        });
    }
    
    // For nested patterns, we now extract ALL captures via projection (using full pattern matching)
    // So we can use direct RHS reconstruction for all cases
    let rhs_term = generate_rhs_reconstruction(rhs, &all_capture_vars, &rest_vars, theory);
    let rhs_generation = quote! {
        let rhs_term = #rhs_term
    };
    
    // Generate element removal for all matched elements
    let elem_removal = if elem_vars.len() == 1 {
        let elem_var = &elem_vars[0];
        quote! {
            let remaining = {
                let mut b = bag.clone();
                b.remove(#elem_var);
                b
            }
        }
    } else {
        let removals = elem_vars.iter().map(|ev| quote! { b.remove(#ev); });
        quote! {
            let remaining = {
                let mut b = bag.clone();
                #(#removals)*
                b
            }
        }
    };
    
    quote! {
        #rw_rel(parent, result) <--
            #(#projection_calls),*,
            #rhs_generation,
            if let #parent_cat::#constructor(ref bag) = parent,
            #elem_removal,
            let result = #parent_cat::#constructor({
                let mut bag_result = remaining;
                #parent_cat::#insert_helper(&mut bag_result, rhs_term);
                bag_result
            });
    }
}

/// Helper to generate RHS reconstruction from captured variables
/// For congruence clauses, extracts elements from collection RHS
fn generate_rhs_reconstruction(
    rhs: &Expr,
    captures: &[(Ident, crate::congruence_analysis::CaptureInfo)],
    rest_vars: &[(Ident, Ident)],  // (rest_binding_ident, rest_var_name)
    theory: &TheoryDef,
) -> TokenStream {
    // Build a bindings map for RHS generator
    let mut bindings = std::collections::HashMap::new();
    for (var_ident, capture_info) in captures {
        // For rewrite RHS, bindings should just be the variable name
        bindings.insert(capture_info.var_name.clone(), quote! { #var_ident.clone() });
    }
    
    // Add rest variables to bindings (as HashBag)
    for (rest_ident, rest_var_name) in rest_vars {
        bindings.insert(rest_var_name.to_string(), quote! { #rest_ident.clone() });
    }
    
    // Special case: if RHS is a collection constructor with collection pattern inside,
    // extract just the elements (since insert_into_X will add them to the collection)
    if let Expr::Apply { constructor, args } = rhs {
        // Check if this has a collection argument
        for arg in args {
            if let Expr::CollectionPattern { elements, rest, .. } = arg {
                // This is a collection RHS like (PPar {(subst P x (NQuote Q))})
                // We need to generate code for the element(s), not the whole PPar
                
                if elements.len() == 1 && rest.is_none() {
                    // Single element, no rest - just generate that element
                    return crate::rewrite_gen::generate_ascent_rhs_inner(&elements[0], &bindings, theory);
                } else if elements.len() > 1 || rest.is_some() {
                    // Multiple elements or rest - need to handle merging
                    let element_terms: Vec<_> = elements.iter()
                        .map(|e| crate::rewrite_gen::generate_ascent_rhs_inner(e, &bindings, theory))
                        .collect();
                    
                    if let Some(rest_var) = rest {
                        // Has rest - need to merge
                        if let Some(rest_binding) = bindings.get(&rest_var.to_string()) {
                            // Generate: insert all elements into rest
                            return quote! {
                                {
                                    let mut bag = (#rest_binding).clone();
                                    #(bag.insert(#element_terms);)*
                                    bag
                                }
                            };
                        }
                    }
                    
                    // No rest - just the elements
                    // For communication, the RHS should be a single element
                    if element_terms.len() == 1 {
                        return element_terms[0].clone();
                    }
                }
            }
        }
    }
    
    // Default: use rewrite RHS generation (handles Subst correctly with .substitute_X methods)
    crate::rewrite_gen::generate_ascent_rhs_inner(rhs, &bindings, theory)
}

/// Generate congruence clause for a regular congruence
/// Uses the projection and recursively calls rw_rel on the body
fn generate_regular_congruence_clause(
    cong_idx: usize,
    reg_idx: usize,
    cong_info: &crate::congruence_analysis::CollectionCongruenceInfo,
    pattern: &crate::congruence_analysis::RegularCongruencePattern,
    rw_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    insert_helper: &Ident,
) -> TokenStream {
    let rel_name = format_ident!(
        "{}_proj_c{}_r{}",
        pattern.constructor.to_string().to_lowercase(),
        cong_idx,
        reg_idx
    );
    
    let elem_rw_rel = format_ident!("rw_{}", pattern.category.to_string().to_lowercase());
    let elem_constructor = &pattern.constructor;
    let elem_cat = &pattern.category;
    
    // Generate reconstruction based on whether it's a binding constructor
    let reconstruction = if pattern.is_binding {
        // IMPORTANT: Use from_parts_unsafe to avoid rebinding (which would change variable IDs)
        // The body still has Bound variables, so we preserve them
        quote! {
            let scope_tmp = mettail_runtime::Scope::from_parts_unsafe(binder_var.clone(), Box::new(body_rewritten.clone())),
            let rewritten = #elem_cat::#elem_constructor(scope_tmp)
        }
    } else {
        // For non-binding, we need to reconstruct with the rewritten field
        // This is a simplification - in reality we'd need all fields
        quote! {
            let rewritten = body_rewritten.clone()
        }
    };
    
    let proj_args = if pattern.is_binding {
        quote! { (parent, binder_var, body, elem) }
    } else {
        quote! { (parent, body, elem) }
    };
    
    quote! {
        #rw_rel(parent, result) <--
            #rel_name #proj_args,
            #elem_rw_rel(body, body_rewritten),
            if let #parent_cat::#constructor(ref bag) = parent,
            let remaining = {
                let mut b = bag.clone();
                b.remove(elem);
                b
            },
            #reconstruction,
            let result = #parent_cat::#constructor({
                let mut bag = remaining;
                #parent_cat::#insert_helper(&mut bag, rewritten);
                bag
            });
    }
}

