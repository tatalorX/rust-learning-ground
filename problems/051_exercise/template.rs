// Exercise 051: Result Type - Basic Error Handling
// 
// The Result<T, E> type is Rust's primary way of handling recoverable errors.
// It's an enum with two variants: Ok(T) for success and Err(E) for failure.
//
// Learning Objectives:
// - Understand the Result<T, E> enum
// - Learn to create Ok and Err variants
// - Practice returning Results from functions
//
// Your task: Implement a function that parses a string to an integer and returns a Result.

/// Parses a string to a positive integer.
/// Returns Ok(number) if the string can be parsed and the number is positive.
/// Returns Err with an appropriate error message otherwise.
fn parse_positive_number(input: &str) -> Result<i32, String> {
    // TODO: Parse the input string to an i32 using .parse::<i32>()
    // TODO: Check if the parsed number is positive (> 0)
    // TODO: Return Ok(number) if successful, or Err with a descriptive message if not
    todo!()
}

fn main() {
    // Test cases
    let test_cases = vec!["42", "-5", "0", "abc", "100"];
    
    for case in test_cases {
        match parse_positive_number(case) {
            Ok(n) => println!("✓ '{}' parsed successfully: {}", case, n),
            Err(e) => println!("✗ '{}' failed: {}", case, e),
        }
    }
    
    // Expected output:
    // ✓ '42' parsed successfully: 42
    // ✗ '-5' failed: Number must be positive
    // ✗ '0' failed: Number must be positive
    // ✗ 'abc' failed: Invalid number format
    // ✓ '100' parsed successfully: 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(parse_positive_number("42"), Ok(42));
    }

    #[test]
    fn test_large_positive() {
        assert_eq!(parse_positive_number("999"), Ok(999));
    }

    #[test]
    fn test_negative_number() {
        assert!(parse_positive_number("-5").is_err());
    }

    #[test]
    fn test_zero() {
        assert!(parse_positive_number("0").is_err());
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_positive_number("abc").is_err());
    }

    #[test]
    fn test_empty_string() {
        assert!(parse_positive_number("").is_err());
    }
}
