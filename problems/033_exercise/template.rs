// Exercise 033: Slices - Array Slices
//
// Learning Objective: Learn about array slices and how they work similarly
// to string slices but for arrays and vectors.
//
// Topics covered:
// - Array slices (&[T])
// - Slices work with both arrays and vectors
// - Slice methods like len(), is_empty(), first(), last()

fn main() {
    // An array of integers
    let numbers = [10, 20, 30, 40, 50, 60, 70, 80];
    
    // TODO: Create a slice of the entire array
    // let all = &numbers[..];
    // println!("All: {:?}", all);

    // TODO: Create a slice of first 3 elements
    // let first_three = &numbers[___];
    // println!("First three: {:?}", first_three);

    // TODO: Create a slice of elements 2 through 5 (30, 40, 50, 60)
    // let middle = &numbers[___];
    // println!("Middle: {:?}", middle);

    // TODO: Create a slice from index 5 to the end
    // let last_three = &numbers[___];
    // println!("Last three: {:?}", last_three);

    // Slices work with vectors too!
    let vec = vec![100, 200, 300, 400, 500];
    
    // TODO: Create a slice of the vector (first 3 elements)
    // let vec_slice = &vec[___];
    // println!("Vector slice: {:?}", vec_slice);

    // Slice methods
    let data = [5, 10, 15, 20, 25];
    let slice = &data[..];
    
    // TODO: Print the length of the slice
    // println!("Length: {}", slice.len());
    
    // TODO: Check if slice is empty
    // println!("Is empty: {}", slice.is_empty());
    
    // TODO: Get the first element (returns Option<&T>)
    // if let Some(first) = slice.first() {
    //     println!("First: {}", first);
    // }
    
    // TODO: Get the last element
    // if let Some(last) = slice.last() {
    //     println!("Last: {}", last);
    // }

    // Mutable slices
    let mut values = [1, 2, 3, 4, 5];
    
    // TODO: Create a mutable slice and double each value
    // let mut_slice = &mut values[1..4];  // Elements 2, 3, 4
    // for val in mut_slice {
    //     *val *= 2;  // Dereference to modify
    // }
    // println!("Modified: {:?}", values);

    // Pass slices to functions
    let scores = [85, 92, 78, 95, 88];
    
    // TODO: Call calculate_average with a slice of scores
    // let avg = calculate_average(&scores);
    // println!("Average: {}", avg);
    
    // TODO: Call calculate_average with just the first 3 scores
    // let avg_first3 = calculate_average(&scores[___]);
    // println!("Average of first 3: {}", avg_first3);
}

// Calculates the average of values in a slice
fn calculate_average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// COMPLETION CHECKLIST:
// [ ] Create and print slice of entire array
// [ ] Create and print slice of first 3 elements
// [ ] Create and print slice of middle elements (2..6)
// [ ] Create and print slice from index 5 to end
// [ ] Create and print slice of vector
// [ ] Print slice length and is_empty
// [ ] Print first and last elements
// [ ] Create mutable slice and double values
// [ ] Call calculate_average with full array and with slice
