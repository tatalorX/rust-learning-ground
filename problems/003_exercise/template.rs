// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 003: Data Types
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    Learn about Rust's scalar data types: integers, floating-point numbers,
//    booleans, and characters. Practice explicit type annotations.
//
// 💡 CONCEPTS COVERED:
//    • Scalar types: integers (i32, u32, i64, etc.), floats (f32, f64)
//    • Boolean type (bool)
//    • Character type (char) - 4 bytes, Unicode!
//    • Type annotations with colon syntax: let x: i32 = 5;
//
// 🎯 YOUR TASK:
//    Fix the type errors by adding correct type annotations.
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    println!("🧪 Rust Type System Explorer\n");
    
    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 1: Integer Types
    // ═══════════════════════════════════════════════════════════════════════
    
    // Integer types in Rust: i8, i16, i32, i64, i128, isize (signed)
    //                        u8, u16, u32, u64, u128, usize (unsigned)
    // i = can be negative, u = only positive (and zero)
    // The number is the number of bits (size)
    //
    // 📝 TODO: Add type annotation :i32
    // Currently Rust can't infer what type of integer this should be
    
    let age = 25;  // ← Add :i32 after 'age'
    println!("Age: {} (type: i32)", age);
    
    // 📝 TODO: This needs to be unsigned (can't be negative)
    // Use u32 for the number of students
    
    let student_count = 150;  // ← Change to :u32
    println!("Students: {} (type: u32)", student_count);
    
    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 2: Floating-Point Types
    // ═══════════════════════════════════════════════════════════════════════
    
    // Rust has f32 and f64. f64 is the default (more precision)
    // Floats are for numbers with decimal points
    
    // 📝 TODO: Add :f64 type annotation
    let pi = 3.14159;
    println!("Pi: {:.5} (type: f64)", pi);
    
    // 📝 TODO: This temperature needs to allow decimal values
    // Change the type to f64
    
    let temperature = 72.5;  // ← Make this f64
    println!("Temperature: {}°F", temperature);
    
    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 3: Boolean Type
    // ═══════════════════════════════════════════════════════════════════════
    
    // Booleans: true or false
    // Used for conditions and flags
    
    // 📝 TODO: Add :bool annotation
    let is_rust_fun = true;
    println!("Is Rust fun? {}", is_rust_fun);
    
    // 📝 TODO: Create a boolean for 'is_logged_in' set to false
    // let is_logged_in: bool = false;
    
    let is_logged_in = false;  // ← Add :bool
    println!("Logged in: {}", is_logged_in);
    
    // ═══════════════════════════════════════════════════════════════════════
    // SECTION 4: Character Type
    // ═══════════════════════════════════════════════════════════════════════
    // 
    // Rust's char is 4 bytes and represents a Unicode Scalar Value
    // This means it can represent more than just ASCII:
    //   - Letters: 'a', 'Z', 'ñ', 'ф', '山'
    //   - Emojis: '🔥', '🦀', '🎉'
    //   - Accented characters: 'é', 'ü'
    // Use SINGLE quotes for chars: 'a'
    // Use DOUBLE quotes for strings: "hello"
    
    // 📝 TODO: Add :char annotation
    let initial = 'R';
    println!("Initial: {}", initial);
    
    // 📝 TODO: Create a char variable with your favorite emoji
    // Try: let emoji = '🦀';
    
    let emoji = '🚀';  // ← Add :char
    println!("Favorite emoji: {}", emoji);
    
    // ═══════════════════════════════════════════════════════════════════════
    // BONUS: Type Inference
    // ═══════════════════════════════════════════════════════════════════════
    //
    // Rust can often infer types, but being explicit helps readability
    // and is sometimes required (like when parsing or with generics)
    
    // This works - Rust infers the type from the literal
    let inferred = 42;  // Rust knows this is i32
    println!("\n✨ Bonus: Type inference works too! inferred = {}", inferred);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 📖 TYPE REFERENCE GUIDE:
//
// INTEGERS:
//   Signed:   i8, i16, i32, i64, i128, isize
//   Unsigned: u8, u16, u32, u64, u128, usize
//   Default integer type is i32 (fast even on 64-bit systems)
//
// FLOATING-POINT:
//   f32 - Single precision (about 6 decimal digits)
//   f64 - Double precision (about 15 decimal digits, DEFAULT)
//
// BOOLEAN:
//   bool - true or false
//   Size: 1 byte
//
// CHARACTER:
//   char - Single Unicode scalar value
//   Size: 4 bytes (can represent any Unicode character!)
//   Syntax: SINGLE quotes 'a', NOT double quotes "a"
//
// ═══════════════════════════════════════════════════════════════════════════════
