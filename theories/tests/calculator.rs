use mettail_theories::calculator::parse_and_eval;

#[test]
fn test_numeric_literal() {
    assert_eq!(parse_and_eval("3").unwrap(), 3);
}

#[test]
fn test_addition() {
    assert_eq!(parse_and_eval("3 + 3").unwrap(), 6);
    assert_eq!(parse_and_eval("10+5").unwrap(), 15);
}

#[test]
fn test_subtraction() {
    assert_eq!(parse_and_eval("5-2").unwrap(), 3);
    assert_eq!(parse_and_eval("10 - 7").unwrap(), 3);
}

#[test]
fn test_left_associativity() {
    // (1+2)-3 == 0 and 1+2-3 parsed left-to-right -> (1+2)-3
    assert_eq!(parse_and_eval("1+2-3").unwrap(), 0);
    assert_eq!(parse_and_eval("(1+2)-3").unwrap(), 0);
}
