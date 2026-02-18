// Exercise 032: Slices - String Slices
//
// Learning Objective: Learn about string slices (&str) - references to a
// contiguous sequence of characters in a String or string literal.
//
// Topics covered:
// - String slices (&str)
// - Creating slices with range syntax [start..end]
// - Slices as immutable references
// - String literals are string slices

fn main() {
    // String literals are string slices (immutable)
    let literal: &str = "Hello, world!";
    
    // TODO: Print the literal
    // println!("Literal: {}", literal);

    // Creating a String
    let s = String::from("Hello, Rustaceans!");
    
    // TODO: Create a slice of the first 5 characters "Hello"
    // let hello = &s[0..5];  // or &s[..5]
    // println!("Slice 1: {}", hello);

    // TODO: Create a slice of "Rustaceans" (positions 7 to 17)
    // let rust = &s[7..17];
    // println!("Slice 2: {}", rust);

    // TODO: Create a slice from position 7 to the end
    // let rest = &s[7..];  // or &s[7..s.len()]
    // println!("Rest: {}", rest);

    // TODO: Create a slice of the entire string
    // let all = &s[..];  // or &s[0..s.len()]
    // println!("All: {}", all);

    // String slices in functions
    let sentence = String::from("The quick brown fox");
    
    // TODO: Call first_word with a reference to sentence
    // let word = first_word(&sentence);
    // println!("First word: {}", word);

    // IMPORTANT: Slices are references - they borrow!
    // The original String cannot be modified while slices exist
    let mut text = String::from("hello world");
    let first = first_word(&text);
    
    // text.clear(); // ERROR! Can't mutate while borrowed
    
    // TODO: Print first (after this, the borrow ends)
    // println!("First word: {}", first);
    
    // Now we can modify text because first is no longer used
    text.clear();
    text.push_str("goodbye moon");
    // TODO: Print the modified text
    // println!("Modified: {}", text);

    // String slices can be used with string literals too
    // TODO: Call first_word with a string literal
    // let lit_word = first_word("slice of life");
    // println!("Literal word: {}", lit_word);
}

// Returns the first word (up to first space) as a string slice
// Using &str instead of &String makes this more flexible!
fn first_word(s: &str) -> &str {
    // Convert to bytes to search for space
    let bytes = s.as_bytes();
    
    // Iterate with index and reference to byte
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Found a space, return slice from start to here
            return &s[0..i];
        }
    }
    
    // No space found, return entire string
    &s[..]
}

// COMPLETION CHECKLIST:
// [ ] Print the string literal
// [ ] Create and print slice "Hello"
// [ ] Create and print slice "Rustaceans"
// [ ] Create and print slice from position 7 to end
// [ ] Create and print slice of entire string
// [ ] Call first_word and print result
// [ ] Print first, then modify text, then print modified text
// [ ] Call first_word with a string literal
