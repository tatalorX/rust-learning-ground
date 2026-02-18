// Exercise 116: Sum of Digits
// ===========================
//
// Learning Objective:
// Learn different ways to calculate the sum of digits of a number.
// This involves arithmetic operations and understanding number systems.

fn main() {
    let test_cases = vec![
        (123, 6),       // 1 + 2 + 3 = 6
        (456, 15),      // 4 + 5 + 6 = 15
        (0, 0),
        (5, 5),
        (9999, 36),     // 9 + 9 + 9 + 9 = 36
        (1000, 1),      // 1 + 0 + 0 + 0 = 1
        (123456789, 45),
    ];
    
    println!("=== Sum of Digits ===\n");
    
    for (n, expected) in &test_cases {
        let result_iterative = sum_of_digits(*n);
        let result_recursive = sum_of_digits_recursive(*n);
        let result_string = sum_of_digits_string(*n);
        
        println!("Number: {}", n);
        println!("  Iterative: {} (expected: {})", result_iterative, expected);
        println!("  Recursive: {} (expected: {})", result_recursive, expected);
        println!("  String:    {} (expected: {})", result_string, expected);
        println!();
        
        assert_eq!(result_iterative, *expected);
        assert_eq!(result_recursive, *expected);
        assert_eq!(result_string, *expected);
    }
    
    // Digital root demonstration
    println!("=== Digital Root ===");
    for n in vec![16, 942, 132189] {
        let dr = digital_root(n);
        println!("Digital root of {} = {}", n, dr);
    }
    
    println!("\n✓ Sum of digits completed successfully!");
}

/// Sum of digits using iteration
///
/// Algorithm:
/// 1. Initialize sum to 0
/// 2. While number > 0:
///    a. Extract last digit (number % 10)
///    b. Add to sum
///    c. Remove last digit (number /= 10)
/// 3. Return sum
fn sum_of_digits(mut n: u64) -> u64 {
    // TODO: Initialize sum
    let mut sum = 0;
    
    // TODO: Loop while n > 0
    while n > 0 {
        // TODO: Add last digit to sum
        sum += n % 10;
        
        // TODO: Remove last digit
        n /= 10;
    }
    
    sum
}

/// Sum of digits using recursion
///
/// Base case: if n < 10, return n
/// Recursive case: return (n % 10) + sum_of_digits(n / 10)
fn sum_of_digits_recursive(n: u64) -> u64 {
    // TODO: Implement base case
    if n < 10 {
        return n;
    }
    
    // TODO: Implement recursive case
    (n % 10) + sum_of_digits_recursive(n / 10)
}

/// Sum of digits using string conversion
///
/// This is less efficient but shows a different approach.
fn sum_of_digits_string(n: u64) -> u64 {
    // TODO: Convert number to string
    // Iterate over characters, convert each to digit, sum them
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

/// Digital Root
///
/// The digital root is the recursive sum of all digits until a single digit is obtained.
/// Example: digital_root(942) = 9 + 4 + 2 = 15 → 1 + 5 = 6
///
/// Mathematical formula: digital_root(n) = 1 + (n - 1) % 9 (for n > 0)
fn digital_root(n: u64) -> u64 {
    // TODO: Implement iterative approach
    // Keep summing digits until we get a single digit
    let mut result = n;
    
    while result >= 10 {
        result = sum_of_digits(result);
    }
    
    result
}

/// Digital Root using mathematical formula (O(1) time!)
fn digital_root_formula(n: u64) -> u64 {
    // TODO: Bonus - implement using the mathematical formula
    if n == 0 {
        0
    } else {
        1 + (n - 1) % 9
    }
}

/// Sum of digits in a specific base (Bonus)
fn sum_of_digits_base(n: u64, base: u64) -> u64 {
    // TODO: Bonus - calculate sum of digits in any base
    let mut sum = 0;
    let mut num = n;
    
    while num > 0 {
        sum += num % base;
        num /= base;
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_of_digits(123), 6);
        assert_eq!(sum_of_digits(456), 15);
        assert_eq!(sum_of_digits(0), 0);
        assert_eq!(sum_of_digits(999), 27);
    }

    #[test]
    fn test_sum_recursive() {
        assert_eq!(sum_of_digits_recursive(123), 6);
        assert_eq!(sum_of_digits_recursive(0), 0);
    }

    #[test]
    fn test_digital_root() {
        assert_eq!(digital_root(16), 7);      // 1 + 6 = 7
        assert_eq!(digital_root(942), 6);     // 9 + 4 + 2 = 15 → 1 + 5 = 6
        assert_eq!(digital_root(132189), 6);  // 1 + 3 + 2 + 1 + 8 + 9 = 24 → 2 + 4 = 6
    }

    #[test]
    fn test_digital_root_formula() {
        // Both methods should give same results
        for n in 0..=1000 {
            assert_eq!(digital_root(n), digital_root_formula(n));
        }
    }

    #[test]
    fn test_sum_base() {
        assert_eq!(sum_of_digits_base(0b1010, 2), 2); // Binary: 1010 → 1+0+1+0 = 2
        assert_eq!(sum_of_digits_base(0xFF, 16), 30); // Hex: FF → 15+15 = 30
    }
}
