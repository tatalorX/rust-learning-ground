// Exercise 114: Anagram Check
// ===========================
//
// Learning Objective:
// Learn how to determine if two strings are anagrams of each other.
// An anagram is a word or phrase formed by rearranging the letters
// of a different word or phrase, using all the original letters exactly once.
//
// Examples:
// - "listen" and "silent" are anagrams
// - "hello" and "world" are not anagrams

fn main() {
    let test_cases = vec![
        ("listen", "silent", true),
        ("triangle", "integral", true),
        ("hello", "world", false),
        ("a", "a", true),
        ("a", "b", false),
        ("", "", true),
        ("abc", "abcd", false),
        ("Dormitory", "Dirty room", true), // With spaces and case difference
        ("anagram", "nagaram", true),
    ];
    
    println!("=== Anagram Check ===\n");
    
    for (s1, s2, expected) in &test_cases {
        let result_sort = is_anagram_sort(s1, s2);
        let result_count = is_anagram_count(s1, s2);
        
        let status_sort = if result_sort == *expected { "✓" } else { "✗" };
        let status_count = if result_count == *expected { "✓" } else { "✗" };
        
        println!("'{}' vs '{}'", s1, s2);
        println!("  Sort method: {} {}", status_sort, result_sort);
        println!("  Count method: {} {}", status_count, result_count);
        println!();
        
        assert_eq!(result_sort, *expected);
        assert_eq!(result_count, *expected);
    }
    
    println!("✓ Anagram check completed successfully!");
}

/// Anagram check using sorting
///
/// Algorithm:
/// 1. Normalize both strings (lowercase, remove spaces)
/// 2. Sort the characters of both strings
/// 3. Compare the sorted strings
///
/// Time Complexity: O(n log n) due to sorting
/// Space Complexity: O(n)
fn is_anagram_sort(s1: &str, s2: &str) -> bool {
    // TODO: Normalize strings - convert to lowercase and remove non-alphanumeric
    let normalized1: Vec<char> = s1
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    let normalized2: Vec<char> = s2
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    // TODO: Quick check: if lengths differ, they can't be anagrams
    if normalized1.len() != normalized2.len() {
        return false;
    }
    
    // TODO: Sort both character vectors
    let mut sorted1 = normalized1;
    sorted1.sort_unstable();
    
    let mut sorted2 = normalized2;
    sorted2.sort_unstable();
    
    // TODO: Compare sorted vectors
    sorted1 == sorted2
}

/// Anagram check using character counting
///
/// Algorithm:
/// 1. Normalize both strings (lowercase, remove spaces)
/// 2. Count occurrences of each character in both strings
/// 3. Compare the character counts
///
/// Time Complexity: O(n)
/// Space Complexity: O(k) where k is the character set size
fn is_anagram_count(s1: &str, s2: &str) -> bool {
    // TODO: Normalize strings
    let normalized1: Vec<char> = s1
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    let normalized2: Vec<char> = s2
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    // TODO: Quick check: if lengths differ, they can't be anagrams
    if normalized1.len() != normalized2.len() {
        return false;
    }
    
    // TODO: Count characters in first string using a HashMap
    use std::collections::HashMap;
    let mut char_count: HashMap<char, i32> = HashMap::new();
    
    for c in normalized1 {
        *char_count.entry(c).or_insert(0) += 1;
    }
    
    // TODO: Subtract counts for characters in second string
    for c in normalized2 {
        let count = char_count.entry(c).or_insert(0);
        *count -= 1;
        
        // If count goes negative, we have a mismatch
        if *count < 0 {
            return false;
        }
    }
    
    // TODO: Verify all counts are zero (optional, since lengths are equal)
    // If lengths are equal and we didn't return early, they must be anagrams
    true
}

/// Optimized anagram check for ASCII only (Bonus)
///
/// Uses a fixed-size array instead of HashMap for better performance.
/// Assumes input contains only ASCII characters.
fn is_anagram_ascii(s1: &str, s2: &str) -> bool {
    // TODO: Bonus - implement using array instead of HashMap
    // This is more efficient for ASCII-only strings
    
    if s1.len() != s2.len() {
        return false;
    }
    
    let s1_lower = s1.to_lowercase();
    let s2_lower = s2.to_lowercase();
    
    // Use a fixed-size array for 26 lowercase letters
    let mut counts = [0i32; 26];
    
    for c in s1_lower.chars() {
        if c.is_ascii_lowercase() {
            counts[(c as u8 - b'a') as usize] += 1;
        }
    }
    
    for c in s2_lower.chars() {
        if c.is_ascii_lowercase() {
            counts[(c as u8 - b'a') as usize] -= 1;
            if counts[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }
    }
    
    counts.iter().all(|&c| c == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_basic() {
        assert!(is_anagram_sort("listen", "silent"));
        assert!(is_anagram_count("listen", "silent"));
    }

    #[test]
    fn test_not_anagram() {
        assert!(!is_anagram_sort("hello", "world"));
        assert!(!is_anagram_count("hello", "world"));
    }

    #[test]
    fn test_anagram_case_insensitive() {
        assert!(is_anagram_sort("Triangle", "Integral"));
        assert!(is_anagram_count("Triangle", "Integral"));
    }

    #[test]
    fn test_anagram_with_spaces() {
        assert!(is_anagram_sort("Dormitory", "Dirty room"));
        assert!(is_anagram_count("Dormitory", "Dirty room"));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!is_anagram_sort("abc", "abcd"));
        assert!(!is_anagram_count("abc", "abcd"));
    }

    #[test]
    fn test_empty_strings() {
        assert!(is_anagram_sort("", ""));
        assert!(is_anagram_count("", ""));
    }

    #[test]
    fn test_ascii_optimized() {
        assert!(is_anagram_ascii("listen", "silent"));
        assert!(!is_anagram_ascii("hello", "world"));
    }
}
