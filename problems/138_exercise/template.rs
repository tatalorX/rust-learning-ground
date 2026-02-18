// Exercise 138: LRU Cache Implementation
//
// Learning Objective:
// Implement a Least Recently Used (LRU) Cache using a HashMap and Doubly Linked List.
// Understand how to combine data structures for efficient operations.
//
// Key Concepts:
// - HashMap for O(1) lookup
// - Doubly Linked List for O(1) move/remove operations
// - Cache eviction policy
// - Interior mutability patterns in Rust

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Node in the doubly linked list
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

/// LRU Cache implementation
#[derive(Debug)]
struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> LRUCache<K, V> 
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// TODO: Create a new LRU Cache with given capacity
    fn new(capacity: usize) -> Self {
        // TODO: Initialize with empty HashMap and None for head/tail
        // Handle edge case: capacity == 0
        
        LRUCache {
            capacity: 0,
            cache: HashMap::new(),
            head: None,
            tail: None,
        }
    }
    
    /// TODO: Get value by key
    /// Returns Some(value) if found, None otherwise
    /// Marks the accessed item as most recently used
    fn get(&mut self, key: &K) -> Option<V> {
        // TODO: Check if key exists in cache
        // If yes, move the node to front (most recent) and return value
        // If no, return None
        
        None
    }
    
    /// TODO: Insert or update key-value pair
    /// If key exists, update value and mark as most recent
    /// If key doesn't exist, insert and evict LRU if at capacity
    fn put(&mut self, key: K, value: V) {
        // TODO: If key exists, update value and move to front
        
        // TODO: If key doesn't exist:
        //   Create new node
        //   Add to front of list
        //   Insert into HashMap
        //   If over capacity, remove LRU (tail) from both list and HashMap
    }
    
    /// TODO: Move a node to the front (most recently used)
    fn move_to_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        // TODO: Remove node from current position
        // Add node to front of list
    }
    
    /// TODO: Add a new node to the front
    fn add_to_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        // TODO: Handle empty list case
        // Link node as new head
    }
    
    /// TODO: Remove the tail node (least recently used)
    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node<K, V>>>> {
        // TODO: Remove tail node from list
        // Return the removed node
        
        None
    }
    
    /// TODO: Remove a node from its current position in the list
    fn remove_node(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        // TODO: Update prev and next pointers to skip this node
    }
    
    /// Get current size of cache
    fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Check if cache is empty
    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

/// Alternative implementation using VecDeque for simplicity
/// Note: This has O(n) operations but is simpler to understand
#[derive(Debug)]
struct SimpleLRUCache<K, V> {
    capacity: usize,
    items: Vec<(K, V)>,
}

impl<K: PartialEq + Clone, V: Clone> SimpleLRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        SimpleLRUCache {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }
    
    fn get(&mut self, key: &K) -> Option<V> {
        // TODO: Find key, remove it, and push to front
        None
    }
    
    fn put(&mut self, key: K, value: V) {
        // TODO: Remove if exists, push to front, pop back if over capacity
    }
}

fn main() {
    let mut cache = SimpleLRUCache::new(3);
    
    println!("Testing Simple LRU Cache with capacity 3\n");
    
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    println!("After inserting 1, 2, 3: {:?}", cache);
    
    cache.put(4, "four"); // Should evict 1
    println!("After inserting 4 (evicts 1): {:?}", cache);
    
    if let Some(val) = cache.get(&2) {
        println!("Get 2: {} (moves to front)", val);
    }
    println!("Cache after get(2): {:?}", cache);
    
    cache.put(5, "five"); // Should evict 3 (not 2, since 2 was accessed)
    println!("After inserting 5 (evicts 3): {:?}", cache);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lru_basic() {
        let mut cache = SimpleLRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(&1), Some(1));
        cache.put(3, 3); // Evicts key 2
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(3));
    }
    
    #[test]
    fn test_lru_update() {
        let mut cache = SimpleLRUCache::new(2);
        cache.put(1, 1);
        cache.put(1, 10); // Update existing
        assert_eq!(cache.get(&1), Some(10));
    }
    
    #[test]
    fn test_lru_eviction_order() {
        let mut cache = SimpleLRUCache::new(3);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.get(&1); // Access 1, making it most recent
        cache.put(4, 4); // Should evict 2
        assert_eq!(cache.get(&1), Some(1));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(3));
        assert_eq!(cache.get(&4), Some(4));
    }
    
    #[test]
    fn test_lru_empty() {
        let cache = SimpleLRUCache::<i32, i32>::new(2);
        assert!(cache.is_empty());
    }
    
    #[test]
    fn test_lru_single_capacity() {
        let mut cache = SimpleLRUCache::new(1);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(2));
    }
}
