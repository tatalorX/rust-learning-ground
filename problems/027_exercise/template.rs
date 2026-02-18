// Exercise 027: Ownership - Clone
//
// Learning Objective: Learn how to explicitly clone data to keep the original
// while creating a deep copy. Use .clone() when you need both copies.
//
// Topics covered:
// - Deep copying with .clone()
// - When to use clone (explicit and expensive operation)
// - Heap-allocated data types like String require clone for deep copy

fn main() {
    // TODO: Create a String "Hello, Clone!"
    // let original = ___;

    // TODO: Create a clone of original using .clone()
    // let copy = ___;

    // Now we have TWO independent Strings on the heap

    // TODO: Pass original to a function that takes ownership
    // consume_string(___);

    // TODO: Print the clone to prove we still have it
    // println!("Clone still available: {}", ___);

    // TODO: Also print the original to prove it was moved (this won't work!)
    // Uncomment to see the error:
    // println!("Original: {}", original);

    // Demonstration: clone is expensive but sometimes necessary
    let data = String::from("Important data");
    
    // TODO: Create two independent clones of data
    // let backup1 = ___;
    // let backup2 = ___;

    // TODO: Print all three strings to show they all exist
    // println!("Original: {}", data);
    // println!("Backup 1: {}", backup1);
    // println!("Backup 2: {}", backup2);
}

fn consume_string(s: String) {
    println!("Consumed: {}", s);
    // s is dropped here
}

// COMPLETION CHECKLIST:
// [ ] Create original String
// [ ] Clone it using .clone()
// [ ] Pass original to consume_string()
// [ ] Print the clone successfully
// [ ] Create two backups from data
// [ ] Print all three strings
// [ ] Verify the program compiles and runs
