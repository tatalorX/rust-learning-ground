// Exercise 098: Tests - Unit Tests
//
// Learning objective: Learn how to write effective unit tests
// including setup, teardown, and test organization.
//
// Unit tests verify individual components in isolation.
// They live in the same file as the code, in a test module.

// Function to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with edge cases to test
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Struct with methods to test
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
    
    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// A more complex struct to test
#[derive(Debug, PartialEq)]
pub struct BankAccount {
    balance: f64,
    transactions: Vec<f64>,
}

impl BankAccount {
    pub fn new(initial: f64) -> Self {
        let mut account = BankAccount {
            balance: 0.0,
            transactions: Vec::new(),
        };
        account.deposit(initial);
        account
    }
    
    pub fn deposit(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount < 0.0 {
            return Err("Cannot deposit negative amount");
        }
        self.balance += amount;
        self.transactions.push(amount);
        Ok(())
    }
    
    pub fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount < 0.0 {
            return Err("Cannot withdraw negative amount");
        }
        if amount > self.balance {
            return Err("Insufficient funds");
        }
        self.balance -= amount;
        self.transactions.push(-amount);
        Ok(())
    }
    
    pub fn balance(&self) -> f64 {
        self.balance
    }
    
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

fn main() {
    println!("Run tests with: cargo test");
    println!("Run specific test: cargo test test_name");
    println!("Run tests in this file: cargo test --bin template");
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: Write a basic test for add function
    #[test]
    fn test_add_basic() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    
    // TODO: Write a test with should_panic for divide by zero
    // Actually, divide returns Result, so test the error case
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero");
    }
    
    // TODO: Write a test for successful division
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert_eq!(divide(7.0, 2.0).unwrap(), 3.5);
    }
    
    // TODO: Write tests for Rectangle
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }
    
    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle::new(3.0, 4.0);
        assert_eq!(rect.perimeter(), 14.0);
    }
    
    #[test]
    fn test_rectangle_is_square() {
        let square = Rectangle::new(5.0, 5.0);
        assert!(square.is_square());
        
        let rect = Rectangle::new(5.0, 10.0);
        assert!(!rect.is_square());
    }
    
    #[test]
    fn test_rectangle_scale() {
        let mut rect = Rectangle::new(2.0, 3.0);
        rect.scale(2.0);
        assert_eq!(rect.width, 4.0);
        assert_eq!(rect.height, 6.0);
        assert_eq!(rect.area(), 24.0);
    }
    
    #[test]
    fn test_rectangle_can_hold() {
        let big = Rectangle::new(10.0, 10.0);
        let small = Rectangle::new(5.0, 5.0);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }
    
    // TODO: Write tests for BankAccount
    #[test]
    fn test_account_new() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
        assert_eq!(account.transaction_count(), 1); // Initial deposit
    }
    
    #[test]
    fn test_account_deposit() {
        let mut account = BankAccount::new(0.0);
        assert!(account.deposit(50.0).is_ok());
        assert_eq!(account.balance(), 50.0);
    }
    
    #[test]
    fn test_account_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        assert!(account.deposit(-10.0).is_err());
        assert_eq!(account.balance(), 100.0); // Unchanged
    }
    
    #[test]
    fn test_account_withdraw() {
        let mut account = BankAccount::new(100.0);
        assert!(account.withdraw(30.0).is_ok());
        assert_eq!(account.balance(), 70.0);
    }
    
    #[test]
    fn test_account_withdraw_too_much() {
        let mut account = BankAccount::new(50.0);
        assert!(account.withdraw(100.0).is_err());
        assert_eq!(account.balance(), 50.0); // Unchanged
    }
    
    // TODO: Write a test using result? operator
    #[test]
    fn test_multiple_transactions() -> Result<(), &'static str> {
        let mut account = BankAccount::new(0.0);
        account.deposit(100.0)?;
        account.deposit(50.0)?;
        account.withdraw(30.0)?;
        assert_eq!(account.balance(), 120.0);
        assert_eq!(account.transaction_count(), 4); // 3 + initial
        Ok(())
    }
    
    // TODO: Write a test that ignores certain conditions
    #[test]
    #[ignore = "expensive computation"]
    fn test_expensive_computation() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        assert_eq!(add(1, 1), 2);
    }
    
    // TODO: Write a test with custom failure message
    #[test]
    fn test_with_message() {
        let rect = Rectangle::new(3.0, 4.0);
        assert!(
            rect.area() > 0.0,
            "Area should be positive, got {}",
            rect.area()
        );
    }
    
    // TODO: Use common setup with helper function
    fn create_test_account() -> BankAccount {
        BankAccount::new(1000.0)
    }
    
    #[test]
    fn test_with_helper() {
        let account = create_test_account();
        assert_eq!(account.balance(), 1000.0);
    }
}

// TODO: Create a separate test module for edge cases
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    #[test]
    fn test_rectangle_zero_dimensions() {
        let rect = Rectangle::new(0.0, 10.0);
        assert_eq!(rect.area(), 0.0);
    }
    
    #[test]
    fn test_account_zero_deposit() {
        let mut account = BankAccount::new(100.0);
        assert!(account.deposit(0.0).is_ok());
        assert_eq!(account.balance(), 100.0);
    }
}
