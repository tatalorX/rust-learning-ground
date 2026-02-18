// Exercise 030: References - Mutable Borrow
//
// Learning Objective: Learn to use mutable references (&mut T) to modify
// borrowed values. Only ONE mutable reference can exist at a time.
//
// Topics covered:
// - Creating mutable references with &mut
// - Only one mutable reference allowed at a time
// - Mutable references require the original to be mutable
// - Original owner cannot be used while mutably borrowed

fn main() {
    // TODO: Create a mutable String "Hello"
    // let mut greeting = ___;

    // TODO: Create a mutable reference to greeting
    // let ref1 = &mut greeting;

    // Modify through the mutable reference
    // TODO: Push ", World!" to ref1
    // ref1.push_str(___);

    // Print the modified string
    // println!("Modified: {}", ref1);

    // The mutable reference is no longer used after this point
    // We can create a new one
    // TODO: Create ref2 as a new mutable reference
    // let ref2 = &mut greeting;
    
    // TODO: Push "!!!" through ref2
    // ref2.push_str(___);

    // TODO: Print greeting to see all changes
    // println!("Final: {}", greeting);

    // Demonstrate: mutable reference in a function
    // TODO: Create a mutable number
    // let mut number = 10;
    
    // TODO: Call double_value with a mutable reference
    // double_value(___);
    
    // TODO: Print number to verify it changed
    // println!("Doubled: {}", number);

    // Demonstrate: scope allows multiple mutable borrows (sequentially)
    let mut data = String::from("Start");
    
    // TODO: Create a scope where you borrow mutably and modify,
    // then show you can use data again after the scope
    // {
    //     let data_ref = &mut data;
    //     data_ref.push_str(" - Middle");
    //     println!("Inside scope: {}", data_ref);
    // }
    // data.push_str(" - End");
    // println!("After scope: {}", data);
}

fn double_value(n: &mut i32) {
    // Dereference with * to modify the value
    *n = *n * 2;
}

// COMPLETION CHECKLIST:
// [ ] Create mutable greeting String
// [ ] Create ref1 as &mut greeting
// [ ] Push ", World!" through ref1
// [ ] Print the modified string
// [ ] Create ref2 as new mutable reference
// [ ] Push "!!!" through ref2
// [ ] Print final greeting
// [ ] Create mutable number and call double_value(&mut number)
// [ ] Print the doubled number
// [ ] Create scope with mutable borrow, then use data after
