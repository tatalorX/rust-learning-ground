// Exercise 070: Static Lifetime
//
// The 'static lifetime denotes data that lives for the entire program duration.
// String literals have 'static lifetime, as do some global constants.
//
// Learning Objectives:
// - Understand when 'static lifetime is used
// - Create 'static data
// - Use 'static in trait objects and generic contexts
//
// Your task: Work with 'static lifetime in various contexts.

use std::any::Any;

/// Returns a static string literal.
fn get_static_string() -> &'static str {
    // TODO: Return a string literal (which has 'static lifetime)
    todo!()
}

/// Returns a static error message based on an error code.
fn error_message(code: u32) -> &'static str {
    // TODO: Match on code and return appropriate static message:
    // 404 -> "Not found"
    // 500 -> "Server error"
    // other -> "Unknown error"
    todo!()
}

/// A trait for objects that can provide a static description.
trait StaticallyDescribable {
    fn static_description() -> &'static str;
}

// Types to implement the trait for
struct User;
struct Admin;
struct Guest;

// TODO: Implement StaticallyDescribable for User
// static_description returns "A regular user account"

// TODO: Implement StaticallyDescribable for Admin
// static_description returns "An administrator with full privileges"

// TODO: Implement StaticallyDescribable for Guest
// static_description returns "A guest with limited access"

/// A registry of static strings for lookup.
static STATIC_STRINGS: &[&str] = &[
    "Rust",
    "is",
    "awesome",
];

/// Returns a reference to the static strings array.
fn get_static_strings() -> &'static [&'static str] {
    // TODO: Return STATIC_STRINGS
    todo!()
}

/// A function that accepts only 'static data.
fn accept_static<T: 'static>(_value: T) {
    // TODO: This function accepts any 'static value
    // It doesn't need to do anything, just compile
    todo!()
}

/// A trait object type that requires 'static lifetime.
type StaticAny = Box<dyn Any + 'static>;

/// Creates a boxed 'static value.
fn box_static<T: 'static>(value: T) -> StaticAny {
    // TODO: Return Box::new(value)
    todo!()
}

/// A configuration struct with static strings.
struct StaticConfig {
    name: &'static str,
    version: &'static str,
    author: &'static str,
}

impl StaticConfig {
    /// Creates a default configuration.
    fn default() -> Self {
        // TODO: Return StaticConfig with static string values
        todo!()
    }
    
    /// Returns a formatted description.
    fn description(&self) -> String {
        // TODO: Return format!("{} v{} by {}", name, version, author)
        todo!()
    }
}

fn main() {
    // After implementing, uncomment and run:
    
    // println!("Static string: {}", get_static_string());
    // 
    // println!("\nError messages:");
    // for code in &[404, 500, 999] {
    //     println!("  Code {}: {}", code, error_message(*code));
    // }
    // 
    // println!("\nStatic descriptions:");
    // println!("  User: {}", User::static_description());
    // println!("  Admin: {}", Admin::static_description());
    // println!("  Guest: {}", Guest::static_description());
    // 
    // println!("\nStatic strings: {:?}", get_static_strings());
    // 
    // // These all have 'static lifetime
    // accept_static(42i32);
    // accept_static("hello");
    // accept_static(vec![1, 2, 3]);
    // 
    // let boxed: StaticAny = box_static(42);
    // if let Some(num) = boxed.downcast_ref::<i32>() {
    //     println!("\nBoxed value: {}", num);
    // }
    // 
    // let config = StaticConfig::default();
    // println!("\nConfig: {}", config.description());
    
    println!("Implement all TODOs to see 'static in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_static_string() {
        let s = get_static_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_error_message_404() {
        assert_eq!(error_message(404), "Not found");
    }

    #[test]
    fn test_error_message_500() {
        assert_eq!(error_message(500), "Server error");
    }

    #[test]
    fn test_error_message_other() {
        assert_eq!(error_message(999), "Unknown error");
    }

    #[test]
    fn test_user_description() {
        assert_eq!(User::static_description(), "A regular user account");
    }

    #[test]
    fn test_admin_description() {
        assert_eq!(Admin::static_description(), "An administrator with full privileges");
    }

    #[test]
    fn test_guest_description() {
        assert_eq!(Guest::static_description(), "A guest with limited access");
    }

    #[test]
    fn test_get_static_strings() {
        let strings = get_static_strings();
        assert!(!strings.is_empty());
        assert_eq!(strings[0], "Rust");
    }

    #[test]
    fn test_box_static() {
        let boxed = box_static(42);
        assert!(boxed.downcast_ref::<i32>().is_some());
    }

    #[test]
    fn test_static_config_default() {
        let config = StaticConfig::default();
        assert!(!config.name.is_empty());
        assert!(!config.description().is_empty());
    }
}
