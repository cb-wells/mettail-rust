# REPL Next Steps: Basic Features

**Status:** Foundation Complete (November 2025)

This document tracks the essential features to complete the REPL's basic functionality.

---

## ✅ Completed (Week 1)

### Core Navigation
- [x] Theory loading and registry
- [x] Term parsing with theory integration
- [x] Ascent execution (full graph computation)
- [x] Rewrite listing from current term
- [x] Apply specific rewrites
- [x] View normal forms
- [x] Jump to normal forms
- [x] Graph ID tracking (correct navigation)

### UI/UX
- [x] Clean REPL interface
- [x] Colored output (theory names, arrows, success/error)
- [x] Command help
- [x] Error messages for invalid commands

---

## 🎯 Priority: Essential Features (Week 2)

### 1. History Navigation (2-3 hours)
**Status:** Code exists in `state.rs`, needs wiring

**Implementation:**
- [x] `ReplState::go_back()` - exists
- [x] `ReplState::go_forward()` - exists
- [ ] `cmd_back()` - wire to command dispatch
- [ ] `cmd_forward()` - wire to command dispatch
- [ ] `cmd_history()` - display navigation history

**Commands:**
```
rhocalc> back      # Go to previous term
rhocalc> forward   # Go to next term
rhocalc> history   # Show visited terms
```

### 2. Current Term Display (1-2 hours)
**Status:** Basic display exists, needs enhancement

**Add Commands:**
- [ ] `show` / `current` - Display current term with context
  - Show term
  - Show available rewrites count
  - Show if it's a normal form
  - Show position in history

**Example:**
```
rhocalc> show

Current term (step 3 of 5):
  {for(@(0)->y0){*y0}, a!(0)}

Status: Intermediate (not a normal form)
Available rewrites: 2
```

### 3. Equivalence Classes (2-3 hours)
**Status:** Data exists in `AscentResults`, needs display

**Implementation:**
- [ ] `cmd_equiv()` - Show equivalence class of current term
- [ ] Extract from `results.equivalences`
- [ ] Group by equivalence class ID

**Command:**
```
rhocalc> equiv

Equivalence class (3 terms):
  - {a!(0), for(a->x){*x}}
  - {for(a->x){*x}, a!(0)}     ← current
  - {a!(0), {for(a->x){*x}}}
```

### 4. Path Visualization (2-3 hours)
**Status:** Rewrite data exists, needs path extraction

**Implementation:**
- [ ] `cmd_path()` - Show rewrite path from initial term
- [ ] Track path in history entries
- [ ] Display numbered steps with applied rules

**Command:**
```
rhocalc> path

Rewrite path (3 steps):

  0) {a!(0), for(a->x){*x}}
     ↓ communication
  1) {*(@(0))}
     ↓ drop
  2) 0                         ← current
```

### 5. Add Ambient Calculus (2-4 hours)
**Status:** Theory exists in examples, needs REPL integration

**Implementation:**
- [ ] Create `src/ambient_theory.rs` (copy pattern from `rhocalc_theory.rs`)
- [ ] Implement `Theory` trait for Ambient
- [ ] Register in `build_registry()`
- [ ] Test basic navigation

**Goal:** Two working theories in REPL

---

## 📊 Statistics & Analysis (Week 2-3)

### 6. Statistics Command (1-2 hours)
**Status:** Data available, needs formatting

**Command:**
```
rhocalc> stats

Rewrite Graph Statistics:
  Total terms:         50
  Total rewrites:      66
  Normal forms:        13
  Equivalence classes: 8

  Max depth:           7
  Avg branching:       1.32

  Initial term:        {a!(0), for(a->x){*x}, ...}
  Current term:        {*(@(0))}  (step 2/5)
```

### 7. Error Recovery (2-3 hours)
**Status:** Basic errors work, needs polish

**Improvements:**
- [ ] Better parse error messages (show position)
- [ ] Suggest corrections for typos
- [ ] Graceful handling of invalid commands
- [ ] Don't crash on parse failures

---

## 🎨 Polish & UX (Week 3-4)

### 8. Better Output Formatting
- [ ] Table formatting for rewrite lists
- [ ] Syntax highlighting for terms
- [ ] Progress indicators for long computations
- [ ] Truncate very large terms with "..."

### 9. Command Aliases
- [x] `nf` for `normal-forms`
- [ ] `r` for `rewrites`
- [ ] `a` for `apply`
- [ ] `g` for `goto`
- [ ] `b` for `back`
- [ ] `f` for `forward`
- [ ] `h` for `help`

### 10. Command History (rustyline feature)
- [x] Up/down arrows work
- [ ] Persistent history across sessions
- [ ] History search (Ctrl-R)
- [ ] Auto-completion

---

## 🚀 Advanced Features (Q1 2026)

### Graph Visualization (2-3 weeks)
- [ ] Export to DOT format
- [ ] Generate SVG/PNG
- [ ] Interactive web visualizer
- [ ] Highlight current term

### Term Generation Integration (1-2 weeks)
- [ ] `generate <depth>` - Generate random term
- [ ] `generate-all <depth>` - Generate all terms
- [ ] Fuzz mode (random walks)

### Multi-Theory Mode (2-3 weeks)
- [ ] Load multiple theories simultaneously
- [ ] Compare rewrite graphs side-by-side
- [ ] Translation hints between theories

### Session Management (1 week)
- [ ] Save session to file
- [ ] Load session from file
- [ ] Export session as script
- [ ] Replay session

---

## 📝 Documentation Needs

### User Documentation
- [x] REPL-GUIDE.md - basic usage
- [ ] Tutorial: First steps with RhoCalc
- [ ] Tutorial: Exploring rewrite graphs
- [ ] Video demo/screencast

### Developer Documentation
- [ ] Architecture overview
- [ ] How to add a new theory
- [ ] How to add a new command
- [ ] Testing guide

---

## 🐛 Known Issues to Fix

### Critical
1. **None currently** - basic navigation works!

### Medium Priority
1. **Large term display** - Terms with >100 constructors are unwieldy
2. **Long-running Ascent** - No progress indication for slow computations
3. **Memory usage** - Full graph stored in memory

### Low Priority
1. **Windows support** - Colored output may not work on older Windows
2. **Prompt customization** - No way to configure prompt format
3. **Logging** - No debug logging for troubleshooting

---

## ⏱️ Time Estimates

**Week 2 (Essential):** 10-15 hours
- History navigation: 2-3h
- Current term display: 1-2h
- Equivalence classes: 2-3h
- Path visualization: 2-3h
- Ambient calculus: 2-4h

**Week 3 (Polish):** 5-10 hours
- Statistics: 1-2h
- Error recovery: 2-3h
- Output formatting: 2-3h
- Command aliases: 1-2h

**Week 4+ (Advanced):** 4-8 weeks
- Graph visualization: 2-3 weeks
- Term generation: 1-2 weeks
- Multi-theory: 2-3 weeks
- Session mgmt: 1 week

---

## 🎯 Success Criteria

**MVP (Minimum Viable Product):**
- ✅ Load theories
- ✅ Parse terms
- ✅ Navigate rewrites
- ✅ View normal forms
- [ ] History navigation
- [ ] Show current context
- [ ] Two theories (RhoCalc + Ambient)

**V1.0 (Production Ready):**
- All MVP features
- Statistics dashboard
- Path visualization
- Error recovery
- Documentation complete
- Tutorial videos

**V2.0 (Advanced):**
- Graph export
- Term generation
- Multi-theory comparison
- Session management
- Interactive visualizer

---

## 📌 Next Actions

**Immediate (this session):**
1. Wire up history navigation commands
2. Add `show` command for current term display
3. Test with complex RhoCalc examples

**This week:**
1. Implement equivalence class viewing
2. Add path visualization
3. Integrate Ambient Calculus

**Next week:**
1. Statistics dashboard
2. Better error messages
3. Polish UI/UX

---

**Last Updated:** November 10, 2025

