// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 002: Variables and Mutability
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    Understand that variables in Rust are immutable by default, and learn
//    how to make them mutable using the 'mut' keyword.
//
// 💡 CONCEPTS COVERED:
//    • Immutability by default - Rust's safety feature
//    • The 'mut' keyword - Making variables changeable
//    • Variable shadowing (mentioned as advanced tip)
//    • The compiler as a helpful teacher
//
// 🎯 YOUR TASK:
//    1. Make the 'score' variable mutable by adding 'mut' after 'let'
//    2. Uncomment the line that changes score to 20
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    // 📝 EXERCISE PART 1: Make this variable mutable
    // 
    // Currently, this line creates an IMMUTABLE variable.
    // In Rust, once you bind a value to a name, you can't change it unless
    // you explicitly say it's mutable.
    //
    // CHANGE THIS LINE:
    //   Add 'mut' between 'let' and 'score'
    //   let mut score = 10;
    //
    // This tells Rust: "I want to change this variable later"
    
    let score = 10;  // ← Make this mutable!
    
    // 📝 EXERCISE PART 2: Try changing the score
    //
    // Once you've made 'score' mutable, uncomment the line below:
    
    // score = 20;  // ← Uncomment this line (remove the // at the start)
    
    // 💭 WHY COMMENT IT OUT?
    // If you try to change an immutable variable, Rust will give you a
    // helpful compiler error. We're starting with it commented so you can
    // see the error, then fix it by adding 'mut' above!
    
    // This prints the final value of score
    println!("The score is: {}", score);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 🤔 DEEPER UNDERSTANDING:
//
// Q: Why does Rust make variables immutable by default?
// A: This prevents bugs! When you know a value won't change, you can reason
//    about your code more easily. The compiler guarantees no accidental changes.
//    It's one of Rust's "pit of success" features - the default is the safest.
//
// Q: What error do I get without 'mut'?
// A: Something like: "cannot assign twice to immutable variable `score`"
//    Rust's error messages are famously helpful - read them carefully!
//
// 🎓 ADVANCED CONCEPT (Optional):
// Variable Shadowing - You can also do this (different from mutation!):
//   let score = 10;
//   let score = score + 5;  // This creates a NEW variable, shadows the old one
// This is useful when transforming values between types.
//
// ═══════════════════════════════════════════════════════════════════════════════
