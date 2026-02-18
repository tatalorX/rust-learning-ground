// Exercise 099: Tests - Assert Macros
//
// Learning objective: Master the various assert macros available
// in Rust for writing comprehensive tests.
//
// Rust provides many assert macros for different testing scenarios:
// assert!, assert_eq!, assert_ne!, debug_assert!, and custom messages.

// Functions to test
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn find_max(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().max()
}

pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("'{}' is not a valid number", s))
}

#[derive(Debug, PartialEq)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
    
    pub fn can_vote(&self) -> bool {
        self.age >= 18
    }
    
    pub fn is_senior(&self) -> bool {
        self.age >= 65
    }
}

// A collection type to test
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    println!("Run tests with: cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: Use assert! macro to test boolean conditions
    #[test]
    fn test_assert_macro() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        
        // Test the opposite with !
        assert!(!is_even(3));
        assert!(!is_even(1));
        assert!(!is_even(-7));
    }
    
    // TODO: Use assert_eq! macro for equality comparisons
    #[test]
    fn test_assert_eq() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(5), false);
        
        let person = Person::new("Alice", 25);
        assert_eq!(person.can_vote(), true);
        assert_eq!(person.is_senior(), false);
    }
    
    // TODO: Use assert_ne! macro for inequality comparisons
    #[test]
    fn test_assert_ne() {
        assert_ne!(is_even(4), false);
        assert_ne!(is_even(5), true);
        
        let person1 = Person::new("Alice", 25);
        let person2 = Person::new("Bob", 30);
        assert_ne!(person1.name, person2.name);
    }
    
    // TODO: Use assert! with custom error messages
    #[test]
    fn test_custom_messages() {
        let numbers = vec![1, 2, 3, 4, 5];
        let max = find_max(&numbers);
        
        assert!(
            max.is_some(),
            "Expected Some(max) for non-empty list, got None"
        );
        
        assert_eq!(
            max.unwrap(),
            &5,
            "Expected max to be 5, got {:?}",
            max
        );
    }
    
    // TODO: Use assert_eq! with custom messages
    #[test]
    fn test_parse_number_success() {
        let result = parse_number("42");
        assert_eq!(
            result,
            Ok(42),
            "parse_number should succeed for valid input"
        );
    }
    
    // TODO: Test error cases with assert!
    #[test]
    fn test_parse_number_failure() {
        let result = parse_number("not a number");
        assert!(result.is_err(), "parse_number should fail for invalid input");
        
        // Check the error message contains expected text
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("not a valid number"),
            "Error message should indicate invalid number: {}",
            err_msg
        );
    }
    
    // TODO: Test partial equality with assert_eq!
    #[test]
    fn test_person_equality() {
        let p1 = Person::new("Alice", 25);
        let p2 = Person::new("Alice", 25);
        let p3 = Person::new("Bob", 25);
        
        assert_eq!(p1, p2, "Same name and age should be equal");
        assert_ne!(p1, p3, "Different names should not be equal");
    }
    
    // TODO: Test Stack operations with various assertions
    #[test]
    fn test_stack_new_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty(), "New stack should be empty");
        assert_eq!(stack.len(), 0, "New stack should have length 0");
    }
    
    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        
        stack.push(10);
        assert!(!stack.is_empty(), "Stack should not be empty after push");
        assert_eq!(stack.len(), 1, "Stack should have length 1");
        
        let item = stack.pop();
        assert_eq!(item, Some(10), "Pop should return the pushed item");
        assert!(stack.is_empty(), "Stack should be empty after pop");
    }
    
    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        // Peek should return the last item without removing it
        assert_eq!(stack.peek(), Some(&3), "Peek should return last item");
        assert_eq!(stack.len(), 3, "Peek should not modify stack");
        
        // Pop should still return 3
        assert_eq!(stack.pop(), Some(3));
    }
    
    #[test]
    fn test_stack_multiple_items() {
        let mut stack = Stack::new();
        let items = vec![1, 2, 3, 4, 5];
        
        for item in &items {
            stack.push(*item);
        }
        
        assert_eq!(stack.len(), items.len());
        
        // Pop all items and verify LIFO order
        let mut popped = vec![];
        while let Some(item) = stack.pop() {
            popped.push(item);
        }
        
        assert_eq!(popped, vec![5, 4, 3, 2, 1], "Stack should be LIFO");
    }
    
    // TODO: Use assert_matches! pattern (if available) or match with assert!
    #[test]
    fn test_find_max_variants() {
        let empty: &[i32] = &[];
        assert!(find_max(empty).is_none(), "Max of empty should be None");
        
        let single = &[42];
        assert_eq!(find_max(single), Some(&42), "Max of single element");
        
        let multiple = &[3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(find_max(multiple), Some(&9), "Max should be 9");
    }
    
    // TODO: Complex assertion scenarios
    #[test]
    fn test_complex_assertions() {
        let people = vec![
            Person::new("Alice", 17),
            Person::new("Bob", 25),
            Person::new("Carol", 70),
        ];
        
        // Count voters and seniors
        let voters: Vec<_> = people.iter().filter(|p| p.can_vote()).collect();
        let seniors: Vec<_> = people.iter().filter(|p| p.is_senior()).collect();
        
        assert_eq!(voters.len(), 2, "Should have 2 voters");
        assert_eq!(seniors.len(), 1, "Should have 1 senior");
        
        // Verify specific people
        assert!(voters.iter().any(|p| p.name == "Bob"), "Bob should be a voter");
        assert!(seniors.iter().any(|p| p.name == "Carol"), "Carol should be a senior");
    }
    
    // TODO: Test edge cases
    #[test]
    fn test_edge_cases() {
        // Test with i32::MAX
        let numbers = &[1, 2, i32::MAX];
        assert_eq!(find_max(numbers), Some(&i32::MAX));
        
        // Test with negative numbers
        let negatives = &[-5, -3, -10, -1];
        assert_eq!(find_max(negatives), Some(&-1));
        
        // Test zero
        assert!(is_even(0), "Zero should be even");
        
        // Test boundary ages
        let minor = Person::new("Young", 17);
        let adult = Person::new("Just Adult", 18);
        let senior = Person::new("Senior", 65);
        
        assert!(!minor.can_vote(), "17-year-old cannot vote");
        assert!(adult.can_vote(), "18-year-old can vote");
        assert!(senior.is_senior(), "65-year-old is senior");
        assert!(!adult.is_senior(), "18-year-old is not senior");
    }
}

// TODO: Add integration-style tests that use multiple features
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_person_workflow() {
        // Create a person
        let mut person = Person::new("Test", 17);
        assert!(!person.can_vote());
        
        // Age them up (in a real scenario this would be different)
        person.age = 18;
        assert!(person.can_vote());
    }
    
    #[test]
    fn test_stack_as_collection() {
        let mut stack = Stack::new();
        
        // Push some items
        for i in 0..100 {
            stack.push(i);
        }
        
        assert_eq!(stack.len(), 100);
        
        // Pop half
        for _ in 0..50 {
            stack.pop();
        }
        
        assert_eq!(stack.len(), 50);
        
        // The next pop should be 49
        assert_eq!(stack.pop(), Some(49));
    }
}
