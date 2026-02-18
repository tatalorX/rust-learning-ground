// Exercise 108: Fibonacci (Iterative with Memoization)
// ====================================================
//
// Learning Objective:
// Learn how to optimize the Fibonacci calculation using iteration with
// memoization (caching previous results). This avoids the exponential
// time complexity of the naive recursive approach.
//
// Time Complexity: O(n), Space Complexity: O(1) or O(n) depending on approach

fn main() {
    // Test cases
    let test_cases = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (10, 55),
        (30, 832040),
        (50, 12586269025),
    ];
    
    println!("=== Iterative Approach ===");
    for (n, expected) in &test_cases {
        let result = fibonacci_iterative(*n);
        println!("F({}) = {} (expected: {})", n, result, expected);
        assert_eq!(result, *expected);
    }
    
    println!("\n=== Memoization Approach ===");
    for (n, expected) in &test_cases {
        let result = fibonacci_memoized(*n);
        println!("F({}) = {} (expected: {})", n, result, expected);
        assert_eq!(result, *expected);
    }
    
    println!("\n✓ Fibonacci (iterative with memoization) completed successfully!");
}

/// Iterative Fibonacci with O(1) space
///
/// This approach uses constant space by only keeping track of
/// the last two Fibonacci numbers at any time.
///
/// Algorithm:
/// 1. Handle base cases
/// 2. Initialize variables for the previous two Fibonacci numbers
/// 3. Iterate from 2 to n, computing each Fibonacci number
/// 4. Update the previous two numbers after each iteration
fn fibonacci_iterative(n: u64) -> u64 {
    // TODO: Handle base cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // TODO: Initialize variables for F(n-2) and F(n-1)
    let mut prev2 = 0; // F(n-2)
    let mut prev1 = 1; // F(n-1)
    let mut current = 0;
    
    // TODO: Iterate from 2 to n (inclusive)
    for _ in 2..=n {
        // TODO: Calculate current Fibonacci number
        current = prev1 + prev2;
        
        // TODO: Update prev2 and prev1 for next iteration
        prev2 = prev1;
        prev1 = current;
    }
    
    current
}

/// Memoized Fibonacci with O(n) space
///
/// This approach stores all Fibonacci numbers up to n in a vector.
/// While it uses more space, it allows O(1) lookup for any F(k) where k <= n.
///
/// This is also useful if you need to access multiple Fibonacci numbers
/// and want to avoid recalculating.
fn fibonacci_memoized(n: u64) -> u64 {
    // TODO: Handle base cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // TODO: Create a memoization vector
    // Initialize with base cases: memo[0] = 0, memo[1] = 1
    let mut memo = vec![0u64; (n + 1) as usize];
    memo[0] = 0;
    memo[1] = 1;
    
    // TODO: Fill in the memoization table iteratively
    for i in 2..=n as usize {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    
    // TODO: Return the nth Fibonacci number
    memo[n as usize]
}

/// Dynamic Programming with closure-based memoization (Advanced)
///
/// This demonstrates a more functional approach using Rust closures.
/// This is advanced - try implementing the simpler versions first.
fn fibonacci_with_cache(n: u64, cache: &mut std::collections::HashMap<u64, u64>) -> u64 {
    // TODO: Check if result is already in cache
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    
    // TODO: Handle base cases
    if n <= 1 {
        return n;
    }
    
    // TODO: Calculate recursively with memoization
    let result = fibonacci_with_cache(n - 1, cache) + fibonacci_with_cache(n - 2, cache);
    
    // TODO: Store result in cache
    cache.insert(n, result);
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(10), 55);
        assert_eq!(fibonacci_iterative(20), 6765);
    }

    #[test]
    fn test_fibonacci_memoized() {
        assert_eq!(fibonacci_memoized(0), 0);
        assert_eq!(fibonacci_memoized(1), 1);
        assert_eq!(fibonacci_memoized(10), 55);
        assert_eq!(fibonacci_memoized(30), 832040);
    }

    #[test]
    fn test_fibonacci_large() {
        // This would be too slow with naive recursion!
        let result = fibonacci_iterative(50);
        assert_eq!(result, 12586269025);
    }

    #[test]
    fn test_fibonacci_with_cache() {
        let mut cache = std::collections::HashMap::new();
        assert_eq!(fibonacci_with_cache(10, &mut cache), 55);
        assert_eq!(fibonacci_with_cache(20, &mut cache), 6765);
    }
}
