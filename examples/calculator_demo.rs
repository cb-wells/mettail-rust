use mettail_theories::calculator::*;

fn main() {
    let tests = vec!["3", "3 + 3", "5-2", "1+2-3", "(1+2)-3"];

    for t in tests {
        match parse_and_eval(t) {
            Ok(v) => println!("Input: {:<10} Output: {}", t, v),
            Err(e) => println!("Input: {:<10} Parse error: {:?}", t, e),
        }
    }
}
