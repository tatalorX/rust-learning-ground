// Exercise 134: Longest Common Subsequence (LCS)
//
// Learning Objective:
// Implement the LCS algorithm using dynamic programming.
// Understand how to find patterns in sequences.
//
// Key Concepts:
// - Dynamic programming
// - Subsequence vs substring
// - 2D DP table
// - Backtracking to find the actual sequence

/// TODO: Find the length of the longest common subsequence
/// Returns just the length (not the sequence itself)
/// 
/// Algorithm:
/// Create a DP table where dp[i][j] = LCS length of s1[0..i] and s2[0..j]
/// If s1[i-1] == s2[j-1]: dp[i][j] = dp[i-1][j-1] + 1
/// Else: dp[i][j] = max(dp[i-1][j], dp[i][j-1])
fn lcs_length(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    
    // TODO: Create DP table with dimensions (m+1) x (n+1)
    // Initialize first row and column with 0
    
    // TODO: Fill the DP table
    // Compare characters and fill based on match/mismatch
    
    // TODO: Return dp[m][n]
    
    0
}

/// TODO: Find and return the actual longest common subsequence string
fn lcs(s1: &str, s2: &str) -> String {
    let m = s1.len();
    let n = s2.len();
    
    // TODO: Create and fill DP table (same as lcs_length)
    
    // TODO: Backtrack from dp[m][n] to reconstruct the LCS
    // Start from bottom-right and move:
    //   - If characters match: add to result, move diagonally up-left
    //   - Else: move in direction of larger value (up or left)
    
    // TODO: Reverse the result string before returning
    
    String::new()
}

/// TODO: Space-optimized version that uses O(min(m,n)) space
/// Returns only the length
fn lcs_length_optimized(s1: &str, s2: &str) -> usize {
    // TODO: Use only two rows instead of full matrix
    // Swap rows if needed so s2 is the shorter string
    // Keep only previous row and current row
    
    0
}

/// TODO: Find all longest common subsequences
/// Returns a set of all LCS strings (there may be multiple)
fn all_lcs(s1: &str, s2: &str) -> Vec<String> {
    // BONUS CHALLENGE
    // This is more complex - requires storing backtracking information
    // and exploring all possible paths when dp[i-1][j] == dp[i][j-1]
    
    vec![]
}

fn main() {
    let s1 = "ABCDGH";
    let s2 = "AEDFHR";
    
    println!("String 1: {}", s1);
    println!("String 2: {}", s2);
    
    let length = lcs_length(s1, s2);
    println!("\nLCS Length: {}", length);
    
    let sequence = lcs(s1, s2);
    println!("LCS: {}", sequence);
    
    // More examples
    let test_cases = vec![
        ("AGGTAB", "GXTXAYB"),
        ("ABC", "AC"),
        ("XMJYAUZ", "MZJAWXU"),
    ];
    
    println!("\n--- Additional Test Cases ---");
    for (a, b) in test_cases {
        println!("\n'{a}' vs '{b}'");
        println!("  LCS Length: {}", lcs_length(a, b));
        println!("  LCS: '{}'", lcs(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lcs_basic() {
        assert_eq!(lcs_length("ABCDGH", "AEDFHR"), 3);
        assert_eq!(lcs("ABCDGH", "AEDFHR"), "ADH");
    }
    
    #[test]
    fn test_lcs_example2() {
        assert_eq!(lcs_length("AGGTAB", "GXTXAYB"), 4);
        assert_eq!(lcs("AGGTAB", "GXTXAYB"), "GTAB");
    }
    
    #[test]
    fn test_lcs_one_empty() {
        assert_eq!(lcs_length("ABC", ""), 0);
        assert_eq!(lcs("ABC", ""), "");
    }
    
    #[test]
    fn test_lcs_both_empty() {
        assert_eq!(lcs_length("", ""), 0);
        assert_eq!(lcs("", ""), "");
    }
    
    #[test]
    fn test_lcs_no_common() {
        assert_eq!(lcs_length("ABC", "DEF"), 0);
        assert_eq!(lcs("ABC", "DEF"), "");
    }
    
    #[test]
    fn test_lcs_identical() {
        assert_eq!(lcs_length("ABC", "ABC"), 3);
        assert_eq!(lcs("ABC", "ABC"), "ABC");
    }
    
    #[test]
    fn test_lcs_subsequence() {
        assert_eq!(lcs_length("ABCDE", "ACE"), 3);
        assert_eq!(lcs("ABCDE", "ACE"), "ACE");
    }
    
    #[test]
    fn test_lcs_optimized_matches() {
        let s1 = "ABCDGH";
        let s2 = "AEDFHR";
        assert_eq!(lcs_length(s1, s2), lcs_length_optimized(s1, s2));
    }
}
