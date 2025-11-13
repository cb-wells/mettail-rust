# REPL Performance Notes

**Date**: November 11, 2025  
**Context**: Collection Congruence Redesign

## Observed Issue

The `multi_path` example runs significantly slower in the REPL compared to running the same term in `examples/rhocalc.rs`.

## Root Cause

The REPL's `run_ascent()` implementation does substantial post-processing that `rhocalc.rs` doesn't:

### REPL Post-Processing (O(N²) worst case)

```rust
// 1. Collect ALL terms (O(N))
let all_procs: Vec<Proc> = prog.proc.iter().map(|(p,)| p.clone()).collect();

// 2. Collect ALL rewrites (O(M))
let rewrites: Vec<(Proc, Proc)> = prog.rw_proc.iter().map(...).collect();

// 3. Build term info for EVERY term (O(N*M) worst case)
for proc in &all_procs {
    // For each term, scan ALL rewrites to check if it's a normal form
    let has_rewrites = rewrites.iter().any(|(from, _)| from == proc);  // O(M)
    // ... compute IDs, build display strings, etc.
}

// 4. Map ALL rewrites to Rewrite structs (O(M))
let rewrite_list: Vec<Rewrite> = rewrites.iter().map(...).collect();
```

**Total Complexity**: O(N) + O(M) + O(N*M) + O(M) = **O(N*M)**

Where:
- N = number of terms (e.g., 50 for `multi_path`)
- M = number of rewrites (e.g., 66 for `multi_path`)

For `multi_path`: 50 * 66 = 3,300 comparisons just to identify normal forms!

### rhocalc.rs (O(1) for metrics, O(N) for display)

```rust
// Just access counts - O(1)
println!("Terms: {}", prog.proc.len());
println!("Rewrites: {}", prog.rw_proc.len());

// Sort and display one relation - O(N log N)
let mut path_full = prog.path_full.clone();
path_full.sort_by(|a,b| a.0.cmp(&b.0));
for (_, t) in path_full {
    println!("{}", t);
}
```

## Solutions

### Short-term: Optimize REPL Post-Processing

1. **Lazy normal form detection**: Don't check all terms upfront
   ```rust
   // Instead of checking every term:
   let has_rewrites = rewrites.iter().any(|(from, _)| from == proc);
   
   // Use a HashSet for O(1) lookup:
   let sources: HashSet<_> = rewrites.iter().map(|(from, _)| from).collect();
   let has_rewrites = sources.contains(&proc);
   ```

2. **Index rewrites by source**: Build once, query many
   ```rust
   let rewrites_by_source: HashMap<&Proc, Vec<&Proc>> = 
       rewrites.iter()
           .fold(HashMap::new(), |mut map, (from, to)| {
               map.entry(from).or_default().push(to);
               map
           });
   ```

3. **Lazy term info**: Only compute when displaying
   - Don't build full `term_infos` vector upfront
   - Compute on-demand in `cmd_rewrites` and `cmd_info`

### Long-term: Streaming API

Instead of collecting everything into memory:

```rust
trait Theory {
    fn run_ascent_streaming(&self, term: &dyn Term) 
        -> impl Iterator<Item = AscentFact>;
}
```

This would allow the REPL to:
- Display initial stats without full materialization
- Compute rewrites on-demand
- Stream results for large term spaces

## Performance Impact Estimate

For `multi_path` (50 terms, 66 rewrites):
- **Current**: O(3,300) comparisons
- **With HashSet**: O(50) insertions + O(50) lookups = O(100) operations
- **Speedup**: ~33x for normal form detection alone

## Related

- The indexed projection approach also contributes to performance
- Collection deconstruction adds facts but is necessary for correctness
- Future work: Lazy evaluation in Ascent itself

## Action Items

- [ ] Implement HashSet-based normal form detection
- [ ] Profile REPL on large examples
- [ ] Consider lazy term info computation
- [ ] Document performance characteristics in REPL guide

