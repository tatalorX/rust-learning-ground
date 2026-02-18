// Exercise 121: Queue Implementation
// ===================================
//
// Learning Objective:
// Learn how to implement a Queue data structure using VecDeque.
// A queue follows FIFO (First In, First Out) ordering.
//
// Operations:
// - enqueue: Add element to rear (O(1))
// - dequeue: Remove and return front element (O(1))
// - peek: View front element without removing (O(1))

fn main() {
    println!("=== Queue Implementation ===\n");
    
    let mut queue = Queue::new();
    
    println!("Enqueuing: Alice, Bob, Charlie");
    queue.enqueue("Alice");
    queue.enqueue("Bob");
    queue.enqueue("Charlie");
    
    println!("Queue size: {}", queue.size());
    println!("Front: {:?}", queue.peek());
    
    println!("\nDequeuing:");
    while let Some(person) = queue.dequeue() {
        println!("  Dequeued: {}", person);
    }
    
    println!("\nIs empty? {}", queue.is_empty());
    
    // Demonstrate practical use: BFS level order
    println!("\n=== BFS Simulation ===");
    simulate_bfs();
    
    println!("\n✓ Queue implementation completed successfully!");
}

/// Queue data structure
///
/// Uses VecDeque internally for efficient push/pop at both ends.
/// The front of the queue is the front of the VecDeque.
use std::collections::VecDeque;

#[derive(Debug)]
struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    fn new() -> Self {
        // TODO: Initialize an empty queue
        Queue {
            items: VecDeque::new(),
        }
    }
    
    /// Add an item to the rear of the queue
    ///
    /// Time Complexity: O(1)
    fn enqueue(&mut self, item: T) {
        // TODO: Add item to the back of the queue
        self.items.push_back(item);
    }
    
    /// Remove and return the front item
    ///
    /// Returns None if the queue is empty.
    /// Time Complexity: O(1)
    fn dequeue(&mut self) -> Option<T> {
        // TODO: Remove and return the front item
        self.items.pop_front()
    }
    
    /// View the front item without removing it
    ///
    /// Returns None if the queue is empty.
    /// Time Complexity: O(1)
    fn peek(&self) -> Option<&T> {
        // TODO: Return a reference to the front item
        self.items.front()
    }
    
    /// Check if the queue is empty
    fn is_empty(&self) -> bool {
        // TODO: Return true if queue is empty
        self.items.is_empty()
    }
    
    /// Get the size of the queue
    fn size(&self) -> usize {
        // TODO: Return the number of items
        self.items.len()
    }
}

/// Simulate BFS using a queue
///
/// This demonstrates a practical use case for queues.
fn simulate_bfs() {
    // TODO: Implement BFS traversal simulation
    // We'll traverse a simple tree:
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<usize>, // Indices of children
    }
    
    let tree = vec![
        Node { value: 1, children: vec![1, 2] },
        Node { value: 2, children: vec![3, 4] },
        Node { value: 3, children: vec![] },
        Node { value: 4, children: vec![] },
        Node { value: 5, children: vec![] },
    ];
    
    let mut queue = Queue::new();
    queue.enqueue(0); // Start with root node index
    
    println!("BFS traversal order:");
    while let Some(node_idx) = queue.dequeue() {
        let node = &tree[node_idx];
        println!("  Node: {}", node.value);
        
        // Enqueue children
        for &child_idx in &node.children {
            queue.enqueue(child_idx);
        }
    }
}

/// Circular Queue implementation (Bonus)
///
/// A fixed-size queue that reuses space.
#[derive(Debug)]
struct CircularQueue<T> {
    // TODO: Bonus - implement circular queue
    buffer: Vec<Option<T>>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}

impl<T: Clone> CircularQueue<T> {
    fn new(capacity: usize) -> Self {
        CircularQueue {
            buffer: vec![None; capacity],
            front: 0,
            rear: 0,
            size: 0,
            capacity,
        }
    }
    
    fn enqueue(&mut self, item: T) -> bool {
        // TODO: Implement circular queue enqueue
        if self.size == self.capacity {
            return false; // Queue is full
        }
        
        self.buffer[self.rear] = Some(item);
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
        true
    }
    
    fn dequeue(&mut self) -> Option<T> {
        // TODO: Implement circular queue dequeue
        if self.size == 0 {
            return None;
        }
        
        let item = self.buffer[self.front].clone();
        self.buffer[self.front] = None;
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        item
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_queue_fifo_order() {
        let mut queue = Queue::new();
        queue.enqueue("first");
        queue.enqueue("second");
        queue.enqueue("third");
        
        assert_eq!(queue.dequeue(), Some("first"));
        assert_eq!(queue.peek(), Some(&"second"));
        assert_eq!(queue.dequeue(), Some("second"));
        assert_eq!(queue.dequeue(), Some("third"));
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = Queue::new();
        assert_eq!(queue.peek(), None);
        
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&42));
        assert_eq!(queue.size(), 1); // Peek shouldn't remove
    }

    #[test]
    fn test_queue_is_empty() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        
        queue.enqueue(1);
        assert!(!queue.is_empty());
        
        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_circular_queue() {
        let mut cq = CircularQueue::new(3);
        assert!(cq.is_empty());
        
        assert!(cq.enqueue(1));
        assert!(cq.enqueue(2));
        assert!(cq.enqueue(3));
        assert!(!cq.enqueue(4)); // Full
        
        assert_eq!(cq.dequeue(), Some(1));
        assert!(cq.enqueue(4)); // Now there's space
        
        assert_eq!(cq.dequeue(), Some(2));
        assert_eq!(cq.dequeue(), Some(3));
        assert_eq!(cq.dequeue(), Some(4));
        assert_eq!(cq.dequeue(), None);
    }
}
