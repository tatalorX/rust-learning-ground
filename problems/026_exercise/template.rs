// Exercise 026: Ownership - Move Semantics
// 
// Learning Objective: Understand how ownership and move semantics work in Rust.
// In Rust, each value has a single owner. When a value is assigned to another
// variable or passed to a function, ownership is MOVED (not copied).
//
// Topics covered:
// - Ownership transfer (move)
// - The owner is responsible for dropping the value
// - Once moved, the original variable can no longer be used

fn main() {
    // TODO: Create a String with value "Hello, Rust!"
    // let greeting = ___;

    // This function takes ownership of greeting
    // TODO: Call take_ownership with greeting
    // take_ownership(___);

    // TODO: Try to use greeting here after moving it
    // This will cause a compile error - uncomment to see:
    // println!("{}", greeting);

    // Fix: We need to create a new string since greeting was moved
    // TODO: Create a new String and use it instead
    // let new_greeting = ___;
    // println!("I can still use: {}", new_greeting);
}

// This function takes ownership of the String
fn take_ownership(s: String) {
    // s is now owned by this function
    println!("I own: {}", s);
    // When this function ends, s is dropped and memory is freed
}

// COMPLETION CHECKLIST:
// [ ] Create a String assigned to `greeting`
// [ ] Call take_ownership(greeting) to move the value
// [ ] Observe the compile error if you try to use greeting after the move
// [ ] Create a new string to print at the end
// [ ] Run `rustc template.rs && ./template` to verify it compiles and runs
