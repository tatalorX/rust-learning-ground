// Exercise 050: HashMap - Basic Operations
//
// Learning Objective: Learn how to use HashMap to store key-value pairs.
// HashMaps are unordered collections with O(1) average lookup time.
//
// Topics covered:
// - Creating HashMaps
// - Inserting key-value pairs
// - Accessing values
// - Updating entries
// - Checking for keys

use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    // TODO: Create an empty HashMap<String, i32>
    // let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting values
    // TODO: Insert some scores
    // scores.insert(String::from("Alice"), 95);
    // scores.insert(String::from("Bob"), 87);
    // scores.insert(String::from("Charlie"), 92);
    // println!("Scores: {:?}", scores);

    // Accessing values with get
    // TODO: Get a value using get() (returns Option<&V>)
    // match scores.get("Alice") {
    //     Some(score) => println!("Alice's score: {}", score),
    //     None => println!("Alice not found"),
    // }

    // TODO: Try to get a non-existent key
    // match scores.get("David") {
    //     Some(score) => println!("David's score: {}", score),
    //     None => println!("David not in map"),
    // }

    // Using if let for cleaner access
    // TODO: Use if let to check for Bob
    // if let Some(score) = scores.get("Bob") {
    //     println!("Bob scored {}", score);
    // }

    // Checking if key exists
    // TODO: Check if Charlie is in the map
    // if scores.contains_key("Charlie") {
    //     println!("Charlie is in the map");
    // }

    // Updating values
    // TODO: Update Bob's score
    // scores.insert(String::from("Bob"), 90);
    // println!("Bob's updated score: {:?}", scores.get("Bob"));

    // Inserting only if key doesn't exist (or_insert)
    // TODO: Use entry API
    // scores.entry(String::from("David")).or_insert(88);
    // scores.entry(String::from("Alice")).or_insert(100);  // Won't change
    // println!("After entry ops: {:?}", scores);

    // Updating based on old value
    // TODO: Increment a value
    // let count = scores.entry(String::from("Eve")).or_insert(0);
    // *count += 1;  // Increment
    // println!("Eve's count: {:?}", scores.get("Eve"));

    // Iterating over entries
    // TODO: Print all entries
    // for (name, score) in &scores {
    //     println!("{} => {}", name, score);
    // }

    // HashMap from vectors (collect)
    // TODO: Create from vector of tuples
    // let teams = vec![("Blue", 10), ("Red", 5), ("Green", 15)];
    // let team_scores: HashMap<_, _> = teams.into_iter().collect();
    // println!("Team scores: {:?}", team_scores);

    // Counting occurrences
    // TODO: Count character frequencies
    // let text = "hello world";
    // let mut char_counts = HashMap::new();
    // for ch in text.chars() {
    //     if ch != ' ' {
    //         let count = char_counts.entry(ch).or_insert(0);
    //         *count += 1;
    //     }
    // }
    // println!("Character counts: {:?}", char_counts);

    // Removing entries
    // TODO: Remove a key
    // scores.remove("Bob");
    // println!("After removing Bob: {:?}", scores);
    // println!("Bob exists? {}", scores.contains_key("Bob"));

    // Length and is_empty
    // TODO: Check map size
    // println!("Number of entries: {}", scores.len());
    // println!("Is empty? {}", scores.is_empty());

    // Clear all entries
    // TODO: Clear the map
    // scores.clear();
    // println!("After clear: {:?}, len: {}", scores, scores.len());
}

// COMPLETION CHECKLIST:
// [ ] Create empty HashMap
// [ ] Insert key-value pairs
// [ ] Access with get() using match
// [ ] Handle non-existent key
// [ ] Use if let for access
// [ ] Use contains_key()
// [ ] Update existing value
// [ ] Use entry().or_insert()
// [ ] Update value based on old value
// [ ] Iterate over entries
// [ ] Create HashMap from vector using collect()
// [ ] Count character frequencies
// [ ] Remove entries
// [ ] Check length and is_empty
// [ ] Clear the map
// [ ] Verify the program compiles and runs
