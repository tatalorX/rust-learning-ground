// Exercise 004: String vs &str
// Learning objective: Understand the difference between owned String and string slice (&str)

fn main() {
    // TODO: Create a String (owned, growable) with value "Hello"
    // Hint: Use String::from() or "text".to_string()
    let owned_string = // ...
    
    // TODO: Create a string slice (&str) - a borrowed reference to string data
    // Hint: Use a string literal or borrow from the owned String with &
    let string_slice = // ...
    
    println!("Owned String: {}", owned_string);
    println!("String slice: {}", string_slice);
    
    // String can be modified because it's owned and mutable
    let mut mutable_string = String::from("Hello");
    // TODO: Append ", World!" to mutable_string using push_str()
    // ...
    
    println!("Modified string: {}", mutable_string);
}
