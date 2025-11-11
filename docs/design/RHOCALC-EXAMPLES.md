# RhoCalc Example Processes

A collection of interesting Rho Calculus processes demonstrating various communication patterns and execution behaviors.

---

## Simple Examples

### 1. Basic Communication
**Name:** `simple_comm`  
**Pattern:** Single channel, immediate communication

```rust
{a!(0), for(a->x){*x}}
```

**Behavior:**
- `a` sends `0`
- Listener on `a` receives and drops the quoted `0`
- Result: `0`

**Graph:** 3 terms, 2 rewrites, 1 normal form

---

### 2. Sequential Communication
**Name:** `sequential`  
**Pattern:** Two independent channels

```rust
{
    a!(0), for(a->x){*x},
    b!(0), for(b->y){*y}
}
```

**Behavior:**
- Two independent communications
- Can happen in either order
- Both reduce to `0`

**Graph:** ~7 terms, multiple paths to `{0, 0}`

---

### 3. Reflection Loop
**Name:** `reflection`  
**Pattern:** Quote/drop cycle

```rust
{for(@(0)->x){*x}, @(0)!(0)}
```

**Behavior:**
- Listener on quoted `0`
- Sender sends `0` on quoted `0`
- Result: `0`

**Graph:** 3 terms, demonstrates quote/unquote

---

## Branching Examples

### 4. Choice Point
**Name:** `choice`  
**Pattern:** Multiple listeners on same channel

```rust
{
    a!(0),
    for(a->x){x!(0)},
    for(a->y){y!(0)}
}
```

**Behavior:**
- Non-deterministic choice: which listener receives?
- Two possible paths depending on order
- Different end states

**Graph:** Branching rewrite graph

---

### 5. Race Condition
**Name:** `race`  
**Pattern:** Multiple senders, one listener

```rust
{
    a!(0), a!(1),
    for(a->x){*x}
}
```

**Behavior:**
- Two messages on same channel
- Listener can receive either
- Leftover message remains

**Graph:** Multiple normal forms: `{0, a!(1)}` or `{1, a!(0)}`

---

## Complex Patterns

### 6. Forwarder
**Name:** `forward`  
**Pattern:** Relay messages between channels

```rust
{
    a!(0),
    for(a->x){b!(*x)},
    for(b->y){*y}
}
```

**Behavior:**
- `a` sends to forwarder
- Forwarder relays to `b`
- `b` receives and drops
- Result: `0`

**Graph:** 4 terms, linear chain

---

### 7. Circular Communication
**Name:** `circular`  
**Pattern:** Messages cycle through channels

```rust
{
    a!(0),
    for(a->x){b!(*x)},
    for(b->y){a!(*y)}
}
```

**Behavior:**
- `a` → `b` → `a` → ...
- Infinite loop (no normal form!)
- Demonstrates non-termination

**Graph:** Cycle in rewrite graph

---

### 8. Three-Way Handshake
**Name:** `handshake`  
**Pattern:** Multi-step protocol

```rust
{
    a!(0),
    for(a->x){
        {b!(*x), for(c->z){*z}}
    },
    for(b->y){c!(*y)}
}
```

**Behavior:**
- `a` → first process
- First process → `b` (and waits on `c`)
- `b` → `c`
- Completes handshake
- Result: `0`

**Graph:** ~5 terms, demonstrates protocol

---

## Parallel Computation

### 9. Multiple Paths (Current Complex Example)
**Name:** `multi_path`  
**Pattern:** Multiple concurrent communications with dependencies

```rust
{
    a!(0),
    for(a->x0){ {x0!(0), for(b->y1){y1!(*a)}} },
    b!(0),
    for(b->x1){a!(*b)},
    c!(0),
    for(c->x2){x2!(0)},
    for(@(0)->y0){*y0}
}
```

**Behavior:**
- 3 channels with interdependencies
- `a` communicates, spawning `x0` sender and `b` listener
- `b` has independent sender/listener
- `c` has independent communication
- Reflection on `@(0)`
- Many interleaving orders possible

**Graph:** 50 terms, 66 rewrites, 13 normal forms, ~18 seconds

---

### 10. Parallel Independent
**Name:** `parallel`  
**Pattern:** Multiple independent parallel processes

```rust
{
    a!(0), for(a->x){*x},
    b!(1), for(b->y){*y},
    c!(2), for(c->z){*z},
    d!(3), for(d->w){*w}
}
```

**Behavior:**
- 4 independent communications
- Can happen in any order (4! = 24 orderings)
- All lead to same result: `{0, 1, 2, 3}`

**Graph:** Many paths, single normal form

---

### 11. Barrier Synchronization
**Name:** `barrier`  
**Pattern:** Wait for all inputs before proceeding

```rust
{
    a!(0), b!(0),
    for(a->x){for(b->y){{*x, *y}}}
}
```

**Behavior:**
- Nested listeners wait for both `a` and `b`
- Must receive in order (a first, then b)
- Result: `{0, 0}`

**Graph:** Demonstrates synchronization

---

## Advanced Patterns

### 12. Recursive Spawning
**Name:** `spawn`  
**Pattern:** Process creates new processes

```rust
{
    a!(0),
    for(a->x){
        {
            a!(*x),
            for(a->y){*y}
        }
    }
}
```

**Behavior:**
- Listener spawns sender and new listener
- Can communicate again
- Demonstrates process creation

**Graph:** Multiple steps before termination

---

### 13. Name Generation
**Name:** `fresh_names`  
**Pattern:** Using bound variables as channels

```rust
{
    for(new->chan){
        {chan!(0), for(chan->x){*x}}
    },
    new!(@(0))
}
```

**Behavior:**
- Receives fresh channel name via `new`
- Uses that name for internal communication
- Result: `0`

**Graph:** Demonstrates capability passing

---

### 14. Contract Net
**Name:** `contract`  
**Pattern:** Broadcast request, collect responses

```rust
{
    req!(0),
    for(req->x){{resp1!(*x), resp2!(*x)}},
    for(resp1->a){*a},
    for(resp2->b){*b}
}
```

**Behavior:**
- One request spawns two responses
- Both responses get processed
- Result: `{0, 0}`

**Graph:** Fork-join pattern

---

### 15. Deadlock Example
**Name:** `deadlock`  
**Pattern:** Circular dependency

```rust
{
    for(a->x){for(b->y){{*x, *y}}},
    for(b->z){a!(z)}
}
```

**Behavior:**
- First process waits on `a`, then `b`
- Second process waits on `b` before sending on `a`
- Neither can proceed!
- Normal form: stuck state

**Graph:** Demonstrates deadlock detection

---

### 16. Dining Philosophers (2 Philosophers)
**Name:** `philosophers`  
**Pattern:** Resource contention

```rust
{
    fork1!(0), fork2!(0),
    for(fork1->f1){for(fork2->f2){done1!({*f1, *f2})}},
    for(fork2->f2){for(fork1->f1){done2!({*f2, *f1})}}
}
```

**Behavior:**
- Two philosophers compete for two forks
- Order determines who eats first
- Potential for deadlock if modified

**Graph:** Race condition, multiple outcomes

---

## Performance Benchmarks

### 17. Wide Fanout
**Name:** `fanout`  
**Pattern:** One-to-many broadcast

```rust
{
    bcast!(0),
    for(bcast->x){{
        a!(*x), b!(*x), c!(*x), d!(*x),
        e!(*x), f!(*x), g!(*x), h!(*x)
    }},
    for(a->_){*_}, for(b->_){*_},
    for(c->_){*_}, for(d->_){*_},
    for(e->_){*_}, for(f->_){*_},
    for(g->_){*_}, for(h->_){*_}
}
```

**Behavior:**
- One sender, 8 receivers
- Demonstrates parallel reduction
- Performance benchmark

**Graph:** Large graph, many rewrites

---

### 18. Deep Pipeline
**Name:** `pipeline`  
**Pattern:** Sequential message passing

```rust
{
    a!(0),
    for(a->x){b!(*x)},
    for(b->x){c!(*x)},
    for(c->x){d!(*x)},
    for(d->x){e!(*x)},
    for(e->x){f!(*x)},
    for(f->x){*x}
}
```

**Behavior:**
- Message flows through 6 stages
- Linear chain of rewrites
- Depth benchmark

**Graph:** Deep but narrow

---

## Testing Edge Cases

### 19. Empty Parallel
**Name:** `empty`  
**Pattern:** Degenerate case

```rust
{}
```

**Behavior:**
- Empty process (equivalent to `0` by flattening)
- Tests identity normalization

**Graph:** 1 term (normalizes immediately)

---

### 20. Self-Communication
**Name:** `self_comm`  
**Pattern:** Process talks to itself

```rust
{
    for(x->y){x!(y)},
    x!(@(0))
}
```

**Behavior:**
- Receives name on `x`
- Sends same name back on `x`
- Infinite loop!

**Graph:** Demonstrates infinite behavior

---

## Summary Statistics

| Example | Terms | Rewrites | Normal Forms | Complexity |
|---------|-------|----------|--------------|------------|
| simple_comm | 3 | 2 | 1 | Trivial |
| sequential | ~7 | ~6 | 1 | Easy |
| reflection | 3 | 2 | 1 | Easy |
| choice | ~5 | ~4 | 2 | Medium |
| forward | 4 | 3 | 1 | Easy |
| multi_path | 50 | 66 | 13 | Complex |
| parallel | ~50 | ~100 | 1 | Complex |
| barrier | ~3 | ~2 | 1 | Medium |
| deadlock | 1 | 0 | 1 | Stuck |
| fanout | ~100+ | ~150+ | 1 | Large |
| pipeline | ~7 | ~6 | 1 | Deep |

---

## Usage in REPL

These examples will be accessible via:

```
rhocalc> example simple_comm
rhocalc> example multi_path
rhocalc> list-examples
```

See implementation in `mettail-repl/src/examples.rs`

