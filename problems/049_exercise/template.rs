// Exercise 049: Strings - Concatenation
//
// Learning Objective: Learn different ways to concatenate strings in Rust
// and understand ownership implications.
//
// Topics covered:
// - Using + operator
// - Using format! macro
// - Concatenating with iterators
// - Ownership with string concatenation

fn main() {
    // Using + operator
    // Note: + takes ownership of the left operand and borrows the right
    // TODO: Concatenate with +
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed
    // println!("Result: '{}'", s3);
    // println!("s2 still valid: '{}'", s2);
    // println!("s1 is moved: '{}'", s1);  // Error! Uncomment to see

    // Chaining + operations
    // TODO: Concatenate multiple strings
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let result = s1 + "-" + &s2 + "-" + &s3;
    // println!("Chained: '{}'", result);

    // Using format! macro (cleaner for multiple strings)
    // TODO: Use format! macro
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let result = format!("{}-{}-{}", s1, s2, s3);
    // println!("Formatted: '{}'", result);
    // println!("All originals still valid: {}, {}, {}", s1, s2, s3);

    // Building strings efficiently
    // TODO: Build a comma-separated list
    // let items = vec!["apple", "banana", "cherry"];
    // let mut result = String::new();
    // for (i, item) in items.iter().enumerate() {
    //     if i > 0 {
    //         result.push_str(", ");
    //     }
    //     result.push_str(item);
    // }
    // println!("List: '{}'", result);

    // Using join() on iterator
    // TODO: Use iterator join
    // let items = vec!["red", "green", "blue"];
    // let result = items.join(", ");
    // println!("Joined: '{}'", result);

    // Concatenating with collect()
    // TODO: Use concat() on iterator of &str
    // let parts = ["Hello", " ", "World"];
    // let result: String = parts.concat();
    // println!("Concatenated: '{}'", result);

    // TODO: Use join() with separator
    // let parts = ["2024", "02", "07"];
    // let date = parts.join("-");
    // println!("Date: '{}'", date);

    // String slicing and concatenation
    // TODO: Combine slices of strings
    // let s = String::from("Hello, World!");
    // let first = &s[0..5];
    // let second = &s[7..12];
    // let result = format!("{} {}", first, second);
    // println!("Combined slices: '{}'", result);

    // Practical: Building a sentence
    // TODO: Build a sentence from words
    // let words = vec!["The", "quick", "brown", "fox"];
    // let mut sentence = String::new();
    // for word in &words {
    //     if !sentence.is_empty() {
    //         sentence.push(' ');
    //     }
    //     sentence.push_str(word);
    // }
    // sentence.push('.');
    // println!("Sentence: '{}'", sentence);
}

// COMPLETION CHECKLIST:
// [ ] Concatenate with + operator
// [ ] Observe ownership with +
// [ ] Chain multiple + operations
// [ ] Use format! macro
// [ ] Verify originals still valid with format!
// [ ] Build comma-separated list manually
// [ ] Use join() on iterator
// [ ] Use concat() on array of &str
// [ ] Join with different separators
// [ ] Combine string slices
// [ ] Build a complete sentence
// [ ] Verify the program compiles and runs
