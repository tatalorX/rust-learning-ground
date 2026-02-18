// Exercise 113: Reverse a String
// ================================
//
// Learning Objective:
// Learn different approaches to reverse a string in Rust, handling
// Unicode correctly. This is trickier than it seems due to UTF-8 encoding!

fn main() {
    let test_cases = vec![
        "hello",
        "Rust",
        "12345",
        "",
        "a",
        "Hello, World!",
        "こんにちは", // Japanese (multi-byte UTF-8)
        "👋🌍",       // Emoji (4-byte UTF-8)
    ];
    
    println!("=== String Reversal ===\n");
    
    for s in &test_cases {
        let reversed = reverse_string(s);
        let reversed_grapheme = reverse_string_grapheme(s);
        
        println!("Original: '{}'", s);
        println!("Reversed (char): '{}'", reversed);
        println!("Reversed (grapheme): '{}'", reversed_grapheme);
        println!();
    }
    
    // Verify specific cases
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("Rust"), "tsuR");
    assert_eq!(reverse_string("12345"), "54321");
    assert_eq!(reverse_string(""), "");
    
    println!("✓ String reversal completed successfully!");
}

/// Reverse a string by characters
///
/// This approach reverses the Unicode scalar values (chars).
/// It works correctly for ASCII and most Unicode text.
///
/// Algorithm:
/// 1. Convert string to iterator of chars
/// 2. Reverse the iterator
/// 3. Collect back into a String
fn reverse_string(s: &str) -> String {
    // TODO: Implement string reversal using chars
    // Hint: Use .chars().rev().collect()
    s.chars().rev().collect()
}

/// Reverse a string by grapheme clusters
///
/// This is the "correct" way to reverse text for display purposes.
/// Grapheme clusters are what users perceive as single characters,
/// even if they're composed of multiple Unicode code points.
///
/// Example: "é" can be represented as 'e' + combining acute accent
/// or as a single precomposed character.
///
/// Note: This requires the unicode-segmentation crate in real projects.
/// For this exercise, we'll implement a simplified version.
fn reverse_string_grapheme(s: &str) -> String {
    // TODO: Implement a grapheme-aware reversal
    // For this exercise, we'll use the char-based approach
    // In production, you'd use unicode_segmentation::UnicodeSegmentation
    
    // As a simple approach, we can use .chars() but note this isn't
    // perfect for all Unicode edge cases (combining characters)
    s.chars().rev().collect()
}

/// Reverse a string in place (for mutable String)
///
/// This modifies the String directly without allocating a new one.
fn reverse_string_in_place(s: &mut String) {
    // TODO: Implement in-place reversal
    // Since Rust strings are UTF-8 encoded, true in-place reversal
    // is complex. We'll use a Vec<char> as an intermediate.
    
    let chars: Vec<char> = s.chars().collect();
    let reversed: String = chars.into_iter().rev().collect();
    *s = reversed;
}

/// Reverse words in a string (Bonus)
///
/// Example: "Hello World" -> "World Hello"
fn reverse_words(s: &str) -> String {
    // TODO: Bonus - reverse the order of words
    // Split by whitespace, reverse the iterator, join with spaces
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Reverse each word but keep word order (Bonus)
///
/// Example: "Hello World" -> "olleH dlroW"
fn reverse_each_word(s: &str) -> String {
    // TODO: Bonus - reverse each word individually
    s.split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string("12345"), "54321");
    }

    #[test]
    fn test_reverse_string_empty() {
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_reverse_string_single() {
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_reverse_in_place() {
        let mut s = String::from("hello");
        reverse_string_in_place(&mut s);
        assert_eq!(s, "olleh");
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("Hello World"), "World Hello");
        assert_eq!(reverse_words("The quick brown fox"), "fox brown quick The");
    }

    #[test]
    fn test_reverse_each_word() {
        assert_eq!(reverse_each_word("Hello World"), "olleH dlroW");
    }
}
