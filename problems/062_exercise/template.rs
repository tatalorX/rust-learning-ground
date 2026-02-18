// Exercise 062: Traits - Defining
//
// Traits define shared behavior across types, similar to interfaces in other languages.
// They specify methods that types must implement.
//
// Learning Objectives:
// - Define custom traits
// - Declare required methods in traits
// - Use associated types and constants in traits
//
// Your task: Define several useful traits for different scenarios.

/// A trait for types that can be summarized with a short description.
trait Summarizable {
    /// Returns a brief summary of the object.
    fn summary(&self) -> String;
    
    /// Returns a detailed description (has default implementation).
    fn details(&self) -> String {
        format!("Details: {}", self.summary())
    }
}

/// A trait for measuring the size of something.
trait Measurable {
    /// Returns the size as a f64 value.
    fn size(&self) -> f64;
    
    /// Returns the unit of measurement.
    fn unit(&self) -> &'static str;
    
    /// Formats the size with unit.
    fn format_size(&self) -> String {
        format!("{:.2} {}", self.size(), self.unit())
    }
}

/// A trait for types that can be validated.
trait Validatable {
    /// Returns true if the object is valid.
    fn is_valid(&self) -> bool;
    
    /// Returns a list of validation errors.
    fn errors(&self) -> Vec<String>;
}

/// A trait for objects that can be serialized to string.
trait Serializable {
    /// Serializes the object to a string.
    fn serialize(&self) -> String;
    
    /// Returns the format name (JSON, XML, etc.).
    fn format() -> &'static str where Self: Sized;
}

/// A trait for objects that can be compared for equality in a specific way.
trait Equivalent<Rhs = Self> {
    /// Returns true if self is equivalent to other.
    fn is_equivalent(&self, other: &Rhs) -> bool;
}

/// A trait for objects that have a unique identifier.
trait Identifiable {
    /// The type of the identifier.
    type Id: std::fmt::Display;
    
    /// Returns the unique identifier.
    fn id(&self) -> Self::Id;
}

// Example implementations for primitive types
impl Summarizable for i32 {
    fn summary(&self) -> String {
        format!("Integer: {}", self)
    }
}

impl Summarizable for String {
    fn summary(&self) -> String {
        if self.len() > 20 {
            format!("\"{}...\" ({} chars)", &self[..20], self.len())
        } else {
            format!("\"{}\"", self)
        }
    }
}

// TODO: Implement Measurable for f64 (representing meters)
// TODO: Implement Validatable for String (valid if not empty and length <= 100)
// TODO: Implement Serializable for i32 (format: "JSON")
// TODO: Implement Equivalent<String> for &str (compare ignoring case)
// TODO: Implement Identifiable for a custom struct

fn main() {
    // Test Summarizable
    let num = 42i32;
    let text = String::from("Hello, World!");
    let long_text = String::from("This is a very long string that exceeds twenty characters");
    
    println!("Summarizable examples:");
    println!("  Number: {}", num.summary());
    println!("  Text: {}", text.summary());
    println!("  Long text: {}", long_text.summary());
    println!("  Long text details: {}", long_text.details());
    
    // After implementing the traits above, these should work:
    // let length = 5.5f64;
    // println!("\nMeasurable: {}", length.format_size());
    
    // let empty = String::new();
    // let valid = String::from("Hello");
    // println!("\nValidatable:");
    // println!("  Empty is valid: {}", empty.is_valid());
    // println!("  Valid is valid: {}", valid.is_valid());
    
    // let num = 42i32;
    // println!("\nSerializable: {} format: {}", num.serialize(), i32::format());
    
    // let s = "HELLO";
    // println!("\nEquivalent: 'HELLO' == 'hello': {}", s.is_equivalent(&"hello".to_string()));
    
    println!("\nImplement the TODOs to see more examples!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarizable_i32() {
        let num = 42;
        assert_eq!(num.summary(), "Integer: 42");
    }

    #[test]
    fn test_summarizable_string_short() {
        let s = String::from("Hi");
        assert_eq!(s.summary(), "\"Hi\"");
    }

    #[test]
    fn test_summarizable_string_long() {
        let s = String::from("This is more than twenty characters long");
        assert!(s.summary().contains("..."));
        assert!(s.summary().contains("chars"));
    }

    // TODO: Add tests for Measurable, Validatable, Serializable, Equivalent, Identifiable
    // after implementing them
}
