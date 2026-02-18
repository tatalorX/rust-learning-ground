// Exercise 117: Power Calculation (Fast Exponentiation)
// =====================================================
//
// Learning Objective:
// Learn fast exponentiation (also known as "exponentiation by squaring"),
// an efficient algorithm for computing large powers of a number.
//
// Naive approach: O(n) multiplications
// Fast exponentiation: O(log n) multiplications

fn main() {
    let test_cases = vec![
        (2, 10, 1024),
        (3, 5, 243),
        (5, 0, 1),      // Any number^0 = 1
        (10, 3, 1000),
        (2, 20, 1048576),
    ];
    
    println!("=== Fast Exponentiation ===\n");
    
    for (base, exp, expected) in &test_cases {
        let result = fast_power(*base, *exp);
        let result_recursive = fast_power_recursive(*base, *exp);
        
        println!("{}^{} = {} (expected: {})", base, exp, result, expected);
        assert_eq!(result, *expected);
        assert_eq!(result_recursive, *expected);
    }
    
    // Demonstrate the efficiency with large exponents
    println!("\n=== Large Exponent Demo ===");
    let big_result = fast_power(2, 60);
    println!("2^60 = {}", big_result);
    
    println!("\n✓ Fast exponentiation completed successfully!");
}

/// Fast Exponentiation (Iterative)
///
/// Algorithm (Exponentiation by Squaring):
/// - If exp is 0, return 1
/// - While exp > 0:
///   - If exp is odd: result *= base
///   - base *= base (square the base)
///   - exp /= 2 (divide exponent by 2)
///
/// Example: 3^13
/// Iteration 1: exp=13 (odd), result=3, base=9, exp=6
/// Iteration 2: exp=6  (even), result=3, base=81, exp=3
/// Iteration 3: exp=3  (odd), result=243, base=6561, exp=1
/// Iteration 4: exp=1  (odd), result=1594323, base=..., exp=0
/// Return: 1594323
fn fast_power(base: u64, exp: u32) -> u64 {
    // TODO: Handle special case: anything^0 = 1
    if exp == 0 {
        return 1;
    }
    
    // TODO: Initialize result and mutable copies
    let mut result = 1u64;
    let mut b = base;
    let mut e = exp;
    
    // TODO: Implement the main loop
    while e > 0 {
        // TODO: If exponent is odd, multiply result by base
        if e % 2 == 1 {
            result = result.wrapping_mul(b);
        }
        
        // TODO: Square the base
        b = b.wrapping_mul(b);
        
        // TODO: Divide exponent by 2
        e /= 2;
    }
    
    result
}

/// Fast Exponentiation (Recursive)
///
/// Recursive formulation:
/// - power(x, 0) = 1
/// - power(x, n) = power(x², n/2)          if n is even
/// - power(x, n) = x * power(x², n/2)      if n is odd
fn fast_power_recursive(base: u64, exp: u32) -> u64 {
    // TODO: Implement base case
    if exp == 0 {
        return 1;
    }
    
    // TODO: Calculate half power recursively
    let half = fast_power_recursive(base, exp / 2);
    
    // TODO: If exp is even, return half * half
    if exp % 2 == 0 {
        half.wrapping_mul(half)
    } else {
        // TODO: If exp is odd, return base * half * half
        base.wrapping_mul(half).wrapping_mul(half)
    }
}

/// Modular Exponentiation (Bonus)
///
/// Computes (base^exp) % modulus efficiently.
/// Essential for cryptography and competitive programming.
fn modular_power(base: u64, exp: u32, modulus: u64) -> u64 {
    // TODO: Bonus - implement modular exponentiation
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1u64;
    let mut b = base % modulus;
    let mut e = exp;
    
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus;
        e /= 2;
    }
    
    result
}

/// Naive power (for comparison/testing)
fn naive_power(base: u64, exp: u32) -> u64 {
    // TODO: Implement the naive O(n) approach for comparison
    let mut result = 1u64;
    for _ in 0..exp {
        result = result.wrapping_mul(base);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_power() {
        assert_eq!(fast_power(2, 10), 1024);
        assert_eq!(fast_power(3, 5), 243);
        assert_eq!(fast_power(5, 0), 1);
        assert_eq!(fast_power(10, 3), 1000);
    }

    #[test]
    fn test_fast_power_recursive() {
        assert_eq!(fast_power_recursive(2, 10), 1024);
        assert_eq!(fast_power_recursive(3, 5), 243);
        assert_eq!(fast_power_recursive(5, 0), 1);
    }

    #[test]
    fn test_against_naive() {
        // Verify fast_power gives same results as naive_power
        for base in 0..=10 {
            for exp in 0..=10 {
                assert_eq!(
                    fast_power(base, exp),
                    naive_power(base, exp),
                    "Mismatch for {}^{}", base, exp
                );
            }
        }
    }

    #[test]
    fn test_modular_power() {
        assert_eq!(modular_power(2, 10, 1000), 24);     // 1024 % 1000 = 24
        assert_eq!(modular_power(3, 100, 101), 1);      // Fermat's little theorem
        assert_eq!(modular_power(7, 0, 100), 1);        // Any number^0 = 1
    }
}
