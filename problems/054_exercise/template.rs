// Exercise 054: Result - ? operator
//
// The ? operator is syntactic sugar for propagating errors.
// It unwraps Ok values or returns Err early from the function.
//
// Learning Objectives:
// - Use the ? operator for error propagation
// - Understand when ? can be used (functions returning Result)
// - Chain multiple fallible operations cleanly
//
// Your task: Refactor functions to use the ? operator for cleaner error handling.

/// A custom error type for our operations.
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    InvalidNumber(String),
    NegativeResult,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            MathError::NegativeResult => write!(f, "Result would be negative"),
        }
    }
}

impl std::error::Error for MathError {}

/// Parses a string to i32, returning a MathError.
fn parse_number(input: &str) -> Result<i32, MathError> {
    // TODO: Parse input as i32, map_err to MathError::InvalidNumber if it fails
    // Hint: input.parse::<i32>().map_err(|_| ...)
    todo!()
}

/// Divides two numbers represented as strings.
/// Uses ? operator to propagate errors.
fn divide_strings(a: &str, b: &str) -> Result<i32, MathError> {
    // TODO: Parse both strings using parse_number and ?
    // TODO: Check if divisor is zero, return Err(MathError::DivisionByZero) if so
    // TODO: Return the division result
    todo!()
}

/// Calculates (a / b) - c, all represented as strings.
/// Uses ? operator to chain multiple fallible operations.
fn complex_calculation(a: &str, b: &str, c: &str) -> Result<i32, MathError> {
    // TODO: First divide a by b using divide_strings and ?
    // TODO: Parse c using parse_number and ?
    // TODO: Subtract c from the division result
    // TODO: Return Err(MathError::NegativeResult) if result is negative
    todo!()
}

fn main() {
    // Test cases
    let cases = vec![
        ("10", "2", "3"),   // (10/2) - 3 = 2
        ("10", "0", "3"),   // Division by zero
        ("10", "2", "10"),  // Negative result
        ("abc", "2", "3"),  // Invalid number
    ];
    
    for (a, b, c) in cases {
        match complex_calculation(a, b, c) {
            Ok(result) => println!("({}/{}) - {} = {}", a, b, c, result),
            Err(e) => println!("({}/{}) - {} failed: {}", a, b, c, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_valid() {
        assert_eq!(parse_number("42"), Ok(42));
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(matches!(parse_number("abc"), Err(MathError::InvalidNumber(_))));
    }

    #[test]
    fn test_divide_strings_success() {
        assert_eq!(divide_strings("10", "2"), Ok(5));
    }

    #[test]
    fn test_divide_strings_by_zero() {
        assert_eq!(divide_strings("10", "0"), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_complex_calculation_success() {
        assert_eq!(complex_calculation("10", "2", "3"), Ok(2));
    }

    #[test]
    fn test_complex_calculation_negative_result() {
        assert_eq!(complex_calculation("10", "2", "10"), Err(MathError::NegativeResult));
    }

    #[test]
    fn test_complex_calculation_invalid_input() {
        assert!(matches!(complex_calculation("x", "2", "3"), Err(MathError::InvalidNumber(_))));
    }
}
