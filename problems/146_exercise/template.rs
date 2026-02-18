// Exercise 146: atoi Implementation (String to Integer)
//
// Learning Objective:
// Implement the atoi function that converts a string to a 32-bit integer,
// handling edge cases like whitespace, signs, and overflow.
//
// Key Concepts:
// - String parsing
// - Integer overflow handling
// - Character to digit conversion
// - Whitespace handling

/// TODO: Implement my_atoi function
/// 
/// Requirements:
/// 1. Discard leading whitespace
/// 2. Handle optional '+' or '-' sign
/// 3. Read digits until non-digit or end of string
/// 4. Convert digits to integer
/// 5. Handle overflow: clamp to i32::MAX or i32::MIN
/// 6. Return 0 for invalid input
/// 
/// Algorithm:
/// 1. Skip leading whitespace
/// 2. Check for sign
/// 3. Process digits, building result
/// 4. Check for overflow at each step
/// 5. Apply sign and return
fn my_atoi(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = 0;
    
    // TODO: Step 1: Skip leading whitespace
    // While i < n and bytes[i] is whitespace, increment i
    
    // TODO: Step 2: Handle sign
    // Check if current char is '+' or '-'
    // Remember the sign and move past it
    
    // TODO: Step 3: Process digits
    // Initialize result = 0
    // While i < n and bytes[i] is a digit:
    //   digit = bytes[i] - b'0'
    //   Check for overflow before multiplying
    //   result = result * 10 + digit
    //   Increment i
    
    // TODO: Step 4: Apply sign and return
    // Handle overflow cases
    
    0
}

/// TODO: Helper function to check if a character is whitespace
fn is_whitespace(c: u8) -> bool {
    // TODO: Return true for space, tab, newline, etc.
    
    false
}

/// TODO: Helper function to check if a character is a digit
fn is_digit(c: u8) -> bool {
    // TODO: Return true if c is between '0' and '9'
    
    false
}

/// TODO: Convert character to digit
fn to_digit(c: u8) -> i32 {
    // TODO: Return digit value (0-9)
    // Assume c is a valid digit
    
    0
}

/// TODO: Parse string to i64 first, then clamp to i32
/// Alternative implementation that avoids manual overflow checking
fn my_atoi_i64(s: &str) -> i32 {
    // TODO: Similar logic but use i64 for intermediate calculation
    // Then clamp to i32 range
    
    0
}

/// TODO: Handle floating point numbers and return (integer_part, fractional_part)
fn parse_float(s: &str) -> Option<(i32, u32)> {
    // BONUS CHALLENGE
    // Parse strings like "123.456" into (123, 456)
    // Handle negative numbers
    
    None
}

fn main() {
    let test_cases = vec![
        "42",
        "   -42",
        "4193 with words",
        "words and 987",
        "-91283472332", // Overflow
        "91283472332",  // Overflow
        "+1",
        "+-12",
        "00000-42a1234",
        "  0000000000012345678",
    ];
    
    println!("atoi Results:\n");
    for s in test_cases {
        let result = my_atoi(s);
        println!("'{}' -> {}", s, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_atoi_positive() {
        assert_eq!(my_atoi("42"), 42);
        assert_eq!(my_atoi("+42"), 42);
    }
    
    #[test]
    fn test_atoi_negative() {
        assert_eq!(my_atoi("-42"), -42);
    }
    
    #[test]
    fn test_atoi_whitespace() {
        assert_eq!(my_atoi("   42"), 42);
        assert_eq!(my_atoi("42   "), 42);
        assert_eq!(my_atoi("  -42  "), -42);
    }
    
    #[test]
    fn test_atoi_words() {
        assert_eq!(my_atoi("4193 with words"), 4193);
        assert_eq!(my_atoi("words and 987"), 0);
    }
    
    #[test]
    fn test_atoi_overflow() {
        assert_eq!(my_atoi("-91283472332"), i32::MIN);
        assert_eq!(my_atoi("91283472332"), i32::MAX);
        assert_eq!(my_atoi("2147483647"), i32::MAX);
        assert_eq!(my_atoi("-2147483648"), i32::MIN);
    }
    
    #[test]
    fn test_atoi_edge_cases() {
        assert_eq!(my_atoi(""), 0);
        assert_eq!(my_atoi("   "), 0);
        assert_eq!(my_atoi("+"), 0);
        assert_eq!(my_atoi("-"), 0);
        assert_eq!(my_atoi("+-12"), 0);
        assert_eq!(my_atoi("-+12"), 0);
    }
    
    #[test]
    fn test_atoi_leading_zeros() {
        assert_eq!(my_atoi("00042"), 42);
        assert_eq!(my_atoi("-00042"), -42);
        assert_eq!(my_atoi("  0000000000012345678"), 12345678);
    }
    
    #[test]
    fn test_atoi_mixed_input() {
        assert_eq!(my_atoi("00000-42a1234"), 0);
        assert_eq!(my_atoi("3.14159"), 3);
    }
}
