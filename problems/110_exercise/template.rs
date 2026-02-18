// Exercise 110: Prime Number Check
// ==================================
//
// Learning Objective:
// Learn how to determine if a number is prime. A prime number is a natural
// number greater than 1 that has no positive divisors other than 1 and itself.
//
// This exercise covers:
/// - Basic prime checking
/// - Optimization techniques (checking only up to √n)
/// - Handling edge cases

fn main() {
    // Test cases: (number, is_prime)
    let test_cases = vec![
        (0, false),
        (1, false),
        (2, true),   // Smallest prime
        (3, true),
        (4, false),
        (17, true),
        (18, false),
        (97, true),
        (100, false),
        (104729, true), // Large prime
    ];
    
    println!("=== Prime Check ===");
    for (n, expected) in &test_cases {
        let result = is_prime(*n);
        let status = if result == *expected { "✓" } else { "✗" };
        println!("{} is_prime({}) = {} (expected: {})", status, n, result, expected);
        assert_eq!(result, *expected);
    }
    
    // Test finding primes in a range
    println!("\n=== Primes up to 50 ===");
    let primes: Vec<u64> = (2..=50).filter(|&n| is_prime(n)).collect();
    println!("{:?}", primes);
    
    println!("\n✓ Prime number check completed successfully!");
}

/// Check if a number is prime
///
/// A prime number is a natural number greater than 1 that has exactly
/// two distinct divisors: 1 and itself.
///
/// Optimizations used:
/// 1. Handle numbers less than 2 (not prime)
/// 2. Handle 2 separately (it's the only even prime)
/// 3. Eliminate all other even numbers
/// 4. Only check odd divisors up to √n
///
/// Why √n? If n = a × b, at least one of a or b must be ≤ √n
fn is_prime(n: u64) -> bool {
    // TODO: Handle edge cases
    // Numbers less than 2 are not prime
    if n < 2 {
        return false;
    }
    
    // TODO: Handle 2 (the only even prime)
    if n == 2 {
        return true;
    }
    
    // TODO: Eliminate all other even numbers
    if n % 2 == 0 {
        return false;
    }
    
    // TODO: Check odd divisors from 3 up to √n
    // Start at 3, increment by 2 (only check odd numbers)
    let limit = (n as f64).sqrt() as u64 + 1;
    
    for i in (3..limit).step_by(2) {
        // TODO: If n is divisible by i, it's not prime
        if n % i == 0 {
            return false;
        }
    }
    
    // TODO: If no divisors found, n is prime
    true
}

/// Naive prime check (for comparison)
///
/// This is a simpler but much slower implementation that checks
/// all numbers from 2 to n-1.
fn is_prime_naive(n: u64) -> bool {
    // TODO: Implement naive version for comparison
    if n < 2 {
        return false;
    }
    
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

/// Generate all primes up to n using trial division
fn primes_up_to(n: u64) -> Vec<u64> {
    // TODO: Generate all primes up to n
    (2..=n).filter(|&x| is_prime(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_basic() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
    }

    #[test]
    fn test_prime_larger() {
        assert!(is_prime(17));
        assert!(is_prime(97));
        assert!(is_prime(541));
        assert!(!is_prime(100));
        assert!(!is_prime(1000));
    }

    #[test]
    fn test_prime_very_large() {
        assert!(is_prime(104729)); // 10000th prime
        assert!(!is_prime(104730));
    }

    #[test]
    fn test_primes_up_to() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_naive_matches_optimized() {
        // Verify both implementations give the same results
        for n in 0..100 {
            assert_eq!(is_prime(n), is_prime_naive(n));
        }
    }
}
