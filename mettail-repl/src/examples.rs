/// Library of example processes for exploration
/// 
/// Each example demonstrates a different communication pattern or execution behavior.

pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub category: ExampleCategory,
    pub theory: TheoryName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheoryName {
    RhoCalculus,
    AmbientCalculus,
}

impl TheoryName {
    pub fn as_str(&self) -> &'static str {
        match self {
            TheoryName::RhoCalculus => "rhocalc",
            TheoryName::AmbientCalculus => "ambient",
        }
    }
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
    Mobility,      // For ambient calculus
    Security,      // For ambient calculus
}

impl Example {
    pub fn all() -> Vec<&'static Example> {
        vec![
            // RhoCalculus examples
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
            // Ambient Calculus examples
            &AMB_ENTER,
            &AMB_EXIT,
            &AMB_OPEN,
            &AMB_FIREWALL,
            &AMB_MOBILE_AGENT,
            &AMB_COLOCATED,
            &AMB_NESTED_MOBILITY,
            &AMB_CAPABILITY_PASSING,
            &AMB_SAFE_AMBIENT,
            &AMB_PARENT_CHILD,
            // Rest pattern demonstrations
            &AMB_REST_EMPTY,
            &AMB_REST_NONEMPTY,
            &AMB_REST_CONTEXT,
            &AMB_REST_NESTED,
            &AMB_REST_MULTIPLE,
            &AMB_REST_PRESERVATION,
            &AMB_REST_COMPLEX,
        ]
    }

    pub fn by_name(name: &str) -> Option<&'static Example> {
        Self::all().into_iter().find(|e| e.name == name)
    }

    pub fn by_category(_theory: TheoryName, cat: ExampleCategory) -> Vec<&'static Example> {
        Self::all().into_iter().filter(|e| e.category == cat).collect()
    }       
    
    pub fn by_theory(theory: TheoryName) -> Vec<&'static Example> {
        Self::all().into_iter().filter(|e| e.theory == theory).collect()
    }
    
    pub fn by_theory_and_category(theory: TheoryName, cat: ExampleCategory) -> Vec<&'static Example> {
        Self::all().into_iter()
            .filter(|e| e.theory == theory && e.category == cat)
            .collect()
    }
}

//=============================================================================
// RHO CALCULUS EXAMPLES
//=============================================================================

// Simple Examples

pub static SIMPLE_COMM: Example = Example {
    name: "simple_comm",
    description: "Basic communication: single channel, immediate communication",
    source: "{a!(0), for(a->x){*x}}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

pub static SEQUENTIAL: Example = Example {
    name: "sequential",
    description: "Two independent channels communicating in parallel",
    source: "{a!(0), for(a->x){*x}, b!(0), for(b->y){*y}}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

pub static REFLECTION: Example = Example {
    name: "reflection",
    description: "Quote/drop cycle demonstrating reflection",
    source: "{for(@(0)->x){*x}, @(0)!(0)}",
    category: ExampleCategory::Simple,
    theory: TheoryName::RhoCalculus,
};

// Branching Examples

pub static CHOICE: Example = Example {
    name: "choice",
    description: "Non-deterministic choice: multiple listeners on same channel",
    source: "{a!(0), for(a->x){x!(0)}, for(a->y){y!(0)}}",
    category: ExampleCategory::Branching,
    theory: TheoryName::RhoCalculus,
};

pub static RACE: Example = Example {
    name: "race",
    description: "Race condition: multiple senders, one listener",
    source: "{a!(0), a!(1), for(a->x){*x}}",
    category: ExampleCategory::Branching,
    theory: TheoryName::RhoCalculus,
};

// Complex Patterns

pub static FORWARD: Example = Example {
    name: "forward",
    description: "Relay messages between channels (forwarder pattern)",
    source: "{a!(0), for(a->x){b!(*x)}, for(b->y){*y}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
};

pub static CIRCULAR: Example = Example {
    name: "circular",
    description: "Circular communication (infinite loop, no normal form)",
    source: "{a!(0), for(a->x){b!(*x)}, for(b->y){a!(*y)}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
};

pub static HANDSHAKE: Example = Example {
    name: "handshake",
    description: "Three-way handshake protocol",
    source: "{a!(0), for(a->x){{b!(*x), for(c->z){*z}}}, for(b->y){c!(*y)}}",
    category: ExampleCategory::Complex,
    theory: TheoryName::RhoCalculus,
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
    theory: TheoryName::RhoCalculus,
};

pub static PARALLEL: Example = Example {
    name: "parallel",
    description: "Four independent parallel processes (many execution orders)",
    source: "{
        a!(0), for(a->x){*x},
        b!(0), for(b->y){*y},
        c!(0), for(c->z){*z},
        d!(0), for(d->w){*w}
    }",
    category: ExampleCategory::Parallel,
    theory: TheoryName::RhoCalculus,
};

pub static BARRIER: Example = Example {
    name: "barrier",
    description: "Barrier synchronization: wait for all inputs",
    source: "{a!(0), b!(0), for(a->x){for(b->y){{*x, *y}}}}",
    category: ExampleCategory::Parallel,
    theory: TheoryName::RhoCalculus,
};

// Advanced Patterns

pub static SPAWN: Example = Example {
    name: "spawn",
    description: "Recursive spawning: process creates new processes",
    source: "{a!(0), for(a->x){{a!(*x), for(a->y){*y}}}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static FRESH_NAMES: Example = Example {
    name: "fresh_names",
    description: "Name generation via bound variables (capability passing)",
    source: "{for(new->chan){{chan!(0), for(chan->x){*x}}}, new!(@(0))}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static CONTRACT: Example = Example {
    name: "contract",
    description: "Contract net: broadcast request, collect responses",
    source: "{req!(0), for(req->x){{resp1!(*x), resp2!(*x)}}, for(resp1->a){*a}, for(resp2->b){*b}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static DEADLOCK: Example = Example {
    name: "deadlock",
    description: "Deadlock: circular dependency, no progress possible",
    source: "{for(a->x){for(b->y){{*x, *y}}}, for(b->z){a!(z)}}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
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
    theory: TheoryName::RhoCalculus,
};

pub static REPLICATED_INPUT: Example = Example {
    name: "replicated_input",
    description: "Replication encoding: persistent input listener",
    source: "{
        for(dup->y){ {*y, dup!(*y)} },
        dup!(for(req->x){
            { resp!(*x), for(dup->y){ {*y, dup!(*y)} } }
        }),
        req!(0)
    }",
    category: ExampleCategory::Advanced,
    theory: TheoryName::RhoCalculus,
};

pub static DROP_QUOTE_TEST: Example = Example {
    name: "drop_quote_test",
    description: "Test the *@(P) => P rewrite rule",
    source: "{*@(0), a!(0)}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::RhoCalculus,
};

// Performance Benchmarks

pub static FANOUT: Example = Example {
    name: "fanout",
    description: "Wide fanout: one-to-many broadcast (performance benchmark)",
    source: "{
        bcast!(0),
        for(bcast->x){{a!(*x), b!(*x), c!(*x), d!(*x), e!(*x), f!(*x), g!(*x), h!(*x)}},
        for(a->y){*y}, for(b->y){*y}, for(c->y){*y}, for(d->y){*y},
        for(e->y){*y}, for(f->y){*y}, for(g->y){*y}, for(h->y){*y}
    }",
    category: ExampleCategory::Performance,
    theory: TheoryName::RhoCalculus,
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
    theory: TheoryName::RhoCalculus,
};

// Edge Cases

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
    source: "{for(x->y){x!(y)}, x!(@(0))}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::RhoCalculus,
};

//=============================================================================
// AMBIENT CALCULUS EXAMPLES
//=============================================================================

// Simple Mobility Examples

pub static AMB_ENTER: Example = Example {
    name: "amb_enter",
    description: "Basic ambient entry: n[{in(m,p)}] | m[r] => m[{n[{p}], r}] (rest is empty)",
    source: "{n[{in(m,p)}], m[r]}",
    category: ExampleCategory::Simple,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_EXIT: Example = Example {
    name: "amb_exit",
    description: "Ambient exit: m[{n[{out(m,p)}], r}] => {n[{p}], m[r]} (rest is empty)",
    source: "m[{n[{out(m,p)}], r}]",
    category: ExampleCategory::Simple,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_OPEN: Example = Example {
    name: "amb_open",
    description: "Open ambient: {open(n,p), n[q]} => {p, q}",
    source: "{open(n,p), n[q]}",
    category: ExampleCategory::Simple,
    theory: TheoryName::AmbientCalculus,
};

// Mobility Patterns

pub static AMB_FIREWALL: Example = Example {
    name: "amb_firewall",
    description: "Firewall pattern: agent enters trusted zone but not untrusted",
    source: "{firewall[{agent[{in(firewall,0)}], trusted[0]}], untrusted[{agent[{in(untrusted,0)}]}]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_MOBILE_AGENT: Example = Example {
    name: "amb_mobile_agent",
    description: "Mobile agent: travels through multiple locations sequentially",
    source: "{agent[{in(loc1, in(loc2, 0))}], loc1[0], loc2[0]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_COLOCATED: Example = Example {
    name: "amb_colocated",
    description: "Co-location: two ambients enter the same parent",
    source: "{parent[0], child1[{in(parent,0)}], child2[{in(parent,0)}]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_NESTED_MOBILITY: Example = Example {
    name: "amb_nested_mobility",
    description: "Nested mobility: child moves with parent",
    source: "{grandparent[0], parent[{in(grandparent,0), child[0]}]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

// Advanced Patterns

pub static AMB_CAPABILITY_PASSING: Example = Example {
    name: "amb_capability_passing",
    description: "Capability passing: new creates fresh ambient names",
    source: "new(x, {agent[{in(x,0)}], x[0]})",
    category: ExampleCategory::Advanced,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_SAFE_AMBIENT: Example = Example {
    name: "amb_safe_ambient",
    description: "Safe ambient: only authorized agents can access resources",
    source: "{safe[{open(key,0)}], key[{secret[0]}], agent[{in(safe,0), open(key,0)}]}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_PARENT_CHILD: Example = Example {
    name: "amb_parent_child",
    description: "Parent-child coordination: child exits and parent opens",
    source: "parent[{child[{out(parent,0)}], open(child,result)}]",
    category: ExampleCategory::Advanced,
    theory: TheoryName::AmbientCalculus,
};

//=============================================================================
// REST PATTERN DEMONSTRATIONS
//=============================================================================

pub static AMB_REST_EMPTY: Example = Example {
    name: "amb_rest_empty",
    description: "Rest pattern with empty context: demonstrates ...rest matching empty bag",
    source: "{n[{in(m,p)}], m[r]}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_NONEMPTY: Example = Example {
    name: "amb_rest_nonempty",
    description: "Rest pattern with non-empty context: ...rest captures additional processes",
    source: "{n[{in(m,p), q, s}], m[r]}",
    category: ExampleCategory::EdgeCase,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_CONTEXT: Example = Example {
    name: "amb_rest_context",
    description: "Context preservation: rest maintains ambient state during mobility",
    source: "{n[{in(m,p), state1, state2, counter}], m[{r, local}]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_NESTED: Example = Example {
    name: "amb_rest_nested",
    description: "Nested rest patterns: multiple levels of context preservation",
    source: "{outer[{inner[{in(target, data), ctx1}], ctx2}], target[base]}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_MULTIPLE: Example = Example {
    name: "amb_rest_multiple",
    description: "Multiple mobility operations with independent contexts",
    source: "{a[{in(parent, x), ctxA}], b[{in(parent, y), ctxB}], parent[z]}",
    category: ExampleCategory::Parallel,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_PRESERVATION: Example = Example {
    name: "amb_rest_preservation",
    description: "Sequential mobility preserving context through multiple steps",
    source: "{agent[{in(loc1, in(loc2, done)), state}], loc1[0], loc2[0]}",
    category: ExampleCategory::Mobility,
    theory: TheoryName::AmbientCalculus,
};

pub static AMB_REST_COMPLEX: Example = Example {
    name: "amb_rest_complex",
    description: "Complex interaction: entry, exit, and open with context preservation",
    source: "{container[{child[{out(container, result), data}], open(child, final)}], observer}",
    category: ExampleCategory::Advanced,
    theory: TheoryName::AmbientCalculus,
};
