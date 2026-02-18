// Exercise 053: Result - match handling
//
// The match expression is the most comprehensive way to handle Result types.
// It allows you to handle both Ok and Err variants with full control.
//
// Learning Objectives:
// - Use match to handle Result types
// - Extract and use values from Ok/Err variants
// - Chain multiple Result operations
//
// Your task: Implement functions using match to handle Results properly.

/// Divides two numbers, returning a Result with proper error messages.
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // TODO: Use match to check if b is 0.0
    // Return Err("Division by zero") if b is 0.0
    // Return Ok(a / b) otherwise
    todo!()
}

/// Parses two strings as numbers and adds them.
/// Returns detailed error messages for each parsing failure.
fn add_strings(a: &str, b: &str) -> Result<i32, String> {
    // TODO: Parse 'a' using match, return Err with message if it fails
    // TODO: Parse 'b' using match, return Err with message if it fails
    // TODO: Return Ok(sum) if both succeed
    todo!()
}

/// Attempts to open a "file" (simulated with input validation).
/// Returns Ok(filename) if valid, Err otherwise.
fn validate_filename(filename: &str) -> Result<&str, String> {
    // TODO: Check if filename is empty, return Err("Filename cannot be empty")
    // TODO: Check if filename contains '.', return Err("Filename needs extension") if not
    // TODO: Return Ok(filename) if all checks pass
    todo!()
}

fn main() {
    // Test safe_divide
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test add_strings
    println!("\nString addition:");
    match add_strings("10", "20") {
        Ok(sum) => println!("10 + 20 = {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    
    match add_strings("10", "abc") {
        Ok(sum) => println!("Sum = {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test validate_filename
    println!("\nFilename validation:");
    for filename in &["document.txt", "", "readme"] {
        match validate_filename(filename) {
            Ok(f) => println!("✓ Valid: {}", f),
            Err(e) => println!("✗ Invalid '{}': {}", filename, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_normal() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_add_strings_both_valid() {
        assert_eq!(add_strings("10", "20"), Ok(30));
    }

    #[test]
    fn test_add_strings_first_invalid() {
        assert!(add_strings("abc", "20").is_err());
    }

    #[test]
    fn test_add_strings_second_invalid() {
        assert!(add_strings("10", "xyz").is_err());
    }

    #[test]
    fn test_validate_filename_valid() {
        assert_eq!(validate_filename("test.txt"), Ok("test.txt"));
    }

    #[test]
    fn test_validate_filename_empty() {
        assert!(validate_filename("").is_err());
    }

    #[test]
    fn test_validate_filename_no_extension() {
        assert!(validate_filename("readme").is_err());
    }
}
