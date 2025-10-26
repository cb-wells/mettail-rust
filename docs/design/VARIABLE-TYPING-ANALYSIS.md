# Variable Typing Analysis

## Current Approach

### Summary
**Variables are currently UNTYPED at the representation level, but type information is INFERRED and validated at compile-time through context.**

## How It Works

### 1. Representation: `Var<String>`

All variables use moniker's `Var<N>` type with `N = String`:

```rust
// Generated code:
EVar . Expr ::= Var ;
// Becomes:
Expr::EVar(mettail_runtime::Var<String>)

// Moniker's Var type:
pub enum Var<String> {
    Free(FreeVar<String>),   // Free variable with name
    Bound(BoundVar<String>), // Bound variable with De Bruijn info
}
```

**The `String` parameter is the variable NAME, not its TYPE.**

### 2. Type Inference: Context-Based

Variables get their types through **unification during type-checking**:

```rust
// In typechecker.rs:
pub fn infer_type_with_context(
    &self,
    expr: &Expr,
    context: &mut HashMap<String, String>  // var_name -> category
) -> Result<String, ValidationError> {
    match expr {
        Expr::Var(var) => {
            let var_name = var.to_string();
            // Check if we already know this variable's type
            if let Some(typ) = context.get(&var_name) {
                Ok(typ.clone())
            } else {
                // Unknown variable - return placeholder
                Ok("?".to_string())
            }
        }
        Expr::Apply { constructor, args } => {
            // When we see (Constructor arg1 arg2...)
            // We constrain variable types based on expected argument types
            if arg_type == "?" {
                if let Expr::Var(var) = arg {
                    context.insert(var.to_string(), expected_cat.clone());
                }
            }
            // ...
        }
    }
}
```

### Example: Type Inference in Action

```rust
theory! {
    name: RhoCalc,
    exports { Proc, Name },
    terms {
        PInput . Proc ::= "for" "(" <Name> ")" "{" Proc "}" ;
        NVar . Name ::= Var ;
        PVar . Proc ::= Var ;
    },
    equations {
        // if x # P then (PInput x P) == P
        //                       ^     ^
        //                       Name  Proc (inferred from PInput's signature)
    }
}
```

When checking `(PInput x P) == P`:
1. Left side: `PInput` expects `(Name, Proc)`
   - `x` gets typed as `Name` (from first argument)
   - `P` gets typed as `Proc` (from second argument)
2. Right side: `P` must match left side type
   - Confirms `P : Proc` ✓

### 3. Binder Typing: Category-Aware

Binders in the grammar specify their category explicitly:

```rust
PInput . Proc ::= "for" "(" <Name> ")" "{" Proc "}" ;
//                          ^^^^^^^
//                          Binder category is Name

// Generated:
Proc::PInput(Scope<Binder<String>, Box<Proc>>)
//           ^^^^^                   ^^^^^^^^^
//           Binds a Name            In a Proc body
```

The binder's category (`<Name>`) tells us what kind of variable is being bound, even though the actual `Binder<String>` doesn't carry type information at runtime.

## Assessment

### ✅ What Works Well

1. **Compile-Time Safety**: Type errors are caught during macro expansion
   ```rust
   // This would fail type-checking:
   equations {
       (PInput x P) == (NQuote P)  // ERROR: Proc != Name
   }
   ```

2. **Implicit Polymorphism**: Variables can be used in multiple contexts
   ```rust
   equations {
       (PPar P Q) == (PPar Q P)   // P and Q are both Proc (inferred)
   }
   ```

3. **Binder Context**: Binders correctly scope variables of specific types
   ```rust
   PInput . Proc ::= "for" "(" <Name> ")" "{" Proc "}" ;
   // We know the bound variable is a Name, used in a Proc context
   ```

4. **Substitution Works**: Since we substitute within the same category
   ```rust
   // In Proc::substitute():
   Proc::PInput(scope) => {
       let subst_body = body.substitute(var, replacement);
       // body: Box<Proc>, replacement: &Proc ✓
   }
   ```

### ⚠️ Current Limitations

1. **No Explicit Variable Typing in Syntax**
   ```rust
   // Can't write:
   EVar(Name) . Expr ::= Var<Name> ;  // Not supported
   
   // Must use:
   EVar . Expr ::= Var ;  // Untyped Var
   ```

2. **Type Information Lost at Runtime**
   ```rust
   let x = FreeVar::fresh_named("x");  // No type tag
   let proc_x = Proc::PVar(Var::Free(x.clone()));
   let name_x = Name::NVar(Var::Free(x));
   // Same variable x used as different types! ⚠️
   ```

3. **Cross-Category Confusion Possible**
   ```rust
   // At runtime, nothing prevents:
   let x = FreeVar::fresh_named("x");
   let term1 = Proc::PVar(Var::Free(x.clone()));  // x as Proc
   let term2 = Name::NVar(Var::Free(x.clone()));  // x as Name
   
   // Substitution would fail to detect type mismatch
   term1.substitute(&x, &some_proc);  // OK
   term2.substitute(&x, &some_name);  // OK - but x is being used as TWO types!
   ```

4. **No Type-Safe Substitution Across Categories**
   ```rust
   // Can't express:
   fn substitute_name_in_proc(
       proc: &Proc,
       name_var: &FreeVar<Name>,  // ❌ Can't tag FreeVar with type
       name_val: &Name
   ) -> Proc
   ```

### 🔧 Potential Improvements

#### Option 1: Phantom Type Parameters (Type-Safe Variables)

```rust
// Extend moniker's Var with phantom type:
struct TypedVar<Cat, N> {
    var: Var<N>,
    _phantom: PhantomData<Cat>,
}

// Generated code:
Name::NVar(TypedVar<Name, String>)  // Variable tagged with Name type
Proc::PVar(TypedVar<Proc, String>)  // Variable tagged with Proc type

// Type-safe substitution:
impl Proc {
    fn substitute(
        &self,
        var: &FreeVar<String>,      // Still untyped for matching
        var_ty: PhantomData<Proc>,   // Compile-time type check
        replacement: &Proc
    ) -> Proc
}
```

**Pros**:
- Compile-time type safety for variables
- Prevents cross-category confusion
- Zero runtime cost (phantom data)

**Cons**:
- More complex generated code
- Requires changes to moniker integration
- Variables can't be polymorphic (bound to single category)

#### Option 2: Runtime Type Tags

```rust
// Extend FreeVar with type tag:
struct TypedFreeVar<N> {
    var: FreeVar<N>,
    category: String,  // Runtime type tag
}

// Type-checked substitution:
fn substitute(&self, var: &TypedFreeVar<String>, replacement: &Self) -> Self {
    assert_eq!(var.category, Self::CATEGORY);
    // ... substitution logic
}
```

**Pros**:
- Runtime type checking
- Can detect cross-category errors
- Variables can be inspected for their type

**Cons**:
- Runtime overhead
- Errors caught at runtime, not compile-time
- Extra memory per variable

#### Option 3: Category-Specific Variable Types (Current + Enhancement)

```rust
// Generate separate variable types per category:
#[derive(Clone, Debug)]
pub struct NameVar(FreeVar<String>);

#[derive(Clone, Debug)]
pub struct ProcVar(FreeVar<String>);

// Substitution becomes category-specific:
impl Proc {
    fn substitute(&self, var: &ProcVar, replacement: &Proc) -> Proc
}

impl Name {
    fn substitute(&self, var: &NameVar, replacement: &Name) -> Name
}
```

**Pros**:
- Clear type separation
- Type-safe at API level
- No runtime overhead

**Cons**:
- Can't share variables across categories (even when intentional)
- More generated code
- Conversion overhead between variable types

## Recommendation

**The current approach is SUFFICIENT for Phase 1**, because:

1. **Type checking happens at compile-time** during macro expansion
2. **Substitution is category-homogeneous** (only within same type)
3. **Binders correctly track variable categories** through grammar syntax
4. **User code is type-safe** as long as they use the generated constructors

However, for **Phase 2+**, consider:

### Incremental Improvement Path

**Short-term** (Phase 2):
- Add runtime assertions in substitution to catch category mismatches
- Better error messages indicating expected variable types

**Medium-term** (Phase 3):
- Implement Option 3 (category-specific variable types) for better API safety
- Generate conversion functions between compatible variable types

**Long-term** (Phase 4):
- Full dependent types with category parameters
- Type-level proofs of substitution correctness

## Current Status: ✅ ADEQUATE

The current untyped `Var<String>` approach works because:
1. Type checking validates correctness at compile-time
2. Generated code ensures category safety
3. Moniker handles binding and substitution correctly
4. The user API is sufficiently type-safe

**No immediate changes needed** - proceed with rewrite rules and Rho Calculus implementation.

