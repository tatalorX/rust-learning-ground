// Exercise 019: Comments and Documentation
// Learning objective: Learn about different types of comments in Rust

// This is a single-line comment

/*
 * This is a 
 * multi-line comment
 */

/// This is a documentation comment (doc comment)
/// Use three slashes to document the item that follows
/// This function calculates the area of a rectangle
/// 
/// # Examples
/// ```
/// let area = calculate_area(5, 10);
/// ```
fn calculate_area(width: i32, height: i32) -> i32 {
    // TODO: Return width * height
    // Add an inline comment explaining this calculation
    0 // placeholder - replace with actual calculation
}

/// TODO: Add doc comments for this function
/// It should describe what the function does
fn greet_user(name: &str) -> String {
    // TODO: Return a formatted greeting string
    // Example: "Hello, Alice! Welcome!"
    format!("Hello, {}!", name)
}

fn main() {
    // TODO: Add a comment explaining what this line does
    let area = calculate_area(5, 10);
    println!("Area: {}", area);
    
    // TODO: Call greet_user and print the result
    // let greeting = greet_user("Bob");
    // println!("{}", greeting);
}
