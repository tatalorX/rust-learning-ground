// Exercise 041: Enums - Option<T>
//
// Learning Objective: Learn about Option<T>, Rust's way of representing
// values that may or may not exist. No null pointers in Rust!
//
// Topics covered:
// - Option<T> with Some(T) and None variants
// - Using Option to handle missing values
// - Common Option methods: is_some(), is_none(), unwrap(), expect()

fn main() {
    // Option<T> is defined in the standard library:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // TODO: Create a Some variant with an i32
    // let some_number = Some(5);
    // println!("Some number: {:?}", some_number);

    // TODO: Create a None variant (need type annotation!)
    // let absent_number: Option<i32> = None;
    // println!("Absent number: {:?}", absent_number);

    // Working with Option values
    // TODO: Check if values are Some or None
    // println!("some_number is_some: {}", some_number.is_some());
    // println!("some_number is_none: {}", some_number.is_none());
    // println!("absent_number is_none: {}", absent_number.is_none());

    // Unwrapping Options (be careful!)
    // TODO: Safely unwrap using match
    // match some_number {
    //     Some(n) => println!("Got number: {}", n),
    //     None => println!("No number"),
    // }

    // TODO: Use unwrap() (panics on None!)
    // let value = some_number.unwrap();
    // println!("Unwrapped: {}", value);

    // TODO: Use expect() with a message
    // let value = some_number.expect("Should have a number");
    // println!("Expected: {}", value);

    // Safe unwrapping with default
    // TODO: Use unwrap_or to provide a default
    // let result = absent_number.unwrap_or(0);
    // println!("With default: {}", result);

    // Common use case: finding in a collection
    let numbers = vec![10, 20, 30, 40, 50];
    
    // TODO: Find an element that exists
    // let found = numbers.iter().find(|&&x| x == 30);
    // println!("Found 30: {:?}", found);

    // TODO: Try to find an element that doesn't exist
    // let not_found = numbers.iter().find(|&&x| x == 100);
    // println!("Found 100: {:?}", not_found);

    // TODO: Handle the Option from find
    // match found {
    //     Some(&val) => println!("Found value: {}", val),
    //     None => println!("Value not found"),
    // }

    // Dividing safely with Option
    // TODO: Call divide function
    // println!("10 / 2 = {:?}", divide(10, 2));
    // println!("10 / 0 = {:?}", divide(10, 0));

    // Using if let for concise matching
    // TODO: Use if let to extract value
    // if let Some(n) = Some(42) {
    //     println!("The number is: {}", n);
    // }
}

// TODO: Implement a function that returns Option
// fn divide(numerator: f64, denominator: f64) -> Option<f64> {
//     if denominator == 0.0 {
//         None
//     } else {
//         Some(numerator / denominator)
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Create Some(5)
// [ ] Create None with type annotation Option<i32>
// [ ] Use is_some() and is_none()
// [ ] Use match to safely unwrap
// [ ] Use unwrap() (on a Some value)
// [ ] Use expect()
// [ ] Use unwrap_or() with default
// [ ] Use find() on a vector
// [ ] Implement divide() returning Option
// [ ] Use if let to extract value
// [ ] Verify the program compiles and runs
