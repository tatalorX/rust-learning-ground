// Exercise 148: Valid Parentheses (Stack)
//
// Learning Objective:
// Implement a stack-based solution to check if a string of parentheses
// is valid (properly opened and closed).
//
// Key Concepts:
// - Stack data structure (LIFO)
// - Matching pairs
// - Balanced expression checking

/// TODO: Check if string of brackets is valid
/// 
/// Valid if:
/// - Open brackets are closed by the same type
/// - Open brackets are closed in the correct order
/// - Every close bracket has a corresponding open bracket
/// 
/// Pairs: () [] {}
/// 
/// Algorithm:
/// 1. Use a stack to track open brackets
/// 2. For each character:
///    - If open bracket: push to stack
///    - If close bracket: check if stack top matches, if not return false
/// 3. Return true if stack is empty at end
fn is_valid(s: &str) -> bool {
    // TODO: Create a stack (Vec can be used as stack)
    // Iterate through characters
    // Push open brackets, pop and check for close brackets
    // Return stack.is_empty() at the end
    
    false
}

/// TODO: Get the matching opening bracket for a closing bracket
fn matching_open(close: char) -> Option<char> {
    // TODO: Return matching open bracket
    // ')' -> '(', ']' -> '[', '}' -> '{'
    
    None
}

/// TODO: Check if character is an opening bracket
fn is_open(c: char) -> bool {
    // TODO: Return true for '(', '[', '{'
    
    false
}

/// TODO: Check if character is a closing bracket
fn is_close(c: char) -> bool {
    // TODO: Return true for ')', ']', '}'
    
    false
}

/// TODO: Find the position of first invalid character
/// Returns Some(index) if invalid, None if valid
fn find_invalid_position(s: &str) -> Option<usize> {
    // TODO: Similar to is_valid but track position
    // Return position when mismatch found or unexpected close bracket
    
    None
}

/// TODO: Minimum number of brackets to add to make string valid
fn min_add_to_make_valid(s: &str) -> usize {
    // TODO: Count unmatched open and close brackets
    // Return total needed to balance
    
    0
}

/// TODO: Check if string can be valid by removing exactly one bracket
fn can_be_valid_by_removing_one(s: &str) -> bool {
    // BONUS CHALLENGE
    // Check if removing any single bracket makes the string valid
    
    false
}

/// TODO: Longest valid parentheses substring length
fn longest_valid_parentheses(s: &str) -> usize {
    // BONUS CHALLENGE
    // Find length of longest valid parentheses substring
    // Use stack or DP approach
    
    0
}

fn main() {
    let test_cases = vec![
        "()",
        "()[]{}",
        "(]",
        "([)]",
        "{[]}",
        "",
        "((",
        "))",
        "([{}])",
    ];
    
    println!("Valid Parentheses Check:\n");
    for s in test_cases {
        let result = is_valid(s);
        let status = if result { "VALID" } else { "INVALID" };
        println!("'{}' -> {}", s, status);
    }
    
    // Additional examples
    println!("\nMinimum brackets to add:");
    for s in vec!["())", "(((", ")", "()"] {
        println!("'{}' needs {} brackets", s, min_add_to_make_valid(s));
    }
    
    println!("\nLongest valid substring:");
    for s in vec!["(()", ")()())", "") {
        println!("'{}' longest valid: {}", s, longest_valid_parentheses(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_simple() {
        assert!(is_valid("()"));
        assert!(is_valid("[]"));
        assert!(is_valid("{}"));
    }
    
    #[test]
    fn test_valid_combined() {
        assert!(is_valid("()[]{}"));
        assert!(is_valid("{[]}"));
        assert!(is_valid("([{}])"));
    }
    
    #[test]
    fn test_invalid_mismatch() {
        assert!(!is_valid("(]"));
        assert!(!is_valid("([)]"));
        assert!(!is_valid("{(})"));
    }
    
    #[test]
    fn test_invalid_unbalanced() {
        assert!(!is_valid("("));
        assert!(!is_valid("]"));
        assert!(!is_valid("(("));
        assert!(!is_valid("))"));
    }
    
    #[test]
    fn test_empty() {
        assert!(is_valid(""));
    }
    
    #[test]
    fn test_nested() {
        assert!(is_valid("((()))"));
        assert!(is_valid("[[[[]]]]"));
        assert!(is_valid("{{{{}}}}"));
        assert!(is_valid("({[]})"));
    }
    
    #[test]
    fn test_min_add() {
        assert_eq!(min_add_to_make_valid("())"), 1);
        assert_eq!(min_add_to_make_valid("((("), 3);
        assert_eq!(min_add_to_make_valid(")"), 1);
        assert_eq!(min_add_to_make_valid("()"), 0);
        assert_eq!(min_add_to_make_valid("()))(("), 4);
    }
    
    #[test]
    fn test_find_invalid() {
        assert_eq!(find_invalid_position("(]"), Some(1));
        assert_eq!(find_invalid_position("()"), None);
        assert_eq!(find_invalid_position("(("), Some(2));
    }
    
    #[test]
    fn test_longest_valid() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses(")()())"), 4);
        assert_eq!(longest_valid_parentheses(""), 0);
        assert_eq!(longest_valid_parentheses("()(()"), 4);
    }
}
