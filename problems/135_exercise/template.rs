// Exercise 135: Levenshtein Distance (Edit Distance)
//
// Learning Objective:
// Calculate the minimum number of single-character edits needed to transform
// one string into another. Useful for spell checking, DNA sequencing, etc.
//
// Key Concepts:
// - Dynamic programming
// - Edit operations: insert, delete, replace
// - Wagner-Fischer algorithm
// - Space optimization

/// TODO: Calculate Levenshtein distance between two strings
/// Returns the minimum number of edits (insert, delete, replace) needed
/// 
/// Algorithm (Wagner-Fischer):
/// dp[i][j] = edit distance between s1[0..i] and s2[0..j]
/// dp[0][j] = j (insert j characters)
/// dp[i][0] = i (delete i characters)
/// If s1[i-1] == s2[j-1]: dp[i][j] = dp[i-1][j-1] (no edit needed)
/// Else: dp[i][j] = 1 + min(dp[i-1][j],    // delete
///                          dp[i][j-1],    // insert
///                          dp[i-1][j-1])  // replace
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    
    // TODO: Create DP table
    
    // TODO: Initialize base cases (first row and column)
    
    // TODO: Fill the DP table using the recurrence relation
    
    // TODO: Return dp[m][n]
    
    0
}

/// TODO: Space-optimized version using O(min(m,n)) space
fn levenshtein_distance_optimized(s1: &str, s2: &str) -> usize {
    // TODO: Use only two rows (previous and current)
    // Ensure we iterate over the shorter string for columns
    
    0
}

/// TODO: Calculate distance and return the sequence of operations
/// Returns (distance, operations) where operations is a Vec of strings
/// describing each edit operation
fn levenshtein_with_operations(s1: &str, s2: &str) -> (usize, Vec<String>) {
    let m = s1.len();
    let n = s2.len();
    
    // TODO: Fill DP table
    
    // TODO: Backtrack to find the operations
    // Start from dp[m][n] and trace back to dp[0][0]
    // Record each operation along the way
    
    (0, vec![])
}

/// TODO: Calculate Damerau-Levenshtein distance
/// Same as Levenshtein but also allows transposition of adjacent characters
/// This allows "AB" -> "BA" in one operation instead of two
fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    // BONUS CHALLENGE
    // Similar to Levenshtein but add:
    // If s1[i-1] == s2[j-2] && s1[i-2] == s2[j-1] (transposition)
    // Then dp[i][j] = min(dp[i][j], dp[i-2][j-2] + 1)
    
    0
}

/// Calculate similarity ratio (0.0 to 1.0) based on edit distance
fn similarity(s1: &str, s2: &str) -> f64 {
    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        return 1.0;
    }
    let distance = levenshtein_distance(s1, s2);
    1.0 - (distance as f64 / max_len as f64)
}

fn main() {
    let test_cases = vec![
        ("kitten", "sitting"),
        ("saturday", "sunday"),
        ("intention", "execution"),
        ("", "abc"),
        ("same", "same"),
    ];
    
    println!("Levenshtein Distance Examples:\n");
    
    for (s1, s2) in test_cases {
        let dist = levenshtein_distance(s1, s2);
        let sim = similarity(s1, s2);
        println!("'{}' -> '{}'", s1, s2);
        println!("  Distance: {}", dist);
        println!("  Similarity: {:.2}%\n", sim * 100.0);
    }
    
    // Show operations
    let (dist, ops) = levenshtein_with_operations("kitten", "sitting");
    println!("Operations to transform 'kitten' to 'sitting':");
    for op in ops {
        println!("  - {}", op);
    }
    println!("  Total: {} operations", dist);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_levenshtein_kitten() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        // kitten -> sitten (replace 'k' with 's')
        // sitten -> sittin (replace 'e' with 'i')
        // sittin -> sitting (insert 'g')
    }
    
    #[test]
    fn test_levenshtein_saturday() {
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    }
    
    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
    
    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein_distance("same", "same"), 0);
    }
    
    #[test]
    fn test_levenshtein_one_char() {
        assert_eq!(levenshtein_distance("a", "b"), 1);
        assert_eq!(levenshtein_distance("a", "ab"), 1);
    }
    
    #[test]
    fn test_levenshtein_optimized_matches() {
        let s1 = "kitten";
        let s2 = "sitting";
        assert_eq!(
            levenshtein_distance(s1, s2),
            levenshtein_distance_optimized(s1, s2)
        );
    }
    
    #[test]
    fn test_similarity() {
        assert_eq!(similarity("same", "same"), 1.0);
        assert_eq!(similarity("", ""), 1.0);
        assert!(similarity("abc", "xyz") < 0.5);
    }
}
