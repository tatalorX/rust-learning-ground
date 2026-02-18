// Exercise 045: Vectors - Creating and Adding
//
// Learning Objective: Learn how to create vectors and add elements to them.
// Vectors are growable arrays that store values of the same type on the heap.
//
// Topics covered:
// - Creating vectors with Vec::new() and vec![]
// - Pushing elements
// - Vector capacity and reallocation
// - Creating with initial capacity

fn main() {
    // Creating an empty vector
    // TODO: Create an empty vector of i32 using Vec::new()
    // let mut v1: Vec<i32> = Vec::new();
    // println!("Empty vector: {:?}, length: {}", v1, v1.len());

    // Using the vec! macro
    // TODO: Create a vector using vec![] macro
    // let v2 = vec![1, 2, 3];
    // println!("vec! created: {:?}", v2);

    // Adding elements with push
    // TODO: Push elements to v1
    // v1.push(10);
    // v1.push(20);
    // v1.push(30);
    // println!("After pushes: {:?}", v1);

    // Vector of Strings
    // TODO: Create a vector of Strings
    // let mut names: Vec<String> = Vec::new();
    // names.push(String::from("Alice"));
    // names.push(String::from("Bob"));
    // names.push(String::from("Charlie"));
    // println!("Names: {:?}", names);

    // Creating with initial capacity
    // TODO: Create a vector with capacity for 100 elements
    // let mut v3: Vec<i32> = Vec::with_capacity(100);
    // println!("Capacity: {}, Length: {}", v3.capacity(), v3.len());

    // TODO: Add 10 elements
    // for i in 0..10 {
    //     v3.push(i * 10);
    // }
    // println!("After adding: capacity={}, length={}", v3.capacity(), v3.len());

    // Vector of custom types
    // TODO: Define a Point struct
    // struct Point { x: i32, y: i32 }

    // TODO: Create a vector of Points
    // let mut points: Vec<Point> = Vec::new();
    // points.push(Point { x: 0, y: 0 });
    // points.push(Point { x: 10, y: 20 });
    // points.push(Point { x: -5, y: 15 });
    // println!("Points: {:?}", points);

    // Extending a vector
    // TODO: Create a vector and extend it
    // let mut v4 = vec![1, 2, 3];
    // let more = vec![4, 5, 6];
    // v4.extend(more);
    // println!("Extended: {:?}", v4);

    // Inserting at a specific position
    // TODO: Insert an element at index 1
    // let mut v5 = vec![1, 3, 4];
    // v5.insert(1, 2);  // Insert 2 at position 1
    // println!("After insert: {:?}", v5);

    // Creating from an array
    // TODO: Create a vector from an array
    // let arr = [10, 20, 30];
    // let v6: Vec<i32> = arr.to_vec();
    // println!("From array: {:?}", v6);
}

// TODO: Define Point struct
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// COMPLETION CHECKLIST:
// [ ] Create empty vector with Vec::new()
// [ ] Create vector with vec![] macro
// [ ] Push elements to a vector
// [ ] Create vector of Strings
// [ ] Create vector with with_capacity()
// [ ] Observe capacity vs length
// [ ] Define Point struct
// [ ] Create vector of Points
// [ ] Extend a vector with another vector
// [ ] Insert at specific position
// [ ] Create vector from array using to_vec()
// [ ] Verify the program compiles and runs
