// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 034: Match Expressions
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    Master pattern matching with match expressions - Rust's powerful control
//    flow construct that ensures exhaustiveness (all cases covered).
//
// 💡 CONCEPTS COVERED:
//    • match expression syntax
//    • Patterns and arms
//    • Exhaustiveness checking
//    • Wildcard pattern (_)
//    • Match as expression (returns value)
//
// 🎯 YOUR TASK:
//    Complete the match expressions to handle all cases.
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    // ═══════════════════════════════════════════════════════════════════════
    // PART 1: Basic Match with Numbers
    // ═══════════════════════════════════════════════════════════════════════
    
    let number = 3;
    
    // 📝 TODO: Complete this match expression
    // Add arms for 2, 3, and a default case
    match number {
        1 => println!("One!"),
        // Add: 2 => ...
        // Add: 3 => ...
        // Add: _ => ... (default/wildcard)
        _ => println!("Something else"),
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 2: Match as Expression (returns a value!)
    // ═══════════════════════════════════════════════════════════════════════
    
    let score = 85;
    
    // 📝 TODO: Complete this match to return a letter grade
    let grade = match score {
        90..=100 => "A",  // Range pattern: 90 to 100 inclusive
        80..=89 => "B",   // Range pattern: 80 to 89
        // Add: 70..=79 => ...
        // Add: 60..=69 => ...
        // Add: default case for F
        _ => "F",
    };
    
    println!("Score {} = Grade {}", score, grade);
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 3: Matching on Enums
    // ═══════════════════════════════════════════════════════════════════════
    
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let heading = Direction::North;
    
    // 📝 TODO: Complete the match for all directions
    match heading {
        Direction::North => println!("Heading North!"),
        // Add: Direction::South => ...
        // Add: Direction::East => ...
        // Add: Direction::West => ...
        _ => println!("Unknown direction"),
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 4: Your Challenge - Coin Sorter
    // ═══════════════════════════════════════════════════════════════════════
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String),  // String indicates the state design
    }
    
    // 📝 TODO: Implement this function to return the value in cents
    // Hint: Quarter has a String value you can access
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            // Coin::Nickel => ...
            // Coin::Dime => ...
            // Coin::Quarter(state) => {
            //     println!("Quarter from {:?}!", state);
            //     25
            // }
            _ => 0,
        }
    }
    
    let my_coin = Coin::Quarter(String::from("Alaska"));
    println!("Coin value: {} cents", value_in_cents(my_coin));
}

// ═══════════════════════════════════════════════════════════════════════════════
// 🤔 WHY IS MATCH SO POWERFUL?
//
// 1. EXHAUSTIVENESS: Rust checks that you've handled ALL cases at compile time
//    - No runtime surprises!
//    - If you add a variant to an enum, Rust tells you where to update
//
// 2. PATTERN MATCHING: Can destructure complex data
//    - Tuples: match point { (0, 0) => ..., (x, 0) => ..., ... }
//    - Structs: match person { Person { name: "Alice", age } => ... }
//    - Enums with data: Some(x) => ..., None => ...
//
// 3. EXPRESSION, NOT STATEMENT: match returns a value
//    let result = match option {
//        Some(v) => v * 2,
//        None => 0,
//    };
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📖 COMMON PATTERNS:
//
// // Match with ranges
// match age {
//     0..=12 => "child",
//     13..=19 => "teenager",
//     20..=64 => "adult",
//     _ => "senior",
// }
//
// // Match with guards
// match num {
//     n if n < 0 => "negative",
//     n if n > 0 => "positive",
//     _ => "zero",
// }
//
// // Multiple patterns
// match letter {
//     'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
//     _ => "consonant",
// }
//
// ═══════════════════════════════════════════════════════════════════════════════
