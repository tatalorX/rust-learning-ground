// Exercise 093: Smart Pointers - Deref Trait
//
// Learning objective: Learn how to implement the Deref trait
// to enable custom types to behave like references.
//
// Deref allows a type to be treated as a reference, enabling
// automatic dereferencing and the * operator. This is how
// Box<T>, Rc<T>, etc. allow you to access the inner value.

use std::ops::Deref;

// TODO: Create a custom smart pointer MyBox<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// TODO: Implement Deref for MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Create a custom string type that derefs to &str
struct MyString {
    data: String,
}

impl MyString {
    fn new(s: &str) -> Self {
        MyString {
            data: s.to_string(),
        }
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl Deref for MyString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// TODO: Create a countable pointer that tracks dereferences
struct CountingBox<T> {
    value: T,
    deref_count: std::cell::RefCell<usize>,
}

impl<T> CountingBox<T> {
    fn new(value: T) -> Self {
        CountingBox {
            value,
            deref_count: std::cell::RefCell::new(0),
        }
    }
    
    fn deref_count(&self) -> usize {
        *self.deref_count.borrow()
    }
}

impl<T> Deref for CountingBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        *self.deref_count.borrow_mut() += 1;
        &self.value
    }
}

// TODO: Create a Vec wrapper with bounds checking via Deref
struct SafeVec<T> {
    data: Vec<T>,
}

impl<T> SafeVec<T> {
    fn new() -> Self {
        SafeVec { data: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

impl<T> Deref for SafeVec<T> {
    type Target = [T];
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    // TODO: Test MyBox dereferencing
    let x = 5;
    let y = MyBox::new(x);
    
    println!("MyBox dereferencing:");
    println!("x = {}", x);
    println!("*y = {}", *y); // Uses Deref
    assert_eq!(x, *y);

    // TODO: Demonstrate automatic dereferencing in function calls
    let name = MyBox::new(String::from("Rust"));
    // &MyBox<String> -> &String -> &str via deref coercion
    hello(&name);

    // TODO: Test MyString
    let my_str = MyString::new("Hello, World!");
    println!("\nMyString operations:");
    println!("Length: {}", my_str.len());
    println!("Uppercase: {}", my_str.to_uppercase()); // Deref to str
    println!("Contains 'World': {}", my_str.contains("World"));

    // TODO: Test CountingBox
    let counter = CountingBox::new(42);
    println!("\nCountingBox:");
    println!("Deref count before: {}", counter.deref_count());
    let _val = *counter;
    let _val = *counter;
    let _val = *counter;
    println!("Deref count after 3 derefs: {}", counter.deref_count());

    // TODO: Test SafeVec
    let mut vec = SafeVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("\nSafeVec iteration:");
    for item in &vec { // Uses Deref to get &[T]
        println!("  {}", item);
    }
    
    // Can use slice methods
    println!("First element: {:?}", vec.first());
    println!("Contains 2: {}", vec.contains(&2));
}

// TODO: This function takes &str, demonstrate deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// TODO: Complete this function that works with any Deref type
fn get_length<T: Deref>(item: &T) -> usize
where
    T::Target: AsRef<[u8]>,
{
    item.as_ref().len()
}

// TODO: Complete this generic function using Deref bounds
fn process_reference<T, U>(item: &T) -> &U
where
    T: Deref<Target = U>,
{
    &*item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_deref() {
        let b = MyBox::new(5);
        assert_eq!(*b, 5);
    }

    #[test]
    fn test_mystring_deref() {
        let s = MyString::new("hello");
        assert_eq!(s.len(), 5);
        assert!(s.contains("ell"));
    }

    #[test]
    fn test_counting_box() {
        let c = CountingBox::new(100);
        let _ = *c;
        let _ = *c;
        assert_eq!(c.deref_count(), 2);
    }

    #[test]
    fn test_safe_vec() {
        let mut v = SafeVec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
    }

    #[test]
    fn test_deref_coercion() {
        let b = MyBox::new(String::from("test"));
        // &MyBox<String> coerces to &String, then to &str
        fn takes_str(s: &str) -> bool {
            s == "test"
        }
        assert!(takes_str(&b));
    }
}
