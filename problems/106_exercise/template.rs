// Exercise 106: Factorial (Recursive)
// ===================================
//
// Learning Objective:
// Learn how to implement a recursive function to calculate factorial.
// Factorial of n (written as n!) is the product of all positive integers
// less than or equal to n.
//
// Example: 5! = 5 × 4 × 3 × 2 × 1 = 120
//
// Recursive formula: n! = n × (n-1)! with base case 0! = 1

fn main() {
    // Test cases
    let test_cases = vec![(0, 1), (1, 1), (5, 120), (7, 5040), (10, 3628800)];
    
    for (n, expected) in test_cases {
        let result = factorial(n);
        println!("{}! = {} (expected: {})", n, result, expected);
        assert_eq!(result, expected);
    }
    
    println!("✓ Factorial (recursive) completed successfully!");
}

/// Recursive Factorial Implementation
///
/// The factorial function can be defined recursively:
/// - Base case: 0! = 1 and 1! = 1
/// - Recursive case: n! = n × (n-1)!
///
/// Example trace for factorial(4):
/// factorial(4) = 4 × factorial(3)
///              = 4 × (3 × factorial(2))
///              = 4 × (3 × (2 × factorial(1)))
///              = 4 × (3 × (2 × 1))
///              = 24
///
/// Hint: Handle the base case first, then implement the recursive step.
fn factorial(n: u64) -> u64 {
    // TODO: Implement base case
    // If n is 0 or 1, return 1
    if n <= 1 {
        return 1;
    }
    
    // TODO: Implement recursive case
    // Return n multiplied by factorial of (n-1)
    n * factorial(n - 1)
}

/// Iterative Factorial - Bonus implementation
///
/// While the exercise focuses on recursion, here's how you'd
/// implement it iteratively for comparison.
fn factorial_iterative(n: u64) -> u64 {
    // TODO: Bonus - implement iterative version
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_ten() {
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_factorial_iterative() {
        assert_eq!(factorial_iterative(5), 120);
        assert_eq!(factorial_iterative(0), 1);
    }
}
