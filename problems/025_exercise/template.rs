// Exercise 025: Array Iteration
// Learning objective: Learn to iterate over array elements

fn main() {
    let numbers = [10, 20, 30, 40, 50];
    
    // TODO: Iterate over the numbers array using a for loop
    // Print each number on its own line
    // for number in // ... {
    //     println!("{}", number);
    // }
    
    println!("---");
    
    let names = ["Alice", "Bob", "Charlie", "Diana"];
    
    // TODO: Iterate over names and print a greeting for each
    // Output: "Hello, Alice!", "Hello, Bob!", etc.
    // for name in // ... {
    //     println!("Hello, {}!", name);
    // }
    
    println!("---");
    
    let scores = [85, 92, 78, 95, 88, 91];
    
    // TODO: Calculate the sum of all scores using iteration
    let mut sum = 0;
    // for score in // ... {
    //     // TODO: Add score to sum
    // }
    println!("Sum of scores: {}", sum);
    
    // TODO: Calculate the average
    // Hint: Convert to f64 for floating point division
    let average = sum as f64 / scores.len() as f64;
    println!("Average score: {:.1}", average);
    
    println!("---");
    
    // TODO: Find the highest score
    // Start with the first element, then update if you find a higher score
    let mut highest = scores[0];
    // for &score in // ... {
    //     // TODO: Update highest if score > highest
    // }
    println!("Highest score: {}", highest);
}
