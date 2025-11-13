pub fn split_commas_outside_parens(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                result.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    
    // Add the last segment
    if start <= s.len() {
        result.push(&s[start..]);
    }
    
    result
}

pub fn print_rule(line: &str) {
    if line.trim().is_empty() {
        return;
    }
    let (head, body) = line.split_once("<- -").unwrap_or((line.trim(), ""));
    let clauses = split_commas_outside_parens(body);
    let (last, rest) = clauses.split_last().unwrap_or((&"", &[]));
    if !body.trim().is_empty() {
        eprintln!("{} <--", head.trim());
        for clause in rest {
            eprintln!("    {},", clause.trim());
        }
        eprintln!("    {};", last.trim());
    } else {
        eprintln!("{};", line.trim());
    }
}