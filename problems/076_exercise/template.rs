// Exercise 076: Box<T> - Heap Allocation
//
// Learning objective: Understand how to use Box<T> for heap allocation
// and when it's useful for storing data on the heap.
//
// Box<T> is a smart pointer that allows you to store data on the heap
// rather than the stack. It's useful when:
// - You have a large amount of data and want to move ownership without copying
// - You want to own a value but don't know its size at compile time
// - You want to ensure a type implements a specific trait (trait objects)

fn main() {
    // TODO: Create a Box that holds an i32 with value 42
    // Hint: Use Box::new()
    let boxed_value: Box<i32> = // YOUR CODE HERE

    // TODO: Print the boxed value using dereferencing
    // The value should print: "The boxed value is: 42"
    println!("The boxed value is: {}", /* YOUR CODE HERE */);

    // TODO: Create a function that takes ownership of a Box<i32>
    // and returns the value inside it
    let inner_value = unbox_value(boxed_value);
    println!("Inner value: {}", inner_value);

    // TODO: Demonstrate that Box allows moving large data without copying
    // Create a large array on the heap using Box
    let large_array: Box<[i32; 1000]> = // YOUR CODE HERE
    println!("Large array created on heap, first element: {}", large_array[0]);

    // TODO: Show that Box is automatically dereferenced when needed
    // Use the * operator to dereference and get the value
    let first_element = /* YOUR CODE HERE */
    println!("First element via dereference: {}", first_element);
}

// TODO: Complete this function that takes a Box<i32> and returns the i32 value
fn unbox_value(b: Box<i32>) -> i32 {
    // YOUR CODE HERE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbox_value() {
        let b = Box::new(100);
        assert_eq!(unbox_value(b), 100);
    }
}
