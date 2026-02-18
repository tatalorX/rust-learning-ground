// Exercise 085: Channels - Sending Multiple Values
//
// Learning objective: Learn patterns for sending and receiving
// multiple values efficiently using iterators and drop.
//
// Channels can be used with for loops for receiving, and the
// sender can be dropped to signal completion.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // TODO: Create a channel and send values using an iterator pattern
    let (tx, rx) = mpsc::channel::<i32>();
    
    thread::spawn(move || {
        let values = vec![1, 2, 3, 4, 5];
        
        // TODO: Use a for loop to send all values
        for val in values {
            // YOUR CODE HERE
            println!("Sent: {}", val);
            thread::sleep(Duration::from_millis(100));
        }
        // Channel is closed when tx is dropped (automatic at end of scope)
        println!("Sender dropped, channel closed");
    });

    // TODO: Receive values as an iterator
    // The loop will automatically end when the channel is closed
    println!("Receiving values:");
    for received in /* YOUR CODE HERE */ {
        println!("Got: {}", received);
    }
    println!("Channel closed, no more values");

    // TODO: Demonstrate try_recv() for non-blocking receive
    let (tx, rx) = mpsc::channel::<String>();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send(String::from("Delayed message")).unwrap();
    });
    
    // TODO: Use try_recv() in a loop until message arrives
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received: {}", msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("No message yet, doing other work...");
                thread::sleep(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Sender disconnected!");
                break;
            }
        }
    }

    // TODO: Create a channel that sends computed results
    let (tx, rx) = mpsc::channel();
    
    // Spawn multiple workers that compute squares
    for i in 1..=5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let result = i * i;
            tx_clone.send((i, result)).unwrap();
        });
    }
    
    // Drop the original tx so the channel closes when all clones are done
    drop(tx);
    
    // TODO: Collect all results into a vector
    let mut results = vec![];
    for (input, square) in rx {
        results.push((input, square));
        println!("{}^2 = {}", input, square);
    }
    
    // Sort by input number
    results.sort_by_key(|(n, _)| *n);
    println!("All results: {:?}", results);
}

// TODO: Complete this function that computes squares in parallel
fn parallel_squares(numbers: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn a thread for each number
    for n in numbers {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            // TODO: Send the square of n
            // YOUR CODE HERE
        });
        handles.push(handle);
    }
    
    // TODO: Drop the original sender
    // YOUR CODE HERE
    
    // TODO: Collect all results
    let mut results = vec![];
    // YOUR CODE HERE
    
    // Wait for all threads
    for h in handles {
        h.join().unwrap();
    }
    
    results.sort();
    results
}

// TODO: Complete this function that creates a bounded channel
fn create_bounded_channel<T>(capacity: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    // Note: Standard library doesn't have bounded channels
    // Return a regular channel for now
    mpsc::channel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_receive() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for i in 1..=3 {
                tx.send(i).unwrap();
            }
        });
        
        let received: Vec<i32> = rx.iter().collect();
        assert_eq!(received, vec![1, 2, 3]);
    }

    #[test]
    fn test_parallel_squares() {
        let numbers = vec![1, 2, 3, 4, 5];
        let results = parallel_squares(numbers);
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }
}
