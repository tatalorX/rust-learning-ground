// Exercise 079: RefCell<T> - Interior Mutability
//
// Learning objective: Understand interior mutability pattern with RefCell<T>
// and the difference between compile-time and runtime borrowing.
//
// RefCell<T> allows mutable borrowing at runtime (checked, not unsafe).
// Unlike Box<T> or Rc<T>, RefCell<T> enforces borrowing rules at runtime,
// not compile time. This allows mutation even when RefCell is immutable.
//
// IMPORTANT: Still single-threaded only! For multi-threaded, use Mutex<T>.

use std::cell::RefCell;

fn main() {
    // TODO: Create a RefCell containing a Vec<i32> with [1, 2, 3]
    let data: RefCell<Vec<i32>> = // YOUR CODE HERE

    // TODO: Even though `data` is not mutable, we can mutate the contents!
    // Use borrow_mut() to get a mutable reference and push 4
    // YOUR CODE HERE

    // TODO: Use borrow() to get an immutable reference and print the vector
    println!("Vector contents: {:?}", /* YOUR CODE HERE */);

    // TODO: Mutate again - push 5
    // YOUR CODE HERE

    // TODO: Print the final state
    println!("Final vector: {:?}", data.borrow());

    // TODO: Demonstrate runtime borrow checking
    // This is OK - multiple immutable borrows
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("Two immutable borrows work: {:?}, {:?}", borrow1, borrow2);
    drop(borrow1);
    drop(borrow2);

    // TODO: Show that we can't have mutable and immutable borrows at same time
    // Uncomment and run to see the panic (then comment back)
    // let mut_borrow = data.borrow_mut();
    // let imm_borrow = data.borrow(); // This will panic at runtime!

    // TODO: Create a function that takes &RefCell<i32> and increments it
    let counter = RefCell::new(0);
    increment_counter(&counter);
    increment_counter(&counter);
    println!("Counter after increments: {}", counter.borrow());
}

// TODO: Complete this function that increments a RefCell<i32>
// Note: The parameter is an immutable reference, but we can still mutate!
fn increment_counter(counter: &RefCell<i32>) {
    // YOUR CODE HERE
}

// TODO: Complete this function that appends a value to a RefCell<Vec<i32>>
fn append_to_vec(vec: &RefCell<Vec<i32>>, value: i32) {
    // YOUR CODE HERE
}

// TODO: Create a struct with RefCell fields to allow interior mutability
struct BankAccount {
    // The balance should be a RefCell<i32> so we can modify it
    // even when the BankAccount is borrowed immutably
    balance: RefCell<i32>,
}

impl BankAccount {
    fn new(initial_balance: i32) -> Self {
        // YOUR CODE HERE
    }

    fn deposit(&self, amount: i32) {
        // YOUR CODE HERE - should be able to modify balance even with &self
    }

    fn get_balance(&self) -> i32 {
        // YOUR CODE HERE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_counter() {
        let counter = RefCell::new(5);
        increment_counter(&counter);
        assert_eq!(*counter.borrow(), 6);
    }

    #[test]
    fn test_bank_account() {
        let account = BankAccount::new(100);
        assert_eq!(account.get_balance(), 100);
        
        account.deposit(50);
        assert_eq!(account.get_balance(), 150);
    }
}
