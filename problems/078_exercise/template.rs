// Exercise 078: Rc<T> - Reference Counting
//
// Learning objective: Understand Rc<T> for shared ownership
// and how reference counting works.
//
// Rc<T> (Reference Counted) allows multiple owners of the same data.
// It keeps track of the number of references and only cleans up
// when the last reference is dropped.
// Note: Rc<T> is only for single-threaded scenarios.

use std::rc::Rc;

fn main() {
    // TODO: Create an Rc containing a String "shared data"
    let data: Rc<String> = // YOUR CODE HERE
    println!("Reference count after creation: {}", Rc::strong_count(&data));

    // TODO: Create two more references to the same data using clone()
    // Rc::clone() only increments the reference count, not deep copy
    let data2: Rc<String> = // YOUR CODE HERE
    println!("Reference count after first clone: {}", Rc::strong_count(&data));

    let data3: Rc<String> = // YOUR CODE HERE
    println!("Reference count after second clone: {}", Rc::strong_count(&data));

    // TODO: Print the string from all three references
    println!("data: {}", data);
    println!("data2: {}", data2);
    println!("data3: {}", data3);

    // TODO: Create a scope and demonstrate that reference count decreases
    // when references go out of scope
    {
        let data4 = Rc::clone(&data);
        println!("Reference count inside scope: {}", Rc::strong_count(&data));
        // data4 will be dropped here
    }
    println!("Reference count after scope: {}", Rc::strong_count(&data));

    // TODO: Create a function that takes Rc<String> and prints it
    // Call it with data, data2, and data3
    // YOUR CODE HERE

    // TODO: Demonstrate that all references point to the same data
    // by comparing pointers using Rc::ptr_eq()
    println!("data and data2 point to same data: {}", 
             Rc::ptr_eq(&data, &data2));
}

// TODO: Complete this function that accepts an Rc<String> and prints it
fn print_shared(data: Rc<String>) {
    // YOUR CODE HERE
}

// TODO: Complete this function that returns an Rc<String> containing "hello"
fn create_shared_hello() -> Rc<String> {
    // YOUR CODE HERE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_counting() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);
        
        let _data2 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        
        {
            let _data3 = Rc::clone(&data);
            assert_eq!(Rc::strong_count(&data), 3);
        }
        
        assert_eq!(Rc::strong_count(&data), 2);
    }

    #[test]
    fn test_create_shared_hello() {
        let hello = create_shared_hello();
        assert_eq!(*hello, "hello");
    }
}
