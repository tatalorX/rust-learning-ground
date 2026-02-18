// Exercise 120: Stack Implementation using Vec
// ==============================================
//
// Learning Objective:
// Learn how to implement a Stack data structure using Rust's Vec.
// A stack follows LIFO (Last In, First Out) ordering.
//
// Operations:
// - push: Add element to top (O(1))
// - pop: Remove and return top element (O(1))
// - peek: View top element without removing (O(1))

fn main() {
    println!("=== Stack Implementation ===\n");
    
    let mut stack = Stack::new();
    
    println!("Pushing: 10, 20, 30");
    stack.push(10);
    stack.push(20);
    stack.push(30);
    
    println!("Stack size: {}", stack.size());
    println!("Top element: {:?}", stack.peek());
    
    println!("\nPopping:");
    while let Some(val) = stack.pop() {
        println!("  Popped: {}", val);
    }
    
    println!("\nIs empty? {}", stack.is_empty());
    
    // Demonstrate practical use: balanced parentheses
    println!("\n=== Balanced Parentheses Check ===");
    let expressions = vec![
        "((()))",
        "()()",
        "(()",
        "())(",
        "",
        "([{}])",
    ];
    
    for expr in expressions {
        let result = is_balanced(expr);
        println!("'{}' -> {}", expr, if result { "balanced" } else { "not balanced" });
    }
    
    println!("\n✓ Stack implementation completed successfully!");
}

/// Stack data structure
///
/// Uses Vec internally for dynamic array storage.
/// The top of the stack is the end of the Vec.
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack
    fn new() -> Self {
        // TODO: Initialize an empty stack
        Stack { items: Vec::new() }
    }
    
    /// Push an item onto the stack
    ///
    /// Time Complexity: O(1) amortized
    fn push(&mut self, item: T) {
        // TODO: Add item to the top (end of Vec)
        self.items.push(item);
    }
    
    /// Pop an item from the stack
    ///
    /// Returns None if the stack is empty.
    /// Time Complexity: O(1)
    fn pop(&mut self) -> Option<T> {
        // TODO: Remove and return the top item
        self.items.pop()
    }
    
    /// Peek at the top item without removing it
    ///
    /// Returns None if the stack is empty.
    /// Time Complexity: O(1)
    fn peek(&self) -> Option<&T> {
        // TODO: Return a reference to the top item
        self.items.last()
    }
    
    /// Check if the stack is empty
    fn is_empty(&self) -> bool {
        // TODO: Return true if stack is empty
        self.items.is_empty()
    }
    
    /// Get the size of the stack
    fn size(&self) -> usize {
        // TODO: Return the number of items
        self.items.len()
    }
    
    /// Clear all items from the stack
    fn clear(&mut self) {
        // TODO: Remove all items
        self.items.clear();
    }
}

/// Check if parentheses are balanced using a stack
///
/// Algorithm:
/// 1. For each character in the string:
///    - If it's an opening bracket, push to stack
///    - If it's a closing bracket:
///      - If stack is empty or top doesn't match, return false
///      - Otherwise, pop from stack
/// 2. Return true if stack is empty (all brackets matched)
fn is_balanced(s: &str) -> bool {
    // TODO: Implement balanced parentheses check
    let mut stack = Stack::new();
    
    for c in s.chars() {
        match c {
            // TODO: Push opening brackets
            '(' | '[' | '{' => stack.push(c),
            
            // TODO: Check closing brackets
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {} // Ignore other characters
        }
    }
    
    // TODO: Stack should be empty if balanced
    stack.is_empty()
}

/// Reverse a string using a stack (Bonus)
fn reverse_with_stack(s: &str) -> String {
    // TODO: Bonus - reverse string using a stack
    let mut stack = Stack::new();
    
    // Push all characters
    for c in s.chars() {
        stack.push(c);
    }
    
    // Pop to get reversed order
    let mut result = String::new();
    while let Some(c) = stack.pop() {
        result.push(c);
    }
    
    result
}

/// Evaluate postfix expression (Bonus)
///
/// Example: "3 4 + 2 *" = (3+4)*2 = 14
fn evaluate_postfix(expr: &str) -> Option<i32> {
    // TODO: Bonus - evaluate postfix expression
    let mut stack = Stack::new();
    
    for token in expr.split_whitespace() {
        match token {
            "+" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a / b);
            }
            num => {
                stack.push(num.parse().ok()?);
            }
        }
    }
    
    stack.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.size(), 1); // Peek shouldn't remove
    }

    #[test]
    fn test_is_empty() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        assert!(!stack.is_empty());
        
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_balanced_parentheses() {
        assert!(is_balanced("((()))"));
        assert!(is_balanced("()()"));
        assert!(is_balanced("([{}])"));
        assert!(!is_balanced("(()"));
        assert!(!is_balanced("())("));
        assert!(!is_balanced("([)]"));
    }

    #[test]
    fn test_reverse_with_stack() {
        assert_eq!(reverse_with_stack("hello"), "olleh");
        assert_eq!(reverse_with_stack("Rust"), "tsuR");
        assert_eq!(reverse_with_stack(""), "");
    }

    #[test]
    fn test_evaluate_postfix() {
        assert_eq!(evaluate_postfix("3 4 +"), Some(7));
        assert_eq!(evaluate_postfix("5 1 2 + 4 * + 3 -"), Some(14));
        assert_eq!(evaluate_postfix("3 4 + 2 *"), Some(14));
    }
}
