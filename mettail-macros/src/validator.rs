use crate::ast::{TheoryDef, GrammarItem, Expr, Equation, RewriteRule};
use crate::typechecker::TypeChecker;
use crate::error::ValidationError;
use std::collections::HashSet;

pub fn validate_theory(theory: &TheoryDef) -> Result<(), ValidationError> {
    // Build set of exported categories
    let exported: HashSet<_> = theory.exports
        .iter()
        .map(|e| e.name.to_string())
        .collect();
    
    // Build set of all defined categories (result types from all rules)
    let defined: HashSet<_> = theory.terms
        .iter()
        .map(|r| r.category.to_string())
        .collect();
    
    // Check each rule
    for rule in &theory.terms {
        // Check that the rule's category is exported
        // (We require that constructor result types are exported)
        let cat_name = rule.category.to_string();
        if !exported.contains(&cat_name) {
            return Err(ValidationError::CategoryNotExported {
                category: cat_name,
                rule: rule.label.to_string(),
                span: rule.category.span(),
            });
        }
        
        // Check that all non-terminal items reference valid categories
        // Valid means: exported OR defined as a result type OR built-in (like Var)
        for item in &rule.items {
            match item {
                GrammarItem::NonTerminal(ident) => {
                    let ref_name = ident.to_string();
                    // Built-in types are always valid
                    if ref_name == "Var" {
                        continue;
                    }
                    // Must be either exported or defined (or both)
                    if !exported.contains(&ref_name) && !defined.contains(&ref_name) {
                        return Err(ValidationError::UndefinedCategoryReference {
                            category: ref_name,
                            rule: rule.label.to_string(),
                            span: ident.span(),
                        });
                    }
                }
                GrammarItem::Binder { category } => {
                    let ref_name = category.to_string();
                    // Built-in types are always valid
                    if ref_name == "Var" {
                        continue;
                    }
                    // Binder categories must also be valid
                    if !exported.contains(&ref_name) && !defined.contains(&ref_name) {
                        return Err(ValidationError::UndefinedCategoryReference {
                            category: ref_name,
                            rule: rule.label.to_string(),
                            span: category.span(),
                        });
                    }
                }
                _ => {}
            }
        }
    }
    
    // Validate expressions in equations
    for eq in theory.equations.iter() {
        validate_expr(&eq.left, &theory)?;
        validate_expr(&eq.right, &theory)?;
        
        // Validate freshness conditions
        validate_equation_freshness(eq)?;
    }
    
    // Validate expressions in rewrites
    for rw in theory.rewrites.iter() {
        validate_expr(&rw.left, &theory)?;
        validate_expr(&rw.right, &theory)?;
        
        // Validate freshness conditions
        validate_rewrite_freshness(rw)?;
    }
    
    // Type-check equations
    let type_checker = TypeChecker::new(theory);
    type_checker.validate_equations(&theory.equations)?;
    
    // Type-check rewrite rules
    type_checker.validate_rewrites(&theory.rewrites)?;
    
    Ok(())
}

fn validate_expr(expr: &Expr, theory: &TheoryDef) -> Result<(), ValidationError> {
    match expr {
        Expr::Var(_) => Ok(()), // Variables are always OK
        Expr::Apply { constructor, args } => {
            // Check that constructor references a known rule
            let constructor_name = constructor.to_string();
            let found = theory.terms.iter().any(|r| r.label.to_string() == constructor_name);
            
            if !found {
                return Err(ValidationError::UnknownConstructor {
                    name: constructor_name,
                    span: constructor.span(),
                });
            }
            
            // Recursively validate arguments
            for arg in args {
                validate_expr(arg, theory)?;
            }
            
            Ok(())
        }
        Expr::Subst { term, var: _, replacement } => {
            // Validate the term being substituted into
            validate_expr(term, theory)?;
            
            // Validate the replacement expression
            validate_expr(replacement, theory)?;
            
            // var is just an identifier, no validation needed
            Ok(())
        }
    }
}

/// Validate freshness conditions in an equation
/// 
/// Checks that:
/// 1. Variables in freshness conditions actually appear in the equation
/// 2. The freshness constraint is semantically meaningful
/// 
/// Freshness condition `x # Q` means "x does not appear free in Q"
fn validate_equation_freshness(eq: &Equation) -> Result<(), ValidationError> {
    // Collect all variables that appear in the equation
    let mut equation_vars = HashSet::new();
    collect_vars(&eq.left, &mut equation_vars);
    collect_vars(&eq.right, &mut equation_vars);
    
    // Validate each freshness condition
    for cond in &eq.conditions {
        let var_name = cond.var.to_string();
        let term_name = cond.term.to_string();
        
        // Check that the variable appears in the equation
        if !equation_vars.contains(&var_name) {
            return Err(ValidationError::FreshnessVariableNotInEquation {
                var: var_name,
                span: cond.var.span(),
            });
        }
        
        // Check that the term variable appears in the equation
        if !equation_vars.contains(&term_name) {
            return Err(ValidationError::FreshnessTermNotInEquation {
                var: var_name,
                term: term_name,
                span: cond.term.span(),
            });
        }
        
        // Check that x does not appear free in term
        // For now, we do a simple check: if term is a variable, x != term
        // More sophisticated checking will be added with scoping
        if var_name == term_name {
            return Err(ValidationError::FreshnessSelfReference {
                var: var_name,
                span: cond.var.span(),
            });
        }
    }
    
    Ok(())
}

/// Validate freshness conditions in a rewrite rule
/// Same logic as equations
fn validate_rewrite_freshness(rw: &RewriteRule) -> Result<(), ValidationError> {
    // Collect all variables that appear in the rewrite
    let mut rewrite_vars = HashSet::new();
    collect_vars(&rw.left, &mut rewrite_vars);
    collect_vars(&rw.right, &mut rewrite_vars);
    
    // Validate each freshness condition
    for cond in &rw.conditions {
        let var_name = cond.var.to_string();
        let term_name = cond.term.to_string();
        
        // Check that the variable appears in the rewrite
        if !rewrite_vars.contains(&var_name) {
            return Err(ValidationError::FreshnessVariableNotInEquation {
                var: var_name,
                span: cond.var.span(),
            });
        }
        
        // Check that the term variable appears in the rewrite
        if !rewrite_vars.contains(&term_name) {
            return Err(ValidationError::FreshnessTermNotInEquation {
                var: var_name,
                term: term_name,
                span: cond.term.span(),
            });
        }
        
        // Check that x != term (can't be fresh in itself)
        if var_name == term_name {
            return Err(ValidationError::FreshnessSelfReference {
                var: var_name,
                span: cond.var.span(),
            });
        }
    }
    
    Ok(())
}

/// Collect all variable names from an expression
fn collect_vars(expr: &Expr, vars: &mut HashSet<String>) {
    match expr {
        Expr::Var(ident) => {
            vars.insert(ident.to_string());
        }
        Expr::Apply { args, .. } => {
            for arg in args {
                collect_vars(arg, vars);
            }
        }
        Expr::Subst { term, var, replacement } => {
            // Collect from the term being substituted into
            collect_vars(term, vars);
            // The substitution variable itself
            vars.insert(var.to_string());
            // Collect from the replacement
            collect_vars(replacement, vars);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use syn::parse_quote;
    
    #[test]
    fn test_valid_theory() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Elem) }
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(Zero),
                    category: parse_quote!(Elem),
                    items: vec![GrammarItem::Terminal("0".to_string())],
                    bindings: vec![],
                }
            ],
            equations: vec![],
            rewrites: vec![],
        };
        
        assert!(validate_theory(&theory).is_ok());
    }
    
    #[test]
    fn test_invalid_category() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Elem) }
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(Quote),
                    category: parse_quote!(Name),  // Not exported!
                    items: vec![
                        GrammarItem::Terminal("@".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Elem)),
                    ],
                    bindings: vec![],
                }
            ],
            equations: vec![],
            rewrites: vec![],
        };
        
        assert!(validate_theory(&theory).is_err());
    }
    
    #[test]
    fn test_invalid_reference() {
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Elem) }
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(Quote),
                    category: parse_quote!(Elem),
                    items: vec![
                        GrammarItem::Terminal("@".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Name)),  // Not exported!
                    ],
                    bindings: vec![],
                }
            ],
            equations: vec![],
            rewrites: vec![],
        };
        
        let result = validate_theory(&theory);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message();
        assert!(err_msg.contains("Name"));
    }
    
    #[test]
    fn test_freshness_valid() {
        // Valid freshness condition: if x # P then (@*(new x P)) == P
        // This is type-correct: both sides have type Proc
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Name) },
                Export { name: parse_quote!(Proc) },
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(NQuote),
                    category: parse_quote!(Name),
                    items: vec![
                        GrammarItem::Terminal("@".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Proc)),
                    ],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(PDrop),
                    category: parse_quote!(Proc),
                    items: vec![
                        GrammarItem::Terminal("*".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Name)),
                    ],
                    bindings: vec![],
                },
                GrammarRule {
                    label: parse_quote!(PNew),
                    category: parse_quote!(Proc),
                    items: vec![
                        GrammarItem::Terminal("new".to_string()),
                        GrammarItem::NonTerminal(parse_quote!(Name)),
                        GrammarItem::NonTerminal(parse_quote!(Proc)),
                    ],  
                    bindings: vec![],
                },
            ],
            equations: vec![
                Equation {
                    conditions: vec![
                        FreshnessCondition {
                            var: parse_quote!(x),
                            term: parse_quote!(P),
                        }
                    ],
                    // (PDrop (NQuote (PNew x P)))  -- has type Proc
                    left: Expr::Apply {
                        constructor: parse_quote!(PDrop),
                        args: vec![
                            Expr::Apply {
                                constructor: parse_quote!(NQuote),
                                args: vec![
                                    Expr::Apply {
                                        constructor: parse_quote!(PNew),
                                        args: vec![
                                            Expr::Var(parse_quote!(x)),
                                            Expr::Var(parse_quote!(P)),
                                        ],
                                    }
                                ],
                            }
                        ],
                    },
                    // P  -- has type Proc
                    right: Expr::Var(parse_quote!(P)),
                }
            ],
            rewrites: vec![],
        };
        
        // Should pass - x and P both appear in equation, types match
        let result = validate_theory(&theory);
        if let Err(e) = &result {
            eprintln!("Validation error: {}", e);
        }
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_freshness_variable_not_in_equation() {
        // Invalid: freshness variable doesn't appear in equation
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Name) },
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(NZero),
                    category: parse_quote!(Name),
                    items: vec![GrammarItem::Terminal("@0".to_string())],
                    bindings: vec![],
                },
            ],
            equations: vec![
                Equation {
                    conditions: vec![
                        FreshnessCondition {
                            var: parse_quote!(x),  // x doesn't appear in equation!
                            term: parse_quote!(Q),
                        }
                    ],
                    // (NZero) == (NZero) - no variables
                    left: Expr::Apply {
                        constructor: parse_quote!(NZero),
                        args: vec![],
                    },
                    right: Expr::Apply {
                        constructor: parse_quote!(NZero),
                        args: vec![],
                    },
                }
            ],
            rewrites: vec![],
        };
        
        let result = validate_theory(&theory);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message();
        assert!(err_msg.contains("does not appear in equation"));
    }
    
    #[test]
    fn test_freshness_self_reference() {
        // Invalid: x # x (variable fresh in itself)
        let theory = TheoryDef {
            name: parse_quote!(Test),
            params: vec![],
            exports: vec![
                Export { name: parse_quote!(Name) },
            ],
            terms: vec![
                GrammarRule {
                    label: parse_quote!(NVar),
                    category: parse_quote!(Name),
                    items: vec![GrammarItem::Terminal("var".to_string())],
                    bindings: vec![],
                },
            ],
            equations: vec![
                Equation {
                    conditions: vec![
                        FreshnessCondition {
                            var: parse_quote!(x),
                            term: parse_quote!(x),  // x # x is invalid
                        }
                    ],
                    // x == (NVar)
                    left: Expr::Var(parse_quote!(x)),
                    right: Expr::Apply {
                        constructor: parse_quote!(NVar),
                        args: vec![],
                    },
                }
            ],
            rewrites: vec![],
        };
        
        let result = validate_theory(&theory);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().message();
        assert!(err_msg.contains("cannot be fresh in itself"));
    }
}

