// Exercise 111: Sieve of Eratosthenes
// ===================================
//
// Learning Objective:
// Learn the Sieve of Eratosthenes, an ancient and efficient algorithm
// for finding all prime numbers up to a specified integer. It's one of
// the most efficient ways to find all smaller primes.
//
// Time Complexity: O(n log log n)
// Space Complexity: O(n)

fn main() {
    let limit = 50;
    
    println!("Finding all primes up to {}...", limit);
    
    let primes = sieve_of_eratosthenes(limit);
    
    println!("Primes found: {:?}", primes);
    println!("Count: {}", primes.len());
    
    // Verify against known primes up to 50
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    assert_eq!(primes, expected);
    
    // Test with larger limit
    let large_primes = sieve_of_eratosthenes(100);
    assert_eq!(large_primes.len(), 25); // 25 primes under 100
    
    println!("✓ Sieve of Eratosthenes completed successfully!");
}

/// Sieve of Eratosthenes Implementation
///
/// Algorithm:
/// 1. Create a boolean array "is_prime[0..n]" and initialize all entries as true
/// 2. Mark 0 and 1 as not prime
/// 3. For each number p from 2 to √n:
///    a. If is_prime[p] is true, mark all multiples of p as not prime
/// 4. Collect all indices that are still marked as prime
///
/// Key optimization: Start marking multiples from p*p (not 2*p)
/// because smaller multiples would have been marked by smaller primes.
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // TODO: Handle edge cases
    if n < 2 {
        return vec![];
    }
    
    // TODO: Create a boolean vector to track primality
    // Initialize all entries as true (assuming prime until proven otherwise)
    let mut is_prime = vec![true; n + 1];
    
    // TODO: Mark 0 and 1 as not prime
    is_prime[0] = false;
    is_prime[1] = false;
    
    // TODO: Implement the sieve
    // Iterate from 2 to √n
    let limit = (n as f64).sqrt() as usize + 1;
    
    for p in 2..limit {
        // TODO: If p is still marked as prime
        if is_prime[p] {
            // TODO: Mark all multiples of p as not prime
            // Start from p*p (optimization: smaller multiples already marked)
            // Step by p to hit all multiples
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }
    
    // TODO: Collect all prime numbers
    // Iterate through is_prime and collect indices that are still true
    let mut primes = Vec::new();
    for (i, &prime) in is_prime.iter().enumerate() {
        if prime {
            primes.push(i);
        }
    }
    
    primes
}

/// Optimized Sieve - Bit Vector Version (Bonus)
///
/// This version uses less memory by packing booleans into bits.
fn sieve_optimized(n: usize) -> Vec<usize> {
    // TODO: Bonus - implement using bit manipulation for memory efficiency
    // This is more advanced and requires understanding of bit operations
    sieve_of_eratosthenes(n) // Placeholder - implement optimized version
}

/// Count primes without storing them all (Bonus)
fn count_primes(n: usize) -> usize {
    // TODO: Bonus - return just the count without storing all primes
    sieve_of_eratosthenes(n).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_small() {
        assert_eq!(sieve_of_eratosthenes(1), vec![]);
        assert_eq!(sieve_of_eratosthenes(2), vec![2]);
        assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_sieve_30() {
        assert_eq!(sieve_of_eratosthenes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_sieve_100() {
        assert_eq!(sieve_of_eratosthenes(100).len(), 25);
    }

    #[test]
    fn test_count_primes() {
        assert_eq!(count_primes(100), 25);
        assert_eq!(count_primes(1000), 168);
    }
}
