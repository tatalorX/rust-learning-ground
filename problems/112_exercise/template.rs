// Exercise 112: Palindrome Check
// ================================
//
// Learning Objective:
// Learn how to check if a string or number is a palindrome. A palindrome
// reads the same forwards and backwards (e.g., "radar", "level", 12321).
//
// This exercise covers both string and integer palindromes.

fn main() {
    // Test string palindromes
    let string_tests = vec![
        ("radar", true),
        ("level", true),
        ("hello", false),
        ("A man a plan a canal Panama", true), // With spaces (case-insensitive)
        ("racecar", true),
        ("", true),  // Empty string is a palindrome
        ("a", true), // Single character is a palindrome
    ];
    
    println!("=== String Palindrome Check ===");
    for (s, expected) in &string_tests {
        let result = is_palindrome_string(s);
        let status = if result == *expected { "✓" } else { "✗" };
        println!("{} '{}' -> {} (expected: {})", status, s, result, expected);
        assert_eq!(result, *expected);
    }
    
    // Test number palindromes
    let number_tests = vec![
        (121, true),
        (12321, true),
        (12345, false),
        (0, true),
        (5, true),
        (10, false),
        (1001, true),
    ];
    
    println!("\n=== Number Palindrome Check ===");
    for (n, expected) in &number_tests {
        let result = is_palindrome_number(*n);
        let status = if result == *expected { "✓" } else { "✗" };
        println!("{} {} -> {} (expected: {})", status, n, result, expected);
        assert_eq!(result, *expected);
    }
    
    println!("\n✓ Palindrome check completed successfully!");
}

/// Check if a string is a palindrome (case-insensitive, ignoring spaces)
///
/// Algorithm:
/// 1. Filter out non-alphanumeric characters and convert to lowercase
/// 2. Compare characters from both ends moving towards center
/// 3. If any pair doesn't match, it's not a palindrome
fn is_palindrome_string(s: &str) -> bool {
    // TODO: Convert string to lowercase and filter only alphanumeric chars
    let chars: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    // TODO: Handle edge cases
    if chars.len() <= 1 {
        return true;
    }
    
    // TODO: Use two pointers to check from both ends
    let mut left = 0;
    let mut right = chars.len() - 1;
    
    while left < right {
        // TODO: If characters don't match, return false
        if chars[left] != chars[right] {
            return false;
        }
        
        // TODO: Move pointers towards center
        left += 1;
        right -= 1;
    }
    
    true
}

/// Check if a number is a palindrome
///
/// Algorithm:
/// 1. Handle edge case: negative numbers are not palindromes
/// 2. Reverse the number by repeatedly extracting the last digit
/// 3. Compare the reversed number with the original
///
/// Alternative approach: Convert to string and use string palindrome check
fn is_palindrome_number(n: i32) -> bool {
    // TODO: Handle edge cases
    // Negative numbers are not palindromes
    if n < 0 {
        return false;
    }
    
    // TODO: Handle single digit numbers (always palindromes)
    if n < 10 {
        return true;
    }
    
    // TODO: Reverse the number mathematically
    let original = n;
    let mut reversed = 0;
    let mut temp = n;
    
    while temp > 0 {
        // TODO: Extract last digit and add to reversed
        let digit = temp % 10;
        reversed = reversed * 10 + digit;
        temp /= 10;
    }
    
    // TODO: Return true if reversed equals original
    original == reversed
}

/// Alternative: Check palindrome using string conversion
fn is_palindrome_number_string(n: i32) -> bool {
    // TODO: Implement using string conversion
    // Convert number to string, then check if it's a palindrome
    let s = n.to_string();
    is_palindrome_string(&s)
}

/// Recursive palindrome check (Bonus)
fn is_palindrome_recursive(s: &str) -> bool {
    // TODO: Bonus - implement recursive palindrome check
    let filtered: String = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    
    if filtered.len() <= 1 {
        return true;
    }
    
    let chars: Vec<char> = filtered.chars().collect();
    if chars[0] != chars[chars.len() - 1] {
        return false;
    }
    
    let inner = &filtered[1..filtered.len() - 1];
    is_palindrome_recursive(inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_palindrome() {
        assert!(is_palindrome_string("radar"));
        assert!(is_palindrome_string("level"));
        assert!(!is_palindrome_string("hello"));
    }

    #[test]
    fn test_string_with_spaces() {
        assert!(is_palindrome_string("A man a plan a canal Panama"));
        assert!(is_palindrome_string("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_number_palindrome() {
        assert!(is_palindrome_number(121));
        assert!(is_palindrome_number(12321));
        assert!(!is_palindrome_number(12345));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_palindrome_string(""));
        assert!(is_palindrome_string("a"));
        assert!(is_palindrome_number(0));
        assert!(is_palindrome_number(5));
        assert!(!is_palindrome_number(-121));
    }

    #[test]
    fn test_recursive() {
        assert!(is_palindrome_recursive("radar"));
        assert!(!is_palindrome_recursive("hello"));
    }
}
