# Equation Generation - Status & Limitations

## Status: Phase 3 Complete ✅

Equation generation successfully works for:
- ✅ Reflexivity, Symmetry, Transitivity (all categories)
- ✅ Simple equations with variables: `(PPar P Q) == (PPar Q P)`
- ✅ Nested constructors: `(PPar P (PPar Q R)) == (PPar (PPar P Q) R)`
- ✅ Cross-category equations: `(PDrop (NQuote P)) == P`
- ✅ **Nullary constructors**: `(PPar P PZero) == P` - FIXED!

## ~~Limitation 1: Nullary Constructors in Equation LHS~~ - RESOLVED ✅

### Solution
The issue was using `&**field` when we should use `**field` for nullary constructor matching:

**Working code:**
```rust
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc::PPar(P, field_1) = p0,
    if let Proc::PZero = **field_1,  // Correct: **field_1 not &**field_1
    let p1 = (**P).clone();
```

When `field_1` is `&Box<Proc>`, `**field_1` gives us `Proc`, which is what we need to match against `Proc::PZero`.

## Limitation: Binding Constructors in Equations

### Issue
Equations with nullary constructors (constructors with no non-terminal arguments) in the LHS fail to compile in Ascent.

**Example:** `(PPar P PZero) == P` where `PZero . Proc ::= "0"`

### Root Cause
When we attempted to match nullary constructors inline:
```rust
eq_proc(p0, p1) <--
    proc(p0),
    if let Proc::PPar(P, Proc::PZero) = p0,  // Inline match
    let p1 = (**P).clone();
```

Ascent's pattern matching doesn't handle literal constructor patterns inline within parent patterns. This causes type confusion in Ascent's generated code, resulting in `Convert<Box<Proc>>` trait errors.

### Attempted Solutions
1. **Inline matching (failed)**: Tried `if let Proc::PPar(P, Proc::PZero) = p0` - causes Ascent type errors
2. **Temp binding with nested match (failed)**: 
   ```rust
   if let Proc::PPar(P, field_1) = p0,
   if let Proc::PZero = &**field_1,
   ```
   This also causes type errors, likely because Ascent struggles with matching unit variants against dereferenced boxed values.

### Resolution Path
The proper solution requires one of:
1. **Don't match nullary constructors in equations**: Use rewrite rules instead for these cases
2. **Use Ascent guards**: Instead of pattern matching, use conditional guards:
   ```rust
   eq_proc(p0, p1) <--
       proc(p0),
       if let Proc::PPar(P, q) = p0,
       if matches!(&**q, Proc::PZero),  // Guard instead of pattern match
       let p1 = (**P).clone();
   ```
3. **Generate specialized matching code**: Detect nullary constructors and generate completely different matching logic

### Workaround
For now, equations with nullary constructors must be hand-written or expressed as rewrite rules instead.

## Limitation 2: Binding Constructors in Equations

### Issue
Equations involving binding constructors (those with `<Binder>` in their grammar) are not yet supported.

**Example:** `(PNew x P) ≡α (PNew y [y/x]P)` (alpha-equivalence)

### Root Cause
Binding constructors use `Scope<Binder<Name>, Box<Proc>>` which requires:
1. Unbinding the scope to access the binder and body
2. Potentially performing capture-avoiding substitution
3. Rebinding to create new scopes

The current equation generation handles regular patterns and nested Apply patterns, but doesn't handle:
- Unbinding scopes in the LHS pattern
- Performing substitution in the RHS construction
- Alpha-equivalence checking

### Current Pattern Generation
```rust
// Regular constructor:
if let Proc::PPar(P, Q) = p0,

// Binding constructor (not handled):
if let Proc::PNew(scope) = p0,
let (binder, body) = scope.clone().unbind();
// How do we bind 'binder' as a variable in the equation?
```

### Challenges
1. **Variable capture**: Equations like `(PNew x P) == (PNew y [y/x]P)` require:
   - Binding the binder names (`x`, `y`) as pattern variables
   - Generating substitution code `[y/x]P`
   - Ensuring freshness conditions are respected

2. **Scope construction on RHS**: When constructing a binding term on the RHS:
   ```rust
   let new_scope = Scope::new(new_binder, Box::new(new_body));
   let p1 = Proc::PNew(new_scope);
   ```

3. **Alpha-equivalence vs structural equality**: Most binding equations are about alpha-equivalence, which is already handled by the underlying moniker library. Structural equations on binding constructors are rare and complex.

### Resolution Path
1. **Phase 1**: Detect equations with binding constructors and skip them (current behavior)
2. **Phase 2**: Support simple binding equations without substitution:
   - Pattern: `(Constructor <x> P) == RHS` where RHS doesn't rebind
   - Generate unbinding code in pattern
   - Generate scope construction in RHS
3. **Phase 3**: Support substitution in RHS using `Subst` expr:
   - Detect `Subst { term, var, replacement }` in RHS
   - Generate `substitute(term, var, replacement)` code
4. **Phase 4**: Support freshness conditions:
   - Parse `if x # Q then LHS == RHS`
   - Generate freshness checks using moniker's `fresh_for` API

### Workaround
For now, binding-related equations must be:
- Omitted (rely on alpha-equivalence from moniker)
- Hand-written in the Ascent source
- Expressed as rewrite rules if directional

## Implementation Notes

### Type Handling in Generated Code
The key insight for cross-category equations:
```rust
// Variables bound from patterns are &Box<T>
if let Proc::PDrop(field_0) = p0,
if let Name::NQuote(P) = &**field_0,  // P is &Box<Proc>

// On RHS:
// - In constructor: P.clone() → Box<Proc>
// - At top-level: (**P).clone() → Proc
let p1 = (**P).clone();
```

### Nested Constructor Handling
Nested constructors on RHS must be wrapped:
```rust
// RHS: (PPar (PPar P Q) R)
Proc::PPar(Box::new(Proc::PPar(P.clone(), Q.clone())), R.clone())
```

### Constructor Detection
Must distinguish constructors from variables:
```rust
fn is_constructor(ident: &Ident, theory: &TheoryDef) -> bool {
    theory.terms.iter().any(|rule| rule.label == *ident)
}
```

## Testing Status

### Working Examples (RhoCalc) - ALL PASSING ✅
- ✅ `(PPar P Q) == (PPar Q P)` - Commutativity
- ✅ `(PPar P (PPar Q R)) == (PPar (PPar P Q) R)` - Associativity  
- ✅ `(PPar P PZero) == P` - Identity with nullary constructor  
- ✅ `(PDrop (NQuote P)) == P` - Cross-category

### Not Yet Tested (Ambient)
- Binding equations with alpha-equivalence
- Equations with freshness conditions

## Recommendation

For Phase 4 (Rewrite Rules):
1. The same pattern/RHS generation logic will apply
2. Binding constructors in rewrites ARE supported (already working in category deconstruction)
3. Congruence rules explicitly declared - no auto-generation needed
4. Base rewrites can handle nullary constructors differently (they're in try_rewrite_rule_N functions)

The equation limitations are acceptable because:
- Most structural equations work fine
- Alpha-equivalence is handled by moniker
- Identity equations with nullary constructors can be added as rewrites
- Binding-heavy equations are rare and can be hand-written if needed

