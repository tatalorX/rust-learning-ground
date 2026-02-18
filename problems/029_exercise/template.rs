// Exercise 029: References - Immutable Borrow
//
// Learning Objective: Learn to use references (&T) to borrow values without
// taking ownership. Multiple immutable references can exist simultaneously.
//
// Topics covered:
// - Creating references with &
// - Borrowing vs moving
// - Multiple immutable references are allowed
// - Original owner remains valid after borrowing

fn main() {
    // TODO: Create a String "Borrow me!"
    // let message = ___;

    // Create an immutable reference
    // TODO: Create ref1 as a reference to message using &
    // let ref1 = ___;

    // TODO: Print message and ref1 to show both are valid
    // println!("Original: {}", message);
    // println!("Reference: {}", ref1);

    // Multiple immutable references are allowed
    // TODO: Create ref2 and ref3 as additional references
    // let ref2 = ___;
    // let ref3 = ___;

    // TODO: Print all references
    // println!("ref1: {}", ref1);
    // println!("ref2: {}", ref2);
    // println!("ref3: {}", ref3);

    // We can also pass references to functions
    // TODO: Call print_length with a reference to message
    // print_length(___);

    // Original is still valid!
    // TODO: Print message again
    // println!("Message is still mine: {}", message);

    // References can be created in scopes
    // TODO: Create a scope (using { }) where you create a reference
    // and print it. Show that message is still valid after the scope.
    // {
    //     let inner_ref = &message;
    //     println!("Inside scope: {}", inner_ref);
    // }
    // println!("After scope: {}", message);
}

// This function borrows a String (doesn't take ownership)
fn print_length(s: &String) {
    // s is a reference, we can read but not modify
    println!("Length: {}", s.len());
    // s is NOT dropped here - we don't own it
}

// COMPLETION CHECKLIST:
// [ ] Create message String
// [ ] Create ref1 as &message
// [ ] Print both message and ref1
// [ ] Create ref2 and ref3 as additional references
// [ ] Print all three references
// [ ] Call print_length(&message)
// [ ] Print message again to show it's still valid
// [ ] Create a scope with a reference and print before/after
