// Exercise 147: Reverse Integer
//
// Learning Objective:
// Reverse the digits of a 32-bit signed integer, handling overflow.
//
// Key Concepts:
// - Integer manipulation
// - Digit extraction
// - Overflow detection
// - Mathematical approach vs string conversion

/// TODO: Reverse a 32-bit signed integer
/// 
/// Requirements:
/// - Reverse digits: 123 -> 321, -123 -> -321
/// - Return 0 if reversed integer overflows i32
/// - Handle trailing zeros: 1200 -> 21
/// 
/// Algorithm (Mathematical):
/// 1. Handle negative numbers
/// 2. While x != 0:
///    - Extract last digit: digit = x % 10
///    - Remove last digit: x /= 10
///    - Check for overflow before adding digit
///    - result = result * 10 + digit
fn reverse(x: i32) -> i32 {
    // TODO: Implement reverse without converting to string
    // Use i64 for intermediate calculations or check overflow before operation
    
    0
}

/// TODO: Reverse using string conversion (easier but less efficient)
fn reverse_string(x: i32) -> i32 {
    // TODO: Convert to string, reverse, parse back
    // Handle negative sign and overflow
    
    0
}

/// TODO: Check if reversed number would overflow
fn would_overflow(x: i32) -> bool {
    // TODO: Check if reversing x would cause overflow
    // Max i32 = 2147483647, Min i32 = -2147483648
    // Numbers ending in 2+ digits might overflow when reversed
    
    false
}

/// TODO: Reverse digits of a positive number only
fn reverse_positive(mut x: i32) -> i32 {
    // TODO: Assume x is positive, reverse digits
    // Still need to check for overflow
    
    0
}

/// TODO: Check if a number is a palindrome
fn is_palindrome(x: i32) -> bool {
    // TODO: Negative numbers are not palindromes
    // Numbers ending in 0 (except 0 itself) are not palindromes
    // Compare x with reversed x
    
    false
}

/// BONUS: Reverse digits in a given base
fn reverse_in_base(x: i32, base: i32) -> i32 {
    // BONUS CHALLENGE
    // Reverse digits of x when represented in given base
    // e.g., reverse_in_base(13, 2) -> 13 is 1101 in binary -> 1011 = 11
    
    0
}

fn main() {
    let test_cases = vec![
        123,
        -123,
        120,
        0,
        1534236469, // This overflows when reversed
        -2147483412,
        1463847412, // This overflows
    ];
    
    println!("Reverse Integer Results:\n");
    for x in test_cases {
        let result = reverse(x);
        println!("{} -> {}", x, result);
    }
    
    // Palindrome check
    println!("\nPalindrome Check:");
    for x in vec![121, -121, 10, 12321, 123456] {
        println!("{} is palindrome: {}", x, is_palindrome(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_positive() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(0), 0);
    }
    
    #[test]
    fn test_reverse_negative() {
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-120), -21);
    }
    
    #[test]
    fn test_reverse_overflow() {
        // 1534236469 reversed is 9646324351 which overflows i32
        assert_eq!(reverse(1534236469), 0);
        // 1463847412 reversed is 2147483641 which is within range
        assert_eq!(reverse(1463847412), 2147483641);
    }
    
    #[test]
    fn test_reverse_max_min() {
        assert_eq!(reverse(i32::MAX), 0);
        assert_eq!(reverse(i32::MIN), 0);
    }
    
    #[test]
    fn test_reverse_trailing_zeros() {
        assert_eq!(reverse(100), 1);
        assert_eq!(reverse(1000), 1);
        assert_eq!(reverse(1200300), 30021);
    }
    
    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(12321));
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        
        assert!(!is_palindrome(-121)); // Negative not palindrome
        assert!(!is_palindrome(10));   // Ends with 0, not palindrome
        assert!(!is_palindrome(123));
    }
    
    #[test]
    fn test_reverse_string_matches() {
        // Both implementations should give same results for valid inputs
        assert_eq!(reverse(123), reverse_string(123));
        assert_eq!(reverse(-456), reverse_string(-456));
        assert_eq!(reverse(0), reverse_string(0));
    }
}
