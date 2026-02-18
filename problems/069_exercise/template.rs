// Exercise 069: Lifetimes - In Structs
//
// Structs that hold references need lifetime annotations.
// The lifetime parameter ensures the struct doesn't outlive its references.
//
// Learning Objectives:
// - Add lifetime parameters to structs
// - Implement methods with proper lifetime annotations
// - Use structs with lifetimes in practice
//
// Your task: Create and use structs with lifetime annotations.

/// A struct that holds a reference to text.
struct TextHolder {
    text: &str,
    start: usize,
    end: usize,
}

impl TextHolder {
    /// Creates a new TextHolder with the given text range.
    fn new(text: &str, start: usize, end: usize) -> TextHolder {
        // TODO: Add lifetime annotations
        // Clamp start and end to valid bounds
        todo!()
    }
    
    /// Returns the held text slice.
    fn get(&self) -> &str {
        // TODO: Add lifetime annotations
        // Return &self.text[self.start..self.end]
        todo!()
    }
    
    /// Returns the length of the held text.
    fn len(&self) -> usize {
        // TODO: Return the length of the held slice
        todo!()
    }
    
    /// Returns true if the holder is empty.
    fn is_empty(&self) -> bool {
        // TODO: Return whether len() == 0
        todo!()
    }
}

/// A struct that holds two references with potentially different lifetimes.
struct PairHolder {
    first: &str,
    second: &str,
}

impl PairHolder {
    /// Creates a new PairHolder.
    fn new(first: &str, second: &str) -> PairHolder {
        // TODO: Add lifetime annotations
        todo!()
    }
    
    /// Returns the longer of the two strings.
    fn longer(&self) -> &str {
        // TODO: Add lifetime annotations
        // Return first if first.len() >= second.len(), else second
        todo!()
    }
    
    /// Returns a concatenated view (conceptually, just for demonstration).
    fn describe(&self) -> String {
        // TODO: Return a description like "First: '...', Second: '...'"
        format!("First: '{}', Second: '{}'", self.first, self.second)
    }
}

/// A generic wrapper that holds a reference to any type.
struct RefBox {
    value: &T,
}

impl RefBox {
    /// Creates a new RefBox.
    fn new(value: &T) -> RefBox {
        // TODO: Add lifetime annotations
        todo!()
    }
    
    /// Returns a reference to the contained value.
    fn get(&self) -> &T {
        // TODO: Add lifetime annotations
        todo!()
    }
    
    /// Maps the contained value using a function.
    fn map<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        // TODO: Apply f to the contained value
        todo!()
    }
}

/// A parser that holds a reference to input text.
struct Parser {
    input: &str,
    position: usize,
}

impl Parser {
    /// Creates a new Parser.
    fn new(input: &str) -> Parser {
        // TODO: Add lifetime annotations
        todo!()
    }
    
    /// Returns the remaining unparsed text.
    fn remaining(&self) -> &str {
        // TODO: Add lifetime annotations
        // Return &self.input[self.position..]
        todo!()
    }
    
    /// Advances the position by n characters.
    fn advance(&mut self, n: usize) {
        // TODO: Move position forward by n (clamped to input length)
        self.position = (self.position + n).min(self.input.len());
    }
    
    /// Parses the next word (alphanumeric characters).
    fn next_word(&mut self) -> Option<&str> {
        // TODO: Add lifetime annotations
        // Skip non-alphanumeric, then collect alphanumeric characters
        // Update position and return the word
        todo!()
    }
}

fn main() {
    // After adding lifetime annotations, uncomment and run:
    
    // let text = "Hello, World!";
    // let holder = TextHolder::new(text, 0, 5);
    // println!("TextHolder: '{}' (len: {})", holder.get(), holder.len());
    // 
    // let pair = PairHolder::new("first", "second longer");
    // println!("\nPairHolder: {}", pair.describe());
    // println!("Longer: '{}'", pair.longer());
    // 
    // let num = 42;
    // let ref_box = RefBox::new(&num);
    // println!("\nRefBox value: {}", ref_box.get());
    // println!("RefBox doubled: {}", ref_box.map(|n| n * 2));
    // 
    // let mut parser = Parser::new("Hello World 123");
    // println!("\nParsing:");
    // while let Some(word) = parser.next_word() {
    //     println!("  Word: '{}'", word);
    // }
    
    println!("Add lifetime annotations to make the structs work!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_holder_new() {
        let holder = TextHolder::new("Hello World", 0, 5);
        assert_eq!(holder.get(), "Hello");
    }

    #[test]
    fn test_text_holder_len() {
        let holder = TextHolder::new("Hello", 0, 5);
        assert_eq!(holder.len(), 5);
    }

    #[test]
    fn test_text_holder_clamping() {
        let holder = TextHolder::new("Hi", 0, 100);
        assert_eq!(holder.len(), 2);
    }

    #[test]
    fn test_pair_holder_longer() {
        let pair = PairHolder::new("short", "much longer text");
        assert_eq!(pair.longer(), "much longer text");
    }

    #[test]
    fn test_ref_box() {
        let value = 42;
        let boxed = RefBox::new(&value);
        assert_eq!(boxed.get(), &42);
    }

    #[test]
    fn test_ref_box_map() {
        let value = 5;
        let boxed = RefBox::new(&value);
        assert_eq!(boxed.map(|n| n * 2), 10);
    }

    #[test]
    fn test_parser_remaining() {
        let parser = Parser::new("Hello");
        assert_eq!(parser.remaining(), "Hello");
    }

    #[test]
    fn test_parser_next_word() {
        let mut parser = Parser::new("Hello World");
        assert_eq!(parser.next_word(), Some("Hello"));
        assert_eq!(parser.next_word(), Some("World"));
        assert_eq!(parser.next_word(), None);
    }
}
