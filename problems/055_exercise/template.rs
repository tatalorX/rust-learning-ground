// Exercise 055: Option - unwrap and expect
//
// Option<T> represents an optional value: either Some(T) or None.
// unwrap() extracts the value from Some, or panics on None.
// expect() is like unwrap() with a custom panic message.
//
// Learning Objectives:
// - Use unwrap() and expect() on Option types
// - Use unwrap_or() and unwrap_or_else() for safe alternatives
// - Understand when panicking is appropriate
//
// Your task: Implement functions that safely extract values from Options.

/// Returns the first element of a slice, or 0 if empty.
fn first_or_zero(numbers: &[i32]) -> i32 {
    // TODO: Get the first element using .first(), use unwrap_or(0)
    todo!()
}

/// Returns the first character of a string, or '?' if empty.
fn first_char_or_default(text: &str) -> char {
    // TODO: Get the first character using .chars().next(), use unwrap_or('?')
    todo!()
}

/// Returns the last element of a vector, panicking with "Empty vector" if empty.
fn last_element_required(numbers: &[i32]) -> i32 {
    // TODO: Get the last element using .last(), use expect() with message "Empty vector"
    todo!()
}

/// Returns the length of the longest string, or 0 if the vector is empty.
fn max_length(strings: &[&str]) -> usize {
    // TODO: Use iter().map(|s| s.len()).max() which returns Option<usize>
    // Use unwrap_or(0) to handle the empty case
    todo!()
}

fn main() {
    // Test first_or_zero
    let numbers = vec![10, 20, 30];
    println!("First of [10, 20, 30]: {}", first_or_zero(&numbers));
    println!("First of []: {}", first_or_zero(&[]));
    
    // Test first_char_or_default
    println!("\nFirst char of 'Hello': {}", first_char_or_default("Hello"));
    println!("First char of '': {}", first_char_or_default(""));
    
    // Test max_length
    let words = vec!["apple", "banana", "cherry"];
    println!("\nMax length in {:?}: {}", words, max_length(&words));
    println!("Max length in []: {}", max_length(&[]));
    
    // Test last_element_required (this will work)
    println!("\nLast element: {}", last_element_required(&numbers));
    
    // This would panic - uncomment to see expect in action
    // println!("Last of empty: {}", last_element_required(&[]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_or_zero_with_elements() {
        assert_eq!(first_or_zero(&[5, 10, 15]), 5);
    }

    #[test]
    fn test_first_or_zero_empty() {
        assert_eq!(first_or_zero(&[]), 0);
    }

    #[test]
    fn test_first_char_or_default_with_text() {
        assert_eq!(first_char_or_default("Hello"), 'H');
    }

    #[test]
    fn test_first_char_or_default_empty() {
        assert_eq!(first_char_or_default(""), '?');
    }

    #[test]
    fn test_last_element_required_with_elements() {
        assert_eq!(last_element_required(&[1, 2, 3]), 3);
    }

    #[test]
    #[should_panic(expected = "Empty vector")]
    fn test_last_element_required_empty() {
        last_element_required(&[]);
    }

    #[test]
    fn test_max_length_normal() {
        assert_eq!(max_length(&["a", "bb", "ccc"]), 3);
    }

    #[test]
    fn test_max_length_empty() {
        assert_eq!(max_length(&[]), 0);
    }

    #[test]
    fn test_max_length_single() {
        assert_eq!(max_length(&["hello"]), 5);
    }
}
