// Exercise 119: Linked List - Singly Linked (Manual)
// ==================================================
//
// Learning Objective:
// Learn how to implement a singly linked list from scratch using
// Box (heap allocation) and Option (null pointer optimization).
//
// This exercise helps you understand memory management and pointers
// in Rust through a fundamental data structure.

fn main() {
    println!("=== Singly Linked List ===\n");
    
    // Create a linked list
    let mut list = LinkedList::new();
    
    println!("Pushing elements...");
    list.push(10);
    list.push(20);
    list.push(30);
    println!("List: {:?}", list);
    
    println!("\nPopping elements...");
    while let Some(val) = list.pop() {
        println!("Popped: {}", val);
    }
    println!("List after popping: {:?}", list);
    
    // Test with multiple operations
    println!("\n--- More operations ---");
    list.push(1);
    list.push(2);
    list.push(3);
    println!("List: {:?}", list);
    println!("Length: {}", list.len());
    
    println!("\n✓ Linked list completed successfully!");
}

/// Node in a singly linked list
///
/// Each node contains:
/// - data: the value stored in this node
/// - next: an Option containing a Box pointing to the next node
///
/// Box<T> is a smart pointer that allocates on the heap
/// Option<Box<Node<T>>> represents either a valid next node or None
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// Singly Linked List
///
/// The list only stores a pointer to the head node.
/// All operations traverse from the head.
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    fn new() -> Self {
        // TODO: Initialize an empty list
        LinkedList { head: None }
    }
    
    /// Push a value to the front of the list (O(1))
    ///
    /// Steps:
    /// 1. Create a new node
    /// 2. Make it point to the current head
    /// 3. Update head to point to the new node
    fn push(&mut self, data: T) {
        // TODO: Create a new node
        let new_node = Box::new(Node {
            data,
            next: self.head.take(), // Take ownership of current head
        });
        
        // TODO: Update head to point to the new node
        self.head = Some(new_node);
    }
    
    /// Pop a value from the front of the list (O(1))
    ///
    /// Steps:
    /// 1. Take ownership of the head node
    /// 2. Update head to point to the next node
    /// 3. Return the data from the removed node
    fn pop(&mut self) -> Option<T> {
        // TODO: Take the head node
        // Update head to the next node
        // Return the data
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    
    /// Get a reference to the first element (peek)
    fn peek(&self) -> Option<&T> {
        // TODO: Return a reference to the head's data without removing it
        self.head.as_ref().map(|node| &node.data)
    }
    
    /// Check if the list is empty
    fn is_empty(&self) -> bool {
        // TODO: Return true if the list is empty
        self.head.is_none()
    }
    
    /// Calculate the length of the list (O(n))
    fn len(&self) -> usize {
        // TODO: Traverse the list and count nodes
        let mut count = 0;
        let mut current = &self.head;
        
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        
        count
    }
    
    /// Reverse the list in-place (O(n))
    fn reverse(&mut self) {
        // TODO: Bonus - reverse the list
        // Use three pointers: prev, current, next
        let mut prev = None;
        let mut current = self.head.take();
        
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        
        self.head = prev;
    }
}

impl<T: Clone> LinkedList<T> {
    /// Convert list to vector (for easier testing)
    fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = &self.head;
        
        while let Some(node) = current {
            vec.push(node.data.clone());
            current = &node.next;
        }
        
        vec
    }
}

/// Implement Drop for the list (already handled by Box)
/// Box automatically frees memory when it goes out of scope

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        
        list.push(42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42)); // Verify peek didn't remove it
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        
        list.push(1);
        assert_eq!(list.len(), 1);
        
        list.push(2);
        assert_eq!(list.len(), 2);
        
        list.pop();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        list.reverse();
        
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }
}
