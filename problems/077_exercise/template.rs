// Exercise 077: Box<T> - Recursive Types
//
// Learning objective: Learn how Box<T> enables recursive types
// and understand the Cons list pattern.
//
// Recursive types like linked lists cannot be sized at compile time
// because they contain themselves. Box<T> provides indirection,
// allowing the compiler to know the size (just a pointer).

// TODO: Define an enum called List that represents a cons list
// It should have two variants:
// - Cons: contains an i32 and a Box<List>
// - Nil: represents the end of the list
enum List {
    // YOUR CODE HERE
}

impl List {
    // TODO: Implement a new() function that returns an empty list (Nil)
    fn new() -> List {
        // YOUR CODE HERE
    }

    // TODO: Implement a prepend() method that takes an i32 and returns
    // a new list with the element added to the front
    fn prepend(self, elem: i32) -> List {
        // YOUR CODE HERE
    }

    // TODO: Implement a len() method that returns the length of the list
    fn len(&self) -> usize {
        // YOUR CODE HERE
        // Hint: Use pattern matching and recursion
    }

    // TODO: Implement a stringify() method that converts the list to a string
    fn stringify(&self) -> String {
        // YOUR CODE HERE
        // Format: "1, 2, 3, Nil"
    }
}

fn main() {
    // TODO: Create an empty list
    let list = List::new();
    println!("Empty list length: {}", list.len());

    // TODO: Create a list: 1 -> 2 -> 3 -> Nil
    // Use prepend to build it (prepend adds to front, so prepend in reverse order)
    let list = list
        // YOUR CODE HERE - prepend 3, then 2, then 1
    println!("List: {}", list.stringify());
    println!("List length: {}", list.len());

    // TODO: Create a longer list and verify its length
    let long_list = List::new()
        .prepend(10)
        .prepend(20)
        .prepend(30)
        .prepend(40)
        .prepend(50);
    println!("Long list: {}", long_list.stringify());
    println!("Long list length: {}", long_list.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_len() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_empty_list() {
        let list = List::new();
        assert_eq!(list.len(), 0);
    }
}
