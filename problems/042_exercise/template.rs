// Exercise 042: Pattern Matching - match
//
// Learning Objective: Learn to use match expressions for exhaustive
// pattern matching - one of Rust's most powerful features.
//
// Topics covered:
// - Basic match expressions
// - Match arms and patterns
// - Exhaustive matching
// - Binding values in patterns
// - Wildcard pattern (_)

fn main() {
    // TODO: Define a Coin enum: Penny, Nickel, Dime, Quarter
    // (Define below main)

    // TODO: Create a Coin value
    // let coin = Coin::Dime;

    // TODO: Use match to get the value in cents
    // let value = match coin {
    //     Coin::Penny => 1,
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter => 25,
    // };
    // println!("Coin value: {} cents", value);

    // Match with actions
    // TODO: Use match with println in an arm
    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky penny!");
    //         1
    //     }
    //     _ => value,  // Wildcard for other cases
    // };

    // Matching with data
    // TODO: Define Message enum with data variants
    // let msg = Message::Write(String::from("Hello"));

    // TODO: Match and extract data
    // match msg {
    //     Message::Quit => println!("Quit"),
    //     Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    //     Message::Write(text) => println!("Text: {}", text),
    //     Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    // }

    // Matching on integers
    // TODO: Match on a number and categorize it
    // let number = 7;
    // match number {
    //     1 => println!("One"),
    //     2 | 3 | 5 | 7 | 11 => println!("Prime"),
    //     13..=19 => println!("Teen"),
    //     _ => println!("Other"),
    // }

    // Binding to values
    // TODO: Match and bind to a variable
    // let msg = Message::Move { x: 10, y: 20 };
    // match msg {
    //     Message::Move { x, y } => {
    //         println!("Moving x by {}", x);
    //         println!("Moving y by {}", y);
    //     }
    //     _ => {}
    // }

    // Matching on Option<T>
    // TODO: Create an Option and match on it
    // let some_value = Some(3);
    // match some_value {
    //     Some(3) => println!("Three!"),
    //     Some(n) => println!("Other number: {}", n),
    //     None => println!("No value"),
    // }

    // match is an expression - returns a value
    // TODO: Use match in a let statement
    // let x = 5;
    // let description = match x {
    //     1 => "one",
    //     2 => "two",
    //     3..=9 => "a few",
    //     _ => "many",
    // };
    // println!("{} is {}", x, description);
}

// TODO: Define Coin enum
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// TODO: Define Message enum
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// COMPLETION CHECKLIST:
// [ ] Define Coin enum
// [ ] Use match to get coin value
// [ ] Use match with actions (println in arm)
// [ ] Use wildcard pattern (_) 
// [ ] Define Message enum with data
// [ ] Match Message and extract data
// [ ] Match on integers with ranges (..=)
// [ ] Match on Option<T>
// [ ] Use match as expression (returns value)
// [ ] Verify the program compiles and runs
