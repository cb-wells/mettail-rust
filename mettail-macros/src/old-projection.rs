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
        
        // if shared_variables.is_empty() {
        //     // No shared variables - no need for indexed projection
        //     return None;
        // }
        
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
