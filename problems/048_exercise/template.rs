// Exercise 048: Strings - Creating and Modifying
//
// Learning Objective: Learn about Rust's String type, how to create and
// modify it. String is a growable, mutable, owned, UTF-8 encoded string.
//
// Topics covered:
// - Creating Strings with new() and from()
// - to_string() method
// - Modifying with push(), push_str()
// - Capacity and length

fn main() {
    // Creating empty strings
    // TODO: Create an empty String with new()
    // let mut s1 = String::new();
    // println!("Empty string: '{}'", s1);

    // Creating from string literal
    // TODO: Create from &str using from()
    // let s2 = String::from("Hello");
    // println!("From literal: '{}'", s2);

    // TODO: Create using to_string()
    // let s3 = "World".to_string();
    // println!("to_string: '{}'", s3);

    // Adding characters with push()
    // TODO: Push characters one by one
    // let mut s4 = String::from("H");
    // s4.push('e');
    // s4.push('l');
    // s4.push('l');
    // s4.push('o');
    // println!("After pushes: '{}'", s4);

    // Adding strings with push_str()
    // TODO: Push a string slice
    // let mut s5 = String::from("Hello");
    // s5.push_str(", World!");
    // println!("After push_str: '{}'", s5);

    // TODO: Push another String (must use & to borrow)
    // let addon = String::from(" Have a nice day.");
    // s5.push_str(&addon);
    // println!("Final: '{}'", s5);

    // Capacity and length
    // TODO: Check capacity and length
    // let mut s6 = String::with_capacity(100);
    // println!("Empty: length={}, capacity={}", s6.len(), s6.capacity());
    
    // s6.push_str("Short text");
    // println!("After text: length={}, capacity={}", s6.len(), s6.capacity());

    // Replacing content
    // TODO: Clear a string
    // let mut s7 = String::from("Temporary");
    // println!("Before clear: '{}'", s7);
    // s7.clear();
    // println!("After clear: '{}' (length: {})", s7, s7.len());

    // Truncating
    // TODO: Truncate a string
    // let mut s8 = String::from("Hello, World!");
    // s8.truncate(5);  // Keep only first 5 bytes
    // println!("After truncate: '{}'", s8);

    // Pop characters
    // TODO: Pop characters from the end
    // let mut s9 = String::from("ABC");
    // while let Some(ch) = s9.pop() {
    //     println!("Popped: '{}'", ch);
    // }
    // println!("After pops: '{}'", s9);

    // Remove at index
    // TODO: Remove a character at specific position
    // let mut s10 = String::from("Hello");
    // let removed = s10.remove(1);  // Removes 'e'
    // println!("Removed: '{}', Remaining: '{}'", removed, s10);

    // Insert at position
    // TODO: Insert a character
    // let mut s11 = String::from("Hllo");
    // s11.insert(1, 'e');
    // println!("After insert: '{}'", s11);

    // Insert string
    // TODO: Insert a string slice
    // let mut s12 = String::from("Hello World");
    // s12.insert_str(5, ",");
    // println!("After insert_str: '{}'", s12);
}

// COMPLETION CHECKLIST:
// [ ] Create empty String with new()
// [ ] Create String from literal with from()
// [ ] Create String using to_string()
// [ ] Use push() to add characters
// [ ] Use push_str() to add string slices
// [ ] Push a String using &
// [ ] Create String with capacity
// [ ] Check length and capacity
// [ ] Use clear() to empty string
// [ ] Use truncate()
// [ ] Use pop() to remove characters
// [ ] Use remove() at index
// [ ] Use insert() for character
// [ ] Use insert_str() for string
// [ ] Verify the program compiles and runs
