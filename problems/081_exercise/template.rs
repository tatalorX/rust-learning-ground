// Exercise 081: Mutex<T> - Mutual Exclusion
//
// Learning objective: Learn Mutex<T> for thread-safe mutable access
// and understand how to properly lock/unlock shared data.
//
// Mutex<T> (Mutual Exclusion) ensures only one thread can access
// data at a time. It provides interior mutability like RefCell,
// but is thread-safe. Combine with Arc<T> to share across threads.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create an Arc<Mutex<i32>> initialized to 0
    // This will be our shared counter
    let counter = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to hold thread handles
    let mut handles = vec![];

    // TODO: Spawn 10 threads that each increment the counter 10 times
    for _ in 0..10 {
        // Clone the Arc for this thread
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // TODO: Lock the mutex, increment the value, and unlock
                // Hint: The lock returns a Result containing a MutexGuard
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                // MutexGuard automatically unlocks when dropped
            }
        });
        
        handles.push(handle);
    }

    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the final counter value
    // It should be 100 (10 threads * 10 increments)
    println!("Final counter: {}", /* YOUR CODE HERE */);

    // TODO: Demonstrate Mutex with more complex data
    // Create a Mutex<Vec<i32>> and have multiple threads push to it
    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    let mut vec_handles = vec![];
    
    for i in 0..5 {
        let vec_clone = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            // TODO: Lock the mutex and push i to the vector
            // YOUR CODE HERE
        });
        vec_handles.push(handle);
    }
    
    // TODO: Wait for all vector threads and print the result
    // YOUR CODE HERE
}

// TODO: Complete this function that safely increments a shared counter
fn increment_shared_counter(counter: &Arc<Mutex<i32>>) {
    // YOUR CODE HERE
}

// TODO: Complete this struct that uses Mutex for thread-safe state
struct ThreadSafeCounter {
    count: Mutex<i32>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        // YOUR CODE HERE
    }
    
    fn increment(&self) {
        // YOUR CODE HERE
    }
    
    fn get(&self) -> i32 {
        // YOUR CODE HERE
    }
}

// TODO: Implement a bank account with Mutex for thread-safe transactions
struct ThreadSafeAccount {
    balance: Mutex<i64>,
}

impl ThreadSafeAccount {
    fn new(initial_balance: i64) -> Self {
        // YOUR CODE HERE
    }
    
    fn deposit(&self, amount: i64) -> Result<(), &'static str> {
        // YOUR CODE HERE - return Err if amount is negative
    }
    
    fn withdraw(&self, amount: i64) -> Result<(), &'static str> {
        // YOUR CODE HERE - return Err if amount > balance or negative
    }
    
    fn balance(&self) -> i64 {
        // YOUR CODE HERE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_basic() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        assert_eq!(*m.lock().unwrap(), 6);
    }

    #[test]
    fn test_thread_safe_counter() {
        let counter = Arc::new(ThreadSafeCounter::new());
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                counter_clone.increment();
            }
        });
        
        for _ in 0..10 {
            counter.increment();
        }
        
        handle.join().unwrap();
        assert_eq!(counter.get(), 20);
    }
}
