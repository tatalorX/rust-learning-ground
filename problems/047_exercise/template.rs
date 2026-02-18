// Exercise 047: Vectors - Iterating
//
// Learning Objective: Learn how to iterate over vectors using different
// methods and patterns.
//
// Topics covered:
// - for loop iteration
// - iter(), iter_mut(), into_iter()
// - enumerate(), map(), filter()
// - Iterator methods: sum(), collect(), count()

fn main() {
    // TODO: Create a vector
    // let numbers = vec![1, 2, 3, 4, 5];

    // Immutable iteration with iter()
    // TODO: Iterate and print each number
    // for num in numbers.iter() {
    //     println!("Value: {}", num);
    // }

    // for loop directly (uses iter() implicitly for collections)
    // TODO: Iterate without explicit iter()
    // for num in &numbers {
    //     println!("Got: {}", num);
    // }

    // Mutable iteration with iter_mut()
    // TODO: Create mutable vector and double each element
    // let mut scores = vec![10, 20, 30, 40];
    // for score in scores.iter_mut() {
    //     *score *= 2;
    // }
    // println!("Doubled: {:?}", scores);

    // Enumerate to get index and value
    // TODO: Iterate with index
    // let names = vec!["Alice", "Bob", "Charlie"];
    // for (index, name) in names.iter().enumerate() {
    //     println!("{}: {}", index, name);
    // }

    // Using map() to transform
    // TODO: Square each number using map
    // let squares: Vec<i32> = numbers.iter().map(|n| n * n).collect();
    // println!("Squares: {:?}", squares);

    // Using filter() to select elements
    // TODO: Keep only even numbers
    // let evens: Vec<i32> = numbers.iter().filter(|&&n| n % 2 == 0).copied().collect();
    // println!("Evens: {:?}", evens);

    // Chaining methods
    // TODO: Get squares of even numbers
    // let even_squares: Vec<i32> = numbers
    //     .iter()
    //     .filter(|&&n| n % 2 == 0)
    //     .map(|n| n * n)
    //     .collect();
    // println!("Even squares: {:?}", even_squares);

    // Sum all elements
    // TODO: Calculate total
    // let total: i32 = numbers.iter().sum();
    // println!("Total: {}", total);

    // Count elements
    // TODO: Count numbers greater than 2
    // let count = numbers.iter().filter(|&&n| n > 2).count();
    // println!("Count > 2: {}", count);

    // Find an element
    // TODO: Find first even number
    // if let Some(&first_even) = numbers.iter().find(|&&n| n % 2 == 0) {
    //     println!("First even: {}", first_even);
    // }

    // Position of element
    // TODO: Find position of value 3
    // if let Some(pos) = numbers.iter().position(|&n| n == 3) {
    //     println!("3 is at index {}", pos);
    // }

    // Any and All
    // TODO: Check if any number is greater than 4
    // let any_big = numbers.iter().any(|&n| n > 4);
    // println!("Any > 4: {}", any_big);

    // TODO: Check if all numbers are positive
    // let all_positive = numbers.iter().all(|&n| n > 0);
    // println!("All positive: {}", all_positive);

    // into_iter() consumes the vector
    // TODO: Use into_iter() (takes ownership)
    // let owned = vec!["a", "b", "c"];
    // for item in owned.into_iter() {
    //     println!("Owned: {}", item);
    // }
    // owned is no longer valid here!
}

// COMPLETION CHECKLIST:
// [ ] Create vector
// [ ] Iterate with iter()
// [ ] Iterate with & reference
// [ ] Use iter_mut() to modify elements
// [ ] Use enumerate() for index and value
// [ ] Use map() to transform elements
// [ ] Use filter() to select elements
// [ ] Chain map and filter
// [ ] Use sum() to calculate total
// [ ] Use count() to count matches
// [ ] Use find() and position()
// [ ] Use any() and all()
// [ ] Use into_iter() (consuming)
// [ ] Verify the program compiles and runs
