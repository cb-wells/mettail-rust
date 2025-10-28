use syn::{Ident, Token, parse::{Parse, ParseStream}, Result as SynResult, Type};

/// Top-level theory definition
/// theory! { name: Foo, params: ..., exports { ... }, terms { ... }, equations { ... }, rewrites { ... } }
pub struct TheoryDef {
    pub name: Ident,
    pub params: Vec<TheoryParam>,
    pub exports: Vec<Export>,
    pub terms: Vec<GrammarRule>,
    pub equations: Vec<Equation>,
    pub rewrites: Vec<RewriteRule>,
}

/// Theory parameter (for generic theories)
/// params: (cm: CommutativeMonoid)
pub struct TheoryParam {
    pub name: Ident,
    pub ty: Type,
}

/// Equation with optional freshness conditions
/// if x # Q then (LHS) == (RHS)
pub struct Equation {
    pub conditions: Vec<FreshnessCondition>,
    pub left: Expr,
    pub right: Expr,
}

/// Freshness condition: x # Term means x is fresh in Term
pub struct FreshnessCondition {
    pub var: Ident,
    pub term: Ident,
}

/// Rewrite rule with optional freshness conditions
/// (LHS) => (RHS) or if x # Q then (LHS) => (RHS)
pub struct RewriteRule {
    pub conditions: Vec<FreshnessCondition>,
    pub left: Expr,
    pub right: Expr,
}

/// Expression in equations (AST patterns)
#[derive(Clone, Debug)]
pub enum Expr {
    Var(Ident),
    Apply {
        constructor: Ident,
        args: Vec<Expr>,
    },
    /// Substitution: subst(term, var, replacement)
    /// Represents term[replacement/var] - substitute replacement for var in term
    Subst {
        term: Box<Expr>,
        var: Ident,
        replacement: Box<Expr>,
    },
}

/// Export: just a category name
/// exports { Elem; Name; }
pub struct Export {
    pub name: Ident,
}

/// Grammar rule
/// Label . Category ::= Item Item Item ;
pub struct GrammarRule {
    pub label: Ident,
    pub category: Ident,
    pub items: Vec<GrammarItem>,
    /// Binding structure: (binder_index, vec![body_indices])
    /// e.g., (0, vec![1]) means item 0 binds in item 1
    pub bindings: Vec<(usize, Vec<usize>)>,
}

/// Item in a grammar rule
#[derive(Debug, Clone, PartialEq)]
pub enum GrammarItem {
    Terminal(String),      // "0"
    NonTerminal(Ident),    // Elem
    /// Binder: <Category> indicates this position binds a variable
    /// The bound variable is used in subsequent items
    Binder { category: Ident },  // <Name>
}

// Implement Parse for TheoryDef
impl Parse for TheoryDef {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse: name: Identifier
        let name_kw = input.parse::<Ident>()?;
        if name_kw != "name" {
            return Err(syn::Error::new(name_kw.span(), "expected 'name'"));
        }
        let _ = input.parse::<Token![:]>()?;
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>()?;
        
        // Parse: params: (...) (optional)
        let params = if input.peek(Ident) {
            let lookahead = input.fork().parse::<Ident>()?;
            if lookahead == "params" {
                parse_params(input)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Parse: exports { ... }
        let exports = if input.peek(Ident) {
            let lookahead = input.fork().parse::<Ident>()?;
            if lookahead == "exports" {
                parse_exports(input)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Parse: terms { ... }
        let terms = if input.peek(Ident) {
            let lookahead = input.fork().parse::<Ident>()?;
            if lookahead == "terms" {
                parse_terms(input)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Parse: equations { ... }
        let equations = if input.peek(Ident) {
            let lookahead = input.fork().parse::<Ident>()?;
            if lookahead == "equations" {
                parse_equations(input)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Parse: rewrites { ... }
        let rewrites = if input.peek(Ident) {
            let lookahead = input.fork().parse::<Ident>()?;
            if lookahead == "rewrites" {
                parse_rewrites(input)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        Ok(TheoryDef {
            name,
            params,
            exports,
            terms,
            equations,
            rewrites,
        })
    }
}

fn parse_params(input: ParseStream) -> SynResult<Vec<TheoryParam>> {
    let params_ident = input.parse::<Ident>()?;
    if params_ident != "params" {
        return Err(syn::Error::new(params_ident.span(), "expected 'params'"));
    }
    
    let _ = input.parse::<Token![:]>()?;
    
    let content;
    syn::parenthesized!(content in input);
    
    let mut params = Vec::new();
    while !content.is_empty() {
        let name = content.parse::<Ident>()?;
        let _ = content.parse::<Token![:]>()?;
        let ty = content.parse::<Type>()?;
        
        params.push(TheoryParam { name, ty });
        
        if content.peek(Token![,]) {
            let _ = content.parse::<Token![,]>()?;
        }
    }
    
    // Optional comma after closing paren
    if input.peek(Token![,]) {
        let _ = input.parse::<Token![,]>()?;
    }
    
    Ok(params)
}

fn parse_exports(input: ParseStream) -> SynResult<Vec<Export>> {
    let exports_ident = input.parse::<Ident>()?;
    if exports_ident != "exports" {
        return Err(syn::Error::new(exports_ident.span(), "expected 'exports'"));
    }
    
    let content;
    syn::braced!(content in input);
    
    let mut exports = Vec::new();
    while !content.is_empty() {
        let name = content.parse::<Ident>()?;
        exports.push(Export { name });
        
        if content.peek(Token![;]) {
            let _ = content.parse::<Token![;]>()?;
        }
    }
    
    // Optional comma after closing brace
    if input.peek(Token![,]) {
        let _ = input.parse::<Token![,]>()?;
    }
    
    Ok(exports)
}

fn parse_terms(input: ParseStream) -> SynResult<Vec<GrammarRule>> {
    let terms_ident = input.parse::<Ident>()?;
    if terms_ident != "terms" {
        return Err(syn::Error::new(terms_ident.span(), "expected 'terms'"));
    }
    
    let content;
    syn::braced!(content in input);
    
    let mut rules = Vec::new();
    while !content.is_empty() {
        rules.push(parse_grammar_rule(&content)?);
    }
    
    //Optional comma after closing brace
    if input.peek(Token![,]) {
        let _ = input.parse::<Token![,]>()?;
    }
    
    Ok(rules)
}

fn parse_grammar_rule(input: ParseStream) -> SynResult<GrammarRule> {
    // Parse: Label . Category ::= ...
    let label = input.parse::<Ident>()?;
    let _ = input.parse::<Token![.]>()?;
    let category = input.parse::<Ident>()?;
    
    // Parse ::= (as two colons followed by equals)
    let _ = input.parse::<Token![::]>()?;
    let _ = input.parse::<Token![=]>()?;
    
    // Parse items until semicolon
    let mut items = Vec::new();
    while !input.peek(Token![;]) {
        if input.peek(syn::LitStr) {
            // Terminal: string literal
            let lit = input.parse::<syn::LitStr>()?;
            items.push(GrammarItem::Terminal(lit.value()));
        } else if input.peek(Token![<]) {
            // Binder: <Category>
            let _ = input.parse::<Token![<]>()?;
            let cat = input.parse::<Ident>()?;
            let _ = input.parse::<Token![>]>()?;
            items.push(GrammarItem::Binder { category: cat });
        } else {
            // NonTerminal: identifier
            let ident = input.parse::<Ident>()?;
            items.push(GrammarItem::NonTerminal(ident));
        }
    }
    
    let _ = input.parse::<Token![;]>()?;
    
    // Infer binding structure: each Binder binds in the next NonTerminal
    let bindings = infer_bindings(&items);
    
    Ok(GrammarRule {
        label,
        category,
        items,
        bindings,
    })
}

/// Infer binding structure from items
/// Each Binder at position i binds in the next NonTerminal/Binder at position j > i
fn infer_bindings(items: &[GrammarItem]) -> Vec<(usize, Vec<usize>)> {
    let mut bindings = Vec::new();
    
    for (i, item) in items.iter().enumerate() {
        if matches!(item, GrammarItem::Binder { .. }) {
            // Find the next non-terminal item(s) that this binder binds in
            let mut bound_indices = Vec::new();
            
            for (j, next_item) in items.iter().enumerate().skip(i + 1) {
                match next_item {
                    GrammarItem::NonTerminal(_) | GrammarItem::Binder { .. } => {
                        bound_indices.push(j);
                        break; // For now, bind only in the immediately following item
                    }
                    GrammarItem::Terminal(_) => continue,
                }
            }
            
            if !bound_indices.is_empty() {
                bindings.push((i, bound_indices));
            }
        }
    }
    
    bindings
}

fn parse_equations(input: ParseStream) -> SynResult<Vec<Equation>> {
    let eq_ident = input.parse::<Ident>()?;
    if eq_ident != "equations" {
        return Err(syn::Error::new(eq_ident.span(), "expected 'equations'"));
    }
    
    let content;
    syn::braced!(content in input);
    
    let mut equations = Vec::new();
    while !content.is_empty() {
        equations.push(parse_equation(&content)?);
    }
    
    // Optional comma after closing brace
    if input.peek(Token![,]) {
        let _ = input.parse::<Token![,]>()?;
    }
    
    Ok(equations)
}

fn parse_equation(input: ParseStream) -> SynResult<Equation> {
    // Parse optional freshness conditions: if x # Q then
    let mut conditions = Vec::new();
    
    if input.peek(Token![if]) {
        let _ = input.parse::<Token![if]>()?;
        
        // Parse one or more freshness conditions
        loop {
            let var = input.parse::<Ident>()?;
            let _ = input.parse::<Token![#]>()?;
            let term = input.parse::<Ident>()?;
            
            conditions.push(FreshnessCondition { var, term });
            
            // Check for 'then' or continue with more conditions
            if input.peek(Ident) {
                let lookahead = input.fork().parse::<Ident>()?;
                if lookahead == "then" {
                    let _ = input.parse::<Ident>()?; // consume 'then'
                    break;
                }
            }
            
            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
                // Continue parsing more conditions
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "expected 'then' or ',' after freshness condition"
                ));
            }
        }
    }
    
    // Parse left-hand side
    let left = parse_expr(input)?;
    
    // Parse ==
    let _ = input.parse::<Token![==]>()?;
    
    // Parse right-hand side
    let right = parse_expr(input)?;
    
    // Parse semicolon
    let _ = input.parse::<Token![;]>()?;
    
    Ok(Equation {
        conditions,
        left,
        right,
    })
}

fn parse_expr(input: ParseStream) -> SynResult<Expr> {
    // Parse parenthesized expression or variable
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        
        // Parse constructor name or 'subst'
        let constructor = content.parse::<Ident>()?;
        
        // Check if this is a substitution
        if constructor == "subst" {
            // Parse: subst term var replacement
            // Where term and replacement are expressions, var is an identifier
            let term = parse_expr(&content)?;
            let var = content.parse::<Ident>()?;
            let replacement = parse_expr(&content)?;
            
            return Ok(Expr::Subst {
                term: Box::new(term),
                var,
                replacement: Box::new(replacement),
            });
        }
        
        // Parse arguments for regular constructor
        let mut args = Vec::new();
        while !content.is_empty() {
            args.push(parse_expr(&content)?);
        }
        
        Ok(Expr::Apply { constructor, args })
    } else {
        // Just a variable
        let var = input.parse::<Ident>()?;
        Ok(Expr::Var(var))
    }
}

fn parse_rewrites(input: ParseStream) -> SynResult<Vec<RewriteRule>> {
    let rewrites_ident = input.parse::<Ident>()?;
    if rewrites_ident != "rewrites" {
        return Err(syn::Error::new(rewrites_ident.span(), "expected 'rewrites'"));
    }
    
    let content;
    syn::braced!(content in input);
    
    let mut rewrites = Vec::new();
    while !content.is_empty() {
        // Skip comments (// ...)
        while content.peek(Token![/]) && content.peek2(Token![/]) {
            let _ = content.parse::<Token![/]>()?;
            let _ = content.parse::<Token![/]>()?;
            // Skip until end of line - consume tokens until we see something we recognize
            while !content.is_empty() && !content.peek(syn::token::Paren) && !content.peek(Token![if]) {
                let _ = content.parse::<proc_macro2::TokenTree>()?;
            }
        }
        
        if content.is_empty() {
            break;
        }
        
        rewrites.push(parse_rewrite_rule(&content)?);
    }
    
    // Optional comma after closing brace
    if input.peek(Token![,]) {
        let _ = input.parse::<Token![,]>()?;
    }
    
    Ok(rewrites)
}

fn parse_rewrite_rule(input: ParseStream) -> SynResult<RewriteRule> {
    // Parse optional freshness conditions: if x # Q then
    let mut conditions = Vec::new();
    
    while input.peek(Token![if]) {
        let _ = input.parse::<Token![if]>()?;
        let var = input.parse::<Ident>()?;
        let _ = input.parse::<Token![#]>()?;
        let term = input.parse::<Ident>()?;
        let _ = input.parse::<Ident>()?; // consume 'then'
        
        conditions.push(FreshnessCondition { var, term });
    }
    
    // Parse left-hand side
    let left = parse_expr(input)?;
    
    // Parse =>
    let _ = input.parse::<Token![=]>()?;
    let _ = input.parse::<Token![>]>()?;
    
    // Parse right-hand side
    let right = parse_expr(input)?;
    
    // Optional semicolon
    if input.peek(Token![;]) {
        let _ = input.parse::<Token![;]>()?;
    }
    
    Ok(RewriteRule {
        conditions,
        left,
        right,
    })
}
