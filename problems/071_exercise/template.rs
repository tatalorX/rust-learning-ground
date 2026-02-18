// Exercise 071: Closures - Basic Syntax
//
// Closures are anonymous functions that can capture their environment.
// They have flexible syntax and type inference.
//
// Learning Objectives:
// - Write closures with different syntax styles
// - Understand closure type inference
// - Use closures with different arities
//
// Your task: Create closures using various syntax forms.

/// Returns a closure that adds a constant to its input.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // TODO: Return a closure that adds n to its argument
    todo!()
}

/// Returns a closure that multiplies its input by a constant.
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    // TODO: Return a closure that multiplies its argument by factor
    todo!()
}

/// Returns a closure that checks if a number is greater than a threshold.
fn make_threshold_checker(threshold: i32) -> impl Fn(i32) -> bool {
    // TODO: Return a closure that returns true if its argument > threshold
    todo!()
}

/// Applies a function to each element of a vector and returns the results.
fn map_vector<F, T, U>(items: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    // TODO: Apply f to each item and collect into a new vector
    todo!()
}

/// Returns a closure that formats a greeting.
fn make_greeter(greeting: &str) -> impl Fn(&str) -> String + '_ {
    // TODO: Return a closure that takes a name and returns "{greeting}, {name}!"
    todo!()
}

/// Creates a closure that concatenates two strings with a separator.
fn make_concatenator(separator: &str) -> impl Fn(&str, &str) -> String + '_ {
    // TODO: Return a closure that takes two strings and joins them with separator
    todo!()
}

/// Returns a closure that counts how many times it's been called.
fn make_counter() -> impl FnMut() -> i32 {
    // TODO: Return a closure that increments and returns a counter on each call
    // Hint: use move and a mutable variable
    todo!()
}

/// Returns a closure that accumulates values.
fn make_accumulator() -> impl FnMut(i32) -> i32 {
    // TODO: Return a closure that adds its argument to a running total
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // println!("Adder closures:");
    // let add_five = make_adder(5);
    // println!("  add_five(10) = {}", add_five(10));
    // println!("  add_five(3) = {}", add_five(3));
    // 
    // let double = make_multiplier(2);
    // println!("\nMultiplier closures:");
    // println!("  double(7) = {}", double(7));
    // println!("  double(15) = {}", double(15));
    // 
    // let is_adult = make_threshold_checker(18);
    // println!("\nThreshold checker:");
    // println!("  is_adult(21) = {}", is_adult(21));
    // println!("  is_adult(16) = {}", is_adult(16));
    // 
    // let numbers = vec![1, 2, 3, 4, 5];
    // let doubled = map_vector(numbers.clone(), |x| x * 2);
    // println!("\nMap vector: {:?} -> {:?}", numbers, doubled);
    // 
    // let square_strings = map_vector(numbers, |x| format!("{}² = {}", x, x * x));
    // println!("As strings: {:?}", square_strings);
    // 
    // let greeter = make_greeter("Hello");
    // println!("\nGreeting: {}", greeter("World"));
    // 
    // let join_with_comma = make_concatenator(", ");
    // println!("Concatenation: '{}'", join_with_comma("apple", "banana"));
    // 
    // let mut counter = make_counter();
    // println!("\nCounter:");
    // println!("  Call 1: {}", counter());
    // println!("  Call 2: {}", counter());
    // println!("  Call 3: {}", counter());
    // 
    // let mut acc = make_accumulator();
    // println!("\nAccumulator:");
    // println!("  Add 5: {}", acc(5));
    // println!("  Add 3: {}", acc(3));
    // println!("  Add 10: {}", acc(10));
    
    println!("Implement all TODOs to see closures in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_adder() {
        let add_ten = make_adder(10);
        assert_eq!(add_ten(5), 15);
        assert_eq!(add_ten(0), 10);
    }

    #[test]
    fn test_make_multiplier() {
        let triple = make_multiplier(3);
        assert_eq!(triple(4), 12);
        assert_eq!(triple(0), 0);
    }

    #[test]
    fn test_make_threshold_checker() {
        let over_100 = make_threshold_checker(100);
        assert!(over_100(101));
        assert!(!over_100(100));
        assert!(!over_100(50));
    }

    #[test]
    fn test_map_vector() {
        let nums = vec![1, 2, 3];
        let doubled = map_vector(nums, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_make_greeter() {
        let hi = make_greeter("Hi");
        assert_eq!(hi("Alice"), "Hi, Alice!");
    }

    #[test]
    fn test_make_concatenator() {
        let join = make_concatenator(" - ");
        assert_eq!(join("A", "B"), "A - B");
    }

    #[test]
    fn test_make_counter() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_make_accumulator() {
        let mut acc = make_accumulator();
        assert_eq!(acc(5), 5);
        assert_eq!(acc(10), 15);
        assert_eq!(acc(3), 18);
    }
}
