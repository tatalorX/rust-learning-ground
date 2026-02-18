// Exercise 024: Array Access
// Learning objective: Learn to access array elements by index

fn main() {
    let fruits = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    
    // TODO: Print the first element (index 0)
    // println!("First fruit: {}", fruits[// ...
    
    // TODO: Print the third element (index 2)
    // println!("Third fruit: {}", // ...
    
    // TODO: Print the last element using fruits.len() - 1
    // let last_index = // ...
    // println!("Last fruit: {}", fruits[last_index]);
    
    let scores = [85, 92, 78, 95, 88];
    
    // TODO: Calculate the sum of the first and last scores
    // let sum = // scores[0] + scores[// ...
    // println!("First + Last score: {}", sum);
    
    // TODO: Calculate the average of scores at indices 1 and 3
    // let average = (scores[1] + scores[3]) / 2;
    // println!("Average of scores[1] and scores[3]: {}", average);
    
    // Accessing array elements in a const context
    const PRIMES: [i32; 5] = [2, 3, 5, 7, 11];
    // TODO: Print the 4th prime number (index 3)
    // println!("The 4th prime number is: {}", PRIMES[// ...
}
