// Exercise 021: Type Inference
// Learning objective: Learn that Rust can infer types automatically

fn main() {
    // Rust infers the type from the value
    // TODO: Declare a variable without type annotation
    // Rust will infer it's an i32
    let a = 42;
    
    // TODO: Declare a float variable without type annotation
    // Rust will infer it's an f64
    let pi = // ...
    
    // TODO: Declare a boolean without type annotation
    let is_active = // ...
    
    // TODO: Declare a string slice without type annotation
    let message = // ...
    
    // Print all values with their types shown in comments
    println!("a = {} (inferred as i32)", a);
    println!("pi = {} (inferred as f64)", pi);
    println!("is_active = {} (inferred as bool)", is_active);
    println!("message = {} (inferred as &str)", message);
    
    // Sometimes you need to help the compiler
    // TODO: Fix the type inference issue below
    // The compiler can't infer what numeric type you want
    // let guess: // Add a type annotation here, like 'i32' or 'f64'
    // guess = 42;
    // println!("guess = {}", guess);
    
    // Type inference works through operations
    let x = 5;
    let y = 10;
    let sum = x + y;  // Rust infers sum is also i32
    println!("{} + {} = {}", x, y, sum);
}
