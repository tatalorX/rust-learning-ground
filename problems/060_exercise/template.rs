// Exercise 060: Generic Structs
//
// Generic structs can store and work with values of any type.
// This allows for reusable data structures like Box, Vec, and Option.
//
// Learning Objectives:
// - Define generic structs with type parameters
// - Implement methods on generic structs
// - Use multiple type parameters
//
// Your task: Implement generic data structures using structs.

/// A generic container that holds a single value.
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    /// Creates a new Container with the given value.
    fn new(value: T) -> Self {
        // TODO: Create and return a Container with the given value
        todo!()
    }
    
    /// Returns a reference to the contained value.
    fn get(&self) -> &T {
        // TODO: Return a reference to the value
        todo!()
    }
    
    /// Replaces the contained value and returns the old one.
    fn set(&mut self, value: T) -> T {
        // TODO: Replace self.value with the new value and return the old one
        // Hint: Use std::mem::replace
        todo!()
    }
}

/// A generic Key-Value pair.
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    /// Creates a new KeyValue pair.
    fn new(key: K, value: V) -> Self {
        // TODO: Create and return a KeyValue
        todo!()
    }
    
    /// Returns references to both key and value.
    fn get(&self) -> (&K, &V) {
        // TODO: Return a tuple of references to key and value
        todo!()
    }
    
    /// Swaps the key and value (only works when K and V are the same type).
    fn swap(self) -> KeyValue<V, K> {
        // TODO: Return a new KeyValue with key and value swapped
        todo!()
    }
}

/// A generic stack data structure.
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    fn new() -> Self {
        // TODO: Create a new empty Stack
        todo!()
    }
    
    /// Pushes an item onto the stack.
    fn push(&mut self, item: T) {
        // TODO: Push the item onto the stack
        todo!()
    }
    
    /// Pops an item from the stack.
    fn pop(&mut self) -> Option<T> {
        // TODO: Pop and return the last item, or None if empty
        todo!()
    }
    
    /// Returns true if the stack is empty.
    fn is_empty(&self) -> bool {
        // TODO: Return whether the stack is empty
        todo!()
    }
    
    /// Returns the number of items in the stack.
    fn len(&self) -> usize {
        // TODO: Return the length of the stack
        todo!()
    }
    
    /// Returns a reference to the top item without removing it.
    fn peek(&self) -> Option<&T> {
        // TODO: Return a reference to the last item, or None if empty
        todo!()
    }
}

fn main() {
    // Test Container
    let mut container = Container::new(42);
    println!("Container value: {}", container.get());
    let old = container.set(100);
    println!("Old value: {}, New value: {}", old, container.get());
    
    // Test Container with different types
    let string_container = Container::new("hello");
    println!("String container: {}", string_container.get());
    
    // Test KeyValue
    let kv = KeyValue::new("name", "Alice");
    let (k, v) = kv.get();
    println!("\nKeyValue: {} = {}", k, v);
    
    let number_kv = KeyValue::new(1, 100);
    let swapped = number_kv.swap();
    let (new_k, new_v) = swapped.get();
    println!("Swapped: {} = {}", new_k, new_v);
    
    // Test Stack
    let mut stack = Stack::new();
    println!("\nStack operations:");
    println!("  Is empty? {}", stack.is_empty());
    
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("  Pushed 10, 20, 30. Length: {}", stack.len());
    println!("  Peek: {:?}", stack.peek());
    
    while let Some(item) = stack.pop() {
        println!("  Popped: {}", item);
    }
    println!("  Is empty? {}", stack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_new() {
        let c = Container::new(42);
        assert_eq!(c.get(), &42);
    }

    #[test]
    fn test_container_set() {
        let mut c = Container::new(42);
        let old = c.set(100);
        assert_eq!(old, 42);
        assert_eq!(c.get(), &100);
    }

    #[test]
    fn test_keyvalue_new() {
        let kv = KeyValue::new("key", "value");
        assert_eq!(kv.get(), (&"key", &"value"));
    }

    #[test]
    fn test_keyvalue_swap() {
        let kv = KeyValue::new(1, "one");
        let swapped = kv.swap();
        assert_eq!(swapped.get(), (&"one", &1));
    }

    #[test]
    fn test_stack_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(10);
        assert_eq!(stack.peek(), Some(&10));
        assert_eq!(stack.len(), 1); // peek doesn't remove
    }
}
