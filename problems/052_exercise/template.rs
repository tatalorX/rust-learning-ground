// Exercise 052: Result - unwrap and expect
//
// unwrap() and expect() are methods to extract values from Result types.
// They return the value if Ok, or panic if Err.
// expect() is like unwrap() but lets you specify a custom panic message.
//
// Learning Objectives:
// - Learn when to use unwrap() vs expect()
// - Understand the risks of panicking
// - Use unwrap_or() and unwrap_or_default() for safe alternatives
//
// Your task: Implement functions that use different unwrap variants safely.

/// Returns the length of a string parsed from input.
/// Uses unwrap_or to return 0 if parsing fails.
fn get_length_or_default(input: &str) -> usize {
    // TODO: Parse the input as usize, use unwrap_or(0) to handle errors
    todo!()
}

/// Returns the number if parsing succeeds, panics with a custom message otherwise.
fn parse_critical_number(input: &str) -> i32 {
    // TODO: Parse the input as i32, use expect() with message "Critical number required"
    todo!()
}

/// Returns the square root of a number if positive, or 0.0 if negative.
fn safe_sqrt(number: f64) -> f64 {
    // TODO: Calculate the square root using number.sqrt()
    // Note: sqrt() returns NaN for negative numbers, handle this with unwrap_or
    // Return the square root if it's a valid number, or 0.0 otherwise
    todo!()
}

fn main() {
    // Test get_length_or_default
    println!("Length of 'hello': {}", get_length_or_default("5"));
    println!("Length of 'invalid': {}", get_length_or_default("abc"));
    
    // Test safe_sqrt
    println!("√16 = {}", safe_sqrt(16.0));
    println!("√(-4) = {}", safe_sqrt(-4.0));
    
    // This will panic - uncomment to see expect in action
    // println!("Critical: {}", parse_critical_number("not a number"));
    println!("Critical: {}", parse_critical_number("42"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length_or_default_valid() {
        assert_eq!(get_length_or_default("5"), 5);
        assert_eq!(get_length_or_default("100"), 100);
    }

    #[test]
    fn test_get_length_or_default_invalid() {
        assert_eq!(get_length_or_default("abc"), 0);
        assert_eq!(get_length_or_default(""), 0);
    }

    #[test]
    fn test_safe_sqrt_positive() {
        assert!((safe_sqrt(16.0) - 4.0).abs() < 0.0001);
        assert!((safe_sqrt(2.0) - 1.4142).abs() < 0.0001);
    }

    #[test]
    fn test_safe_sqrt_negative() {
        assert_eq!(safe_sqrt(-4.0), 0.0);
        assert_eq!(safe_sqrt(-100.0), 0.0);
    }

    #[test]
    fn test_parse_critical_number_valid() {
        assert_eq!(parse_critical_number("42"), 42);
    }

    #[test]
    #[should_panic(expected = "Critical number required")]
    fn test_parse_critical_number_invalid() {
        parse_critical_number("abc");
    }
}
