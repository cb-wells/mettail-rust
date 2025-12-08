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
            },
            _ => {},
        }
    }

    // Add the last segment
    if start <= s.len() {
        result.push(&s[start..]);
    }

    result
}

/// Normalize whitespace in a string by replacing all consecutive whitespace
/// (including newlines) with a single space. This fixes formatting issues
/// from TokenStream::to_string() which can insert unwanted line breaks.
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn print_rule(line: &str) {
    if line.trim().is_empty() {
        return;
    }

    // Normalize whitespace to fix TokenStream formatting issues
    let normalized = normalize_whitespace(line);

    let (head, body) = normalized
        .split_once("<- -")
        .unwrap_or((normalized.trim(), ""));
    let clauses = split_commas_outside_parens(body);
    let (last, rest) = clauses.split_last().unwrap_or((&"", &[]));
    if !body.trim().is_empty() {
        eprintln!("{} <--", head.trim());
        for clause in rest {
            eprintln!("    {},", clause.trim());
        }
        eprintln!("    {};", last.trim());
    } else {
        eprintln!("{};", normalized.trim());
    }
}
