// Exercise 082: Threads - Spawn and Join
//
// Learning objective: Learn how to create threads and wait for them
// to complete using spawn() and join().
//
// Threads allow concurrent execution. std::thread::spawn creates a new
// thread, and join() waits for it to finish.

use std::thread;
use std::time::Duration;

fn main() {
    // TODO: Spawn a thread that prints "Hello from spawned thread!"
    let handle = thread::spawn(|| {
        // YOUR CODE HERE
    });

    println!("Hello from main thread!");

    // TODO: Wait for the spawned thread to complete
    // YOUR CODE HERE

    // TODO: Spawn multiple threads that each print their thread number
    let mut handles = vec![];
    
    for i in 1..=5 {
        // Spawn a thread that prints "Thread N starting" and "Thread N finished"
        let handle = thread::spawn(move || {
            // YOUR CODE HERE
            println!("Thread {} starting", i);
            // Simulate some work
            thread::sleep(Duration::from_millis(100));
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }

    // TODO: Wait for all threads to complete
    // YOUR CODE HERE

    println!("All threads completed!");

    // TODO: Demonstrate that threads run concurrently
    // Spawn 3 threads that sleep for different durations
    // and observe they don't necessarily complete in order
    let mut handles = vec![];
    
    for i in 1..=3 {
        let handle = thread::spawn(move || {
            let duration = Duration::from_millis((4 - i) * 100);
            println!("Thread {} sleeping for {:?}", i, duration);
            thread::sleep(duration);
            println!("Thread {} done!", i);
        });
        handles.push(handle);
    }
    
    // TODO: Wait for all
    // YOUR CODE HERE
}

// TODO: Complete this function that spawns a thread doing some computation
fn spawn_computation<F>(f: F) -> thread::JoinHandle<i32>
where
    F: FnOnce() -> i32 + Send + 'static,
{
    // YOUR CODE HERE
}

// TODO: Complete this function that creates N threads and returns their handles
fn spawn_n_threads(n: usize) -> Vec<thread::JoinHandle<()>> {
    let mut handles = vec![];
    // YOUR CODE HERE
    // Each thread should print "Worker N" where N is the thread index
    handles
}

// TODO: Complete this function that spawns threads to sum chunks of a vector
fn parallel_sum(data: Vec<i32>, num_threads: usize) -> i32 {
    // This is a simplified version - just spawn threads and sum
    // For now, just return the sum (challenge: actually parallelize it!)
    // YOUR CODE HERE - Hint: split work among threads
    data.iter().sum() // Placeholder - replace with parallel version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_computation() {
        let handle = spawn_computation(|| 42);
        assert_eq!(handle.join().unwrap(), 42);
    }

    #[test]
    fn test_spawn_n_threads() {
        let handles = spawn_n_threads(3);
        assert_eq!(handles.len(), 3);
        for h in handles {
            h.join().unwrap();
        }
    }
}
