# Collection Matching Solution: Indexed Projections

## The Problem We Solved Wrong

Our current implementation iterates through collections looking for matches:
```rust
let elem_0 = bag.iter().next()      // Get first element
let elem_1 = bag.iter().nth(1)      // Get second element  
if let Pattern1 = elem_0 { ... }    // Hope it matches!
```

This is **fundamentally wrong** for bags/sets! The whole point of using `HashBag` is **order independence**.

## The Right Solution: Indexed Projections

**Key Insight**: Ascent is a Datalog engine optimized for **joins**. We should:
1. **Project** collection elements into indexed relations
2. **Join** on the index (e.g., channel name)
3. Let Ascent's query optimizer handle the search

### Example: Rho-Calculus Communication

Instead of iterating:
```rust
// BAD: Order-dependent iteration
for elem_0 in bag {
    for elem_1 in bag {
        if matches_input(elem_0) && matches_output(elem_1) && same_channel(...) {
            // found it!
        }
    }
}
```

Use indexed projections:
```rust
// GOOD: Order-independent join
relation input_in_bag(BagId, Channel, Binder, Body);
relation output_in_bag(BagId, Channel, Payload);

// Extract and index by channel
input_in_bag(bag_id, chan, x, body) <--
    proc(Proc::PPar(bag)),
    let bag_id = bag as *const _,  // Unique ID for this bag
    for elem in bag.iter().map(|(e, _)| e),
    if let Proc::PInput(chan_box, scope) = elem,
    let (x, body) = scope.unbind();

output_in_bag(bag_id, chan, payload) <--
    proc(Proc::PPar(bag)),
    let bag_id = bag as *const _,
    for elem in bag.iter().map(|(e, _)| e),
    if let Proc::POutput(chan_box, payload_box) = elem;

// Join on (bag_id, chan) - Ascent optimizes this!
rw_proc(original, result) <--
    proc(original),
    if let Proc::PPar(bag) = original,
    let bag_id = bag as *const _,
    input_in_bag(bag_id, chan, x, body),
    output_in_bag(bag_id, chan, payload),
    // ... construct result
```

## Performance Analysis

| Approach | Time Complexity | Order Dependent? | Ascent Optimized? |
|----------|----------------|------------------|-------------------|
| Current (iteration) | O(1) - only checks first N | ✅ YES (BAD) | ❌ NO |
| Naive enumeration | O(n²) for n elements | ❌ NO | ❌ NO |
| **Indexed projection** | **O(n) extraction + O(1) join** | **❌ NO** | **✅ YES** |

The indexed approach is **optimal**:
- **Linear** in collection size for extraction
- **Constant time** join (hash table lookup on channel name)
- **Order independent** by design
- **Ascent optimized** (uses built-in indexes)

## Automatic Code Generation Plan

### Step 1: Detect Indexable Patterns

When we see a rewrite rule like:
```rust
(PPar {(PInput chan x P), (POutput chan Q), ...rest})
    => (PPar {(subst P x (NQuote Q)), ...rest})
```

Detect:
- Multiple nested patterns sharing a variable (`chan`)
- That variable appears in a **comparable position** (same field index)

### Step 2: Generate Projection Relations

For each pattern with shared variables:
```rust
// Pattern 1: (PInput chan x P)
relation pinput_proj_{rule_id}(
    BagPtr,           // Unique identifier for the bag
    Channel,          // The indexed key
    Binder,           // Other captures
    Body,
    OriginalElement   // Keep reference to original element
);

// Pattern 2: (POutput chan Q)  
relation poutput_proj_{rule_id}(
    BagPtr,
    Channel,          // Same indexed key
    Payload,
    OriginalElement
);
```

### Step 3: Generate Extraction Rules

```rust
pinput_proj_{rule_id}(bag_ptr, chan, x, body, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    let bag_ptr = bag as *const _ as usize,  // Unique bag ID
    for (elem, _count) in bag.iter(),
    if let Proc::PInput(chan_box, scope) = elem,
    let chan = chan_box.as_ref().clone(),
    let (x, body) = scope.clone().unbind();

poutput_proj_{rule_id}(bag_ptr, chan, payload, elem) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    let bag_ptr = bag as *const _ as usize,
    for (elem, _count) in bag.iter(),
    if let Proc::POutput(chan_box, payload_box) = elem,
    let chan = chan_box.as_ref().clone(),
    let payload = payload_box.as_ref().clone();
```

### Step 4: Generate Join-Based Rewrite

```rust
rw_proc(parent, result) <--
    proc(parent),
    if let Proc::PPar(bag) = parent,
    let bag_ptr = bag as *const _ as usize,
    // Join on (bag_ptr, chan) - finds matching pairs automatically!
    pinput_proj_{rule_id}(bag_ptr, chan, x, body, input_elem),
    poutput_proj_{rule_id}(bag_ptr, chan, payload, output_elem),
    // Construct rest by removing matched elements
    let rest = {
        let mut b = bag.clone();
        b.remove(input_elem);
        b.remove(output_elem);
        b
    },
    // Construct result
    let substituted = body.substitute_name(&x.0, &Name::NQuote(Box::new(payload))),
    let result_bag = {
        let mut b = rest;
        b.insert(substituted);
        b
    },
    let result = Proc::PPar(result_bag);
```

## Key Advantages

1. **Order Independent**: Elements can be in any order
2. **Efficient**: O(n) extraction + O(1) join per match
3. **Scalable**: Works for N>2 patterns (join on multiple keys)
4. **Ascent Optimized**: Uses built-in hash-based joins
5. **Automatic**: Generated from high-level pattern syntax

## Implementation Phases

### Phase 6.1: Manual Projection (Immediate)
- Add manual projection relations to `rhocalc.rs`
- Verify performance and correctness
- Document the pattern for users

### Phase 6.2: Automatic Generation (Next)
- Extend `rewrite_gen.rs` to detect shared variables
- Generate projection relations automatically
- Generate join-based rewrite rules
- Remove old iteration-based code

### Phase 6.3: Advanced Optimizations (Future)
- Multi-way joins for patterns with 3+ elements
- Incremental maintenance of projections
- Specialized indexes for common patterns

## Example: Manual Projection in Rhocalc

```rust
theory! {
    name: RhoCalc,
    // ... grammar ...
    
    rewrites {
        // The user writes this simple syntax
        (PPar {(PInput chan x P), (POutput chan Q), ...rest})
            => (PPar {(subst P x (NQuote Q)), ...rest});
    }
    
    // The macro generates this optimized code:
    ascent_code! {
        // Projection relations (auto-generated)
        relation pinput_in_par(usize, Name, Binder<String>, Proc, Proc);
        relation poutput_in_par(usize, Name, Proc, Proc);
        
        // Extraction rules (auto-generated)
        pinput_in_par(bag_id, chan, x, body, elem) <-- /* ... */;
        poutput_in_par(bag_id, chan, payload, elem) <-- /* ... */;
        
        // Join-based rewrite (auto-generated)
        rw_proc(parent, result) <--
            pinput_in_par(bid, c, x, body, ie),
            poutput_in_par(bid, c, payload, oe),
            eq_name(c.clone(), c.clone()),  // Ensure same channel
            /* ... construct result ... */;
    }
}
```

## Benchmark Expectations

With this approach, we expect:
- **Current (order-dependent)**: Fast when lucky, fails otherwise
- **Indexed projection**: Consistent O(n) performance, always works
- **Memory overhead**: 2n additional facts for projections (acceptable)

For `rhocalc` with typical process terms:
- Extraction: ~1ms for 100 processes
- Join: ~0.1ms per match (hash lookup)
- **Total: ~10ms** (same as before, but now **order-independent**!)

## Next Actions

1. **Prototype**: Add manual projections to `rhocalc.rs` to verify approach
2. **Benchmark**: Confirm O(n) scaling and order independence
3. **Automate**: Implement automatic generation in `rewrite_gen.rs`
4. **Document**: Update user guide with performance characteristics

This design aligns with the **fundamental purpose of bags**: efficient, order-independent element access through hashing/indexing!

