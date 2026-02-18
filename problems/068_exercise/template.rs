// Exercise 068: Lifetimes - In Functions
//
// Functions often need lifetime annotations when they take multiple references
// and return a reference. The compiler needs to know how the output relates
// to the inputs.
//
// Learning Objectives:
// - Annotate function signatures with lifetimes
// - Handle multiple input lifetimes
// - Return references with correct lifetimes
//
// Your task: Add appropriate lifetime annotations to functions.

/// Returns the first string if it's longer, otherwise the second.
fn first_if_longer(first: &str, second: &str) -> &str {
    // TODO: Add lifetime annotations
    // Return first if first.len() > second.len(), else second
    todo!()
}

/// Returns a substring from start to end index.
fn substring(text: &str, start: usize, end: usize) -> &str {
    // TODO: Add lifetime annotation
    // Return &text[start..end.min(text.len())]
    todo!()
}

/// Returns the string that comes first alphabetically.
fn earliest_alphabetically(a: &str, b: &str) -> &str {
    // TODO: Add lifetime annotations
    // Return a if a < b, else b
    todo!()
}

/// Finds a word in a sentence and returns it.
fn find_word<'a>(sentence: &'a str, word: &str) -> Option<&'a str> {
    // TODO: Add appropriate lifetime annotations
    // Check if sentence contains word, return Some(word) if found
    todo!()
}

/// Splits a string at the first occurrence of a delimiter.
fn split_at_delimiter(text: &str, delimiter: char) -> (&str, &str) {
    // TODO: Add lifetime annotations
    // Return (before_delimiter, after_delimiter)
    // If delimiter not found, return (text, "")
    todo!()
}

/// Returns all lines from text that contain the pattern.
fn lines_containing<'a>(text: &'a str, pattern: &str) -> Vec<&'a str> {
    // TODO: Add appropriate lifetime annotations
    // Split text by lines, filter those containing pattern
    todo!()
}

/// Trims whitespace from both ends.
fn trim_both_ends(text: &str) -> &str {
    // TODO: Add lifetime annotation
    // Use text.trim()
    todo!()
}

fn main() {
    // After adding lifetime annotations, uncomment and run:
    
    // let text = "The quick brown fox jumps over the lazy dog";
    // 
    // println!("Text: {}", text);
    // println!("First if longer (vs 'hello world'): {}", 
    //     first_if_longer(text, "hello world"));
    // 
    // println!("\nSubstring (0-10): {}", substring(text, 0, 10));
    // println!("Substring (20-30): {}", substring(text, 20, 30));
    // 
    // let word1 = "apple";
    // let word2 = "banana";
    // println!("\nEarliest alphabetically: {}", 
    //     earliest_alphabetically(word1, word2));
    // 
    // match find_word(text, "fox") {
    //     Some(found) => println!("\nFound word: {}", found),
    //     None => println!("\nWord not found"),
    // }
    // 
    // let (before, after) = split_at_delimiter(text, 'j');
    // println!("\nSplit at 'j':");
    // println!("  Before: {}", before);
    // println!("  After: {}", after);
    // 
    // let lines = "Line one\nLine two with fox\nLine three";
    // let matching = lines_containing(lines, "fox");
    // println!("\nLines containing 'fox': {:?}", matching);
    // 
    // let spaced = "   hello world   ";
    // println!("\nTrimmed: '{}'", trim_both_ends(spaced));
    
    println!("Add lifetime annotations to make the functions compile!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_if_longer() {
        assert_eq!(first_if_longer("longer text", "short"), "longer text");
        assert_eq!(first_if_longer("short", "longer text"), "longer text");
    }

    #[test]
    fn test_substring() {
        let text = "Hello World";
        assert_eq!(substring(text, 0, 5), "Hello");
        assert_eq!(substring(text, 6, 11), "World");
    }

    #[test]
    fn test_earliest_alphabetically() {
        assert_eq!(earliest_alphabetically("apple", "banana"), "apple");
        assert_eq!(earliest_alphabetically("zebra", "apple"), "apple");
    }

    #[test]
    fn test_find_word_found() {
        let text = "The quick brown fox";
        assert_eq!(find_word(text, "fox"), Some("fox"));
    }

    #[test]
    fn test_find_word_not_found() {
        let text = "The quick brown fox";
        assert_eq!(find_word(text, "cat"), None);
    }

    #[test]
    fn test_split_at_delimiter() {
        let text = "Hello,World";
        let (before, after) = split_at_delimiter(text, ',');
        assert_eq!(before, "Hello");
        assert_eq!(after, "World");
    }

    #[test]
    fn test_split_at_delimiter_not_found() {
        let text = "Hello World";
        let (before, after) = split_at_delimiter(text, ',');
        assert_eq!(before, "Hello World");
        assert_eq!(after, "");
    }

    #[test]
    fn test_lines_containing() {
        let text = "apple pie\nbanana split\napple turnover";
        let lines = lines_containing(text, "apple");
        assert_eq!(lines, vec!["apple pie", "apple turnover"]);
    }

    #[test]
    fn test_trim_both_ends() {
        assert_eq!(trim_both_ends("  hello  "), "hello");
        assert_eq!(trim_both_ends("hello"), "hello");
    }
}
