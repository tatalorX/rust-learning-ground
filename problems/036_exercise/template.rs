// Exercise 036: Structs - Unit Structs
//
// Learning Objective: Learn about unit structs - structs with no fields.
// Useful for marker types and implementing traits on a type.
//
// Topics covered:
// - Defining unit structs (no fields, no parentheses)
// - Unit structs as marker types
// - Memory efficiency of unit structs (zero-size types)

fn main() {
    // TODO: Define a unit struct named AlwaysEqual
    // struct AlwaysEqual;

    // TODO: Create an instance of AlwaysEqual
    // No parentheses needed for unit struct instances!
    // let subject = AlwaysEqual;

    // Unit structs don't store data, but they ARE types
    // TODO: Try to print the subject (we'll implement Debug later)
    // For now, just create it - it compiles but has no data to display
    // println!("Created an AlwaysEqual instance");

    // Unit structs are often used as markers
    // TODO: Define a unit struct for Admin privilege
    // struct Admin;

    // TODO: Define a unit struct for User privilege  
    // struct User;

    // TODO: Create instances of both
    // let admin = Admin;
    // let user = User;

    // They act as compile-time markers
    // TODO: Call functions that accept these types
    // admin_function(admin);
    // user_function(user);

    // Can't mix them up - type safety at compile time!
    // admin_function(user); // ERROR! Expected Admin, found User

    // Unit structs can have trait implementations
    // TODO: Define a unit struct for a state machine state
    // struct Idle;
    // struct Running;
    // struct Paused;

    // TODO: Create instances representing states
    // let state = Idle;
    // println!("Current state: Idle (zero-size type)");

    // Zero-size optimization: unit structs take no memory
    // TODO: Print sizes to demonstrate
    // println!("Size of AlwaysEqual: {} bytes", std::mem::size_of::<AlwaysEqual>());
    // println!("Size of (): {} bytes", std::mem::size_of::<()>());
    // println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
}

// TODO: Write functions that accept the marker types
// fn admin_function(_admin: Admin) {
//     println!("Admin access granted!");
// }

// fn user_function(_user: User) {
//     println!("User access granted!");
// }

// Define unit structs here:
// struct AlwaysEqual;
// struct Admin;
// struct User;
// struct Idle;
// struct Running;
// struct Paused;

// COMPLETION CHECKLIST:
// [ ] Define AlwaysEqual unit struct
// [ ] Create an instance
// [ ] Define Admin and User unit structs
// [ ] Create instances of Admin and User
// [ ] Write admin_function and user_function
// [ ] Call both functions
// [ ] Define Idle, Running, Paused for state machine
// [ ] Print sizes of different types
// [ ] Verify the program compiles and runs
