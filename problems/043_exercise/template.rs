// Exercise 043: Pattern Matching - if let
//
// Learning Objective: Learn to use if let for concise matching when
// you only care about one pattern. A shorter alternative to match.
//
// Topics covered:
// - if let for single pattern matching
// - else with if let
// - Combining if let with else if
// - When to use if let vs match

fn main() {
    // if let is syntactic sugar for match with one pattern
    // Instead of:
    //   match some_option {
    //       Some(value) => println!("{}", value),
    //       _ => (),
    //   }
    // You can write:
    //   if let Some(value) = some_option {
    //       println!("{}", value);
    //   }

    // TODO: Create a Some value
    // let some_value = Some(5);

    // TODO: Use if let to extract the value
    // if let Some(n) = some_value {
    //     println!("Got: {}", n);
    // }

    // TODO: Create a None value
    // let no_value: Option<i32> = None;

    // TODO: Use if let with else
    // if let Some(n) = no_value {
    //     println!("Got: {}", n);
    // } else {
    //     println!("No value");
    // }

    // if let with enums
    // TODO: Define an enum with variants
    // enum Message {
    //     Hello { id: i32 },
    //     Goodbye,
    // }

    // TODO: Create a Message::Hello
    // let msg = Message::Hello { id: 5 };

    // TODO: Use if let to match specific pattern
    // if let Message::Hello { id } = msg {
    //     println!("Hello with id: {}", id);
    // }

    // Combining with else if
    // let msg = Message::Goodbye;
    // TODO: Use if let with else if
    // if let Message::Hello { id: 3 } = msg {
    //     println!("Hello from 3");
    // } else if let Message::Hello { id } = msg {
    //     println!("Hello from {}", id);
    // } else {
    //     println!("Goodbye!");
    // }

    // Practical example: processing a vector of Options
    let values = vec![Some(1), None, Some(3), None, Some(5)];
    
    // TODO: Iterate and use if let to process only Some values
    // for val in &values {
    //     if let Some(n) = val {
    //         println!("Processing: {}", n);
    //     }
    // }

    // Counting matches
    // TODO: Count how many Some values exist
    // let mut count = 0;
    // for val in &values {
    //     if let Some(_) = val {
    //         count += 1;
    //     }
    // }
    // println!("Found {} Some values out of {}", count, values.len());

    // if let with destructuring
    // TODO: Create a tuple and destructure with if let
    // let pair = (1, 2);
    // if let (1, y) = pair {
    //     println!("First is 1, second is {}", y);
    // }
}

// TODO: Define Message enum
// enum Message {
//     Hello { id: i32 },
//     Goodbye,
// }

// COMPLETION CHECKLIST:
// [ ] Create Some value and use if let
// [ ] Create None value and use if let with else
// [ ] Define Message enum
// [ ] Use if let with Message::Hello pattern
// [ ] Use if let with else if
// [ ] Iterate vector of Options with if let
// [ ] Count Some values using if let
// [ ] Use if let with tuple destructuring
// [ ] Verify the program compiles and runs
