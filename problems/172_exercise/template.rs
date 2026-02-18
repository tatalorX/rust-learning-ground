// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 172: System Monitor
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    /proc filesystem, system calls, real-time data
//
// 🏢 REAL-WORLD SCENARIO:
//    Build a htop-like tool that monitors CPU and memory usage by reading /proc/stat and /proc/meminfo. Display a live updating dashboard showing process information, sorted by CPU usage.
//
// 🎯 YOUR TASK:
//    Implement the functionality described in the scenario.
//    Make sure to handle errors appropriately and write clean, idiomatic Rust code.
//
// 💡 CONCEPTS COVERED:
//    • /proc filesystem
//    • system calls
//    • real-time data
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    // 📝 TODO: Implement your solution here
    println!("Exercise 172: System Monitor");
    
    // Example usage (remove or modify as needed):
    // let result = your_function();
    // println!("Result: {:?}", result);
}

// 📝 TODO: Implement the required functions below

// HINT: Break the problem down into smaller functions.
// HINT: Consider using appropriate data structures from std::collections.

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_case() {
        // Add your test here
    }
    
    #[test]
    fn test_edge_cases() {
        // Test edge cases like empty input, errors, etc.
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 🤔 FOOD FOR THOUGHT:
//    • How would you extend this to handle larger inputs?
//    • What are the performance characteristics of your solution?
//    • How could you make this code more maintainable?
// ═══════════════════════════════════════════════════════════════════════════════
