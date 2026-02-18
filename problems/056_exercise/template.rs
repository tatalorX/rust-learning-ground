// Exercise 056: Option - map and and_then
//
// map() transforms the value inside Some, or returns None if None.
// and_then() (flat_map) chains operations that return Options.
//
// Learning Objectives:
// - Use map() to transform Option values
// - Use and_then() to chain Option-returning operations
// - Understand the difference between map and and_then
//
// Your task: Implement functions using map and and_then for Option chaining.

/// Parses a string to i32 and doubles it.
/// Returns Some(doubled) if parsing succeeds, None otherwise.
fn parse_and_double(input: &str) -> Option<i32> {
    // TODO: Parse input as i32, then use map() to double the value
    // Hint: input.parse::<i32>().ok().map(|n| ...)
    todo!()
}

/// Parses a string, doubles it, then converts back to string.
fn parse_double_string(input: &str) -> Option<String> {
    // TODO: Parse input as i32, map to double, then map to String
    todo!()
}

/// Divides a by b, then adds c to the result.
/// All inputs are Options, and the function returns Option<i32>.
fn divide_and_add(a: Option<i32>, b: Option<i32>, c: Option<i32>) -> Option<i32> {
    // TODO: Use and_then to chain operations:
    // 1. Get a_value from a
    // 2. Get b_value from b using and_then
    // 3. If b_value is 0, return None (division by zero)
    // 4. Divide a_value by b_value
    // 5. Add c (use map since we have the value, not Option)
    todo!()
}

/// Finds a user by id, then finds their manager.
/// Returns the manager's name if both exist.
fn find_manager_name(user_id: i32) -> Option<String> {
    // Simulated database lookup functions
    fn find_user(id: i32) -> Option<User> {
        let users = vec![
            User { id: 1, name: "Alice".to_string(), manager_id: Some(2) },
            User { id: 2, name: "Bob".to_string(), manager_id: None },
            User { id: 3, name: "Carol".to_string(), manager_id: Some(1) },
        ];
        users.into_iter().find(|u| u.id == id)
    }
    
    fn find_user_name(id: i32) -> Option<String> {
        find_user(id).map(|u| u.name)
    }
    
    // TODO: Find the user with user_id, then use and_then to find their manager
    // Return the manager's name if both exist
    todo!()
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    manager_id: Option<i32>,
}

fn main() {
    // Test parse_and_double
    println!("Parsing and doubling:");
    for input in &["10", "25", "abc"] {
        match parse_and_double(input) {
            Some(n) => println!("  {} * 2 = {}", input, n),
            None => println!("  {}: Invalid", input),
        }
    }
    
    // Test divide_and_add
    println!("\nDivide and add:");
    println!("  10 / 2 + 5 = {:?}", divide_and_add(Some(10), Some(2), Some(5)));
    println!("  10 / 0 + 5 = {:?}", divide_and_add(Some(10), Some(0), Some(5)));
    println!("  None / 2 + 5 = {:?}", divide_and_add(None, Some(2), Some(5)));
    
    // Test find_manager_name
    println!("\nFinding managers:");
    for id in &[1, 2, 3, 99] {
        match find_manager_name(*id) {
            Some(name) => println!("  User {}'s manager: {}", id, name),
            None => println!("  User {} has no manager", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double_valid() {
        assert_eq!(parse_and_double("10"), Some(20));
        assert_eq!(parse_and_double("5"), Some(10));
    }

    #[test]
    fn test_parse_and_double_invalid() {
        assert_eq!(parse_and_double("abc"), None);
    }

    #[test]
    fn test_parse_double_string_valid() {
        assert_eq!(parse_double_string("10"), Some("20".to_string()));
    }

    #[test]
    fn test_divide_and_add_success() {
        assert_eq!(divide_and_add(Some(10), Some(2), Some(5)), Some(10));
        assert_eq!(divide_and_add(Some(20), Some(4), Some(3)), Some(8));
    }

    #[test]
    fn test_divide_and_add_division_by_zero() {
        assert_eq!(divide_and_add(Some(10), Some(0), Some(5)), None);
    }

    #[test]
    fn test_divide_and_add_with_none() {
        assert_eq!(divide_and_add(None, Some(2), Some(5)), None);
        assert_eq!(divide_and_add(Some(10), None, Some(5)), None);
        assert_eq!(divide_and_add(Some(10), Some(2), None), None);
    }

    #[test]
    fn test_find_manager_name_existing() {
        assert_eq!(find_manager_name(1), Some("Bob".to_string()));
        assert_eq!(find_manager_name(3), Some("Alice".to_string()));
    }

    #[test]
    fn test_find_manager_name_none() {
        assert_eq!(find_manager_name(2), None); // Bob has no manager
        assert_eq!(find_manager_name(99), None); // User doesn't exist
    }
}
