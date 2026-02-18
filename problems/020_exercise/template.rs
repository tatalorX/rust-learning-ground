// Exercise 020: Shadowing
// Learning objective: Understand variable shadowing in Rust

fn main() {
    let x = 5;
    println!("Original x: {}", x);
    
    // TODO: Shadow x with a new value of 10
    // This creates a new variable 'x' that shadows the old one
    // let x = // ...
    // println!("Shadowed x: {}", x);
    
    // TODO: Shadow x again, this time with a calculation
    // let x = x + 5;  // This uses the previous shadowed value
    // println!("Shadowed x again: {}", x);
    
    // Shadowing also allows changing the type
    let name = "Alice";  // This is &str
    println!("Original name (type &str): {}", name);
    
    // TODO: Shadow name with a String type
    // Use String::from() to create a String
    // let name = // ...
    // println!("Shadowed name (type String): {}", name);
    
    // TODO: Demonstrate the difference between shadowing and mutability
    // First, try to mutate a variable (this works)
    let mut y = 10;
    y = 20;  // This works because y is mutable
    println!("Mutable y: {}", y);
    
    // Shadowing allows transforming values while keeping immutability
    let z = "10";
    println!("z as string: {}", z);
    // TODO: Shadow z, parsing it as an i32
    // Hint: use .parse::<i32>().unwrap() on the string
    // let z = z.parse::<i32>().unwrap();
    // println!("z as number: {}", z);
}
