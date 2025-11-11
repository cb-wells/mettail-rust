/// Pretty-printing utilities for terms
/// 
/// Provides indented multi-line formatting for complex terms.

use std::fmt;

/// Trait for pretty-printing with indentation
pub trait PrettyPrint {
    fn pretty_print(&self) -> String {
        let mut output = String::new();
        self.pretty_print_indent(&mut output, 0);
        output
    }
    
    fn pretty_print_indent(&self, output: &mut String, indent: usize);
}

pub fn indent_str(level: usize) -> String {
    "    ".repeat(level)
}

pub fn format_term_pretty(term_str: &str) -> String {
    // Simple heuristic-based pretty printer
    // Works on the display string since we don't have access to the AST here
    
    let mut result = String::new();
    let mut indent = 0;
    let mut chars = term_str.chars().peekable();
    let mut at_line_start = true;
    let mut is_first_brace = true;
    
    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                result.push(ch);
                
                // First brace stays on same line, subsequent ones get newlines
                if is_first_brace {
                    is_first_brace = false;
                    at_line_start = true;
                } else {
                    indent += 1;
                    result.push('\n');
                    result.push_str(&indent_str(indent));
                    at_line_start = true;
                }
            }
            '}' => {
                if !at_line_start {
                    result.push(' ');
                }
                indent = indent.saturating_sub(1);
                result.push(ch);
                at_line_start = false;
            }
            ',' => {
                result.push(ch);
                // Check if we're inside a collection
                if let Some(&next) = chars.peek() {
                    if next != ' ' {
                        result.push(' ');
                    }
                }
                result.push('\n');
                result.push_str(&indent_str(indent));
                at_line_start = true;
            }
            ' ' if at_line_start => {
                // Skip leading spaces at line start (we already indented)
            }
            _ => {
                result.push(ch);
                at_line_start = false;
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_pretty_print() {
        let input = "{a!(0), for(a->x){*x}}";
        let output = format_term_pretty(input);
        
        assert!(output.contains('\n'));
        assert!(output.contains("    ")); // Has indentation
    }
    
    #[test]
    fn test_nested_pretty_print() {
        let input = "{for(fork1->f1){for(fork2->f2){done1!({*f1, *f2})}}}";
        let output = format_term_pretty(input);
        
        // Check multiple indentation levels
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() > 3);
    }
}

