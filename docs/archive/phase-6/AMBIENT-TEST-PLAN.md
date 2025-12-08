# Ambient Calculus: Comprehensive Test Plan

## Specification Review

### Terms
1. **PZero**: `0` - Null process
2. **PIn**: `in(name, P)` - Enter ambient named `name`, then behave as `P`
3. **POut**: `out(name, P)` - Exit ambient named `name`, then behave as `P`
4. **POpen**: `open(name, P)` - Open ambient named `name`, then behave as `P`
5. **PAmb**: `name[P]` - Ambient named `name` containing process `P`
6. **PNew**: `new(x, P)` - Name restriction: binds fresh name `x` in `P`
7. **PPar**: `{P1, P2, ...}` - Parallel composition (collection type)

### Rewrite Rules (3 base rules)
1. **Entry**: `{n[{in(m,p), ...rest}], m[r]} => m[{n[{p, ...rest}], r}]`
   - Ambient `n` enters ambient `m`
   - Rest pattern preserves other processes in `n`

2. **Exit**: `m[{n[{out(m,p), ...rest}], r}] => {n[{p, ...rest}], m[r]}`
   - Ambient `n` exits from parent `m`
   - Rest pattern preserves other processes in `n`

3. **Open**: `{open(n,p), n[q]} => {p, q}`
   - Dissolves ambient `n`, releasing its contents
   - No rest pattern (matches exactly two elements)

### Congruence Rules (3 rules)
1. **Parallel**: `if S => T then {S, ...rest} => {T, ...rest}`
   - Rewrites apply under parallel composition

2. **New**: `if S => T then new(x, S) => new(x, T)`
   - Rewrites apply under name restriction

3. **Ambient**: `if S => T then n[S] => n[T]`
   - Rewrites apply inside ambients

### Equations (6 rules)
1. **Zero Identity**: `P == {P, 0}`
   - Zero is identity for parallel composition

2. **New Extrusion (Collection)**: `if x # C then {P, new(x, C)} == new(x, {P, C})`
   - Can move `new` out of parallel if `x` not free in `P`

3. **New Extrusion (In)**: `if x # N then new(x, {P, in(N, P)}) == {P, in(N, new(x, P))}`
   - Can move `new` past `in` capability

4. **New Extrusion (Out)**: `if x # N then new(x, {P, out(N, P)}) == {P, out(N, new(x, P))}`
   - Can move `new` past `out` capability

5. **New Extrusion (Open)**: `if x # N then new(x, {P, open(N, P)}) == {P, open(N, new(x, P))}`
   - Can move `new` past `open` capability

6. **New Extrusion (Ambient)**: `if x # N then new(x, {P, n[P]}) == {P, n[new(x, P)]}`
   - Can move `new` past ambient constructor

## Test Categories

### 1. Base Rewrite Rules

#### 1.1 Entry Rule
- ✅ Basic entry (empty rest)
- ✅ Entry with non-empty rest (single element)
- ✅ Entry with multiple elements in rest
- ✅ Entry preserves target ambient's contents
- [ ] Entry with nested ambients in rest
- [ ] Entry with capabilities in rest
- [ ] Multiple entries to same target (non-deterministic)

#### 1.2 Exit Rule
- ✅ Basic exit (empty rest)
- ✅ Exit with non-empty rest (single element)
- ✅ Exit with multiple elements in rest
- [ ] Exit from nested ambient
- [ ] Exit with nested ambients in rest
- [ ] Exit with capabilities in rest
- [ ] Multiple exits from same parent (non-deterministic)

#### 1.3 Open Rule
- ✅ Basic open
- [ ] Open releases multiple processes
- [ ] Open releases nested ambients
- [ ] Open with name binding (future)
- [ ] Multiple opens in parallel

### 2. Congruence Rules

#### 2.1 Parallel Congruence
- ✅ Rewrite in parallel context (observer pattern)
- ✅ Rewrite with multiple parallel processes
- [ ] Nested parallel contexts
- [ ] Rewrite in parallel with capabilities
- [ ] Rewrite in parallel with restrictions

#### 2.2 New Congruence
- ✅ Direct new congruence
- ✅ New with rest patterns
- ✅ New in collection
- ✅ Nested new (ambient inside new)
- [ ] Nested new (new inside new)
- [ ] New with entry
- [ ] New with exit
- [ ] New with open

#### 2.3 Ambient Congruence
- ✅ Rewrite inside ambient
- [ ] Rewrite inside nested ambients
- [ ] Rewrite inside ambient with restriction
- [ ] Multiple rewrites in different ambients

### 3. Equations

**🚨 CRITICAL: Equations are NOT being generated!** (Discovered Nov 19, 2025)

All 6 ambient equations fail to generate due to limitations in `generate_equation_pattern`:
- Bare variable LHS (Equation 0): Can't pattern match on `P`
- Complex patterns in collections (Equations 1-6): Only simple variables supported

**Current Status**: 0/6 equations implemented
**Impact**: Semantic incorrectness - equivalences not enforced
**See**: [EQUATION-IMPLEMENTATION-PLAN.md](EQUATION-IMPLEMENTATION-PLAN.md)

#### 3.1 Zero Identity
- [ ] `{P}` normalizes to `P`
- [ ] `{P, 0}` normalizes to `P`
- [ ] `{P, 0, 0}` normalizes to `P`
- [ ] Zero in nested contexts

**Blocker**: Equation not generated (bare variable LHS)

#### 3.2 New Extrusion
- [ ] Basic extrusion (collection)
- [ ] Extrusion with `in`
- [ ] Extrusion with `out`
- [ ] Extrusion with `open`
- [ ] Extrusion with ambient
- [ ] Multiple extrusions
- [ ] Extrusion blockage (when name is free)

**Blocker**: Equations not generated (complex patterns in collections)

### 4. Complex Interactions

#### 4.1 Sequential Operations
- ✅ Entry then exit from same ambient
- ✅ Sequential mobility through multiple locations
- [ ] Entry, operation inside, exit
- [ ] Open after entry

#### 4.2 Parallel Operations
- ✅ Multiple entries to same target
- [ ] Entry and exit simultaneously
- [ ] Multiple opens
- [ ] Entry/exit with observer ambients

#### 4.3 Name Binding
- [ ] Capability passing via new
- [ ] Name alpha-equivalence
- [ ] Name shadowing
- [ ] Scope extrusion

#### 4.4 Nested Structures
- ✅ Parent with child moves together
- [ ] Deep nesting (3+ levels)
- [ ] Sibling ambients interaction
- [ ] Complex tree structures

### 5. Edge Cases

#### 5.1 Degenerate Cases
- [ ] Empty ambient `n[0]`
- [ ] Empty collection `{}`
- [ ] Singleton collection `{P}`
- [ ] Ambient containing only zero `n[{0}]`

#### 5.2 Non-Matching Patterns
- [ ] Entry to non-existent ambient
- [ ] Exit from root level
- [ ] Open non-existent ambient
- [ ] Mismatched names

#### 5.3 Normal Forms
- [ ] Verify true normal forms have no rewrites
- [ ] Verify non-normal forms have rewrites
- [ ] Deadlock detection (circular dependencies)

### 6. Performance & Correctness

#### 6.1 Confluence
- [ ] Same normal form from different reduction orders
- [ ] Diamond property verification

#### 6.2 Termination
- [ ] All test cases terminate
- [ ] Measure reduction steps
- [ ] Identify potential non-termination patterns

#### 6.3 Performance
- [ ] Time for small terms (< 5 ambients)
- [ ] Time for medium terms (5-10 ambients)
- [ ] Time for large terms (10+ ambients)
- [ ] Memory usage tracking

## Test Implementation Strategy

### Phase 1: Core Functionality (Current)
- ✅ Basic base rules
- ✅ Basic congruences
- ✅ Rest patterns

### Phase 2: Comprehensive Base Rules
- [ ] All variations of entry/exit/open
- [ ] Edge cases for each rule
- [ ] Non-deterministic scenarios

### Phase 3: Equation System
- [ ] Zero identity tests
- [ ] All extrusion equations
- [ ] Equation interaction with rewrites

### Phase 4: Complex Scenarios
- [ ] Sequential compositions
- [ ] Parallel compositions
- [ ] Deep nesting
- [ ] Name binding interactions

### Phase 5: Correctness Properties
- [ ] Confluence testing
- [ ] Termination verification
- [ ] Normal form validation

## Test Structure

```rust
struct TestCase {
    // Identity
    name: &'static str,
    description: &'static str,
    category: TestCategory,

    // Input/Output
    input: &'static str,
    expected_output: Option<&'static str>,  // None if multiple valid outcomes

    // Validation
    should_normalize: bool,
    min_rewrites: usize,
    max_rewrites: Option<usize>,

    // Verification
    check_properties: Vec<Property>,  // e.g., confluence, termination
}

enum TestCategory {
    BaseRule(BaseRule),
    Congruence(CongruenceType),
    Equation(EquationType),
    Complex,
    EdgeCase,
}

enum Property {
    Confluence,
    Termination,
    Deterministic,
    NormalFormVerified,
}
```

## Success Criteria

1. **Completeness**: At least 50 test cases covering all categories
2. **Coverage**: 100% of rewrite rules tested
3. **Edge Cases**: All identified edge cases have tests
4. **Performance**: All tests complete in < 5 seconds total
5. **Correctness**: All tests pass consistently

## Current Status

- **Tests Implemented**: 17 (ambient_tests.rs)
- **Tests Passing**: 17/17 ✅
- **Coverage**: ~40% (base rules and basic congruences)
- **Next Priority**: Equation system tests

