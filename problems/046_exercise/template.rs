// Exercise 046: Vectors - Accessing Elements
//
// Learning Objective: Learn the different ways to access elements in a vector,
// including indexing and safe access methods.
//
// Topics covered:
// - Indexing with []
// - get() method (returns Option)
// - first() and last() methods
// - Handling out-of-bounds access

fn main() {
    // TODO: Create a vector of integers
    // let numbers = vec![10, 20, 30, 40, 50];

    // Indexing with [] (panics if out of bounds!)
    // TODO: Access element at index 0
    // let first = numbers[0];
    // println!("First: {}", first);

    // TODO: Access element at index 2
    // let third = numbers[2];
    // println!("Third: {}", third);

    // Safe access with get() (returns Option<&T>)
    // TODO: Get element at index 0
    // match numbers.get(0) {
    //     Some(n) => println!("Got: {}", n),
    //     None => println!("Not found"),
    // }

    // TODO: Try to get element at index 100 (out of bounds)
    // match numbers.get(100) {
    //     Some(n) => println!("Got: {}", n),
    //     None => println!("Index 100 is out of bounds"),
    // }

    // first() and last() methods
    // TODO: Get first element
    // if let Some(n) = numbers.first() {
    //     println!("First: {}", n);
    // }

    // TODO: Get last element
    // if let Some(n) = numbers.last() {
    //     println!("Last: {}", n);
    // }

    // Mutably accessing elements
    // TODO: Create a mutable vector
    // let mut scores = vec![85, 92, 78, 95];

    // TODO: Modify an element using indexing
    // scores[2] = 88;
    // println!("Updated scores: {:?}", scores);

    // TODO: Modify using get_mut()
    // if let Some(score) = scores.get_mut(0) {
    //     *score += 5;  // Add 5 bonus points
    // }
    // println!("After bonus: {:?}", scores);

    // Working with empty vectors
    // TODO: Check first() on empty vector
    // let empty: Vec<i32> = vec![];
    // match empty.first() {
    //     Some(_) => println!("Has elements"),
    //     None => println!("Empty vector!"),
    // }

    // Practical: safe division using get
    // TODO: Write a function that safely divides elements
    // fn safe_divide(v: &Vec<f64>, idx1: usize, idx2: usize) -> Option<f64> {
    //     let a = v.get(idx1)?;
    //     let b = v.get(idx2)?;
    //     if *b == 0.0 {
    //         None
    //     } else {
    //         Some(*a / *b)
    //     }
    // }

    // TODO: Test the function
    // let values = vec![10.0, 2.0, 0.0];
    // println!("10/2 = {:?}", safe_divide(&values, 0, 1));
    // println!("10/0 = {:?}", safe_divide(&values, 0, 2));
    // println!("out of bounds = {:?}", safe_divide(&values, 0, 10));
}

// TODO: Implement safe_divide function
// fn safe_divide(v: &Vec<f64>, idx1: usize, idx2: usize) -> Option<f64> {
//     let a = v.get(idx1)?;
//     let b = v.get(idx2)?;
//     if *b == 0.0 {
//         None
//     } else {
//         Some(*a / *b)
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Create vector of integers
// [ ] Access elements using indexing []
// [ ] Access using get() with match
// [ ] Handle out-of-bounds with get()
// [ ] Use first() and last() methods
// [ ] Create mutable vector
// [ ] Modify elements using indexing
// [ ] Modify using get_mut()
// [ ] Check first() on empty vector
// [ ] Implement and test safe_divide function
// [ ] Verify the program compiles and runs
