// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 015: Ownership Basics
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    Understand Rust's ownership system - the feature that makes Rust memory-safe
//    without a garbage collector. This is THE most important concept in Rust!
//
// 💡 CONCEPTS COVERED:
//    • Ownership rules: One owner, move semantics
//    • What happens when ownership moves
//    • The stack vs the heap (briefly)
//    • Why ownership matters for memory safety
//
// 🎯 YOUR TASK:
//    Observe how ownership moves and fix the code to compile successfully.
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    // ═══════════════════════════════════════════════════════════════════════
    // PART 1: Understanding Ownership Transfer (Move Semantics)
    // ═══════════════════════════════════════════════════════════════════════
    
    // Create a String (owned by 'name1')
    // A String is allocated on the heap and has an owner
    let name1 = String::from("Rust");
    println!("name1 owns: {}", name1);
    
    // 📝 TODO: Transfer ownership to name2
    // 
    // When we assign name1 to name2, the ownership MOVES.
    // After this line, name1 is NO LONGER VALID!
    //
    // UNCOMMENT and FIX this line:
    // let name2 = name1;
    
    // This line will fail if name1 has been moved!
    // Can you understand why?
    println!("I still want to use: {}", name1);  // ← This needs fixing
    
    // This would work if we moved ownership:
    // println!("name2 now owns: {}", name2);
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 2: Functions and Ownership
    // ═══════════════════════════════════════════════════════════════════════
    
    let my_string = String::from("Hello");
    
    // When we pass my_string to this function, ownership MOVES into the function
    // After this call, my_string is no longer valid here!
    takes_ownership(my_string);
    
    // 📝 TODO: This line will fail - can you understand why?
    // Uncomment to see the error:
    // println!("my_string is: {}", my_string);
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 3: Copy Types (Primitives)
    // ═══════════════════════════════════════════════════════════════════════
    
    // Primitive types like i32, f64, bool, char implement the Copy trait
    // This means they are COPIED, not moved
    let x = 5;
    let y = x;  // x is COPIED to y, both are still valid!
    
    println!("x = {}, y = {}", x, y);  // This works fine!
    
    // ═══════════════════════════════════════════════════════════════════════
    // PART 4: Your Challenge
    // ═══════════════════════════════════════════════════════════════════════
    
    let greeting = String::from("Hello, Ownership!");
    
    // TODO: Create a new variable 'greeting2' that takes ownership from greeting
    // Then print greeting2 (not greeting!)
    
    // Your code here:
    // let greeting2 = ...
    // println!("{}", greeting2);
}

// This function takes ownership of a String
fn takes_ownership(s: String) {
    println!("Function received: {}", s);
    // When this function ends, 's' goes out of scope and memory is freed!
}

// ═══════════════════════════════════════════════════════════════════════════════
// 🤔 THE THREE OWNERSHIP RULES:
//
// 1. Each value in Rust has a variable that's its OWNER.
// 2. There can only be ONE owner at a time.
// 3. When the owner goes out of scope, the value is dropped (memory freed).
//
// ═══════════════════════════════════════════════════════════════════════════════
// 🎓 WHY DOES THIS MATTER?
//
// Without ownership (C/C++ style):
//   • Double-free bugs (free same memory twice)
//   • Use-after-free (access freed memory)
//   • Memory leaks (forget to free)
//
// With Rust's ownership:
//   ✅ Compile-time guarantees - no runtime cost!
//   ✅ No garbage collector needed
//   ✅ Memory safety without performance penalty
//
// ═══════════════════════════════════════════════════════════════════════════════
// 📖 SOLUTION APPROACH:
//
// For Part 1: Either:
//   A) Comment out the second println!(name1) after moving, OR
//   B) Use .clone() if you REALLY need both (costs extra memory):
//      let name2 = name1.clone();
//
// For Part 2: The line correctly fails - that's the point! Don't use moved values.
//
// For Part 4: greeting2 = greeting; then use greeting2 in println!
//
// ═══════════════════════════════════════════════════════════════════════════════
