// Exercise 059: Generic Functions
//
// Generic functions work with multiple types using type parameters.
// This enables code reuse while maintaining type safety.
//
// Learning Objectives:
// - Define generic functions with type parameters
// - Use generics with trait bounds
// - Understand monomorphization
//
// Your task: Implement generic functions that work with multiple types.

/// Returns the first element of a slice, regardless of type.
fn first<T>(items: &[T]) -> Option<&T> {
    // TODO: Return the first element of the slice, or None if empty
    todo!()
}

/// Swaps two values of the same type.
fn swap<T>(a: T, b: T) -> (T, T) {
    // TODO: Return (b, a)
    todo!()
}

/// Returns the larger of two values (works for any comparable type).
fn max<T: PartialOrd>(a: T, b: T) -> T {
    // TODO: Return the larger of a and b
    // Note: The PartialOrd bound is required for comparison
    todo!()
}

/// Prints debug information for any type that implements Debug.
fn debug_info<T: std::fmt::Debug>(value: &T) {
    // TODO: Print the debug representation of value
    // Format: "Debug: {value:?}"
    todo!()
}

/// Combines two values into a tuple.
fn pair<T, U>(a: T, b: U) -> (T, U) {
    // TODO: Return a tuple containing a and b
    todo!()
}

/// Compares two values and returns true if they are equal.
/// Works for any type that implements PartialEq.
fn are_equal<T: PartialEq>(a: &T, b: &T) -> bool {
    // TODO: Return true if a equals b
    todo!()
}

fn main() {
    // Test first with different types
    let numbers = vec![10, 20, 30];
    let words = vec!["hello", "world"];
    
    println!("First of numbers: {:?}", first(&numbers));
    println!("First of words: {:?}", first(&words));
    println!("First of empty: {:?}", first(&[] as &[i32]));
    
    // Test swap
    println!("\nSwapping:");
    let (a, b) = swap(1, 2);
    println!("  swap(1, 2) = ({}, {})", a, b);
    let (x, y) = swap("hello", "world");
    println!("  swap('hello', 'world') = ({}, {})", x, y);
    
    // Test max
    println!("\nMax values:");
    println!("  max(5, 10) = {}", max(5, 10));
    println!("  max(3.14, 2.71) = {}", max(3.14, 2.71));
    println!("  max('a', 'z') = {}", max('a', 'z'));
    
    // Test debug_info
    println!("\nDebug info:");
    debug_info(&42);
    debug_info(&"hello");
    debug_info(&vec![1, 2, 3]);
    
    // Test pair
    println!("\nPairs:");
    let p1 = pair(1, "one");
    println!("  pair(1, 'one') = {:?}", p1);
    let p2 = pair(true, 3.14);
    println!("  pair(true, 3.14) = {:?}", p2);
    
    // Test are_equal
    println!("\nEquality checks:");
    println!("  are_equal(&5, &5) = {}", are_equal(&5, &5));
    println!("  are_equal(&'a', &'b') = {}", are_equal(&'a', &'b'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_with_values() {
        assert_eq!(first(&[1, 2, 3]), Some(&1));
        assert_eq!(first(&["a", "b"]), Some(&"a"));
    }

    #[test]
    fn test_first_empty() {
        let empty: &[i32] = &[];
        assert_eq!(first(empty), None);
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap("a", "b"), ("b", "a"));
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(10, 5), 10);
        assert_eq!(max(3.5, 2.1), 3.5);
    }

    #[test]
    fn test_are_equal() {
        assert!(are_equal(&5, &5));
        assert!(!are_equal(&5, &6));
        assert!(are_equal(&"hello", &"hello"));
    }

    #[test]
    fn test_pair() {
        assert_eq!(pair(1, "a"), (1, "a"));
        assert_eq!(pair(true, 3.14), (true, 3.14));
    }
}
