/// Library of example RhoCalc processes for exploration
/// 
/// Each example demonstrates a different communication pattern or execution behavior.

pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub category: ExampleCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleCategory {
    Simple,
    Branching,
    Complex,
    Parallel,
    Advanced,
    Performance,
    EdgeCase,
}

impl Example {
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
            &FANOUT,
            &PIPELINE,
            &EMPTY,
            &SELF_COMM,
        ]
    }

    pub fn by_name(name: &str) -> Option<&'static Example> {
        Self::all().into_iter().find(|e| e.name == name)
    }

    pub fn by_category(cat: ExampleCategory) -> Vec<&'static Example> {
        Self::all().into_iter().filter(|e| e.category == cat).collect()
    }
}

// Simple Examples

pub static SIMPLE_COMM: Example = Example {
    name: "simple_comm",
    description: "Basic communication: single channel, immediate communication",
    source: "{a!(0), for(a->x){*x}}",
    category: ExampleCategory::Simple,
};

pub static SEQUENTIAL: Example = Example {
    name: "sequential",
    description: "Two independent channels communicating in parallel",
    source: "{a!(0), for(a->x){*x}, b!(0), for(b->y){*y}}",
    category: ExampleCategory::Simple,
};

pub static REFLECTION: Example = Example {
    name: "reflection",
    description: "Quote/drop cycle demonstrating reflection",
    source: "{for(@(0)->x){*x}, @(0)!(0)}",
    category: ExampleCategory::Simple,
};

// Branching Examples

pub static CHOICE: Example = Example {
    name: "choice",
    description: "Non-deterministic choice: multiple listeners on same channel",
    source: "{a!(0), for(a->x){x!(0)}, for(a->y){y!(0)}}",
    category: ExampleCategory::Branching,
};

pub static RACE: Example = Example {
    name: "race",
    description: "Race condition: multiple senders, one listener",
    source: "{a!(0), a!(1), for(a->x){*x}}",
    category: ExampleCategory::Branching,
};

// Complex Patterns

pub static FORWARD: Example = Example {
    name: "forward",
    description: "Relay messages between channels (forwarder pattern)",
    source: "{a!(0), for(a->x){b!(*x)}, for(b->y){*y}}",
    category: ExampleCategory::Complex,
};

pub static CIRCULAR: Example = Example {
    name: "circular",
    description: "Circular communication (infinite loop, no normal form)",
    source: "{a!(0), for(a->x){b!(*x)}, for(b->y){a!(*y)}}",
    category: ExampleCategory::Complex,
};

pub static HANDSHAKE: Example = Example {
    name: "handshake",
    description: "Three-way handshake protocol",
    source: "{a!(0), for(a->x){{b!(*x), for(c->z){*z}}}, for(b->y){c!(*y)}}",
    category: ExampleCategory::Complex,
};

// Parallel Computation

pub static MULTI_PATH: Example = Example {
    name: "multi_path",
    description: "Multiple concurrent communications with dependencies (50 terms, 66 rewrites)",
    source: "{
        a!(0),
        for(a->x0){ {x0!(0), for(b->y1){y1!(*a)}} },
        b!(0),
        for(b->x1){a!(*b)},
        c!(0),
        for(c->x2){x2!(0)},
        for(@(0)->y0){*y0}
    }",
    category: ExampleCategory::Parallel,
};

pub static PARALLEL: Example = Example {
    name: "parallel",
    description: "Four independent parallel processes (many execution orders)",
    source: "{
        a!(0), for(a->x){*x},
        b!(1), for(b->y){*y},
        c!(2), for(c->z){*z},
        d!(3), for(d->w){*w}
    }",
    category: ExampleCategory::Parallel,
};

pub static BARRIER: Example = Example {
    name: "barrier",
    description: "Barrier synchronization: wait for all inputs",
    source: "{a!(0), b!(0), for(a->x){for(b->y){{*x, *y}}}}",
    category: ExampleCategory::Parallel,
};

// Advanced Patterns

pub static SPAWN: Example = Example {
    name: "spawn",
    description: "Recursive spawning: process creates new processes",
    source: "{a!(0), for(a->x){{a!(*x), for(a->y){*y}}}}",
    category: ExampleCategory::Advanced,
};

pub static FRESH_NAMES: Example = Example {
    name: "fresh_names",
    description: "Name generation via bound variables (capability passing)",
    source: "{for(new->chan){{chan!(0), for(chan->x){*x}}}, new!(@(0))}",
    category: ExampleCategory::Advanced,
};

pub static CONTRACT: Example = Example {
    name: "contract",
    description: "Contract net: broadcast request, collect responses",
    source: "{req!(0), for(req->x){{resp1!(*x), resp2!(*x)}}, for(resp1->a){*a}, for(resp2->b){*b}}",
    category: ExampleCategory::Advanced,
};

pub static DEADLOCK: Example = Example {
    name: "deadlock",
    description: "Deadlock: circular dependency, no progress possible",
    source: "{for(a->x){for(b->y){{*x, *y}}}, for(b->z){a!(z)}}",
    category: ExampleCategory::Advanced,
};

pub static PHILOSOPHERS: Example = Example {
    name: "philosophers",
    description: "Dining philosophers (2): resource contention",
    source: "{
        fork1!(0), fork2!(0),
        for(fork1->f1){for(fork2->f2){done1!({*f1, *f2})}},
        for(fork2->f2){for(fork1->f1){done2!({*f2, *f1})}}
    }",
    category: ExampleCategory::Advanced,
};

// Performance Benchmarks

pub static FANOUT: Example = Example {
    name: "fanout",
    description: "Wide fanout: one-to-many broadcast (performance benchmark)",
    source: "{
        bcast!(0),
        for(bcast->x){{a!(*x), b!(*x), c!(*x), d!(*x), e!(*x), f!(*x), g!(*x), h!(*x)}},
        for(a->_){*_}, for(b->_){*_}, for(c->_){*_}, for(d->_){*_},
        for(e->_){*_}, for(f->_){*_}, for(g->_){*_}, for(h->_){*_}
    }",
    category: ExampleCategory::Performance,
};

pub static PIPELINE: Example = Example {
    name: "pipeline",
    description: "Deep pipeline: sequential message passing (depth benchmark)",
    source: "{
        a!(0),
        for(a->x){b!(*x)},
        for(b->x){c!(*x)},
        for(c->x){d!(*x)},
        for(d->x){e!(*x)},
        for(e->x){f!(*x)},
        for(f->x){*x}
    }",
    category: ExampleCategory::Performance,
};

// Edge Cases

pub static EMPTY: Example = Example {
    name: "empty",
    description: "Empty parallel process (tests identity normalization)",
    source: "{}",
    category: ExampleCategory::EdgeCase,
};

pub static SELF_COMM: Example = Example {
    name: "self_comm",
    description: "Self-communication (infinite loop)",
    source: "{for(x->y){x!(y)}, x!(@(0))}",
    category: ExampleCategory::EdgeCase,
};

