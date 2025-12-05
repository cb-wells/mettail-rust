use crate::ast::TheoryDef;
use super::projections::{generate_binding_proj_declaration, generate_binding_proj_population};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;

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
pub fn generate_binding_congruence(
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
            let result = #parent_cat::#constructor(scope_tmp).normalize();
    }
}

/// Generate projection-based binding congruence
/// For: if S => T then (PNew x S) => (PNew x T)
/// Generates:
///   1. Projection relation: pnew_direct_congruence_proj(parent, binder, body)
///   2. Population rule: extracts parent, unbinds, projects body
///   3. Congruence clause: joins projection with rw_proc(body, _)
pub fn generate_projection_based_binding_congruence(
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

