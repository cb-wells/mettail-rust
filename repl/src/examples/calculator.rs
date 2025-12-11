// Calculator example expressions for the REPL
//
// Demonstrates integer arithmetic evaluation in the Calculator theory

use super::{Example, ExampleCategory, TheoryName};

pub fn all() -> Vec<&'static Example> {
    vec![&SIMPLE_NUMBER, &ADDITION, &SUBTRACTION, &COMPLEX_EXPRESSION]
}

//=============================================================================
// EXAMPLES
//=============================================================================

pub static SIMPLE_NUMBER: Example = Example {
    name: "simple_number",
    description: "Single integer literal",
    source: "42",
    category: ExampleCategory::Simple,
    theory: TheoryName::Calculator,
};

pub static ADDITION: Example = Example {
    name: "addition",
    description: "Simple addition of two numbers",
    source: "5 + 3",
    category: ExampleCategory::Simple,
    theory: TheoryName::Calculator,
};

pub static SUBTRACTION: Example = Example {
    name: "subtraction",
    description: "Simple subtraction",
    source: "10 - 4",
    category: ExampleCategory::Simple,
    theory: TheoryName::Calculator,
};

pub static COMPLEX_EXPRESSION: Example = Example {
    name: "complex_expr",
    description: "Multi-operator expression with left-to-right evaluation",
    source: "10 + 5 - 3 + 2",
    category: ExampleCategory::Simple,
    theory: TheoryName::Calculator,
};
