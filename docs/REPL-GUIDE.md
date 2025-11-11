# MeTTaIL Term Explorer REPL

**Status:** Foundation Complete ✅ (November 2025)

An interactive tool for exploring rewrite systems defined in MeTTaIL.

---

## Quick Start

```bash
cargo build --bin mettail
./target/debug/mettail
```

```
mettail> load rhocalc
mettail> term: {a!(0), for(a->x){*x}}
rhocalc> rewrites
rhocalc> apply 0
```

---

## Features

### ✅ Implemented

#### Theory Management
- **`load <name>`** - Load a theory (currently: `rhocalc`)
- **`list-theories`** - Show available theories
- **`info`** - Display current theory metadata

#### Term Exploration
- **`term: <expr>`** - Parse and execute a term
  - Parses using the loaded theory's grammar
  - Runs Ascent to compute full rewrite graph
  - Reports: term count, rewrite count, normal forms
- **`example <name>`** - Load a pre-defined example process
  - See [RhoCalc Examples](RHOCALC-EXAMPLES.md) for full list
  - Categories: Simple, Branching, Complex, Parallel, Advanced, Performance, Edge Cases
- **`list-examples`** - Show all available examples
  - Organized by category
  - Includes descriptions

#### Rewrite Navigation
- **`rewrites`** - List all available rewrites from current term
  - Shows numbered list of target terms
  - Indicates if term is a normal form
- **`apply <N>`** - Apply the Nth rewrite
  - Moves to target term
  - Preserves rewrite graph (no recomputation)
- **`normal-forms`** (alias: `nf`) - List all normal forms
  - Shows numbered list of all reachable normal forms
- **`goto <N>`** - Jump directly to the Nth normal form

#### General
- **`help`** - Show command help
- **`quit`** / `exit` - Exit REPL

---

### 🚧 Planned (Week 2)

#### History & Navigation
- **`back`** - Navigate to previous term
- **`forward`** - Navigate to next term
- **`history`** - View navigation history
- **`show`** / **`current`** - Display current term with context

#### Analysis
- **`equiv`** - Show equivalence class of current term
- **`path`** - Show rewrite path from initial term
- **`stats`** - Display graph statistics
  - Total terms, rewrites, normal forms
  - Graph depth, branching factor
  - Equivalence classes

#### Visualization
- **`graph`** - Export rewrite graph (DOT format)
- **`viz`** - Launch interactive graph visualizer

#### Multi-Theory
- Add Ambient Calculus
- Add Lambda Calculus
- Theory comparison mode

---

## Architecture

### Code Structure

```
mettail-repl/
├── src/
│   ├── theory.rs         # Theory trait & AscentResults
│   ├── registry.rs       # TheoryRegistry (dynamic loading)
│   ├── state.rs          # ReplState (session management)
│   ├── repl.rs           # Main REPL loop & commands
│   ├── rhocalc_theory.rs # RhoCalc Theory impl
│   └── main.rs           # Entry point
├── build.rs              # LALRPOP compilation
└── Cargo.toml
```

### Key Design Decisions

#### 1. Theory Trait
All theories implement the `Theory` trait:
```rust
pub trait Theory {
    fn name(&self) -> &str;
    fn categories(&self) -> Vec<String>;
    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>>;
    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults>;
    // ...
}
```

#### 2. Single Ascent Execution
- Ascent runs **once** when a term is loaded
- Computes **full rewrite graph** (all reachable terms)
- Navigation reuses this graph (no recomputation)
- Fast: typical graphs computed in <1 second

#### 3. Graph ID Tracking
- Terms parsed from strings get different IDs
- State tracks `current_graph_id` separately from `current_term.term_id()`
- Ensures correct rewrite filtering during navigation

#### 4. Dynamic Theory Loading
- Theories registered in `build_registry()`
- Each theory duplicates `theory!` macro invocation
- Ensures access to generated `rhocalc_source!` macro
- Future: Consider theory export/sharing mechanism

---

## Example Sessions

### Basic Navigation

```
mettail> load rhocalc
Loading theory: rhocalc
  ✓ 2 categories
  ✓ 8 constructors
  ✓ 3 equations
  ✓ 3 rewrite rules

✓ Theory loaded successfully!

rhocalc> term: {a!(0), for(a->x){*x}}

Parsing... ✓
Running Ascent... Done!

Computed:
  - 3 terms
  - 2 rewrites
  - 1 normal forms

Current term: {for(a->x){*x}, a!(0)}

rhocalc> rewrites

Rewrites available from current term:

  0) → {*(@(0))}

rhocalc> apply 0

Applied rewrite → {*(@(0))}

rhocalc> rewrites

Rewrites available from current term:

  0) → 0

rhocalc> apply 0

Applied rewrite → 0

rhocalc> rewrites

✓ No rewrites available from current term (it's a normal form).
```

### Complex Example

```
rhocalc> term: {
    a!(0), for(a->x0){ {x0!(0), for(b->y1){y1!(*a)}} },
    b!(0), for(b->x1){a!(*b)},
    c!(0), for(c->x2){x2!(0)},
    for(@(0)->y0){*y0}
}

Computed:
  - 50 terms
  - 66 rewrites
  - 13 normal forms

rhocalc> normal-forms

Normal forms (13 total):

  0) 0
  1) {0, 0}
  2) {0, 0, 0}
  3) {0, 0, 0, 0}
  4) {0, 0, 0, 0, 0}
  5) {for(b->y1){y1!(*a)}, 0, 0}
  ...

rhocalc> goto 0

Navigated to normal form: 0
```

---

## Implementation Notes

### Theory Integration

To add a new theory:

1. **Create theory module** (`src/my_theory.rs`):
```rust
use mettail_macros::theory;

theory! {
    name: MyTheory,
    exports { Term },
    terms { /* ... */ },
    equations { /* ... */ },
    rewrites { /* ... */ }
}

pub struct MyTheoryImpl;

impl Theory for MyTheoryImpl {
    fn name(&self) -> &str { "mytheory" }
    fn parse_term(&self, input: &str) -> Result<Box<dyn Term>> {
        let parser = mytheory::TermParser::new();
        let term = parser.parse(input)?;
        Ok(Box::new(MyTerm(term)))
    }
    fn run_ascent(&self, term: Box<dyn Term>) -> Result<AscentResults> {
        let prog = ascent_run! {
            include_source!(mytheory_source);
            term(initial_term.clone());
        };
        // Extract results...
    }
}
```

2. **Register in `registry.rs`**:
```rust
pub fn build_registry() -> TheoryRegistry {
    let mut registry = TheoryRegistry::new();
    registry.register(Box::new(RhoCalculusTheory));
    registry.register(Box::new(MyTheoryImpl));
    registry
}
```

### Term Display

Terms are displayed using their `Display` implementation (generated by `theory!` macro):
- Pretty-printed with original syntax
- Collections shown with delimiters
- Binders shown with scoped variables

---

## Known Issues

### Limitations

1. **No Incremental Execution**
   - Full graph recomputed on new term
   - Fast for small examples, may be slow for large graphs

2. **Parse-Based Navigation**
   - Target terms re-parsed from display strings
   - Could preserve term objects in graph

3. **No Persistent State**
   - Session lost on exit
   - Future: Save/load sessions

4. **Single Theory at a Time**
   - Can't compare theories side-by-side
   - Future: Multi-theory mode

### Performance

- **RhoCalc example** (8 processes): ~18 seconds, 50 terms
- **Simple examples** (2-3 processes): <1 second
- Bottleneck: Congruence rule application
- Future: Memoization, incremental computation

---

## Future Directions

### Near Term (Q1 2026)
- History navigation
- Equivalence class viewing
- Path visualization
- Ambient Calculus integration

### Medium Term (Q2-Q3 2026)
- Graph export (DOT/GraphML)
- Interactive visualizer (web-based?)
- Term generation integration
- Fuzzing/property testing mode

### Long Term (2027+)
- Multi-theory comparison
- Translation hints between theories
- Proof mode (verify properties)
- Distributed execution

---

## Testing

Run the REPL:
```bash
cargo run --bin mettail
```

Quick test:
```bash
echo -e "load rhocalc\nterm: 0\nrewrites\nquit" | cargo run --bin mettail
```

Full test suite:
```bash
cargo test -p mettail-repl
```

---

## Contributing

The REPL is under active development. Priority areas:

1. **History Navigation** - Wire up existing state methods
2. **Additional Theories** - Add Ambient, Lambda calculi
3. **Visualization** - DOT export, graph rendering
4. **Error Handling** - Better parse error messages
5. **Documentation** - More examples, tutorials

See `docs/POLY-LINGUAL-ROADMAP.md` for strategic priorities.

