// Exercise 100: Documentation Tests
//
// Learning objective: Learn how to write documentation tests (doctests)
// that serve as both documentation and executable tests.
//
// Doctests are code examples in documentation comments (/// or //!)
// that are compiled and run as tests with `cargo test`.

/// A simple add function.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let result = template::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Negative numbers:
///
/// ```
/// let result = template::add(-5, 3);
/// assert_eq!(result, -2);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Doubles the given number.
///
/// # Examples
///
/// ```
/// assert_eq!(template::double(5), 10);
/// assert_eq!(template::double(0), 0);
/// ```
pub fn double(n: i32) -> i32 {
    n * 2
}

/// Calculates the factorial of a number.
///
/// # Panics
///
/// Panics if n is greater than 20 due to overflow.
///
/// # Examples
///
/// ```
/// assert_eq!(template::factorial(0), 1);
/// assert_eq!(template::factorial(5), 120);
/// ```
pub fn factorial(n: u32) -> u64 {
    if n > 20 {
        panic!("factorial({}) would overflow", n);
    }
    (1..=n as u64).product()
}

/// A struct representing a point in 2D space.
///
/// # Examples
///
/// Creating a new point:
///
/// ```
/// use template::Point;
///
/// let p = Point::new(3.0, 4.0);
/// assert_eq!(p.x(), 3.0);
/// assert_eq!(p.y(), 4.0);
/// ```
///
/// Calculating distance from origin:
///
/// ```
/// use template::Point;
///
/// let p = Point::new(3.0, 4.0);
/// assert_eq!(p.distance_from_origin(), 5.0);
/// ```
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Creates a new Point with the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::Point;
    ///
    /// let p = Point::new(1.0, 2.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// Returns the x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }
    
    /// Returns the y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }
    
    /// Calculates the distance from the origin (0, 0).
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// Translates the point by the given delta.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::Point;
    ///
    /// let mut p = Point::new(1.0, 2.0);
    /// p.translate(3.0, 4.0);
    /// assert_eq!(p.x(), 4.0);
    /// assert_eq!(p.y(), 6.0);
    /// ```
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

/// A collection that stores unique items.
///
/// # Examples
///
/// ```
/// use template::UniqueCollection;
///
/// let mut coll = UniqueCollection::new();
/// coll.add(1);
/// coll.add(2);
/// coll.add(1); // Duplicate, won't be added
///
/// assert_eq!(coll.len(), 2);
/// assert!(coll.contains(&1));
/// assert!(coll.contains(&2));
/// ```
#[derive(Debug)]
pub struct UniqueCollection<T: PartialEq> {
    items: Vec<T>,
}

impl<T: PartialEq> UniqueCollection<T> {
    /// Creates a new empty collection.
    pub fn new() -> Self {
        UniqueCollection { items: Vec::new() }
    }
    
    /// Adds an item if it's not already present.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::UniqueCollection;
    ///
    /// let mut coll = UniqueCollection::new();
    /// assert!(coll.add(42));
    /// assert!(!coll.add(42)); // Already exists
    /// ```
    pub fn add(&mut self, item: T) -> bool {
        if self.items.contains(&item) {
            false
        } else {
            self.items.push(item);
            true
        }
    }
    
    /// Checks if the collection contains the item.
    pub fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }
    
    /// Returns the number of items in the collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Returns true if the collection is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::UniqueCollection;
    ///
    /// let mut coll: UniqueCollection<i32> = UniqueCollection::new();
    /// assert!(coll.is_empty());
    ///
    /// coll.add(1);
    /// assert!(!coll.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// A trait for types that can be serialized to string.
///
/// # Examples
///
/// ```
/// use template::Serializable;
///
/// let s = "hello".to_serialized();
/// assert_eq!(s, "hello");
///
/// let n = 42.to_serialized();
/// assert_eq!(n, "42");
/// ```
pub trait Serializable {
    fn to_serialized(&self) -> String;
}

impl Serializable for &str {
    fn to_serialized(&self) -> String {
        self.to_string()
    }
}

impl Serializable for i32 {
    fn to_serialized(&self) -> String {
        self.to_string()
    }
}

impl Serializable for Point {
    fn to_serialized(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
}

/// Divides two numbers, returning an error if divisor is zero.
///
/// # Errors
///
/// Returns an error string if `b` is zero.
///
/// # Examples
///
/// ```
/// use template::safe_divide;
///
/// let result = safe_divide(10.0, 2.0);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 5.0);
/// ```
///
/// Division by zero:
///
/// ```
/// use template::safe_divide;
///
/// let result = safe_divide(10.0, 0.0);
/// assert!(result.is_err());
/// ```
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Hidden function - no docs needed
#[doc(hidden)]
pub fn internal_helper() -> i32 {
    42
}

fn main() {
    println!("Documentation tests are run with: cargo test --doc");
    println!("Run all tests with: cargo test");
    println!("\nView documentation with: cargo doc --open");
}

// Unit tests are still useful alongside doctests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
    
    #[test]
    #[should_panic(expected = "overflow")]
    fn test_factorial_overflow() {
        factorial(21);
    }
    
    #[test]
    fn test_unique_collection() {
        let mut coll = UniqueCollection::new();
        assert!(coll.add(1));
        assert!(!coll.add(1));
        assert_eq!(coll.len(), 1);
    }
    
    #[test]
    fn test_serializable() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.to_serialized(), "Point(1.00, 2.00)");
    }
    
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert!(safe_divide(10.0, 0.0).is_err());
    }
}

// Module-level documentation examples
/// This module provides utility functions for string manipulation.
///
/// # Examples
///
/// ```
/// // Example of using the string utilities
/// let s = "hello";
/// assert_eq!(s.len(), 5);
/// ```
pub mod string_utils {
    /// Reverses a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::string_utils::reverse;
    ///
    /// assert_eq!(reverse("hello"), "olleh");
    /// assert_eq!(reverse(""), "");
    /// assert_eq!(reverse("a"), "a");
    /// ```
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    
    /// Checks if a string is a palindrome.
    ///
    /// # Examples
    ///
    /// ```
    /// use template::string_utils::is_palindrome;
    ///
    /// assert!(is_palindrome("racecar"));
    /// assert!(is_palindrome(""));
    /// assert!(!is_palindrome("hello"));
    /// ```
    pub fn is_palindrome(s: &str) -> bool {
        s == reverse(s)
    }
}
