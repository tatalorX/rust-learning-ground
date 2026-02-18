import os

exercises = [
    (152, "Custom Smart Pointer", "Implement a custom smart pointer similar to Box<T> with Deref and Drop traits", "smart_pointers", 3),
    (153, "CSV Parser", "Parse CSV data with proper error handling for malformed inputs", "file_io", 3),
    (154, "Rate Limiter", "Implement a token bucket rate limiter using time windows", "advanced", 4),
    (155, "JSON Serializer", "Create a simple JSON serializer for custom structs", "advanced", 3),
    (156, "Thread Pool", "Build a fixed-size thread pool for executing tasks concurrently", "advanced", 4),
    (157, "LRU Cache", "Implement an LRU (Least Recently Used) cache with O(1) operations", "advanced", 4),
    (158, "Bloom Filter", "Create a Bloom filter for probabilistic membership testing", "advanced", 5),
    (159, "Web Server", "Build a basic HTTP server that handles GET and POST requests", "advanced", 4),
    (160, "Database Connection Pool", "Implement a connection pool for database connections", "advanced", 4),
    (161, "Memoization", "Add memoization to recursive functions for performance", "advanced", 3),
    (162, "State Machine", "Model a state machine for a TCP connection using enums", "enums", 3),
    (163, "Builder Pattern", "Implement the builder pattern for constructing complex objects", "structs", 3),
    (164, "Observer Pattern", "Create an observer pattern implementation with trait objects", "traits", 3),
    (165, "Dependency Injection", "Build a simple dependency injection container", "advanced", 4),
    (166, "Pub/Sub System", "Implement a publish-subscribe messaging system with channels", "advanced", 4),
    (167, "Circuit Breaker", "Create a circuit breaker pattern for resilient service calls", "advanced", 4),
    (168, "Retry Logic", "Implement exponential backoff retry logic with jitter", "advanced", 3),
    (169, "Type Safe SQL", "Build a type-safe SQL query builder using generics", "generics", 5),
    (170, "Actor System", "Implement a basic actor system for concurrent message passing", "advanced", 5),
]

template = '''// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE {id}: {title}
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    {objective}
//
// 🏢 REAL-WORLD SCENARIO:
//    {scenario}
//
// 🎯 YOUR TASK:
//    Complete the implementation below following the requirements.
//
// 💡 HINTS:
//    • Think about edge cases
//    • Use appropriate Rust patterns
//    • Test your implementation thoroughly
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {{
    println!("Exercise {id}: {title}");
    // TODO: Add test cases here
}}

// TODO: Implement your solution below

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_basic_functionality() {{
        // TODO: Add your test
    }}
    
    #[test]
    fn test_edge_cases() {{
        // TODO: Test edge cases
    }}
}}
'''

scenarios = {
    152: "You're building a custom memory allocator and need a smart pointer that tracks allocations",
    153: "Parse user-uploaded CSV files for data import, handling various formatting errors gracefully",
    154: "Protect your API from abuse by limiting requests to 100 per minute per user",
    155: "Serialize configuration data to JSON format for REST API responses",
    156: "Process thousands of image thumbnails concurrently without spawning unlimited threads",
    157: "Cache recently accessed user profiles to reduce database load",
    158: "Quickly check if a username might exist before querying the database",
    159: "Serve static files and handle basic API requests for a web application",
    160: "Reuse database connections instead of creating new ones for each request",
    161: "Speed up the Fibonacci calculation by caching previously computed values",
    162: "Model TCP connection states (Closed, Listen, SynSent, Established, etc.)",
    163: "Build a complex SQL query step by step with a fluent API",
    164: "Notify multiple components when an event occurs in your application",
    165: "Manage dependencies between services in a modular application",
    166: "Allow different parts of your app to communicate without direct coupling",
    167: "Prevent cascading failures when a downstream service is unavailable",
    168: "Retry failed network requests with increasing delays between attempts",
    169: "Ensure SQL queries are type-safe at compile time, not runtime",
    170: "Build a system where components communicate by sending messages",
}

for id, title, objective, category, difficulty in exercises:
    dir_name = f"{id}_exercise"
    src_dir = os.path.join(dir_name, "src")
    os.makedirs(src_dir, exist_ok=True)
    
    scenario = scenarios.get(id, "Apply this concept to solve a real problem")
    content = template.format(
        id=id,
        title=title,
        objective=objective,
        scenario=scenario
    )
    
    with open(os.path.join(dir_name, "template.rs"), "w") as f:
        f.write(content)
    
    print(f"Created {dir_name}")

print("All exercises created!")
