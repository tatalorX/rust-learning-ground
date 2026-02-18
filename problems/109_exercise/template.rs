// Exercise 109: Greatest Common Divisor (Euclidean Algorithm)
// ===========================================================
//
// Learning Objective:
// Learn how to implement the Euclidean algorithm to find the Greatest
// Common Divisor (GCD) of two numbers. This is one of the oldest and
// most efficient algorithms known.
//
// The GCD of two numbers is the largest positive integer that divides
// both numbers without leaving a remainder.
//
// Euclid's Algorithm:
// gcd(a, b) = gcd(b, a mod b), with base case gcd(a, 0) = a

fn main() {
    // Test cases
    let test_cases = vec![
        ((48, 18), 6),
        ((56, 98), 14),
        ((101, 103), 1), // Coprime numbers
        ((100, 25), 25),
        ((0, 5), 5),
        ((5, 0), 5),
        ((0, 0), 0),
    ];
    
    println!("=== Euclidean Algorithm (Iterative) ===");
    for ((a, b), expected) in &test_cases {
        let result = gcd(*a, *b);
        println!("gcd({}, {}) = {} (expected: {})", a, b, result, expected);
        assert_eq!(result, *expected);
    }
    
    println!("\n=== Euclidean Algorithm (Recursive) ===");
    for ((a, b), expected) in &test_cases {
        let result = gcd_recursive(*a, *b);
        println!("gcd({}, {}) = {} (expected: {})", a, b, result, expected);
        assert_eq!(result, *expected);
    }
    
    println!("\n✓ GCD (Euclidean algorithm) completed successfully!");
}

/// GCD using Euclidean Algorithm (Iterative)
///
/// Algorithm:
/// 1. If b = 0, return a
/// 2. Otherwise, compute gcd(b, a % b) and repeat
///
/// Example: gcd(48, 18)
/// Step 1: gcd(48, 18) → 48 % 18 = 12
/// Step 2: gcd(18, 12) → 18 % 12 = 6
/// Step 3: gcd(12, 6)  → 12 % 6 = 0
/// Step 4: gcd(6, 0)   → return 6
///
/// The result is 6.
fn gcd(a: u64, b: u64) -> u64 {
    // TODO: Handle special case where both are 0
    if a == 0 && b == 0 {
        return 0;
    }
    
    // TODO: Initialize mutable copies of a and b
    let mut a = a;
    let mut b = b;
    
    // TODO: Implement Euclidean algorithm iteratively
    // While b is not 0, continue
    while b != 0 {
        // TODO: Store the remainder of a divided by b
        let temp = a % b;
        
        // TODO: Update a to be b, and b to be the remainder
        a = b;
        b = temp;
    }
    
    // TODO: Return the GCD
    a
}

/// GCD using Euclidean Algorithm (Recursive)
///
/// This is the same algorithm but implemented recursively.
/// It's more elegant but uses more stack space.
fn gcd_recursive(a: u64, b: u64) -> u64 {
    // TODO: Handle special case where both are 0
    if a == 0 && b == 0 {
        return 0;
    }
    
    // TODO: Implement base case
    // If b is 0, return a
    if b == 0 {
        return a;
    }
    
    // TODO: Implement recursive case
    // gcd(a, b) = gcd(b, a % b)
    gcd_recursive(b, a % b)
}

/// Least Common Multiple (LCM) - Bonus
///
/// The LCM of two numbers can be computed using the GCD:
/// lcm(a, b) = |a × b| / gcd(a, b)
fn lcm(a: u64, b: u64) -> u64 {
    // TODO: Bonus - implement LCM using GCD
    // Handle edge cases
    if a == 0 || b == 0 {
        return 0;
    }
    
    // LCM = (a * b) / GCD(a, b)
    // Be careful of overflow - use division first when possible
    (a / gcd(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd(101, 103), 1);
    }

    #[test]
    fn test_gcd_same() {
        assert_eq!(gcd(42, 42), 42);
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_recursive() {
        assert_eq!(gcd_recursive(56, 98), 14);
        assert_eq!(gcd_recursive(101, 103), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
    }
}
