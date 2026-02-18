// Exercise 005: Constants
// Learning objective: Learn to declare constants and understand their immutability

// TODO: Declare a constant named MAX_SCORE of type i32 with value 100
// Hint: Use the 'const' keyword and constants are SCREAMING_SNAKE_CASE by convention
// Constants must also have an explicit type annotation

// TODO: Declare a constant named PI of type f64 with value 3.14159

// TODO: Declare a constant WELCOME_MESSAGE of type &str with value "Welcome to Rust!"

fn main() {
    println!("Maximum score: {}", MAX_SCORE);
    println!("PI value: {}", PI);
    println!("Message: {}", WELCOME_MESSAGE);
    
    // TODO: Calculate what percentage 75 is of MAX_SCORE and print it
    // Hint: percentage = (75.0 / MAX_SCORE as f64) * 100.0
    let percentage = // ...
    println!("75 is {:.1}% of {}", percentage, MAX_SCORE);
}
