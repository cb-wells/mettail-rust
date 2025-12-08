use super::analysis::{
    extract_element_patterns_from_base_rewrite, extract_variable_categories,
    find_base_rewrites_for_category, CaptureInfo, CollectionCongruenceInfo, ElementPatternInfo,
};
use super::regular::{
    extract_regular_congruence_pattern, find_regular_congruences_for_category,
    RegularCongruencePattern,
};
use crate::ast::{Expr, GrammarItem, TheoryDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Generate projection relation declaration
/// pnew_direct_congruence_proj(Proc, Binder<String>, Proc)
pub fn generate_binding_proj_declaration(
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
pub fn generate_binding_proj_population(
    proj_rel: &Ident,
    parent_cat: &Ident,
    constructor: &Ident,
    _body_cat: &Ident,
    rule: &crate::ast::GrammarRule,
    _binder_idx: usize,
) -> TokenStream {
    let cat_lower = format_ident!("{}", parent_cat.to_string().to_lowercase());

    // Count non-terminal fields
    let field_count = rule
        .items
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
    let regular_congruences =
        find_regular_congruences_for_category(&cong_info.element_category, theory);

    // Generate projections for base rewrites
    let mut updated_base_patterns: Vec<Vec<ElementPatternInfo>> = Vec::new();

    for (base_idx, base_rule) in base_rewrites.iter().enumerate() {
        // Extract element patterns from the base rewrite's LHS
        let element_patterns =
            extract_element_patterns_from_base_rewrite(&base_rule.left, cong_info, theory);

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
                &base_rule.left, // Pass the full LHS for nested pattern extraction
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
                cong_idx, reg_idx, cong_info, &pattern, theory,
            );
            projections.extend(proj);
        }
    }

    (projections, updated_base_patterns)
}

/// Generate projection relation and population rule for a base rewrite pattern
/// Returns (generated code, updated pattern with captures)
fn generate_base_rewrite_projection(
    cong_idx: usize,
    base_idx: usize,
    pat_idx: usize,
    cong_info: &CollectionCongruenceInfo,
    pattern: &ElementPatternInfo,
    base_lhs: &Expr, // Full LHS of the base rewrite for nested pattern matching
    theory: &TheoryDef,
) -> (Vec<TokenStream>, ElementPatternInfo) {
    use crate::ascent::rewrites;
    use std::collections::HashMap;

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
        let duplicate_vars = std::collections::HashSet::new(); // No duplicates in single pattern

        // Generate pattern matching clauses for the element expression
        let elem_ident = format_ident!("elem");
        rewrites::generate_ascent_pattern(
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

        // Also build a map for RHS reconstruction and detect rest variables
        let mut rhs_bindings = HashMap::new();
        let mut nested_rest_vars = Vec::new(); // Rest variables from nested patterns

        for (var_name, binding_ts) in &bindings {
            let var_ident = format_ident!("{}", var_name.to_lowercase());
            // Try to get category from variable_categories (for duplicates) or infer from var_categories
            let cat = variable_categories
                .get(var_name)
                .or_else(|| var_categories.get(var_name));

            if let Some(cat) = cat {
                // This is a regular capture variable
                field_types.push(quote! { #cat });
                rel_fields.push(quote! { #var_ident.clone() });
                binding_vars.push((var_ident.clone(), binding_ts.clone()));
            } else {
                // No category found - this is a rest variable from nested pattern matching
                // Add it to projection signature as HashBag<ElementCategory>
                field_types.push(quote! { mettail_runtime::HashBag<#elem_cat> });
                rel_fields.push(quote! { #var_ident.clone() });
                // For rest variables, the binding from pattern matching is just the identifier
                // We need to add .clone() since HashBag doesn't implement Copy
                let rest_binding_with_clone = quote! { (#binding_ts).clone() };
                binding_vars.push((var_ident.clone(), binding_ts.clone()));
                nested_rest_vars.push(var_name.clone());

                // For RHS reconstruction, use the cloned version
                rhs_bindings.insert(var_name.clone(), rest_binding_with_clone);
                continue; // Skip the general rhs_bindings insert below
            }

            // For RHS reconstruction, include ALL variables (including rest)
            rhs_bindings.insert(var_name.clone(), quote! { #var_ident.clone() });
        }
        field_types.push(quote! { #elem_cat });
        rel_fields.push(quote! { elem.clone() });

        // Add rest bag to signature if rest variable present
        if let Some(rest_var) = &pattern.rest_var {
            field_types.push(quote! { mettail_runtime::HashBag<#elem_cat> });
            let rest_ident = format_ident!("rest_{}", rest_var.to_string().to_lowercase());
            rel_fields.push(quote! { #rest_ident.clone() });
        }

        let rel_decl = quote! {
            relation #rel_name(#(#field_types),*);
        };

        // Generate let bindings for captured variables (excluding rest)
        let rest_var_name = pattern.rest_var.as_ref().map(|v| v.to_string());
        let capture_bindings: Vec<_> = binding_vars
            .iter()
            .filter(|(var_ident, _)| {
                // Exclude rest variable from capture bindings
                Some(var_ident.to_string().as_str())
                    != rest_var_name
                        .as_deref()
                        .map(|s| s.to_lowercase())
                        .as_deref()
            })
            .map(|(var_ident, binding)| {
                quote! { let #var_ident = #binding }
            })
            .collect();

        // Generate rest bag computation if needed
        let rest_computation = if let Some(rest_var) = &pattern.rest_var {
            let rest_ident = format_ident!("rest_{}", rest_var.to_string().to_lowercase());
            quote! {
                let #rest_ident = {
                    let mut b = bag_field.clone();
                    b.remove(elem);
                    b
                },
            }
        } else {
            quote! {}
        };

        let population_rule = quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                #(#clauses),*,
                #(#capture_bindings),*
                #rest_computation;
        };

        // Build updated pattern with extracted captures
        // Use the order from bindings to ensure consistent ordering with projection signature
        // Include both regular captures and rest variables (rest variables have no category in variable_categories)
        let updated_captures: Vec<CaptureInfo> = bindings
            .keys()
            .filter_map(|var_name| {
                // Try to get category - if found, it's a regular capture; if not, it's a rest variable
                let cat_opt = variable_categories
                    .get(var_name)
                    .or_else(|| var_categories.get(var_name));

                if let Some(cat) = cat_opt {
                    // Regular capture
                    Some(CaptureInfo {
                        var_name: var_name.clone(),
                        category: cat.clone(),
                        field_idx: 0,     // Not used for nested patterns
                        is_binder: false, // TODO: detect binders properly
                    })
                } else {
                    // Rest variable - include it with elem_cat as placeholder (actual type is HashBag<elem_cat>)
                    // Use field_idx = usize::MAX as a marker that this is a rest variable
                    Some(CaptureInfo {
                        var_name: var_name.clone(),
                        category: elem_cat.clone(), // Placeholder - actual type is HashBag<elem_cat>
                        field_idx: usize::MAX,      // Marker for rest variable
                        is_binder: false,
                    })
                }
            })
            .collect();

        let mut updated_pattern = pattern.clone();
        updated_pattern.captures = updated_captures;

        result.push(rel_decl);
        result.push(population_rule);
        return (result, updated_pattern);
    }

    // Build relation signature: (Parent, Capture1, Capture2, ..., Element, Rest?)
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

    // Add rest bag to signature if rest variable present
    if pattern.rest_var.is_some() {
        field_types.push(quote! { mettail_runtime::HashBag<#elem_cat> });
    }

    // Generate relation declaration
    let rel_decl = quote! {
        relation #rel_name(#(#field_types),*);
    };

    // Generate population rule
    let parent_cat_lower =
        format_ident!("{}", cong_info.parent_category.to_string().to_lowercase());
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

    // Add rest bag to relation tuple if present
    if let Some(rest_var) = &pattern.rest_var {
        let rest_ident = format_ident!("rest_{}", rest_var.to_string().to_lowercase());
        rel_fields.push(quote! { #rest_ident.clone() });
    }

    // Generate rest bag computation if needed
    let rest_computation = if let Some(rest_var) = &pattern.rest_var {
        let rest_ident = format_ident!("rest_{}", rest_var.to_string().to_lowercase());
        quote! {
            let #rest_ident = {
                let mut b = bag_field.clone();
                b.remove(elem);
                b
            },
        }
    } else {
        quote! {}
    };

    // Generate the population rule with conditional capture bindings
    let population_rule = if pattern.captures.is_empty() {
        // No captures - simpler pattern without bindings
        quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                if let #elem_cat::#elem_constructor(#field_patterns) = elem,
                #rest_computation;
        }
    } else {
        // Has captures - include capture bindings
        quote! {
            #rel_name(#(#rel_fields),*) <--
                #parent_cat_lower(parent),
                if let #parent_cat::#collection_constructor(ref bag_field) = parent,
                for (elem, _count) in bag_field.iter(),
                if let #elem_cat::#elem_constructor(#field_patterns) = elem,
                #capture_bindings
                #rest_computation;
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
        field_to_captures
            .entry(capture.field_idx)
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
            // This is a scope field - access unsafe fields directly to preserve bound variables
            let binder_name = format_ident!("cap_{}", binder_capture.var_name.to_lowercase());

            // Find the body capture (should be at the same field index, non-binder)
            if let Some(body_capture) = captures_for_field.iter().find(|c| !c.is_binder) {
                let body_name = format_ident!("cap_{}", body_capture.var_name.to_lowercase());
                // For rewrite pattern matching, use unbind() to get fresh free variables
                // This prevents bound variables from escaping their scope
                capture_bindings.push(quote! {
                    let (binder_tmp, body_tmp) = (* #field_name).clone().unbind(),
                    let #binder_name = binder_tmp,
                    let #body_name = *body_tmp
                });
            } else {
                // Only binder, no body capture
                capture_bindings.push(quote! {
                    let #binder_name = (* #field_name).inner().unsafe_pattern.clone()
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

    let grammar_rule = theory
        .terms
        .iter()
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
        // Binding constructor: extract scope and access unsafe fields directly to preserve bound variables
        quote! {
            if let #elem_cat::#elem_constructor(ref scope) = elem,
            let (binder_var, body_box) = scope.clone().unbind(),
            let rewrite_field = *body_box
        }
    } else {
        // Non-binding constructor: directly extract the rewrite field
        let field_idx = pattern.rewrite_field_idx;
        // Generate field pattern with the rewrite field
        let mut field_pats = Vec::new();
        for i in 0..grammar_rule
            .items
            .iter()
            .filter(|item| !matches!(item, GrammarItem::Terminal(_)))
            .count()
        {
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

    let rel_fields: Vec<TokenStream> = if pattern.is_binding {
        vec![
            quote! { parent.clone() },
            quote! { binder_var.clone() },
            quote! { rewrite_field.clone() },
            quote! { elem.clone() },
        ]
    } else {
        vec![
            quote! { parent.clone() },
            quote! { rewrite_field.clone() },
            quote! { elem.clone() },
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
