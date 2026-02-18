// Exercise 023: Simple Array Declaration
// Learning objective: Learn to declare arrays in Rust

fn main() {
    // TODO: Declare an array of 5 i32 integers
    // Array syntax: [type; size] for uninitialized or [value1, value2, ...] for initialized
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // TODO: Declare an array of temperatures (f64) with values [72.5, 68.0, 75.5, 80.0, 65.5]
    let temperatures = // ...
    
    // TODO: Declare an array of weekdays (&str)
    // Values: "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"
    let weekdays = // ...
    
    // TODO: Declare an array of 10 zeros using the shorthand syntax
    // Hint: [0; 10] creates an array with ten zeros
    let zeros = // ...
    
    // Print array information
    println!("Numbers: {:?}", numbers);
    println!("Temperatures: {:?}", temperatures);
    println!("Weekdays: {:?}", weekdays);
    println!("Zeros: {:?}", zeros);
    
    // Print the length of the numbers array
    // TODO: Use numbers.len() to get the length
    println!("Numbers array has {} elements", numbers.len());
}
