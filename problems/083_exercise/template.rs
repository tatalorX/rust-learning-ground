// Exercise 083: Threads - Move Closures
//
// Learning objective: Understand how to move data into threads
// using move closures and the ownership rules involved.
//
// The `move` keyword transfers ownership of variables into the closure.
// This is essential when spawning threads because the new thread
// may outlive the current scope.

use std::thread;

fn main() {
    // TODO: Create a String and move it into a thread
    let message = String::from("Hello from thread!");
    
    let handle = thread::spawn(/* TODO: Add move keyword */ || {
        // This should work because we moved ownership
        println!("{}", message);
    });
    
    handle.join().unwrap();
    // TODO: Try to use message here and see the compiler error
    // Then comment it out
    // println!("{}", message); // This would fail!

    // TODO: Create a vector and move it into a thread
    let numbers = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        // TODO: Calculate and return the sum of numbers
        numbers.iter().sum::<i32>()
    });
    
    let result = handle.join().unwrap();
    println!("Sum from thread: {}", result);

    // TODO: Demonstrate moving multiple values
    let name = String::from("Alice");
    let age = 30;
    let city = String::from("New York");
    
    let handle = thread::spawn(move || {
        // TODO: Print "Name: {name}, Age: {age}, City: {city}"
        // YOUR CODE HERE
    });
    handle.join().unwrap();

    // TODO: Show what happens without move when data doesn't live long enough
    // This requires careful handling - uncomment to see error, then fix with move
    // let data = String::from("temporary");
    // let handle = thread::spawn(|| {
    //     println!("{}", data); // Error: data may not live long enough
    // });

    // TODO: Create multiple threads, each getting its own clone of data
    let shared_message = String::from("Shared");
    let mut handles = vec![];
    
    for i in 0..3 {
        // TODO: Clone the message for each thread
        let msg = // YOUR CODE HERE
        let handle = thread::spawn(move || {
            println!("Thread {}: {}", i, msg);
        });
        handles.push(handle);
    }
    
    for h in handles {
        h.join().unwrap();
    }
    println!("Original message still available: {}", shared_message);
}

// TODO: Complete this function that takes ownership of data and spawns a thread
fn spawn_with_data<T, F>(data: T, f: F) -> thread::JoinHandle<()>
where
    T: Send + 'static,
    F: FnOnce(T) + Send + 'static,
{
    // YOUR CODE HERE - spawn a thread that applies f to data
}

// TODO: Complete this function that moves a vector into a thread for processing
fn process_vector_in_thread<F>(vec: Vec<i32>, processor: F) -> thread::JoinHandle<Vec<i32>>
where
    F: FnOnce(Vec<i32>) -> Vec<i32> + Send + 'static,
{
    // YOUR CODE HERE
}

// TODO: Complete this struct that owns data and spawns worker threads
struct Worker {
    data: String,
}

impl Worker {
    fn new(data: String) -> Self {
        Worker { data }
    }
    
    fn spawn_work(self) -> thread::JoinHandle<String> {
        // TODO: Spawn a thread that moves self and returns the data
        // YOUR CODE HERE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_with_data() {
        let data = vec![1, 2, 3];
        let handle = spawn_with_data(data, |d| {
            assert_eq!(d.len(), 3);
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_process_vector() {
        let vec = vec![1, 2, 3];
        let handle = process_vector_in_thread(vec, |mut v| {
            v.push(4);
            v
        });
        assert_eq!(handle.join().unwrap(), vec![1, 2, 3, 4]);
    }
}
