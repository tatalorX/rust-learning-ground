// Exercise 115: FizzBuzz Algorithm
// ==================================
//
// Learning Objective:
// Learn how to implement the classic FizzBuzz programming challenge.
// This tests basic programming concepts: loops, conditionals, and modulo.
//
// Rules:
// - Print numbers from 1 to n
// - For multiples of 3, print "Fizz" instead of the number
// - For multiples of 5, print "Buzz" instead of the number
// - For multiples of both 3 and 5, print "FizzBuzz"

fn main() {
    println!("=== Classic FizzBuzz (1 to 20) ===\n");
    
    // Print first 20 FizzBuzz results
    for i in 1..=20 {
        let result = fizzbuzz(i);
        print!("{} ", result);
    }
    println!("\n");
    
    // Verify specific cases
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    
    println!("=== Custom FizzBuzz (with different rules) ===\n");
    
    // Custom version with different divisors
    for i in 1..=20 {
        let result = custom_fizzbuzz(i, 2, 7, "Even", "Lucky");
        print!("{} ", result);
    }
    println!("\n");
    
    println!("✓ FizzBuzz completed successfully!");
}

/// Classic FizzBuzz implementation
///
/// Returns the appropriate string for a given number:
/// - "Fizz" if divisible by 3
/// - "Buzz" if divisible by 5
/// - "FizzBuzz" if divisible by both
/// - The number itself as a string otherwise
///
/// Order of conditions matters! Check for both (3 and 5) first,
/// otherwise "FizzBuzz" will never be returned.
fn fizzbuzz(n: u32) -> String {
    // TODO: Check if n is divisible by both 3 and 5 first
    if n % 3 == 0 && n % 5 == 0 {
        return "FizzBuzz".to_string();
    }
    
    // TODO: Check if n is divisible by 3
    if n % 3 == 0 {
        return "Fizz".to_string();
    }
    
    // TODO: Check if n is divisible by 5
    if n % 5 == 0 {
        return "Buzz".to_string();
    }
    
    // TODO: Return the number as a string
    n.to_string()
}

/// Alternative FizzBuzz using match
///
/// This shows a more idiomatic Rust approach using pattern matching.
fn fizzbuzz_match(n: u32) -> String {
    // TODO: Implement using match on a tuple
    // Match on (n % 3 == 0, n % 5 == 0) to cover all cases
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => n.to_string(),
    }
}

/// Custom FizzBuzz with configurable parameters
///
/// Allows specifying different divisors and replacement words.
fn custom_fizzbuzz(
    n: u32,
    div1: u32,
    div2: u32,
    word1: &str,
    word2: &str,
) -> String {
    // TODO: Implement generalized FizzBuzz
    // Use the provided divisors and words
    
    match (n % div1 == 0, n % div2 == 0) {
        (true, true) => format!("{}{}", word1, word2),
        (true, false) => word1.to_string(),
        (false, true) => word2.to_string(),
        (false, false) => n.to_string(),
    }
}

/// Generate FizzBuzz sequence as a vector
fn fizzbuzz_sequence(n: u32) -> Vec<String> {
    // TODO: Generate a sequence from 1 to n
    (1..=n).map(|i| fizzbuzz(i)).collect()
}

/// FizzBuzz with additional rules (Bonus)
///
/// Extended version that also prints:
/// - "Jazz" for multiples of 7
/// - "FizzBuzzJazz" for multiples of 3, 5, and 7
fn fizzbuzz_extended(n: u32) -> String {
    // TODO: Bonus - extend with additional rules for multiples of 7
    let fizz = n % 3 == 0;
    let buzz = n % 5 == 0;
    let jazz = n % 7 == 0;
    
    match (fizz, buzz, jazz) {
        (true, true, true) => "FizzBuzzJazz".to_string(),
        (true, true, false) => "FizzBuzz".to_string(),
        (true, false, true) => "FizzJazz".to_string(),
        (false, true, true) => "BuzzJazz".to_string(),
        (true, false, false) => "Fizz".to_string(),
        (false, true, false) => "Buzz".to_string(),
        (false, false, true) => "Jazz".to_string(),
        (false, false, false) => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_number() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(4), "4");
    }

    #[test]
    fn test_fizzbuzz_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(6), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(45), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_match() {
        // Both implementations should give same results
        for i in 1..=100 {
            assert_eq!(fizzbuzz(i), fizzbuzz_match(i));
        }
    }

    #[test]
    fn test_custom_fizzbuzz() {
        assert_eq!(custom_fizzbuzz(2, 2, 3, "A", "B"), "A");
        assert_eq!(custom_fizzbuzz(3, 2, 3, "A", "B"), "B");
        assert_eq!(custom_fizzbuzz(6, 2, 3, "A", "B"), "AB");
    }

    #[test]
    fn test_sequence() {
        let seq = fizzbuzz_sequence(5);
        assert_eq!(seq, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn test_extended() {
        assert_eq!(fizzbuzz_extended(21), "FizzJazz"); // 3 * 7
        assert_eq!(fizzbuzz_extended(35), "BuzzJazz"); // 5 * 7
        assert_eq!(fizzbuzz_extended(105), "FizzBuzzJazz"); // 3 * 5 * 7
    }
}
