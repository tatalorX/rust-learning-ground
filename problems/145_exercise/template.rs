// Exercise 145: KMP String Matching Algorithm
//
// Learning Objective:
// Implement the Knuth-Morris-Pratt algorithm for efficient string matching.
// Understand failure function (LPS array) construction.
//
// Key Concepts:
// - LPS (Longest Proper Prefix which is also Suffix) array
// - Pattern preprocessing: O(m)
// - Matching: O(n)
// - Overall: O(n + m)

/// TODO: Build the LPS (Longest Prefix Suffix) array for the pattern
/// lps[i] = length of longest proper prefix of pattern[0..=i] 
///          which is also a suffix of pattern[0..=i]
/// 
/// Algorithm:
/// 1. lps[0] = 0 (single char has no proper prefix)
/// 2. Use two pointers: len (length of previous longest prefix suffix)
///    and i (current position)
/// 3. If pattern[i] == pattern[len]: len++, lps[i] = len, i++
/// 4. Else if len != 0: len = lps[len-1] (try shorter prefix)
/// 5. Else: lps[i] = 0, i++
fn build_lps(pattern: &str) -> Vec<usize> {
    let pattern: Vec<char> = pattern.chars().collect();
    let m = pattern.len();
    let mut lps = vec![0; m];
    
    // TODO: Implement LPS construction
    // Initialize len = 0, i = 1
    // Loop while i < m
    //   If pattern[i] == pattern[len]: increment both, set lps[i] = len
    //   Else if len > 0: len = lps[len - 1]
    //   Else: lps[i] = 0, increment i
    
    lps
}

/// TODO: Find all occurrences of pattern in text using KMP
/// Returns vector of starting indices where pattern is found
fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || text.len() < pattern.len() {
        return vec![];
    }
    
    let lps = build_lps(pattern);
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    let n = text.len();
    let m = pattern.len();
    
    let mut result = Vec::new();
    
    // TODO: Implement KMP matching
    // Use i for text index, j for pattern index
    // While i < n:
    //   If text[i] == pattern[j]: increment both
    //   If j == m: pattern found at i-j, add to result, j = lps[j-1]
    //   Else if i < n and text[i] != pattern[j]:
    //     If j > 0: j = lps[j-1]
    //     Else: increment i
    
    result
}

/// TODO: Find first occurrence of pattern in text
fn kmp_find_first(text: &str, pattern: &str) -> Option<usize> {
    // TODO: Use kmp_search and return first result, or implement optimized version
    
    None
}

/// TODO: Count occurrences of pattern in text
fn kmp_count(text: &str, pattern: &str) -> usize {
    // TODO: Return count of matches
    
    0
}

/// TODO: Check if pattern is a substring of text
fn contains(text: &str, pattern: &str) -> bool {
    // TODO: Return true if kmp_search finds at least one match
    
    false
}

fn main() {
    let text = "ABABDABACDABABCABAB";
    let pattern = "ABABCABAB";
    
    println!("Text: {}", text);
    println!("Pattern: {}", pattern);
    
    let lps = build_lps(pattern);
    println!("\nLPS array: {:?}", lps);
    
    let matches = kmp_search(text, pattern);
    println!("Pattern found at indices: {:?}", matches);
    
    // More examples
    let text2 = "AAAAABAAABA";
    let pattern2 = "AAAB";
    println!("\nText: {}", text2);
    println!("Pattern: {}", pattern2);
    println!("LPS: {:?}", build_lps(pattern2));
    println!("Matches: {:?}", kmp_search(text2, pattern2));
    
    // Overlapping matches
    let text3 = "AAAAA";
    let pattern3 = "AAA";
    println!("\nText: {}", text3);
    println!("Pattern: {}", pattern3);
    println!("Overlapping matches: {:?}", kmp_search(text3, pattern3));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lps_basic() {
        assert_eq!(build_lps("AAAA"), vec![0, 1, 2, 3]);
        assert_eq!(build_lps("ABCDE"), vec![0, 0, 0, 0, 0]);
        assert_eq!(build_lps("AABAACAABAA"), vec![0, 1, 0, 1, 2, 0, 1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_kmp_search_basic() {
        let text = "ABABDABACDABABCABAB";
        let pattern = "ABABCABAB";
        assert_eq!(kmp_search(text, pattern), vec![10]);
    }
    
    #[test]
    fn test_kmp_search_multiple() {
        let text = "AABAACAADAABAABA";
        let pattern = "AABA";
        assert_eq!(kmp_search(text, pattern), vec![0, 9, 12]);
    }
    
    #[test]
    fn test_kmp_overlapping() {
        let text = "AAAAA";
        let pattern = "AAA";
        assert_eq!(kmp_search(text, pattern), vec![0, 1, 2]);
    }
    
    #[test]
    fn test_kmp_no_match() {
        let text = "ABCDEFG";
        let pattern = "XYZ";
        assert!(kmp_search(text, pattern).is_empty());
    }
    
    #[test]
    fn test_kmp_pattern_longer_than_text() {
        let text = "ABC";
        let pattern = "ABCDE";
        assert!(kmp_search(text, pattern).is_empty());
    }
    
    #[test]
    fn test_kmp_empty_pattern() {
        let text = "ABC";
        let pattern = "";
        assert!(kmp_search(text, pattern).is_empty());
    }
    
    #[test]
    fn test_kmp_single_char() {
        let text = "AAAAA";
        let pattern = "A";
        assert_eq!(kmp_search(text, pattern), vec![0, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_contains() {
        assert!(contains("Hello World", "World"));
        assert!(!contains("Hello World", "world")); // Case sensitive
        assert!(!contains("Hello", "Hello World"));
    }
}
