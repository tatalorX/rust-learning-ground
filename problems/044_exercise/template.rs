// Exercise 044: Pattern Matching - while let
//
// Learning Objective: Learn to use while let for looping while a
// pattern continues to match. Useful for iterators and pop operations.
//
// Topics covered:
// - while let for conditional looping
// - Using with pop() on vectors
// - Using with iterators
// - Draining collections

fn main() {
    // while let repeatedly matches a pattern until it fails
    // Syntax: while let PATTERN = EXPRESSION { ... }

    // Example: Popping from a stack until empty
    // TODO: Create a vector
    // let mut stack = vec![1, 2, 3, 4, 5];

    // TODO: Use while let to pop until empty
    // while let Some(top) = stack.pop() {
    //     println!("Popped: {}", top);
    // }
    // println!("Stack is empty: {:?}", stack);

    // Example with mutable iterator
    // TODO: Create a queue and drain it
    // let mut queue = vec!["first", "second", "third"];
    // while let Some(item) = queue.pop() {
    //     println!("Processing: {}", item);
    // }

    // Example with iter_mut (modifying while iterating)
    // TODO: Create a vector of Options
    // let mut values = vec![Some(1), Some(2), Some(3)];
    
    // TODO: Use while let with iterator
    // let mut iter = values.iter_mut();
    // while let Some(opt) = iter.next() {
    //     if let Some(val) = opt {
    //         *val *= 2;  // Double each value
    //     }
    // }
    // println!("Doubled: {:?}", values);

    // Example with peekable iterator
    // TODO: Process until we see a specific value
    // let nums = vec![1, 2, 3, 99, 4, 5];
    // let mut iter = nums.into_iter().peekable();
    // while let Some(&n) = iter.peek() {
    //     if n == 99 {
    //         println!("Found stop value!");
    //         break;
    //     }
    //     println!("Processing: {}", iter.next().unwrap());
    // }

    // Example: reading lines until empty (simulated)
    // TODO: Simulate reading lines
    // let mut lines = vec!["Line 1", "Line 2", "", "Line 4"].into_iter();
    // while let Some(line) = lines.next() {
    //     if line.is_empty() {
    //         println!("Empty line - stopping");
    //         break;
    //     }
    //     println!("Read: {}", line);
    // }

    // Comparing approaches:
    // 1. loop + match
    let mut v1 = vec![1, 2, 3];
    // loop {
    //     match v1.pop() {
    //         Some(n) => println!("loop+match: {}", n),
    //         None => break,
    //     }
    // }

    // 2. while let (cleaner!)
    // let mut v2 = vec![1, 2, 3];
    // while let Some(n) = v2.pop() {
    //     println!("while let: {}", n);
    // }

    // TODO: Implement both approaches above and compare
}

// COMPLETION CHECKLIST:
// [ ] Create stack vector
// [ ] Use while let Some(top) = stack.pop()
// [ ] Process until empty
// [ ] Create queue and drain with while let
// [ ] Use while let with iter_mut
// [ ] Double values in-place
// [ ] Use peekable iterator with while let
// [ ] Simulate reading lines with while let
// [ ] Compare loop+match vs while let
// [ ] Verify the program compiles and runs
