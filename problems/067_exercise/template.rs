// Exercise 067: Lifetimes - Basic Annotation
//
// Lifetimes tell the compiler how references relate to each other.
// They ensure references don't outlive the data they point to.
//
// Learning Objectives:
// - Understand lifetime syntax
// - Annotate references with explicit lifetimes
// - Recognize when lifetimes are needed
//
// Your task: Add lifetime annotations to make the code compile.

/// Returns the longer of two string slices.
/// The returned reference must not outlive either input.
fn longer_str(a: &str, b: &str) -> &str {
    // TODO: Add lifetime annotations to all references
    // Return a if a.len() >= b.len(), otherwise b
    todo!()
}

/// Returns the first element of a slice.
fn first_elem<T>(items: &[T]) -> &T {
    // TODO: Add lifetime annotation
    // Return reference to the first element
    todo!()
}

/// A function that takes two references and returns one of them.
fn choose_reference<'a>(use_first: bool, first: &'a str, second: &'a str) -> &'a str {
    // TODO: Return first if use_first, otherwise second
    todo!()
}

/// Returns a reference to the value with the greater length.
fn longer_by_value<T: std::fmt::Display>(a: &T, b: &T) -> &T {
    // TODO: Add lifetime annotations
    // Return reference to whichever has longer string representation
    todo!()
}

/// A struct that holds a reference to a string.
struct StringHolder {
    text: &str,
}

impl StringHolder {
    /// Creates a new StringHolder.
    fn new(text: &str) -> StringHolder {
        // TODO: Add lifetime annotations
        todo!()
    }
    
    /// Returns the held string.
    fn get(&self) -> &str {
        // TODO: Add lifetime annotations
        todo!()
    }
}

fn main() {
    // After adding lifetime annotations, uncomment and run:
    
    // let s1 = String::from("short");
    // let s2 = String::from("much longer string");
    // 
    // println!("Longer string: {}", longer_str(&s1, &s2));
    // 
    // let items = vec![1, 2, 3, 4, 5];
    // println!("First element: {}", first_elem(&items));
    // 
    // let chosen = choose_reference(true, "first", "second");
    // println!("Chosen: {}", chosen);
    // 
    // let holder = StringHolder::new("hello world");
    // println!("Holder contains: {}", holder.get());
    
    println!("Add lifetime annotations to make this code work!");
    println!("Remember the syntax: &'a T for references, <'a> for generics/impls");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_str_first() {
        assert_eq!(longer_str("longer", "short"), "longer");
    }

    #[test]
    fn test_longer_str_second() {
        assert_eq!(longer_str("short", "longer"), "longer");
    }

    #[test]
    fn test_longer_str_equal() {
        assert_eq!(longer_str("same", "same"), "same");
    }

    #[test]
    fn test_first_elem() {
        let items = vec![10, 20, 30];
        assert_eq!(first_elem(&items), &10);
    }

    #[test]
    fn test_choose_reference() {
        assert_eq!(choose_reference(true, "a", "b"), "a");
        assert_eq!(choose_reference(false, "a", "b"), "b");
    }

    #[test]
    fn test_string_holder() {
        let holder = StringHolder::new("test");
        assert_eq!(holder.get(), "test");
    }
}
