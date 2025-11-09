# Collection Types in MeTTaIL: Design Document

## Status
**Draft** - Design phase

## Motivation

Currently, MeTTaIL uses recursive binary constructors for associative-commutative operations like parallel composition (`PPar`):

```rust
PPar . Proc ::= Proc "|" Proc ;
```

This representation has significant performance issues:
1. **Expensive Equality**: Checking if `(a | b | c)` equals `(c | a | b)` requires reasoning about associativity and commutativity
2. **Large Search Space**: Congruence rules for `PPar` generate exponentially many equal terms
3. **Slow Path Computation**: Computing full reduction paths for complex terms takes 80+ seconds

**Solution**: Represent AC (associative-commutative) operations as **collections** (multisets/hash bags), making equality trivial and dramatically improving performance.

## Desired Syntax

```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name }
    
    terms {
        PZero . Proc ::= "0" ;
        PInput . Proc ::= "for" "(" Name <Name> ")" "{" Proc "}" ;
        POutput . Proc ::= Name "!" "(" Proc ")" ;
        
        // NEW: Collection-based parallel composition
        PPar . Proc ::= HashBag(Proc) sep "|" ;
        
        PDrop . Proc ::= "*" Name ;
        NQuote . Name ::= "@" "(" Proc ")" ;
        NVar . Name ::= Var ;
    }
    
    equations {
        // No need for associativity/commutativity equations!
        // HashBag equality handles them automatically
        
        // Still need identity for empty bag
        (PPar []) == PZero ;
        (PPar [P]) == P ;   // Single-element bag unwraps
    }
    
    rewrites {
        // Pattern matching over bags
        if x # Q then (PPar bag{(PInput chan x P), (POutput chan Q), ...rest})
            => (PPar bag{(subst P x (NQuote Q)), ...rest})
    }
}
```

## Ascent Support Analysis

### ✅ What Ascent Already Supports

Based on examination of the Ascent codebase:

1. **Any Type with `Clone + Eq + Hash`**: Ascent relations can use any Rust type that implements these traits (README.MD:140)
   ```rust
   ascent! {
       relation node(i32, Rc<Vec<i32>>);  // Example from README
   }
   ```

2. **Recursive Types**: The `lists_using_recursive_enums.rs` example demonstrates:
   ```rust
   #[derive(Clone, Eq, PartialEq, Hash, Debug)]
   pub enum List<T> {
       Cons(T, Rc<List<T>>),
       Nil,
   }
   
   ascent! {
       relation list(Rc<List<char>>);
   }
   ```

3. **HashSet Available**: The `hashbrown` crate is already a dependency throughout the Ascent BYODS implementation, providing `HashSet` and `HashMap`.

4. **Custom Data Structures via BYODS**: Ascent's BYODS feature allows custom backing stores for relations, but this is for the **relation itself**, not for column types.

### 🎯 What We Need to Add

**Collection types as AST enum variant fields**:
- `HashBag<T>` (multiset) - allows duplicates, unordered
- `HashSet<T>` - no duplicates, unordered  
- `Vec<T>` - ordered, allows duplicates (for ordered operations)

These would be used as **constructor arguments** in the AST, not as relation backing stores.

## Proposed Implementation

### Phase 1: Core Collection Type Support

#### 1.1 Extended Grammar Syntax (AST)

Add collection type specifiers to `GrammarRuleComponent`:

```rust
// In mettail-macros/src/ast.rs
pub enum GrammarRuleComponent {
    Terminal(String),
    NonTerminal(Ident),
    Binder(Ident),
    
    // NEW: Collection types
    HashBag {
        element_type: Ident,    // e.g., "Proc"
        separator: String,       // e.g., "|"
    },
    HashSet {
        element_type: Ident,
        separator: String,
    },
    Vec {
        element_type: Ident,
        separator: String,
    },
}
```

#### 1.2 AST Generation (`codegen.rs`)

Generate enum variants with collection fields:

```rust
// Before (current):
pub enum Proc {
    PPar(Box<Proc>, Box<Proc>),  // Binary constructor
    // ...
}

// After (with collections):
pub enum Proc {
    PPar(HashBag<Proc>),  // Collection constructor
    // ...
}
```

Where `HashBag<T>` is a newtype wrapper:

```rust
// In mettail-runtime/src/lib.rs
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashBag<T: Clone + Hash + Eq> {
    // Use HashMap<T, usize> to track element counts
    counts: HashMap<T, usize, BuildHasherDefault<FxHasher>>,
}

impl<T: Clone + Hash + Eq> HashBag<T> {
    pub fn new() -> Self { /* ... */ }
    pub fn insert(&mut self, item: T) { /* ... */ }
    pub fn remove(&mut self, item: T) -> bool { /* ... */ }
    pub fn contains(&self, item: &T) -> bool { /* ... */ }
    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> { /* ... */ }
    pub fn len(&self) -> usize { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
    
    // Pattern matching support
    pub fn match_pattern<F>(&self, pattern: &[T], on_match: F) -> bool
    where F: FnOnce(&HashBag<T>) -> bool;
}
```

#### 1.3 Parser Generation (`lalrpop_gen.rs`)

Generate LALRPOP rules for collection syntax:

```lalrpop
// For: PPar . Proc ::= HashBag(Proc) sep "|"

pub Proc: Proc = {
    // ... other rules ...
    
    // Collection rule: parse separated list
    <elems:(<Proc> "|")*> <last:Proc?> => {
        let mut bag = mettail_runtime::HashBag::new();
        for elem in elems {
            bag.insert(elem);
        }
        if let Some(elem) = last {
            bag.insert(elem);
        }
        Proc::PPar(bag)
    },
}
```

**Challenge**: Precedence with collections is complex. May need to use operator precedence parsing or explicit parentheses for disambiguation.

#### 1.4 Display Generation (`display_gen.rs`)

Generate pretty-printing for collections:

```rust
impl Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Proc::PPar(bag) => {
                if bag.is_empty() {
                    write!(f, "0")  // Empty bag => zero
                } else {
                    let elems: Vec<_> = bag.iter()
                        .flat_map(|(elem, count)| 
                            std::iter::repeat(elem).take(*count))
                        .collect();
                    write!(f, "{}", elems.iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join("|"))
                }
            }
            // ...
        }
    }
}
```

#### 1.5 Substitution Support (`subst_gen.rs`)

Generate substitution that recursively applies to collection elements:

```rust
impl Proc {
    pub fn substitute_name(&self, var: &FreeVar<String>, replacement: &Name) -> Self {
        match self {
            Proc::PPar(bag) => {
                let new_bag = bag.iter()
                    .map(|(elem, count)| {
                        (elem.substitute_name(var, replacement), *count)
                    })
                    .collect();
                Proc::PPar(new_bag)
            }
            // ...
        }
    }
}
```

### Phase 2: Rewrite Engine Integration

#### 2.1 Pattern Matching over Collections

Extend rewrite pattern matching to handle collection patterns:

```rust
// Pattern: (PPar bag{(PInput chan x P), (POutput chan Q), ...rest})
//
// Match if:
// 1. The term is PPar(bag)
// 2. bag contains at least one PInput and one POutput with eq_name(chan1, chan2)
// 3. Bind rest to the remaining elements
```

This requires:
- **Partial matching**: Extract matching elements from bag
- **Rest patterns**: Bind remaining elements to a variable
- **Multiple occurrences**: Handle when bag has duplicates

#### 2.2 Ascent Rule Generation

Generate Ascent clauses that match collection patterns:

```rust
// For: (PPar bag{(PInput chan x P), (POutput chan Q), ...rest})
//      => (PPar bag{(subst P x (NQuote Q)), ...rest})

rw_proc(t, t_prime) <--
    proc(s),
    if let Proc::PPar(bag) = s,
    
    // Find matching PInput
    for (elem1, count1) in bag.iter(),
    if let Proc::PInput(chan1_scope) = elem1,
    let (chan1, x_binder, p_inner_scope) = chan1_scope.clone().unbind(),
    let (x, p) = p_inner_scope.unbind(),
    
    // Find matching POutput with equivalent channel
    for (elem2, count2) in bag.iter(),
    if let Proc::POutput(chan2, q) = elem2,
    eq_name((*chan1).clone(), (**chan2).clone()),
    
    // Check freshness
    if mettail_runtime::is_fresh(&x.0, q),
    
    // Build result bag
    let mut result_bag = bag.clone(),
    result_bag.remove(elem1),
    result_bag.remove(elem2),
    let substituted = p.substitute_name(&x.0, &Name::NQuote((**q).clone())),
    result_bag.insert(substituted),
    let t = Proc::PPar(result_bag);
```

**Challenge**: This is more complex than current pattern matching. May need a mini-DSL for bag patterns.

### Phase 3: Optimization & Equations

#### 3.1 Automatic Equation Elimination

When a collection type is used, automatically **skip** generating equations for:
- Associativity (`(PPar P (PPar Q R)) == (PPar (PPar P Q) R)`)
- Commutativity (`(PPar P Q) == (PPar Q P)`)

These are implicit in the collection representation.

#### 3.2 Identity Elements

Support special equations for collection identities:

```rust
equations {
    (PPar []) == PZero ;      // Empty bag is zero
    (PPar [P]) == P ;          // Single element unwraps
}
```

Generate code that automatically unwraps/wraps:
```rust
impl Proc {
    pub fn normalize_par(self) -> Self {
        match self {
            Proc::PPar(bag) if bag.is_empty() => Proc::PZero,
            Proc::PPar(bag) if bag.len() == 1 => 
                bag.iter().next().unwrap().0.clone(),
            _ => self,
        }
    }
}
```

## Comparison: Before vs After

### Before (Current Binary PPar)

```rust
// AST
pub enum Proc {
    PPar(Box<Proc>, Box<Proc>),  // Nested binary tree
}

// Equality check requires reasoning:
(PPar (PPar a b) c) == (PPar a (PPar b c))  // Associativity
(PPar a b) == (PPar b a)                    // Commutativity

// Congruence rules generate exponential number of equivalences
```

**Performance**: 80+ seconds for complex term rewriting

### After (Collection-Based PPar)

```rust
// AST
pub enum Proc {
    PPar(HashBag<Proc>),  // Flat collection
}

// Equality is O(n) hash bag comparison
HashBag([a, b, c]) == HashBag([c, a, b])  // Automatic!

// No congruence rules needed for AC operations
```

**Expected Performance**: < 1 second (100x+ speedup)

## Implementation Roadmap

### Milestone 1: Runtime Support (1-2 days)
- [ ] Implement `HashBag<T>` in `mettail-runtime`
- [ ] Add `PartialOrd`, `Ord` for `HashBag` (for term sorting)
- [ ] Add `BoundTerm` implementation
- [ ] Write tests for `HashBag` operations

### Milestone 2: AST & Grammar (2-3 days)
- [ ] Extend `GrammarRuleComponent` with collection variants
- [ ] Update `codegen.rs` to generate collection-based enum variants
- [ ] Update `display_gen.rs` for collection pretty-printing
- [ ] Update `subst_gen.rs` for collection substitution

### Milestone 3: Parser Integration (2-3 days)
- [ ] Extend `lalrpop_gen.rs` to generate collection parsing rules
- [ ] Handle precedence and associativity for collection separators
- [ ] Test parsing of `a | b | c` into `HashBag([a, b, c])`

### Milestone 4: Rewrite Engine (3-5 days)
- [ ] Design pattern matching syntax for collections
- [ ] Extend `rewrite_gen.rs` to handle collection patterns
- [ ] Generate Ascent clauses for bag matching
- [ ] Test rhocalc rewrite with collection-based `PPar`

### Milestone 5: Optimization (1-2 days)
- [ ] Automatically skip AC equations for collection types
- [ ] Implement identity element normalization
- [ ] Benchmark and compare performance

**Total Estimate**: 9-15 days

## Open Questions

1. **Pattern Syntax**: What's the best syntax for collection patterns in rewrites?
   - Option A: `bag{elem1, elem2, ...rest}`
   - Option B: `[elem1, elem2 | rest]` (Prolog-like)
   - Option C: Special rewrite syntax with explicit `extract` operator

2. **Ordering in Display**: How should we display multiset elements?
   - Arbitrary order (fastest)
   - Sorted (deterministic, easier to test)
   - Parse order (may require tracking)

3. **Empty Collections**: Should `(PPar [])` be syntactically valid or automatically normalized to `PZero`?

4. **Nested Collections**: Should we support `HashBag(HashBag(T))`?

5. **Precedence**: How do collection separators interact with other operators?
   - Should `a | b * c` parse as `(a | (b * c))` or `((a | b) * c)`?

6. **Ascent Representation**: Should collections in the AST also use collection-based data structures in Ascent relations, or continue using individual facts?
   - Current: `proc(PPar(a, PPar(b, c)))` → `proc/1` relation with nested terms
   - Alternative: `proc(PPar([a, b, c]))` → Could use `proc_par/1` with set-based facts

## Related Work

- **Maude**: Supports AC operators with built-in matching modulo theories
- **K Framework**: Has associative lists and sets as built-in structures
- **Stratego/XT**: Term rewriting with AC matching via annotations
- **ASF+SDF**: Supports bag and set matching in rewrite rules

## References

- Ascent README: Collection types supported (any `Clone + Eq + Hash`)
- Ascent BYODS: Custom data structures for **relations** (not directly applicable)
- `lists_using_recursive_enums.rs`: Example of recursive types in Ascent
- Maude AC matching: [Maude Manual Chapter 4.7](http://maude.cs.illinois.edu/w/index.php/The_Maude_System)

## Next Steps

1. **Prototype `HashBag<T>`** in `mettail-runtime` with basic operations
2. **Discuss pattern syntax** with user to decide on best approach
3. **Create minimal test case** with collection-based `PPar` in rhocalc
4. **Measure performance improvement** vs. current binary implementation

