// RhoCalc example processes for the REPL
//
// Demonstrates various communication patterns in the Rho Calculus

use super::{Example, ExampleCategory, TheoryName};

pub fn all() -> Vec<&'static Example> {
    vec![
        &SIMPLE_COMM,
        &SEQUENTIAL,
        &REFLECTION,
        &CHOICE,
        &RACE,
        &FORWARD,
        &CIRCULAR,
        &HANDSHAKE,
        &MULTI_PATH,
        &PARALLEL,
        &BARRIER,
        &SPAWN,
        &FRESH_NAMES,
        &CONTRACT,
        &DEADLOCK,
        &PHILOSOPHERS,
        &REPLICATED_INPUT,
        &FANOUT,
        &PIPELINE,
        &EMPTY,
        &SELF_COMM,
        &DROP_QUOTE_TEST,
    ]
}

//=============================================================================
// SIMPLE EXAMPLES
//=============================================================================

pub static SIMPLE_COMM: Example = Example {
    name: "simple_comm",
    description: "Basic communication: single channel, immediate communication",
    source: "{a!(0) | for(a->x){*x}}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

pub static SEQUENTIAL: Example = Example {
    name: "sequential",
    description: "Two independent channels communicating in parallel",
    source: "{a!(0) | for(a->x){*x} | b!(0) | for(b->y){*y}}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

pub static REFLECTION: Example = Example {
    name: "reflection",
    description: "Quote/drop cycle demonstrating reflection",
    source: "{for(@(0)->x){*x} | @(0)!(0)}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// BRANCHING EXAMPLES
//=============================================================================

pub static CHOICE: Example = Example {
    name: "choice",
    description: "Non-deterministic choice: multiple listeners on same channel",
    source: "{a!(0) | for(a->x){x!(0)} | for(a->y){y!(0)}}",
    category: ExampleCategory::Branching,
    theory: TheoryName::RhoCalculus,
};

pub static RACE: Example = Example {
    name: "race",
    description: "Race condition: multiple senders, one listener",
    source: "{a!(0) | a!(1) | for(a->x){*x}}",
    category: ExampleCategory::Branching,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// COMPLEX PATTERNS
//=============================================================================

pub static FORWARD: Example = Example {
    name: "forward",
    description: "Relay messages between channels (forwarder pattern)",
    source: "{a!(0) | for(a->x){b!(*x)} | for(b->y){*y}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
};

pub static CIRCULAR: Example = Example {
    name: "circular",
    description: "Circular communication (infinite loop, no normal form)",
    source: "{a!(0) | for(a->x){b!(*x)} | for(b->y){a!(*y)}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
};

pub static HANDSHAKE: Example = Example {
    name: "handshake",
    description: "Three-way handshake protocol",
    source: "{a!(0) | for(a->x){{b!(*x) | for(c->z){*z}}} | for(b->y){c!(*y)}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// PARALLEL COMPUTATION
//=============================================================================

pub static MULTI_PATH: Example = Example {
    name: "multi_path",
    description: "Multiple concurrent communications with dependencies (50 terms, 66 rewrites)",
    source: "{
        a!(0) |
        for(a->x0){ {x0!(0) | for(b->y1){y1!(*a)}} } |
        b!(0) |
        for(b->x1){a!(*b)} |
        c!(0) |
        for(c->x2){x2!(0)} |
        for(@(0)->y0){*y0}
    }",
    category: ExampleCategory::Parallel,
    theory: TheoryName::RhoCalculus,
};

pub static PARALLEL: Example = Example {
    name: "parallel",
    description: "Four independent parallel processes (many execution orders)",
    source: "{
        a!(0) | for(a->x){*x} |
        b!(0) | for(b->y){*y} |
        c!(0) | for(c->z){*z} |
        d!(0) | for(d->w){*w}
    }",
    category: ExampleCategory::Parallel,
    theory: TheoryName::RhoCalculus,
};

pub static BARRIER: Example = Example {
    name: "barrier",
    description: "Barrier synchronization: wait for all inputs",
    source: "{a!(0) | b!(0) | for(a->x){for(b->y){{*x | *y}}}}",
    category: ExampleCategory::Parallel,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// ADVANCED PATTERNS
//=============================================================================

pub static SPAWN: Example = Example {
    name: "spawn",
    description: "Recursive spawning: process creates new processes",
    source: "{a!(0) | for(a->x){{a!(*x) | for(a->y){*y}}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static FRESH_NAMES: Example = Example {
    name: "fresh_names",
    description: "Name generation via bound variables (capability passing)",
    source: "{for(new->chan){{chan!(0) | for(chan->x){*x}}} | new!(@(0))}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static CONTRACT: Example = Example {
    name: "contract",
    description: "Contract net: broadcast request, collect responses",
    source: "{req!(0) | for(req->x){{resp1!(*x) | resp2!(*x)}} | for(resp1->a){*a} | for(resp2->b){*b}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static DEADLOCK: Example = Example {
    name: "deadlock",
    description: "Deadlock: circular dependency, no progress possible",
    source: "{for(a->x){for(b->y){{*x | *y}}} | for(b->z){a!(z)}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static PHILOSOPHERS: Example = Example {
    name: "philosophers",
    description: "Dining philosophers (2): resource contention",
    source: "{
        fork1!(0) | fork2!(0) |
        for(fork1->f1){for(fork2->f2){done1!({*(f1) | *(f2)})}} |
        for(fork2->f2){for(fork1->f1){done2!({*(f2) | *(f1)})}}
    }",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static REPLICATED_INPUT: Example = Example {
    name: "replicated_input",
    description: "Replication encoding: persistent input listener",
    source: "{
        for(dup->y){ {*y | dup!(*y)} },
        dup!(for(req->x){
            { resp!(*x) | for(dup->y){ {*y | dup!(*y)} } }
        }),
        req!(0)
    }",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static DROP_QUOTE_TEST: Example = Example {
    name: "drop_quote_test",
    description: "Test the *@(P) => P rewrite rule",
    source: "{*@(0) | a!(0)}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// PERFORMANCE BENCHMARKS
//=============================================================================

pub static FANOUT: Example = Example {
    name: "fanout",
    description: "Wide fanout: one-to-many broadcast (performance benchmark)",
    source: "{
        bcast!(0) |
        for(bcast->x){{a!(*x) | b!(*x) | c!(*x) | d!(*x) | e!(*x) | f!(*x) | g!(*x) | h!(*x)}},
        for(a->y){*y} | for(b->y){*y} | for(c->y){*y} | for(d->y){*y} |
        for(e->y){*y} | for(f->y){*y} | for(g->y){*y} | for(h->y){*y}
    }",
    category: ExampleCategory::Performance,
    theory: TheoryName::RhoCalculus,
};

pub static PIPELINE: Example = Example {
    name: "pipeline",
    description: "Deep pipeline: sequential message passing (depth benchmark)",
    source: "{
        a!(0) |
        for(a->x){b!(*x)} |
        for(b->x){c!(*x)} |
        for(c->x){d!(*x)} |
        for(d->x){e!(*x)} |
        for(e->x){f!(*x)} |
        for(f->x){*x}
    }",
    category: ExampleCategory::Performance,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// EDGE CASES
//=============================================================================

pub static EMPTY: Example = Example {
    name: "empty",
    description: "Empty parallel process (tests identity normalization)",
    source: "{}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::RhoCalculus,
};

pub static SELF_COMM: Example = Example {
    name: "self_comm",
    description: "Self-communication (infinite loop)",
    source: "{for(x->y){x!(y)} | x!(@(0))}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::RhoCalculus,
};

