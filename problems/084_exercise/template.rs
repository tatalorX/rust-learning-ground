// Exercise 084: Channels - mpsc Basics
//
// Learning objective: Learn the multi-producer single-consumer (mpsc)
// channel pattern for message passing between threads.
//
// Channels allow threads to communicate by sending messages.
// mpsc = multiple producers, single consumer.
// tx = transmitter (sender), rx = receiver

use std::sync::mpsc;
use std::thread;

fn main() {
    // TODO: Create a channel that transmits String messages
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = // YOUR CODE HERE

    // TODO: Spawn a thread that sends a message
    let handle = thread::spawn(move || {
        let message = String::from("Hello from sender!");
        // TODO: Send the message using tx
        // YOUR CODE HERE
        println!("Message sent!");
    });

    // TODO: Receive the message in the main thread using rx
    let received = // YOUR CODE HERE
    println!("Received: {}", received);

    handle.join().unwrap();

    // TODO: Send multiple messages from a thread
    let (tx, rx) = mpsc::channel::<i32>();
    
    thread::spawn(move || {
        for i in 1..=5 {
            // TODO: Send each number
            // YOUR CODE HERE
            println!("Sent: {}", i);
        }
    });

    // TODO: Receive all messages
    // Hint: recv() blocks until a message is available
    for _ in 1..=5 {
        let received = // YOUR CODE HERE
        println!("Received: {}", received);
    }

    // TODO: Demonstrate that tx can be cloned for multiple senders
    let (tx, rx) = mpsc::channel::<String>();
    
    // Clone tx for a second sender
    let tx2 = tx.clone();
    
    thread::spawn(move || {
        tx.send(String::from("From thread 1")).unwrap();
    });
    
    thread::spawn(move || {
        tx2.send(String::from("From thread 2")).unwrap();
    });
    
    // TODO: Receive both messages
    // Note: Order is not guaranteed!
    for _ in 0..2 {
        println!("Received: {}", rx.recv().unwrap());
    }
}

// TODO: Complete this function that spawns a worker thread that sends results
fn spawn_worker<T>(data: T) -> mpsc::Receiver<String>
where
    T: Send + 'static + std::fmt::Display,
{
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        // TODO: Send a formatted message with the data
        let msg = format!("Processed: {}", data);
        // YOUR CODE HERE
    });
    
    rx
}

// TODO: Complete this function that creates a channel for i32 and returns sender
fn create_i32_channel() -> mpsc::Sender<i32> {
    // YOUR CODE HERE - create channel and return just the sender
}

// TODO: Create a struct that uses channels internally
struct MessageSystem {
    sender: mpsc::Sender<String>,
}

impl MessageSystem {
    fn new() -> (Self, mpsc::Receiver<String>) {
        // YOUR CODE HERE
    }
    
    fn send(&self, msg: String) {
        // YOUR CODE HERE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_channel() {
        let (tx, rx) = mpsc::channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_spawn_worker() {
        let rx = spawn_worker(100);
        assert_eq!(rx.recv().unwrap(), "Processed: 100");
    }

    #[test]
    fn test_message_system() {
        let (system, rx) = MessageSystem::new();
        system.send(String::from("test"));
        assert_eq!(rx.recv().unwrap(), "test");
    }
}
