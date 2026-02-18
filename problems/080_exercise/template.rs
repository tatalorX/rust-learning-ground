// Exercise 080: Arc<T> - Atomic Reference Counting
//
// Learning objective: Learn Arc<T> for thread-safe shared ownership
// and understand when to use Arc vs Rc.
//
// Arc<T> (Atomically Reference Counted) is like Rc<T> but thread-safe.
// It uses atomic operations for reference counting, which has a small
// performance cost but allows sharing data across threads.

use std::sync::Arc;
use std::thread;

fn main() {
    // TODO: Create an Arc containing a String "shared across threads"
    let data: Arc<String> = // YOUR CODE HERE
    println!("Initial reference count: {}", Arc::strong_count(&data));

    // TODO: Create a vector to hold thread handles
    let mut handles = vec![];

    // TODO: Spawn 5 threads, each getting a clone of the Arc
    for i in 0..5 {
        // Clone the Arc for this thread
        let thread_data: Arc<String> = // YOUR CODE HERE
        
        println!("Before spawn {}: reference count = {}", 
                 i, Arc::strong_count(&data));
        
        let handle = thread::spawn(move || {
            // TODO: Print the string with thread number
            println!("Thread {} sees: {}", i, /* YOUR CODE HERE */);
            
            // TODO: Print reference count from within thread
            // Note: This is the count within this thread's context
            println!("Thread {} reference count: {}", i, 
                     Arc::strong_count(&thread_data));
        });
        
        handles.push(handle);
    }

    println!("After spawns: reference count = {}", Arc::strong_count(&data));

    // TODO: Wait for all threads to complete
    // YOUR CODE HERE

    println!("After joins: reference count = {}", Arc::strong_count(&data));

    // TODO: Create an Arc<Vec<i32>> and share it across threads
    let numbers: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3, 4, 5]);
    
    // Spawn threads that sum different portions of the vector
    // (For simplicity, just print the sum in each thread)
    let mut sum_handles = vec![];
    
    for thread_id in 0..3 {
        let nums = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let sum: i32 = nums.iter().sum();
            println!("Thread {} calculated sum: {}", thread_id, sum);
        });
        sum_handles.push(handle);
    }
    
    // TODO: Wait for sum threads
    // YOUR CODE HERE
}

// TODO: Complete this function that creates an Arc<T> from a value
fn make_shared<T>(value: T) -> Arc<T> {
    // YOUR CODE HERE
}

// TODO: Complete this function that spawns a thread with shared data
fn spawn_with_data<T>(data: Arc<T>) -> thread::JoinHandle<()>
where
    T: Send + Sync + 'static + std::fmt::Debug,
{
    // YOUR CODE HERE
    // The thread should print "Data: {:?}" with the data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_counting() {
        let data = Arc::new(42);
        assert_eq!(Arc::strong_count(&data), 1);
        
        let _data2 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);
    }

    #[test]
    fn test_make_shared() {
        let shared = make_shared(String::from("hello"));
        assert_eq!(*shared, "hello");
    }
}
