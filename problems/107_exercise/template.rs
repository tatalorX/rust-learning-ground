// Exercise 107: Fibonacci (Recursive)
// ===================================
//
// Learning Objective:
// Learn how to implement the Fibonacci sequence using recursion.
// The Fibonacci sequence is where each number is the sum of the two preceding ones.
//
// Sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
// Formula: F(n) = F(n-1) + F(n-2)
// Base cases: F(0) = 0, F(1) = 1

fn main() {
    // Test cases
    let test_cases = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
        (10, 55),
        (20, 6765),
    ];
    
    for (n, expected) in test_cases {
        let result = fibonacci(n);
        println!("F({}) = {} (expected: {})", n, result, expected);
        assert_eq!(result, expected);
    }
    
    println!("✓ Fibonacci (recursive) completed successfully!");
}

/// Recursive Fibonacci Implementation
///
/// The Fibonacci sequence is defined as:
/// - Base case: F(0) = 0, F(1) = 1
/// - Recursive case: F(n) = F(n-1) + F(n-2)
///
/// Example trace for fibonacci(4):
/// fibonacci(4) = fibonacci(3) + fibonacci(2)
///              = (fibonacci(2) + fibonacci(1)) + (fibonacci(1) + fibonacci(0))
///              = ((fibonacci(1) + fibonacci(0)) + 1) + (1 + 0)
///              = ((1 + 0) + 1) + 1
///              = 3
///
/// Note: This naive recursive approach has exponential time complexity O(2^n)
///       due to repeated calculations. See exercise 108 for optimization.
///
/// Hint: Start with the base cases, then implement the recursive relation.
fn fibonacci(n: u64) -> u64 {
    // TODO: Implement base cases
    // F(0) = 0, F(1) = 1
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // TODO: Implement recursive case
    // F(n) = F(n-1) + F(n-2)
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_ten() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fibonacci_twenty() {
        assert_eq!(fibonacci(20), 6765);
    }
}
