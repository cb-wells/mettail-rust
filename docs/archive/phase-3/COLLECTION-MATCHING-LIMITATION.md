# Collection Pattern Matching Limitation

## Current Status

Collection pattern matching with rest patterns (`{P, Q, ...rest}`) is **partially implemented** with a known limitation.

## What Works

✅ Matching when the required elements appear **first** in iteration order:
```rust
{a!(0), for(a->x0){*x0}}
// Successfully matches: POutput(a, 0) and PInput(a, x, *x0) are first two elements
// Result: {*@(0)}
```

## Current Limitation

❌ Matching fails when required elements are **not** the first ones in iteration order:
```rust
{a!(0), b!(0), for(a->x0){*x0}}
// Fails to match: POutput(a, 0) and PInput(a, x, *x0) exist but b!(0) might be checked first
// Expected: {*@(0), b!(0)}
// Actual: No match
```

### Root Cause

The current implementation extracts elements **sequentially**:
```rust
let elem_0 = bag.iter().next().unwrap()      // First element
let elem_1 = bag.iter().nth(1).unwrap()      // Second element
```

Then checks if they match the nested patterns. If they don't, the rule fails.

**What's needed**: Try all **combinations** of elements from the bag until we find elements that match the nested patterns.

## Why This Is Hard

Ascent (Datalog) doesn't have built-in support for:
1. **Backtracking** - trying multiple element combinations
2. **Set comprehensions** - "find all pairs (e1, e2) where e1 matches P and e2 matches Q"
3. **Filtering iterators** - we can't easily filter the bag to find matching elements

## Possible Solutions

### 1. **Pre-filter Collections** (Recommended for MVP)

Before matching, add rules that filter collection elements:

```rust
// Extract only PInput processes
relation pinput_in_par(Proc, Name, Scope<Box<Proc>>);
pinput_in_par(parent, chan, scope) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for elem in bag.iter().map(|(e, _)| e),
    if let Proc::PInput(chan_box, scope) = elem,
    let chan = chan_box.as_ref();

// Extract only POutput processes
relation poutput_in_par(Proc, Name, Proc);
poutput_in_par(parent, chan, payload) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    for elem in bag.iter().map(|(e, _)| e),
    if let Proc::POutput(chan_box, payload_box) = elem,
    let chan = chan_box.as_ref(),
    let payload = payload_box.as_ref();

// Now match on the relations
rw_proc(parent, result) <--
    pinput_in_par(parent, chan1, scope),
    poutput_in_par(parent, chan2, payload),
    eq_name(chan1, chan2),
    // ... rest of logic
```

**Pros**: Works with current Ascent capabilities
**Cons**: More verbose, requires manual relation definitions

### 2. **Enumerate All Pairs** (Medium-term)

Generate Ascent code that tries all index combinations:

```rust
// For a 2-element pattern, generate nested loops
for (elem_0, _) in bag.iter() {
    for (elem_1, _) in bag.iter() {
        if elem_0 != elem_1 {
            // Try matching elem_0 and elem_1 against patterns
        }
    }
}
```

**Pros**: Automatic, no manual relations needed
**Cons**: Requires code generation support, O(n²) complexity for n elements

### 3. **Smart Indexing** (Long-term)

Build specialized indexes for common patterns:

```rust
// Maintain index: channel -> (inputs, outputs)
relation channel_index(Name, Vec<Proc>, Vec<Proc>);

// Use index for O(1) lookup
```

**Pros**: Efficient, scales well
**Cons**: Complex implementation, requires theory analysis

## Workaround for Current Users

**Option A**: Ensure critical elements appear first in input
```rust
// Order matters: put potential matches first
let input = "{for(a->x0){*x0}, a!(0)}"; // Will match
```

**Option B**: Use multiple rewrite rules for different orderings
```rust
// Add explicit rules for common cases
(PPar {(PInput c x P), (POutput c Q), R}) => ...
(PPar {(POutput c Q), (PInput c x P), R}) => ...
(PPar {R, (PInput c x P), (POutput c Q)}) => ...
```

**Option C**: Normalize collections before matching
```rust
// Add a normalization step that sorts collections deterministically
```

## Recommended Next Steps

1. **Short-term** (Phase 5.1): Document this limitation clearly
2. **Medium-term** (Phase 5.2): Implement solution #1 (pre-filtering) as an opt-in
3. **Long-term** (Phase 6): Implement solution #2 (enumeration) with optimization

## Testing

Created test cases:
- ✅ `test_rest_patterns.rs` - Simple 2-element match (PASS)
- ✅ `rhocalc.rs` with 2 elements (PASS)
- ❌ `rhocalc.rs` with 3 elements (FAIL - known limitation)

## Performance Impact

The limitation actually **improves** performance in the current implementation:
- Current: O(1) element extraction (just take first two)
- Full solution: O(n²) or worse for n elements

For production use, we'll need smart heuristics or indexes.

