export interface BookChapter {
  id: string;
  title: string;
  description: string;
  sections: BookSection[];
  estimatedReadTime: number; // in minutes
  difficulty: "beginner" | "intermediate" | "advanced";
}

export interface BookSection {
  id: string;
  title: string;
  content: string;
  codeExamples?: CodeExample[];
  quiz?: QuizQuestion[];
}

export interface CodeExample {
  id: string;
  title: string;
  code: string;
  explanation: string;
  runnable: boolean;
}

export interface QuizQuestion {
  id: string;
  question: string;
  options: string[];
  correctAnswer: number;
  explanation: string;
}

export const RUST_BOOK: BookChapter[] = [
  {
    id: "getting-started",
    title: "Getting Started",
    description: "Installation, Hello World, and Cargo basics",
    estimatedReadTime: 15,
    difficulty: "beginner",
    sections: [
      {
        id: "installation",
        title: "Installation",
        content: `## Installing Rust

The easiest way to install Rust is through **rustup**, the Rust installer and version management tool.

### Installing rustup

On Linux or macOS:
\`\`\`bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
\`\`\`

On Windows, download and run the installer from rust-lang.org.

### Verifying Installation

\`\`\`bash
rustc --version
\`\`\`

This should print the version number, commit hash, and commit date.`,
        codeExamples: [
          {
            id: "check-version",
            title: "Check Rust Version",
            code: `rustc --version`,
            explanation:
              "This command verifies that Rust is installed correctly",
            runnable: false,
          },
        ],
      },
      {
        id: "hello-world",
        title: "Hello, World!",
        content: `## Your First Rust Program

Every programmer's journey begins with a "Hello, world!" program.

### Creating a Project

\`\`\`bash
mkdir hello_world
cd hello_world
\`\`\`

### Writing the Code

Create a file named \`main.rs\` with the following content:

The \`main\` function is special: it is always the first code that runs in every executable Rust program.`,
        codeExamples: [
          {
            id: "hello-world-example",
            title: "Hello World Program",
            code: `fn main() {
    println!("Hello, world!");
}`,
            explanation: "The println! macro prints text to the console",
            runnable: true,
          },
        ],
      },
      {
        id: "hello-cargo",
        title: "Hello, Cargo!",
        content: `## Cargo: Rust's Build System

Cargo is Rust's build system and package manager. It handles:
- Building your code
- Downloading dependencies
- Building dependencies

### Creating a New Project

\`\`\`bash
cargo new hello_cargo
cd hello_cargo
\`\`\`

This creates:
- \`Cargo.toml\` - configuration file
- \`src/main.rs\` - main source file
- \`.gitignore\` - for Git`,
        codeExamples: [
          {
            id: "cargo-new",
            title: "Create New Project",
            code: `cargo new my_project`,
            explanation: "Creates a new Cargo project with all necessary files",
            runnable: false,
          },
        ],
        quiz: [
          {
            id: "cargo-quiz-1",
            question: "What is the purpose of Cargo.toml?",
            options: [
              "It contains the main source code",
              "It's the configuration file for Cargo",
              "It's the compiled binary",
              "It's the documentation file",
            ],
            correctAnswer: 1,
            explanation:
              "Cargo.toml is the manifest file that contains metadata and dependencies for your project",
          },
        ],
      },
    ],
  },
  {
    id: "common-concepts",
    title: "Programming a Guessing Game",
    description: "Learn Rust fundamentals by building a real program",
    estimatedReadTime: 45,
    difficulty: "beginner",
    sections: [
      {
        id: "guessing-game-setup",
        title: "Setting Up",
        content: `## Building a Guessing Game

This chapter will walk you through building a guessing game. It's a great way to learn Rust's common programming concepts.

### Project Setup

\`\`\`bash
cargo new guessing_game
cd guessing_game
\`\`\``,
      },
      {
        id: "processing-guess",
        title: "Processing a Guess",
        content: `## Reading User Input

We'll use the \`std::io\` library for input/output operations.

\`\`\`rust
use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
\`\`\``,
        codeExamples: [
          {
            id: "read-input",
            title: "Reading User Input",
            code: `use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("You entered: {}", input.trim());
}`,
            explanation:
              "String::new() creates a new empty string, and read_line appends user input to it",
            runnable: true,
          },
        ],
      },
      {
        id: "random-number",
        title: "Generating a Secret Number",
        content: `## Using External Crates

We need the \`rand\` crate to generate random numbers.

Add to Cargo.toml:
\`\`\`toml
[dependencies]
rand = "0.8"
\`\`\``,
      },
    ],
  },
  {
    id: "common-programming",
    title: "Common Programming Concepts",
    description: "Variables, types, functions, and control flow",
    estimatedReadTime: 60,
    difficulty: "beginner",
    sections: [
      {
        id: "variables",
        title: "Variables and Mutability",
        content: `## Variables in Rust

By default, variables are immutable. This is one of Rust's key features for safety.

\`\`\`rust
let x = 5;      // immutable
let mut y = 5;  // mutable
y = 6;          // OK
// x = 6;       // ERROR!
\`\`\``,
        codeExamples: [
          {
            id: "mutable-vars",
            title: "Mutable Variables",
            code: `fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}`,
            explanation:
              "mut makes a variable mutable, allowing it to be changed",
            runnable: true,
          },
        ],
      },
      {
        id: "data-types",
        title: "Data Types",
        content: `## Rust's Type System

Rust is a statically typed language with several built-in types:
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples, arrays

### Integer Types
- \`i8\`, \`i16\`, \`i32\`, \`i64\`, \`i128\`, \`isize\` (signed)
- \`u8\`, \`u16\`, \`u32\`, \`u64\`, \`u128\`, \`usize\` (unsigned)`,
        codeExamples: [
          {
            id: "types-example",
            title: "Working with Types",
            code: `fn main() {
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let c: char = '🦀';
    
    println!("x = {}, y = {}, z = {}, c = {}", x, y, z, c);
}`,
            explanation:
              "Explicit type annotations tell the compiler what type a variable should be",
            runnable: true,
          },
        ],
      },
      {
        id: "functions",
        title: "Functions",
        content: `## Defining Functions

Functions are declared with \`fn\` followed by the function name and parameters.

\`\`\`rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
\`\`\``,
      },
      {
        id: "control-flow",
        title: "Control Flow",
        content: `## If Expressions and Loops

\`\`\`rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}

// Loops
loop {
    println!("again!");
    break;
}

for i in 0..5 {
    println!("{}", i);
}
\`\`\``,
        codeExamples: [
          {
            id: "loop-example",
            title: "For Loop",
            code: `fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}`,
            explanation: "1..=5 creates an inclusive range from 1 to 5",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "ownership",
    title: "Understanding Ownership",
    description: "Rust's unique memory management system",
    estimatedReadTime: 90,
    difficulty: "intermediate",
    sections: [
      {
        id: "what-is-ownership",
        title: "What is Ownership?",
        content: `## Ownership Rules

Ownership is Rust's most unique feature. It enables Rust to make memory safety guarantees without needing a garbage collector.

### The Three Rules:
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

\`\`\`rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}   // this scope is now over, and s is no longer valid
\`\`\``,
      },
      {
        id: "references",
        title: "References and Borrowing",
        content: `## Borrowing

References allow you to refer to some value without taking ownership of it.

\`\`\`rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
\`\`\``,
        codeExamples: [
          {
            id: "references-example",
            title: "Using References",
            code: `fn main() {
    let s = String::from("hello");
    
    change(&s);
    
    println!("String is still valid: {}", s);
}

fn change(s: &String) {
    println!("Got reference to: {}", s);
    // Note: can't modify through immutable reference
}`,
            explanation: "References allow borrowing without taking ownership",
            runnable: true,
          },
        ],
      },
      {
        id: "slices",
        title: "The Slice Type",
        content: `## Slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

\`\`\`rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
\`\`\``,
      },
    ],
  },
  {
    id: "structs",
    title: "Using Structs",
    description: "Custom data types and methods",
    estimatedReadTime: 50,
    difficulty: "intermediate",
    sections: [
      {
        id: "defining-structs",
        title: "Defining and Instantiating Structs",
        content: `## Creating Structs

Structs are similar to tuples, but you name each piece of data.

\`\`\`rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
\`\`\``,
        codeExamples: [
          {
            id: "struct-example",
            title: "Creating a Struct",
            code: `struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle: {} x {}", rect.width, rect.height);
}`,
            explanation:
              "Structs group related data together with named fields",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "enums",
    title: "Enums and Pattern Matching",
    description: "Powerful enum types and match expressions",
    estimatedReadTime: 70,
    difficulty: "intermediate",
    sections: [
      {
        id: "defining-enums",
        title: "Defining an Enum",
        content: `## Enums

Enums allow you to define a type by enumerating its possible variants.

\`\`\`rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
\`\`\``,
      },
      {
        id: "match",
        title: "The match Control Flow Construct",
        content: `## Pattern Matching with match

Rust has an extremely powerful control flow operator called \`match\`.

\`\`\`rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
\`\`\``,
        codeExamples: [
          {
            id: "match-example",
            title: "Pattern Matching",
            code: `enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}`,
            explanation:
              "match compares a value against patterns and executes code based on which pattern matches",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "error-handling",
    title: "Error Handling",
    description: "Result, Option, and panic!",
    estimatedReadTime: 60,
    difficulty: "intermediate",
    sections: [
      {
        id: "panic",
        title: "Unrecoverable Errors with panic!",
        content: `## Panicking

Sometimes, bad things happen in your code, and there's nothing you can do about it. In these cases, Rust has the \`panic!\` macro.

\`\`\`rust
fn main() {
    panic!("crash and burn");
}
\`\`\``,
      },
      {
        id: "result",
        title: "Recoverable Errors with Result",
        content: `## The Result Enum

Most errors aren't serious enough to require the program to stop entirely.

\`\`\`rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
\`\`\`

Example:
\`\`\`rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
\`\`\``,
        codeExamples: [
          {
            id: "result-example",
            title: "Using Result",
            code: `fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}`,
            explanation:
              "Result<T, E> represents either success (Ok) or failure (Err)",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "generics",
    title: "Generic Types and Traits",
    description: "Abstract types and shared behavior",
    estimatedReadTime: 80,
    difficulty: "advanced",
    sections: [
      {
        id: "generic-data",
        title: "Generic Data Types",
        content: `## Generics

Generics are abstract stand-ins for concrete types or other properties.

\`\`\`rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
\`\`\``,
      },
      {
        id: "traits",
        title: "Traits: Defining Shared Behavior",
        content: `## Traits

A trait defines functionality a particular type has and can share with other types.

\`\`\`rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
\`\`\``,
      },
    ],
  },
  {
    id: "lifetimes",
    title: "Lifetimes",
    description: "Validating references",
    estimatedReadTime: 75,
    difficulty: "advanced",
    sections: [
      {
        id: "lifetime-syntax",
        title: "Lifetime Syntax",
        content: `## Lifetimes

Lifetimes are another type of generic. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

\`\`\`rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
\`\`\``,
      },
      {
        id: "lifetime-functions",
        title: "Lifetime Annotations in Functions",
        content: `## Generic Lifetimes

\`\`\`rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
\`\`\``,
        codeExamples: [
          {
            id: "lifetime-example",
            title: "Lifetimes in Functions",
            code: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}`,
            explanation:
              "The 'a lifetime annotation ensures the returned reference lives as long as the shortest input reference",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "testing",
    title: "Writing Tests",
    description: "Unit tests, integration tests, and documentation tests",
    estimatedReadTime: 55,
    difficulty: "intermediate",
    sections: [
      {
        id: "writing-tests",
        title: "How to Write Tests",
        content: `## Test Functions

Tests are Rust functions that verify that the non-test code is functioning in the expected manner.

\`\`\`rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
\`\`\``,
        codeExamples: [
          {
            id: "test-example",
            title: "Writing a Test",
            code: `fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Simple assertion example
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(-1, 1), 0);
    println!("All tests passed!");
}`,
            explanation: "assert_eq! macro checks that two values are equal",
            runnable: true,
          },
        ],
      },
    ],
  },
  {
    id: "concurrency",
    title: "Fearless Concurrency",
    description: "Threads, channels, and shared state",
    estimatedReadTime: 85,
    difficulty: "advanced",
    sections: [
      {
        id: "threads",
        title: "Using Threads",
        content: `## Creating Threads

Rust's standard library provides the \`thread\` module for creating concurrent programs.

\`\`\`rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
\`\`\``,
      },
      {
        id: "channels",
        title: "Message Passing",
        content: `## Channels

One approach to concurrency is message passing, where threads communicate by sending each other messages.

\`\`\`rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
\`\`\``,
      },
    ],
  },
  // CHAPTER 12: Smart Pointers
  {
    id: "smart-pointers",
    title: "Smart Pointers",
    description: "Using Box, Rc, RefCell, and other smart pointers",
    estimatedReadTime: 90,
    difficulty: "advanced",
    sections: [
      {
        id: "box-pointer",
        title: "Box<T>",
        content: `## Box - Heap Allocation

Box is the simplest smart pointer. It allows you to store data on the heap.

### When to Use Box:
- When you have a type whose size can't be known at compile time
- When you have a large amount of data and want to transfer ownership without copying
- When you want to own a value and only care that it's a type that implements a specific trait`,
        codeExamples: [
          {
            id: "box-basic",
            title: "Basic Box Usage",
            code: `fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box is automatically freed when it goes out of scope
}`,
            explanation: "Box stores the value 5 on the heap",
            runnable: true,
          },
        ],
      },
      {
        id: "rc-pointer",
        title: "Rc<T>",
        content: `## Reference Counted Smart Pointer

Rc<T> enables multiple ownership of the same data. It keeps track of the number of references.

### Use Cases:
- Sharing data between multiple parts of your program
- Graph data structures with multiple edges pointing to the same node`,
        codeExamples: [
          {
            id: "rc-sharing",
            title: "Sharing Data with Rc",
            code: `use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("shared data"));
    
    let data2 = Rc::clone(&data);
    let data3 = Rc::clone(&data);
    
    println!("Reference count: {}", Rc::strong_count(&data));
}`,
            explanation: "Multiple owners share the same data",
            runnable: true,
          },
        ],
      },
      {
        id: "refcell-pointer",
        title: "RefCell<T>",
        content: `## Interior Mutability

RefCell<T> allows you to mutate data even when there are immutable references to it. It enforces borrowing rules at runtime.

### When to Use:
- When you need interior mutability
- When you're certain your code follows borrowing rules but compiler can't verify`,
        codeExamples: [
          {
            id: "refcell-mutate",
            title: "Interior Mutability",
            code: `use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    {
        let mut val = data.borrow_mut();
        *val += 1;
    }
    
    println!("Value: {}", data.borrow());
}`,
            explanation: "Mutating through immutable reference",
            runnable: true,
          },
        ],
      },
    ],
  },
  // CHAPTER 13: Object-Oriented Patterns
  {
    id: "oop-patterns",
    title: "Object-Oriented Patterns",
    description:
      "OOP concepts in Rust: encapsulation, polymorphism, and inheritance alternatives",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "encapsulation",
        title: "Encapsulation",
        content: `## Encapsulation in Rust

Rust supports encapsulation through modules and pub keyword.

### Key Points:
- Default is private
- Use pub to expose functionality
- Implementation details hidden from users`,
        codeExamples: [
          {
            id: "encapsulation-example",
            title: "Encapsulated Struct",
            code: `mod bank_account {
    pub struct Account {
        balance: f64,
    }
    
    impl Account {
        pub fn new(initial: f64) -> Account {
            Account { balance: initial }
        }
        
        pub fn deposit(&mut self, amount: f64) {
            self.balance += amount;
        }
        
        pub fn balance(&self) -> f64 {
            self.balance
        }
    }
}

fn main() {
    let mut account = bank_account::Account::new(100.0);
    account.deposit(50.0);
    println!("Balance: {}", account.balance());
}`,
            explanation: "Balance is private, accessed through methods",
            runnable: true,
          },
        ],
      },
      {
        id: "trait-objects",
        title: "Trait Objects",
        content: `## Polymorphism with Trait Objects

Trait objects allow you to use different types through a common interface.

### Dynamic Dispatch:
- Uses vtable for method lookup
- Slight runtime cost
- Allows heterogeneous collections`,
        codeExamples: [
          {
            id: "trait-objects-example",
            title: "Drawing Different Shapes",
            code: `trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing circle"); }
}

impl Drawable for Square {
    fn draw(&self) { println!("Drawing square"); }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}`,
            explanation:
              "Different types treated uniformly through trait object",
            runnable: true,
          },
        ],
      },
    ],
  },
  // CHAPTER 14: Patterns and Matching
  {
    id: "patterns",
    title: "Patterns and Matching",
    description: "Advanced pattern matching techniques in Rust",
    estimatedReadTime: 80,
    difficulty: "intermediate",
    sections: [
      {
        id: "all-patterns",
        title: "All the Places Patterns Can Be Used",
        content: `## Pattern Usage Locations

Patterns appear in many places in Rust: match arms, if let, while let, for loops, and let statements.

### Destructuring:
- Tuples
- Structs
- Enums
- References`,
        codeExamples: [
          {
            id: "destructuring",
            title: "Destructuring Patterns",
            code: `struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 0, y: 7 };
    
    // Destructure struct
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);
    
    // Destructure tuple
    let (a, b, c) = (1, 2, 3);
    println!("a: {}, b: {}, c: {}", a, b, c);
    
    // Ignore with _
    let (x, _, z) = (1, 2, 3);
    println!("x: {}, z: {}", x, z);
}`,
            explanation: "Extract values from complex types",
            runnable: true,
          },
        ],
      },
      {
        id: "refutable-patterns",
        title: "Refutability",
        content: `## Refutable vs Irrefutable Patterns

Patterns that can fail to match are refutable. Patterns that always match are irrefutable.

### Examples:
- Irrefutable: let x = 5;
- Refutable: if let Some(x) = option`,
      },
      {
        id: "pattern-syntax",
        title: "Pattern Syntax",
        content: `## Advanced Pattern Syntax

### Matching Multiple Patterns:
\`\`\`rust
match x {
    1 | 2 => println!("one or two"),
    3..=9 => println!("three through nine"),
    _ => println!("something else"),
}
\`\`\`

### Bindings:
\`\`\`rust
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found id: {}", id_variable);
    }
}
\`\`\``,
      },
    ],
  },
  // CHAPTER 15: Unsafe Rust
  {
    id: "unsafe-rust",
    title: "Unsafe Rust",
    description: "Working with unsafe code, raw pointers, and FFI",
    estimatedReadTime: 85,
    difficulty: "advanced",
    sections: [
      {
        id: "unsafe-intro",
        title: "Unsafe Superpowers",
        content: `## What is Unsafe Rust?

Unsafe Rust gives you five superpowers:
1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of a \`union\`

### Safety:
- Unsafe doesn't mean code is dangerous
- You guarantee safety to the compiler
- Still checked at runtime for some operations`,
        codeExamples: [
          {
            id: "unsafe-example",
            title: "Basic Unsafe Block",
            code: `fn main() {
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}`,
            explanation:
              "Raw pointers can only be dereferenced in unsafe blocks",
            runnable: true,
          },
        ],
      },
      {
        id: "raw-pointers",
        title: "Raw Pointers",
        content: `## Raw Pointers (*const T, *mut T)

Similar to references but with fewer guarantees:
- Allowed to ignore borrowing rules
- Not guaranteed to point to valid memory
- Allowed to be null
- Don't implement automatic cleanup`,
      },
      {
        id: "unsafe-functions",
        title: "Unsafe Functions",
        content: `## Calling Unsafe Functions

Functions marked with unsafe must be called within unsafe blocks.

### Creating Unsafe Functions:
\`\`\`rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
\`\`\``,
      },
    ],
  },
  // CHAPTER 16: Advanced Types
  {
    id: "advanced-types",
    title: "Advanced Types",
    description:
      "Newtype pattern, type aliases, never type, and dynamically sized types",
    estimatedReadTime: 70,
    difficulty: "advanced",
    sections: [
      {
        id: "newtype",
        title: "Newtype Pattern",
        content: `## The Newtype Pattern

Create a new type that's a thin wrapper around another type.

### Benefits:
- Type safety
- Implement external traits on external types
- Hide implementation details`,
        codeExamples: [
          {
            id: "newtype-example",
            title: "Wrapper Type",
            code: `struct Meters(u32);
struct Kilometers(u32);

fn main() {
    let distance = Meters(1000);
    println!("Distance: {} meters", distance.0);
}`,
            explanation: "Meters and Kilometers are distinct types",
            runnable: true,
          },
        ],
      },
      {
        id: "type-aliases",
        title: "Type Aliases",
        content: `## Type Aliases with type Keyword

Create a synonym for another type.

### Use Cases:
- Reducing repetition
- Making long types more readable
- Creating platform-specific types`,
        codeExamples: [
          {
            id: "type-alias",
            title: "Type Alias Example",
            code: `type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let distance: Kilometers = 5;
    println!("Distance: {}", distance);
}`,
            explanation: "Kilometers is just another name for i32",
            runnable: true,
          },
        ],
      },
      {
        id: "never-type",
        title: "The Never Type",
        content: `## The ! Type

Rust has a special type called never (!) for functions that never return.

### Examples:
- continue
- panic!
- loop {}
- Functions that exit the process`,
      },
    ],
  },
  // CHAPTER 17: Advanced Functions
  {
    id: "advanced-functions",
    title: "Advanced Functions",
    description:
      "Function pointers, closures as output, and function-like macros",
    estimatedReadTime: 65,
    difficulty: "advanced",
    sections: [
      {
        id: "function-pointers",
        title: "Function Pointers",
        content: `## Passing Functions as Arguments

You can pass functions to other functions using function pointers.

### Syntax:
fn add_one(x: i32) -> i32 { x + 1 }

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}`,
        codeExamples: [
          {
            id: "fn-pointer",
            title: "Function Pointer",
            code: `fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("Answer: {}", answer);
}`,
            explanation: "Functions can be passed as arguments",
            runnable: true,
          },
        ],
      },
      {
        id: "returning-closures",
        title: "Returning Closures",
        content: `## Closures as Return Values

Returning closures requires boxing or using impl Trait.

### Why?
- Each closure has a unique type
- Size not known at compile time`,
        codeExamples: [
          {
            id: "return-closure",
            title: "Returning a Closure",
            code: `fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let triple = make_multiplier(3);
    println!("Triple 4: {}", triple(4));
}`,
            explanation: "impl Fn allows returning closures",
            runnable: true,
          },
        ],
      },
    ],
  },
  // CHAPTER 18: Macros
  {
    id: "macros",
    title: "Macros",
    description: "Declarative and procedural macros in Rust",
    estimatedReadTime: 90,
    difficulty: "advanced",
    sections: [
      {
        id: "declarative-macros",
        title: "Declarative Macros",
        content: `## macro_rules!

Declarative macros allow you to write something similar to a match expression.

### Structure:
- Name of the macro
- Braces with pattern => code blocks
- Recursive expansion`,
        codeExamples: [
          {
            id: "macro-example",
            title: "Simple Macro",
            code: `macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}`,
            explanation: "Macros generate code at compile time",
            runnable: true,
          },
        ],
      },
      {
        id: "procedural-macros",
        title: "Procedural Macros",
        content: `## Custom Derive Macros

Procedural macros accept some code as input, operate on that code, and produce some code as output.

### Types:
- Custom #[derive] attributes
- Attribute-like macros
- Function-like macros`,
      },
    ],
  },
  // CHAPTER 19: Final Project
  {
    id: "final-project",
    title: "Final Project: Building a Web Server",
    description: "Put it all together by building a multi-threaded web server",
    estimatedReadTime: 120,
    difficulty: "advanced",
    sections: [
      {
        id: "project-setup",
        title: "Project Setup",
        content: `## Building a Web Server

In this final project, you'll build a web server using everything you've learned.

### What We'll Build:
- TCP listener
- HTTP request parser
- Thread pool for handling connections
- Graceful shutdown`,
      },
      {
        id: "single-threaded",
        title: "Single-Threaded Server",
        content: `## Starting Simple

First, we'll build a basic single-threaded server.

### Steps:
1. Listen for TCP connections
2. Read HTTP requests
3. Parse requests
4. Send responses`,
        codeExamples: [
          {
            id: "basic-server",
            title: "Basic Server",
            code: `use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        
        let response = "HTTP/1.1 200 OK\\r\\n\\r\\nHello!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}`,
            explanation: "Basic HTTP server listening on port 7878",
            runnable: false,
          },
        ],
      },
      {
        id: "thread-pool",
        title: "Multi-Threaded with Thread Pool",
        content: `## Improving Throughput

Use a thread pool to handle multiple requests concurrently.

### Benefits:
- Limited resource usage
- Better performance under load
- Prevents thread explosion`,
      },
      {
        id: "graceful-shutdown",
        title: "Graceful Shutdown",
        content: `## Clean Shutdown

Implement graceful shutdown to finish handling requests before stopping.

### Implementation:
- Signal channels
- Join threads
- Clean up resources`,
      },
    ],
  },
  // CHAPTER 20: Appendices
  {
    id: "appendices",
    title: "Appendices",
    description: "Reference material and additional resources",
    estimatedReadTime: 60,
    difficulty: "beginner",
    sections: [
      {
        id: "keywords",
        title: "Keywords",
        content: `## Rust Keywords

### Reserved Keywords:
as, break, const, continue, crate, else, enum, extern, false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref, return, self, Self, static, struct, super, trait, true, type, unsafe, use, where, while

### Reserved for Future:
abstract, become, box, do, final, macro, override, priv, typeof, unsized, virtual, yield`,
      },
      {
        id: "operators",
        title: "Operators",
        content: `## Rust Operators

### Arithmetic: +, -, *, /, %
### Comparison: ==, !=, <, >, <=, >=
### Logical: !, &&, ||
### Bitwise: &, |, ^, <<, >>
### Assignment: =, +=, -=, *=, /=, %=
### Dereference: *, &
### Range: .., ..=`,
      },
      {
        id: "attributes",
        title: "Attributes",
        content: `## Common Attributes

### Testing:
- #[test]
- #[ignore]
- #[should_panic]

### Documentation:
- /// Doc comments
- //! Module documentation

### Derive:
- #[derive(Debug, Clone, Copy)]`,
      },
    ],
  },
  // Additional Chapters to Reach 280+ Sections
  {
    id: "cargo-workspaces",
    title: "Cargo Workspaces",
    description: "Organize large projects with multiple crates",
    estimatedReadTime: 45,
    difficulty: "intermediate",
    sections: [
      {
        id: "workspace-setup",
        title: "Creating a Workspace",
        content:
          '## Workspace Setup\n\nA workspace is a collection of related packages.\n\n### Creating a Workspace:\n\`\`\`toml\n[workspace]\nmembers = [\n    "package1",\n    "package2",\n]\n\`\`\`',
      },
      {
        id: "workspace-dependencies",
        title: "Managing Dependencies",
        content:
          "## Workspace Dependencies\n\nShare dependencies across workspace members.\n\n### Benefits:\n- Single version of common dependencies\n- Faster compilation\n- Consistent versions",
      },
    ],
  },
  {
    id: "rustdoc",
    title: "Documentation with rustdoc",
    description: "Write and generate documentation",
    estimatedReadTime: 40,
    difficulty: "beginner",
    sections: [
      {
        id: "doc-comments",
        title: "Documentation Comments",
        content:
          "## Writing Documentation\n\nUse /// for item documentation and //! for module-level docs.\n\n### Example:\n\`\`\`rust\n/// Adds two numbers together\n/// \n/// # Examples\n/// \n/// \`\`\`\n/// let sum = add(2, 3);\n/// assert_eq!(sum, 5);\n/// \`\`\`\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\`\`\`",
      },
      {
        id: "generating-docs",
        title: "Generating Documentation",
        content:
          "## Generate Docs with cargo doc\n\nRun \`cargo doc\` to generate HTML documentation.\n\n### Features:\n- Search functionality\n- Links between items\n- Examples are tested",
      },
    ],
  },
  {
    id: "crates-io",
    title: "Publishing to crates.io",
    description: "Share your Rust code with the world",
    estimatedReadTime: 50,
    difficulty: "intermediate",
    sections: [
      {
        id: "crate-preparation",
        title: "Preparing Your Crate",
        content:
          "## Before Publishing\n\n### Checklist:\n- Choose a unique name\n- Add metadata to Cargo.toml\n- Write comprehensive documentation\n- Add LICENSE file\n- Create README.md",
      },
      {
        id: "semantic-versioning",
        title: "Semantic Versioning",
        content:
          "## Version Numbers\n\nFollow SemVer: MAJOR.MINOR.PATCH\n\n- MAJOR: Breaking changes\n- MINOR: New features, backward compatible\n- PATCH: Bug fixes",
      },
      {
        id: "publishing",
        title: "Publishing Process",
        content:
          "## Publishing Steps\n\n1. \`cargo login\` - Authenticate\n2. \`cargo package\` - Create package\n3. \`cargo publish\` - Upload to crates.io\n\n### Important:\n- Publishing is permanent\n- Can't delete, only yank",
      },
    ],
  },
  {
    id: "ffi",
    title: "Foreign Function Interface",
    description: "Call C code from Rust and vice versa",
    estimatedReadTime: 85,
    difficulty: "advanced",
    sections: [
      {
        id: "calling-c",
        title: "Calling C from Rust",
        content:
          '## FFI Basics\n\nUse extern to declare external functions.\n\n### Example:\n\`\`\`rust\nextern "C" {\n    fn abs(input: i32) -> i32;\n}\n\nfn main() {\n    unsafe {\n        println!("Absolute value of -3: {}", abs(-3));\n    }\n}\n\`\`\`',
      },
      {
        id: "ffi-safety",
        title: "FFI Safety",
        content:
          "## Safe FFI Practices\n\n### Guidelines:\n- Minimize unsafe blocks\n- Validate all inputs\n- Handle null pointers\n- Document safety requirements",
      },
    ],
  },
  {
    id: "webassembly",
    title: "WebAssembly",
    description: "Compile Rust to WebAssembly for the web",
    estimatedReadTime: 70,
    difficulty: "advanced",
    sections: [
      {
        id: "wasm-intro",
        title: "Introduction to WASM",
        content:
          "## WebAssembly with Rust\n\nRust compiles to efficient WebAssembly.\n\n### Use Cases:\n- High-performance web apps\n- Games in browser\n- Image/video processing\n- Cryptography",
      },
      {
        id: "wasm-bindgen",
        title: "wasm-bindgen",
        content:
          "## wasm-bindgen\n\nFacilitate high-level interactions between Rust and JavaScript.\n\n### Features:\n- Export Rust functions to JS\n- Import JS functions to Rust\n- Handle complex types",
      },
    ],
  },
  {
    id: "embedded",
    title: "Embedded Systems",
    description: "Rust for microcontrollers and embedded devices",
    estimatedReadTime: 95,
    difficulty: "advanced",
    sections: [
      {
        id: "no-std",
        title: "No Standard Library",
        content:
          "## #![no_std]\n\nFor systems without an OS, use no_std.\n\n### What's Missing:\n- Heap allocation (by default)\n- Threads\n- File I/O\n- Network\n\n### What's Available:\n- Core library\n- Stack-allocated data\n- Hardware access",
      },
      {
        id: "embedded-hal",
        title: "Embedded HAL",
        content:
          "## Hardware Abstraction Layer\n\nEmbedded HAL provides portable hardware interfaces.\n\n### Benefits:\n- Write once, run on multiple chips\n- Type-safe hardware access\n- Rich ecosystem",
      },
    ],
  },
  {
    id: "async-await",
    title: "Async/Await",
    description: "Asynchronous programming in Rust",
    estimatedReadTime: 100,
    difficulty: "advanced",
    sections: [
      {
        id: "async-basics",
        title: "Async Fundamentals",
        content:
          '## Async Programming\n\nAsync allows concurrent execution without threads.\n\n### Key Concepts:\n- async fn returns Future\n- .await suspends execution\n- Runtime executes futures\n\n### Example:\n\`\`\`rust\nasync fn fetch_data() -> String {\n    // Simulated async operation\n    tokio::time::sleep(Duration::from_secs(1)).await;\n    "Data fetched".to_string()\n}\n\`\`\`',
      },
      {
        id: "futures",
        title: "Working with Futures",
        content:
          "## The Future Trait\n\nA Future represents an async computation.\n\n### States:\n- Pending: Not complete yet\n- Ready: Computation complete\n\n### Polling:\nThe runtime repeatedly polls futures until they're ready.",
      },
      {
        id: "tokio",
        title: "Tokio Runtime",
        content:
          "## Tokio: Rust's Async Runtime\n\nTokio provides:\n- Task scheduler\n- I/O drivers\n- Timer\n- Channels\n\n### Starting Tokio:\n\`\`\`rust\n#[tokio::main]\nasync fn main() {\n    // Your async code here\n}\n\`\`\`",
      },
      {
        id: "async-traits",
        title: "Async Traits",
        content:
          "## Traits with Async Methods\n\nTraits can have async methods using async_trait crate.\n\n### Why needed:\n- Native traits don't support async\n- Desugars to Pin<Box<dyn Future>>",
      },
    ],
  },
  {
    id: "networking",
    title: "Networking",
    description: "Building networked applications",
    estimatedReadTime: 80,
    difficulty: "advanced",
    sections: [
      {
        id: "tcp-server",
        title: "TCP Server",
        content:
          '## Building TCP Servers\n\nUse std::net::TcpListener for TCP servers.\n\n### Basic Example:\n\`\`\`rust\nuse std::net::TcpListener;\n\nfn main() -> std::io::Result<()> {\n    let listener = TcpListener::bind("127.0.0.1:8080")?;\n    \n    for stream in listener.incoming() {\n        let stream = stream?;\n        handle_connection(stream);\n    }\n    Ok(())\n}\n\`\`\`',
      },
      {
        id: "http-clients",
        title: "HTTP Clients",
        content:
          '## Making HTTP Requests\n\nUse the reqwest crate for HTTP.\n\n### Example:\n\`\`\`rust\nlet response = reqwest::get("https://api.example.com/data").await?;\nlet data: ApiResponse = response.json().await?;\n\`\`\`',
      },
      {
        id: "websockets",
        title: "WebSockets",
        content:
          "## WebSocket Communication\n\nWebSockets enable bidirectional communication.\n\n### Use Cases:\n- Real-time games\n- Chat applications\n- Live updates",
      },
    ],
  },
  {
    id: "databases",
    title: "Working with Databases",
    description: "Database access and ORM in Rust",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "sqlx",
        title: "SQLx",
        content:
          '## SQLx: Async SQL\n\nSQLx provides compile-time checked SQL.\n\n### Features:\n- Async/await support\n- Type-safe queries\n- Multiple database backends\n\n### Example:\n\`\`\`rust\nlet user: User = sqlx::query_as!(\n    User,\n    "SELECT * FROM users WHERE id = ?",\n    id\n).fetch_one(&pool).await?;\n\`\`\`',
      },
      {
        id: "diesel",
        title: "Diesel ORM",
        content:
          "## Diesel: Type-Safe ORM\n\nDiesel provides a query builder and ORM.\n\n### Benefits:\n- Type-safe queries\n- Migration system\n- Connection pooling",
      },
    ],
  },
  {
    id: "serialization",
    title: "Serialization",
    description: "Working with JSON, YAML, and other formats",
    estimatedReadTime: 55,
    difficulty: "intermediate",
    sections: [
      {
        id: "serde",
        title: "Serde Framework",
        content:
          "## Serde: Serialization Framework\n\nSerde is the standard for serialization in Rust.\n\n### Supported Formats:\n- JSON (serde_json)\n- YAML (serde_yaml)\n- TOML (toml)\n- MessagePack\n- And more\n\n### Example:\n\`\`\`rust\nuse serde::{Deserialize, Serialize};\n\n#[derive(Serialize, Deserialize)]\nstruct User {\n    name: String,\n    email: String,\n}\n\`\`\`",
      },
      {
        id: "custom-serde",
        title: "Custom Serialization",
        content:
          "## Custom Serialize/Deserialize\n\nImplement custom behavior for serialization.\n\n### Use Cases:\n- Rename fields\n- Skip fields\n- Custom formats\n- Validation",
      },
    ],
  },
  {
    id: "cli-apps",
    title: "Command Line Applications",
    description: "Build powerful CLI tools with Rust",
    estimatedReadTime: 65,
    difficulty: "intermediate",
    sections: [
      {
        id: "clap",
        title: "Clap for Argument Parsing",
        content:
          '## Clap: Command Line Parser\n\nClap makes building CLIs easy.\n\n### Features:\n- Derive macros\n- Automatic help generation\n- Shell completions\n- Validation\n\n### Example:\n\`\`\`rust\nuse clap::Parser;\n\n#[derive(Parser)]\nstruct Cli {\n    #[arg(short, long)]\n    name: String,\n    #[arg(short, long, default_value = "1")]\n    count: u32,\n}\n\`\`\`',
      },
      {
        id: "cli-best-practices",
        title: "CLI Best Practices",
        content:
          "## Building Great CLIs\n\n### Guidelines:\n- Follow POSIX conventions\n- Provide helpful error messages\n- Support --help and --version\n- Use exit codes properly\n- Support colors and styling",
      },
    ],
  },
  {
    id: "testing-advanced",
    title: "Advanced Testing",
    description: "Integration tests, benchmarks, and mocking",
    estimatedReadTime: 70,
    difficulty: "intermediate",
    sections: [
      {
        id: "integration-tests",
        title: "Integration Tests",
        content:
          "## Integration Testing\n\nIntegration tests go in the tests/ directory.\n\n### Structure:\n\`\nmy_project/\n├── src/\n└── tests/\n    ├── integration_test.rs\n    └── common/\n        └── mod.rs\n\`\n\n### Best Practices:\n- Test public API\n- Each test file is a separate binary\n- Share setup code in common module",
      },
      {
        id: "benchmarks",
        title: "Benchmarking with Criterion",
        content:
          '## Performance Benchmarks\n\nUse Criterion.rs for statistical benchmarking.\n\n### Example:\n\`\`\`rust\nuse criterion::{black_box, criterion_group, criterion_main, Criterion};\n\nfn fibonacci(n: u64) -> u64 {\n    match n {\n        0 => 1,\n        1 => 1,\n        n => fibonacci(n-1) + fibonacci(n-2),\n    }\n}\n\nfn criterion_benchmark(c: &mut Criterion) {\n    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));\n}\n\`\`\`',
      },
      {
        id: "mocking",
        title: "Mocking with mockall",
        content:
          "## Mocking for Tests\n\nMockall creates mocks for traits.\n\n### Example:\n\`\`\`rust\nuse mockall::automock;\n\n#[automock]\ntrait Database {\n    fn get_user(&self, id: u64) -> Option<User>;\n}\n\`\`\`",
      },
    ],
  },
  {
    id: "performance",
    title: "Performance Optimization",
    description: "Make your Rust code faster",
    estimatedReadTime: 85,
    difficulty: "advanced",
    sections: [
      {
        id: "profiling",
        title: "Profiling Tools",
        content:
          "## Finding Bottlenecks\n\n### Tools:\n- perf (Linux)\n- Instruments (macOS)\n- cargo flamegraph\n- cachegrind\n\n### Process:\n1. Measure before optimizing\n2. Identify hot paths\n3. Optimize\n4. Verify improvement",
      },
      {
        id: "zero-copy",
        title: "Zero-Copy Techniques",
        content:
          "## Avoiding Unnecessary Copies\n\n### Strategies:\n- Use references instead of owned data\n- Cow (Clone on Write)\n- String slices instead of String\n- Reading into existing buffers\n\n### Example:\n\`\`\`rust\n// Bad: allocates new String\nfn process(data: String) -> String { ... }\n\n// Good: works with slices\nfn process(data: &str) -> String { ... }\n\`\`\`",
      },
      {
        id: "cache-friendly",
        title: "Cache-Friendly Code",
        content:
          "## Optimizing for Cache\n\n### Techniques:\n- Structure of Arrays instead of Array of Structures\n- Sequential access patterns\n- Avoid pointer chasing\n- Align data properly",
      },
    ],
  },
  {
    id: "security",
    title: "Security Best Practices",
    description: "Writing secure Rust code",
    estimatedReadTime: 60,
    difficulty: "intermediate",
    sections: [
      {
        id: "secure-coding",
        title: "Secure Coding Guidelines",
        content:
          "## Security in Rust\n\n### Memory Safety:\n- Buffer overflows prevented by bounds checking\n- Use-after-free prevented by borrow checker\n- Null pointer dereferences prevented by Option\n\n### Input Validation:\n- Never trust user input\n- Validate at boundaries\n- Use safe parsing libraries",
      },
      {
        id: "crypto",
        title: "Cryptography",
        content:
          "## Safe Cryptography\n\nUse well-reviewed crates:\n- ring (TLS, crypto primitives)\n- rustls (TLS implementation)\n- sodiumoxide (libsodium bindings)\n\n### Never:\n- Implement crypto yourself\n- Use outdated algorithms\n- Hardcode secrets",
      },
    ],
  },
  {
    id: "error-handling-advanced",
    title: "Advanced Error Handling",
    description: "Building robust error handling systems",
    estimatedReadTime: 65,
    difficulty: "advanced",
    sections: [
      {
        id: "thiserror",
        title: "Thiserror for Libraries",
        content:
          '## Deriving Error\n\nthiserror makes defining errors easy.\n\n### Example:\n\`\`\`rust\nuse thiserror::Error;\n\n#[derive(Error, Debug)]\npub enum MyError {\n    #[error("io error: {0}")]\n    Io(#[from] std::io::Error),\n    #[error("invalid input: {0}")]\n    InvalidInput(String),\n}\n\`\`\`',
      },
      {
        id: "anyhow",
        title: "Anyhow for Applications",
        content:
          '## Flexible Error Handling\n\nanyhow provides easy error handling for apps.\n\n### Features:\n- Easy error propagation\n- Context attachment\n- Automatic backtraces\n\n### Example:\n\`\`\`rust\nuse anyhow::{Context, Result};\n\nfn read_file(path: &str) -> Result<String> {\n    std::fs::read_to_string(path)\n        .with_context(|| format!("Failed to read {}", path))?;\n}\n\`\`\`',
      },
    ],
  },
  {
    id: "design-patterns",
    title: "Design Patterns",
    description: "Common patterns in Rust",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: " RAII",
        title: "RAII Pattern",
        content:
          '## Resource Acquisition Is Initialization\n\nRust\'s ownership system enforces RAII naturally.\n\n### Benefits:\n- Automatic cleanup\n- Exception-safe\n- Composable\n\n### Example:\n\`\`\`rust\nstruct FileGuard {\n    file: File,\n}\n\nimpl Drop for FileGuard {\n    fn drop(&mut self) {\n        println!("Closing file");\n    }\n}\n\`\`\`',
      },
      {
        id: "type-state",
        title: "Type State Pattern",
        content:
          "## Encoding State in Types\n\nUse types to represent different states.\n\n### Benefits:\n- Invalid states are unrepresentable\n- Compile-time checking\n- Self-documenting code\n\n### Example:\n\`\`\`rust\nstruct Uninitialized;\nstruct Ready;\nstruct Running;\n\nstruct Service<State> {\n    state: State,\n}\n\`\`\`",
      },
      {
        id: "command-pattern",
        title: "Command Pattern",
        content:
          "## Encapsulating Actions\n\nEncapsulate requests as objects.\n\n### Use Cases:\n- Undo/redo\n- Job queues\n- Transactional operations",
      },
    ],
  },
  {
    id: "interior-mutability",
    title: "Interior Mutability Patterns",
    description: "Advanced patterns for shared mutable state",
    estimatedReadTime: 70,
    difficulty: "advanced",
    sections: [
      {
        id: "cell-types",
        title: "Cell, RefCell, Mutex",
        content:
          "## Choosing the Right Cell Type\n\n### Cell<T>:\n- For Copy types\n- get() and set() methods\n\n### RefCell<T>:\n- Runtime borrow checking\n- borrow() and borrow_mut()\n\n### Mutex<T>:\n- Thread-safe\n- Blocking",
      },
      {
        id: "rc-refcell",
        title: "Rc<RefCell<T>> Pattern",
        content:
          "## Shared Mutable Ownership\n\nCombine Rc and RefCell for shared mutable data.\n\n### Use Case:\nGraphs where nodes need multiple owners\n\n### Example:\n\`\`\`rust\nuse std::rc::Rc;\nuse std::cell::RefCell;\n\nstruct Node {\n    value: i32,\n    children: RefCell<Vec<Rc<Node>>>,\n}\n\`\`\`",
      },
      {
        id: "arc-mutex",
        title: "Arc<Mutex<T>> Pattern",
        content:
          "## Thread-Safe Sharing\n\nFor sharing across threads:\n\n### Example:\n\`\`\`rust\nuse std::sync::{Arc, Mutex};\nuse std::thread;\n\nlet counter = Arc::new(Mutex::new(0));\nlet mut handles = vec![];\n\nfor _ in 0..10 {\n    let counter = Arc::clone(&counter);\n    let handle = thread::spawn(move || {\n        let mut num = counter.lock().unwrap();\n        *num += 1;\n    });\n    handles.push(handle);\n}\n\`\`\`",
      },
    ],
  },
  {
    id: "metaprogramming",
    title: "Metaprogramming",
    description: "Macros, derive, and code generation",
    estimatedReadTime: 80,
    difficulty: "advanced",
    sections: [
      {
        id: "declarative-macro-patterns",
        title: "Advanced Macro Patterns",
        content:
          "## Complex Macros\n\n### Repetition:\n\`\`\`rust\nmacro_rules! vec {\n    ($($x:expr),*) => {\n        {\n            let mut temp_vec = Vec::new();\n            $(\n                temp_vec.push($x);\n            )*\n            temp_vec\n        }\n    };\n}\n\`\`\`",
      },
      {
        id: "procedural-macros",
        title: "Procedural Macros",
        content:
          "## Writing Proc Macros\n\nThree types:\n1. Custom derive\n2. Attribute macros\n3. Function-like macros\n\n### Structure:\n- Separate crate\n- Uses proc_macro crate\n- Token manipulation",
      },
      {
        id: "compile-time",
        title: "Compile-Time Computation",
        content:
          "## const fn and Const Generics\n\n### const fn:\nFunctions evaluated at compile time.\n\n### Const Generics:\nGeneric over constant values.\n\n### Example:\n\`\`\`rust\nstruct Array<T, const N: usize> {\n    data: [T; N],\n}\n\nlet arr: Array<i32, 5> = Array::new();\n\`\`\`",
      },
    ],
  },
  {
    id: "parallelism",
    title: "Parallel Programming",
    description: "Data parallelism and parallel iterators",
    estimatedReadTime: 70,
    difficulty: "advanced",
    sections: [
      {
        id: "rayon",
        title: "Rayon for Data Parallelism",
        content:
          "## Easy Parallelism with Rayon\n\nRayon converts sequential iterators to parallel.\n\n### Example:\n\`\`\`rust\nuse rayon::prelude::*;\n\nlet sum: i32 = (0..100)\n    .into_par_iter()\n    .map(|x| x * x)\n    .sum();\n\`\`\`\n\n### Features:\n- Work-stealing scheduler\n- Automatic load balancing\n- Scoped threads",
      },
      {
        id: "parallel-patterns",
        title: "Parallel Patterns",
        content:
          "## Common Parallel Patterns\n\n### Map-Reduce:\nTransform data, then aggregate.\n\n### Fork-Join:\nSplit work, process in parallel, combine results.\n\n### Pipeline:\nProcess data through stages concurrently.",
      },
    ],
  },
  {
    id: "systems-programming",
    title: "Systems Programming",
    description: "Low-level system access and control",
    estimatedReadTime: 90,
    difficulty: "advanced",
    sections: [
      {
        id: "memory-layout",
        title: "Memory Layout and Alignment",
        content:
          "## Understanding Memory Layout\n\n### Struct Layout:\n- Fields ordered by declaration\n- Padding for alignment\n- Size = sum of fields + padding\n\n### Controlling Layout:\n\`\`\`rust\n#[repr(C)]  // C-compatible layout\n#[repr(packed)]  // No padding\n#[repr(align(16))]  // 16-byte alignment\n\`\`\`",
      },
      {
        id: " volatile",
        title: "Volatile Memory Access",
        content:
          "## Volatile Operations\n\nFor memory-mapped I/O and special memory regions.\n\n### Use Cases:\n- Hardware registers\n- Shared memory\n- Signal handlers\n\n### Reading:\n\`\`\`rust\nuse std::ptr::read_volatile;\nlet value = read_volatile(addr);\n\`\`\`",
      },
      {
        id: "inline-assembly",
        title: "Inline Assembly",
        content:
          '## Writing Assembly in Rust\n\nUse asm! macro for inline assembly.\n\n### Example:\n\`\`\`rust\nunsafe {\n    asm!(\n        "mov {}, 42",\n        out(reg) result\n    );\n}\n\`\`\`\n\n### Safety:\n- Must be in unsafe block\n- Compiler doesn\'t verify correctness',
      },
    ],
  },
  // Extended Content - Additional Chapters (Sections 170-280+)
  {
    id: "data-structures",
    title: "Data Structures in Rust",
    description: "Implementing classic data structures",
    estimatedReadTime: 120,
    difficulty: "advanced",
    sections: [
      {
        id: "linked-lists",
        title: "Linked Lists",
        content:
          "## Implementing Linked Lists\n\nMultiple variants: singly-linked, doubly-linked, and with unsafe code.\n\n### Singly-Linked List:\n```rust\nstruct Node<T> {\n    data: T,\n    next: Option<Box<Node<T>>>,\n}\n```\n\n### Challenges:\n- Ownership with multiple nodes\n- Iteration patterns\n- Mutable traversal",
      },
      {
        id: "binary-trees",
        title: "Binary Trees",
        content:
          "## Binary Search Trees\n\nImplement a self-balancing BST in Rust.\n\n### Operations:\n- Insertion\n- Deletion\n- Search\n- Traversal (in-order, pre-order, post-order)\n\n### Self-Balancing:\n- AVL trees\n- Red-Black trees",
      },
      {
        id: "heaps",
        title: "Heaps and Priority Queues",
        content:
          "## Heap Data Structure\n\nBinary heap implementation for priority queues.\n\n### Properties:\n- Complete binary tree\n- Heap property (min or max)\n- O(log n) insert and extract\n\n### Use Cases:\n- Scheduling\n- Dijkstra's algorithm\n- Top-K problems",
      },
      {
        id: "graphs",
        title: "Graph Representations",
        content:
          "## Graph Data Structures\n\nMultiple ways to represent graphs:\n\n### Adjacency Matrix:\n- Good for dense graphs\n- O(1) edge lookup\n- O(V²) space\n\n### Adjacency List:\n- Good for sparse graphs\n- Space efficient\n- Natural for most algorithms",
      },
      {
        id: "tries",
        title: "Tries and Prefix Trees",
        content:
          "## Trie Data Structure\n\nTree-like structure for string storage.\n\n### Benefits:\n- O(m) lookup (m = string length)\n- Prefix searches\n- Autocomplete functionality\n\n### Variants:\n- Compressed tries\n- Suffix trees",
      },
      {
        id: "b-trees",
        title: "B-Trees and B+ Trees",
        content:
          "## B-Tree Family\n\nSelf-balancing search trees optimized for external memory.\n\n### Properties:\n- Multiple keys per node\n- Minimizes disk I/O\n- Used in databases and file systems\n\n### B+ Tree:\n- All data in leaves\n- Linked leaves for range queries",
      },
    ],
  },
  {
    id: "algorithms",
    title: "Algorithm Implementation",
    description: "Classic algorithms in Rust",
    estimatedReadTime: 140,
    difficulty: "advanced",
    sections: [
      {
        id: "sorting-algorithms",
        title: "Sorting Algorithms",
        content:
          "## Implementing Sorts\n\n### Comparison Sorts:\n- Quicksort (average O(n log n))\n- Merge sort (stable, O(n log n))\n- Heap sort (in-place, O(n log n))\n\n### Linear Time Sorts:\n- Counting sort\n- Radix sort\n- Bucket sort",
      },
      {
        id: "searching",
        title: "Search Algorithms",
        content:
          "## Search Techniques\n\n### Linear Search:\nSimple but O(n)\n\n### Binary Search:\nO(log n) on sorted data\n\n### Interpolation Search:\nO(log log n) for uniformly distributed data\n\n### Exponential Search:\nFor unbounded arrays",
      },
      {
        id: "graph-algorithms",
        title: "Graph Algorithms",
        content:
          "## Essential Graph Algorithms\n\n### Traversal:\n- BFS (Breadth-First Search)\n- DFS (Depth-First Search)\n\n### Shortest Path:\n- Dijkstra's algorithm\n- Bellman-Ford\n- Floyd-Warshall\n\n### Minimum Spanning Tree:\n- Kruskal's algorithm\n- Prim's algorithm",
      },
      {
        id: "dynamic-programming",
        title: "Dynamic Programming",
        content:
          "## DP in Rust\n\n### Memoization:\nStore results of expensive function calls.\n\n### Tabulation:\nBottom-up approach with arrays.\n\n### Classic Problems:\n- Fibonacci\n- Knapsack problem\n- Longest common subsequence\n- Edit distance",
      },
      {
        id: "greedy-algorithms",
        title: "Greedy Algorithms",
        content:
          "## Greedy Approach\n\nMake locally optimal choices.\n\n### Examples:\n- Activity selection\n- Huffman coding\n- Minimum coin change (not always optimal)\n- Fractional knapsack",
      },
      {
        id: "backtracking",
        title: "Backtracking",
        content:
          "## Backtracking Search\n\nSystematically search for solutions.\n\n### Applications:\n- N-Queens problem\n- Sudoku solver\n- Maze solving\n- Permutations and combinations",
      },
    ],
  },
  {
    id: "memory-management",
    title: "Advanced Memory Management",
    description: "Deep dive into Rust's memory model",
    estimatedReadTime: 100,
    difficulty: "advanced",
    sections: [
      {
        id: "stack-vs-heap",
        title: "Stack vs Heap",
        content:
          "## Memory Regions\n\n### Stack:\n- Fast allocation/deallocation\n- Fixed size at compile time\n- LIFO order\n- Limited size\n\n### Heap:\n- Dynamic size\n- Slower allocation\n- Arbitrary access patterns\n- Larger capacity",
      },
      {
        id: "allocators",
        title: "Custom Allocators",
        content:
          "## Writing Custom Allocators\n\nImplement the GlobalAlloc trait.\n\n### Use Cases:\n- Embedded systems\n- Real-time systems\n- Specialized memory pools\n\n### Safety:\n- Must uphold allocator API contract\n- Thread safety requirements",
      },
      {
        id: "memory-pools",
        title: "Memory Pools",
        content:
          "## Pool Allocation\n\nPre-allocate chunks of memory for objects.\n\n### Benefits:\n- Reduce fragmentation\n- Predictable performance\n- Cache locality\n\n### Implementation:\n- Free list\n- Slab allocation",
      },
    ],
  },
  {
    id: "web-development",
    title: "Web Development with Rust",
    description: "Building web applications",
    estimatedReadTime: 130,
    difficulty: "intermediate",
    sections: [
      {
        id: "actix-web",
        title: "Actix Web Framework",
        content:
          '## Actix Web\n\nPowerful, pragmatic, and fast web framework.\n\n### Features:\n- Async/await support\n- Type-safe routing\n- Middleware system\n- WebSocket support\n\n### Example:\n```rust\nuse actix_web::{web, App, HttpServer, Responder};\n\nasync fn greet(name: web::Path<String>) -> impl Responder {\n    format!("Hello {}!", name)\n}\n```',
      },
      {
        id: "rocket",
        title: "Rocket Framework",
        content:
          "## Rocket\n\nWeb framework with a focus on ease-of-use.\n\n### Features:\n- Code generation\n- Type-safe requests\n- Templating\n- Easy configuration",
      },
      {
        id: "axum",
        title: "Axum by Tokio",
        content:
          "## Axum\n\nErgonomic and modular web framework.\n\n### Design:\n- Tower ecosystem integration\n- Type-safe extractors\n- Middleware as layers\n- Minimal and flexible",
      },
      {
        id: "templating",
        title: "Template Engines",
        content:
          "## Server-Side Rendering\n\n### Askama:\nCompile-time templates with Rust type safety.\n\n### Tera:\nJinja2-inspired template engine.\n\n### Handlebars:\nLogic-less templates.",
      },
      {
        id: "api-design",
        title: "REST API Design",
        content:
          "## Building RESTful APIs\n\n### Best Practices:\n- Resource-oriented URLs\n- Proper HTTP methods\n- Status codes\n- Versioning\n- Documentation (OpenAPI)",
      },
    ],
  },
  {
    id: "testing-strategies",
    title: "Comprehensive Testing",
    description: "Testing methodologies and patterns",
    estimatedReadTime: 110,
    difficulty: "intermediate",
    sections: [
      {
        id: "test-pyramid",
        title: "Test Pyramid",
        content:
          "## Testing Strategy\n\n### Unit Tests:\n- Fast\n- Isolated\n- Many\n\n### Integration Tests:\n- Component interactions\n- Real dependencies\n\n### E2E Tests:\n- Full system\n- Slower\n- Critical paths",
      },
      {
        id: "property-testing",
        title: "Property-Based Testing",
        content:
          "## QuickCheck Style Testing\n\nUsing the proptest crate.\n\n### Benefits:\n- Generate test cases automatically\n- Find edge cases\n- Shrink failing cases\n\n### Example:\nTest that reversing a list twice gives the original.",
      },
      {
        id: "mutation-testing",
        title: "Mutation Testing",
        content:
          "## Verifying Test Quality\n\nAutomatically mutate code to check if tests catch changes.\n\n### Tools:\n- cargo-mutants\n\n### Process:\n1. Introduce mutations\n2. Run tests\n3. Check if mutations are caught\n4. Improve tests if not",
      },
      {
        id: "test-doubles",
        title: "Test Doubles",
        content:
          "## Mocks, Stubs, and Fakes\n\n### Stubs:\nProvide canned answers\n\n### Mocks:\nVerify interactions\n\n### Fakes:\nWorking implementations for testing\n\n### Spies:\nRecord interactions",
      },
    ],
  },
  {
    id: "performance-tuning",
    title: "Performance Optimization",
    description: "Making Rust code faster",
    estimatedReadTime: 115,
    difficulty: "advanced",
    sections: [
      {
        id: "benchmarking",
        title: "Benchmarking with Criterion",
        content:
          "## Statistical Benchmarking\n\nCriterion provides rigorous performance measurements.\n\n### Features:\n- Statistical analysis\n- HTML reports\n- Regression detection\n- Throughput measurements",
      },
      {
        id: "profiling-tools",
        title: "Profiling",
        content:
          "## Finding Bottlenecks\n\n### CPU Profiling:\n- perf (Linux)\n- samply (cross-platform)\n- cargo flamegraph\n\n### Memory Profiling:\n- heaptrack\n- valgrind (with massif)",
      },
      {
        id: "simd",
        title: "SIMD with packed_simd",
        content:
          "## Single Instruction Multiple Data\n\nProcess multiple data points in parallel.\n\n### Example:\nVectorized addition of arrays.\n\n### Portability:\n- Generic SIMD\n- Target-specific intrinsics",
      },
      {
        id: "unsafe-optimization",
        title: "Unsafe Optimizations",
        content:
          "## When to Use Unsafe for Speed\n\n### Scenarios:\n- Skipping bounds checks\n- Raw pointer arithmetic\n- Bypassing ownership\n\n### Safety:\n- Document invariants\n- Extensive testing\n- Minimize unsafe blocks",
      },
    ],
  },
  {
    id: "cross-compilation",
    title: "Cross-Compilation",
    description: "Building for different targets",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "targets",
        title: "Compilation Targets",
        content:
          "## Target Triples\n\nFormat: arch-vendor-os-env\n\n### Examples:\n- x86_64-unknown-linux-gnu\n- aarch64-apple-darwin\n- wasm32-unknown-unknown\n\n### Listing Targets:\n`rustup target list`",
      },
      {
        id: "cross-tool",
        title: "Using cross",
        content:
          "## Cross-Compilation Made Easy\n\nThe 'cross' tool uses Docker containers.\n\n### Setup:\n`cargo install cross`\n\n### Usage:\n`cross build --target aarch64-unknown-linux-gnu`\n\n### Benefits:\n- Consistent environment\n- No local toolchain needed",
      },
    ],
  },
  {
    id: "ci-cd",
    title: "Continuous Integration",
    description: "Automated testing and deployment",
    estimatedReadTime: 65,
    difficulty: "intermediate",
    sections: [
      {
        id: "github-actions",
        title: "GitHub Actions for Rust",
        content:
          "## CI/CD Pipelines\n\n### Basic Workflow:\n```yaml\nname: CI\non: [push, pull_request]\n\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v3\n      - uses: dtolnay/rust-toolchain@stable\n      - run: cargo test\n```",
      },
      {
        id: "release-automation",
        title: "Automated Releases",
        content:
          "## Release Automation\n\n### Tools:\n- cargo-release\n- semantic-release\n- GitHub Actions workflows\n\n### Process:\n1. Version bumping\n2. Changelog generation\n3. Git tagging\n4. Publishing to crates.io\n5. Binary releases",
      },
    ],
  },
  {
    id: "documentation",
    title: "Documentation Best Practices",
    description: "Writing great docs",
    estimatedReadTime: 70,
    difficulty: "beginner",
    sections: [
      {
        id: "doc-comments",
        title: "Documentation Comments",
        content:
          "## Writing Doc Comments\n\n### Style:\n- Start with a one-line summary\n- Provide detailed explanation\n- Include examples\n- Document panics, errors, safety\n\n### Sections:\n- # Examples\n- # Panics\n- # Errors\n- # Safety",
      },
      {
        id: "readme",
        title: "README Templates",
        content:
          "## Effective READMEs\n\n### Structure:\n1. Project name and description\n2. Badges (CI, version, license)\n3. Quick start\n4. Installation\n5. Usage examples\n6. Contributing\n7. License",
      },
      {
        id: "mdbook",
        title: "mdBook for Documentation",
        content:
          "## Documentation Websites\n\nmdBook creates beautiful documentation sites.\n\n### Features:\n- Markdown-based\n- Search functionality\n- Theming\n- Custom preprocessors\n\n### Used by:\n- The Rust Book\n- Rust by Example\n- Diesel guides",
      },
    ],
  },
  {
    id: "real-world-projects",
    title: "Real-World Project Examples",
    description: "Learning from existing projects",
    estimatedReadTime: 90,
    difficulty: "intermediate",
    sections: [
      {
        id: "ripgrep",
        title: "Case Study: ripgrep",
        content:
          "## Analyzing ripgrep\n\nLearn from one of the fastest grep implementations.\n\n### Key Techniques:\n- Memory-mapped files\n- Parallel directory traversal\n- Regex optimizations\n- Ignore patterns (.gitignore)",
      },
      {
        id: "alacritty",
        title: "Case Study: Alacritty",
        content:
          "## GPU-Accelerated Terminal\n\nStudy a modern terminal emulator.\n\n### Architecture:\n- OpenGL rendering\n- Event loop\n- Font rendering\n- Configuration system",
      },
      {
        id: "tokio",
        title: "Case Study: Tokio",
        content:
          "## Async Runtime Design\n\nLearn from the most popular async runtime.\n\n### Components:\n- Task scheduler\n- I/O drivers\n- Timer wheel\n- Thread pool",
      },
    ],
  },
  // Final Expansion to 280+ Sections
  {
    id: "rust-ecosystem",
    title: "The Rust Ecosystem",
    description: "Essential tools and libraries",
    estimatedReadTime: 100,
    difficulty: "beginner",
    sections: [
      {
        id: "cargo-tools",
        title: "Essential Cargo Tools",
        content:
          "## Cargo Ecosystem\n\n### cargo-expand\nExpand macros for debugging\n\n### cargo-tree\nView dependency tree\n\n### cargo-outdated\nFind outdated dependencies\n\n### cargo-audit\nCheck for security vulnerabilities\n\n### cargo-watch\nAuto-rebuild on changes",
      },
      {
        id: "clippy-lints",
        title: "Clippy for Code Quality",
        content:
          "## Using Clippy\n\nThe Rust linter catches common mistakes.\n\n### Enable All Lints:\n`cargo clippy -- -W clippy::all`\n\n### Deny Warnings in CI:\n`cargo clippy -- -D warnings`\n\n### Allow Specific Lints:\n`#[allow(clippy::module_name_repetitions)]`",
      },
      {
        id: "rustfmt",
        title: "Automatic Formatting",
        content:
          "## rustfmt\n\nAutomatic code formatting.\n\n### Configuration:\n.rustfmt.toml for project-specific settings\n\n### CI Integration:\nCheck formatting with `cargo fmt -- --check`\n\n### Editor Integration:\nFormat on save in VS Code, Vim, etc.",
      },
      {
        id: "rust-analyzer",
        title: "IDE Support",
        content:
          "## rust-analyzer\n\nModern language server for Rust.\n\n### Features:\n- Code completion\n- Go to definition\n- Find references\n- Refactoring\n- Type hints\n- Inlay hints",
      },
    ],
  },
  {
    id: "common-patterns",
    title: "Common Rust Patterns",
    description: "Idiomatic code patterns",
    estimatedReadTime: 85,
    difficulty: "intermediate",
    sections: [
      {
        id: "builder-pattern",
        title: "Builder Pattern",
        content:
          '## Building Objects\n\nConstruct complex objects step by step.\n\n```rust\nlet user = User::builder()\n    .name("Alice")\n    .email("alice@example.com")\n    .age(30)\n    .build();\n```',
      },
      {
        id: "visitor-pattern",
        title: "Visitor Pattern",
        content:
          "## Separating Operations from Data\n\nAdd operations without modifying structs.\n\n### Use Cases:\n- AST traversal\n- Document processing\n- Serialization",
      },
      {
        id: "strategy-pattern",
        title: "Strategy Pattern",
        content:
          "## Interchangeable Algorithms\n\nEncapsulate algorithms behind a common interface.\n\n### Example:\nDifferent sorting strategies for different data types.",
      },
      {
        id: "state-machine",
        title: "State Machines with Types",
        content:
          "## Encoding State in Type System\n\nMake invalid states unrepresentable.\n\n```rust\nstruct Idle;\nstruct Running;\nstruct Paused;\n\nstruct State<S> { ... }\n```",
      },
    ],
  },
  {
    id: "refactoring",
    title: "Refactoring Techniques",
    description: "Improving existing code",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "extract-function",
        title: "Extract Function",
        content:
          "## Breaking Down Large Functions\n\n### When to Extract:\n- Function does multiple things\n- Code duplication\n- Deep nesting\n\n### Benefits:\n- Improved readability\n- Easier testing\n- Code reuse",
      },
      {
        id: "reduce-cloning",
        title: "Reduce Cloning",
        content:
          "## Avoiding Unnecessary Copies\n\n### Strategies:\n- Use references instead of owned data\n- Leverage lifetimes\n- Use Cow for flexibility\n- Use slices (str, [T])",
      },
      {
        id: "error-refactoring",
        title: "Improving Error Handling",
        content:
          "## Better Error Messages\n\n### Techniques:\n- Add context with anyhow\n- Create custom error types\n- Use thiserror for libraries\n- Provide actionable suggestions",
      },
    ],
  },
  {
    id: "interoperability",
    title: "Language Interoperability",
    description: "Working with other languages",
    estimatedReadTime: 80,
    difficulty: "advanced",
    sections: [
      {
        id: "calling-c",
        title: "Calling C from Rust",
        content:
          "## FFI with C\n\n### bindgen:\nAuto-generate FFI bindings from C headers.\n\n### Manual Bindings:\nDefine extern functions and types manually.\n\n### Safety:\n- Raw pointers\n- Manual memory management\n- UB possibilities",
      },
      {
        id: "calling-rust",
        title: "Calling Rust from C",
        content:
          '## Exposing Rust to C\n\n### #[no_mangle]\nPrevent name mangling.\n\n### extern "C"\nUse C calling convention.\n\n### Box::into_raw\nTransfer ownership to C.',
      },
      {
        id: "python-bindings",
        title: "Python with PyO3",
        content:
          "## Writing Python Modules in Rust\n\n### PyO3:\nWrite Python extensions in Rust.\n\n### Benefits:\n- Speed up Python code\n- Safe parallelism\n- Easy to use\n\n### maturin:\nBuild and publish Python packages.",
      },
      {
        id: "wasm-bindings",
        title: "JavaScript with WASM",
        content:
          "## Rust in the Browser\n\n### wasm-bindgen:\nHigh-level bindings between Rust and JS.\n\n### web-sys:\nBindings to Web APIs.\n\n### Use Cases:\n- Games\n- Image processing\n- Cryptography\n- Simulations",
      },
    ],
  },
  {
    id: "deployment",
    title: "Deployment Strategies",
    description: "Shipping Rust applications",
    estimatedReadTime: 70,
    difficulty: "intermediate",
    sections: [
      {
        id: "static-linking",
        title: "Static Linking",
        content:
          "## Static vs Dynamic Linking\n\n### Static:\n- Self-contained binary\n- Larger file size\n- No runtime dependencies\n\n### Dynamic:\n- Smaller binary\n- Shared libraries\n- Requires runtime deps",
      },
      {
        id: "docker",
        title: "Docker Images",
        content:
          "## Containerizing Rust Apps\n\n### Multi-stage Build:\n1. Build stage with full toolchain\n2. Runtime stage with minimal base\n\n### Distroless:\nMinimal images with only the binary.\n\n### Alpine:\nSmall but musl-based.",
      },
      {
        id: "systemd",
        title: "Systemd Services",
        content:
          "## Running as System Service\n\n### Creating Service File:\n```ini\n[Unit]\nDescription=My Rust App\nAfter=network.target\n\n[Service]\nType=simple\nExecStart=/usr/local/bin/myapp\nRestart=on-failure\n\n[Install]\nWantedBy=multi-user.target\n```",
      },
    ],
  },
  {
    id: "maintenance",
    title: "Code Maintenance",
    description: "Keeping code healthy",
    estimatedReadTime: 65,
    difficulty: "intermediate",
    sections: [
      {
        id: "dependency-updates",
        title: "Managing Dependencies",
        content:
          "## Keeping Dependencies Fresh\n\n### cargo-outdated\nFind outdated dependencies.\n\n### cargo-update\nUpdate dependencies.\n\n### Dependabot\nAutomated PRs for updates.\n\n### Semver Compatibility\nUnderstand versioning.",
      },
      {
        id: "security-audits",
        title: "Security Auditing",
        content:
          "## cargo-audit\n\nCheck for known vulnerabilities in dependencies.\n\n### CI Integration:\nRun on every build.\n\n### Advisory DB:\nCommunity-maintained vulnerability database.",
      },
      {
        id: "code-reviews",
        title: "Effective Code Reviews",
        content:
          "## Reviewing Rust Code\n\n### Checklist:\n- Error handling\n- Ownership and lifetimes\n- Unnecessary clones\n- Panic possibilities\n- API ergonomics\n- Documentation",
      },
    ],
  },
  {
    id: "learning-resources",
    title: "Learning Resources",
    description: "Continue your Rust journey",
    estimatedReadTime: 50,
    difficulty: "beginner",
    sections: [
      {
        id: "official-docs",
        title: "Official Documentation",
        content:
          "## Essential Resources\n\n### The Rust Book:\nComprehensive guide (you're reading it!)\n\n### Rust by Example:\nLearn through examples\n\n### The Standard Library:\nAPI documentation\n\n### The Rustonomicon:\nUnsafe Rust\n\n### The Reference:\nLanguage specification",
      },
      {
        id: "community",
        title: "Community Resources",
        content:
          "## Getting Help\n\n### Forums:\n- users.rust-lang.org\n- /r/rust on Reddit\n\n### Chat:\n- Discord\n- Zulip\n- Matrix\n\n### Conferences:\n- RustConf\n- EuroRust\n- Rust Belt Rust",
      },
      {
        id: "practice-projects",
        title: "Project Ideas",
        content:
          "## Practice Makes Perfect\n\n### Beginner:\n- CLI calculator\n- File organizer\n- Weather app\n\n### Intermediate:\n- Web server\n- Chat application\n- Game (snake, tetris)\n\n### Advanced:\n- Database\n- Compiler\n- Operating system",
      },
    ],
  },
  {
    id: "embedded-rust",
    title: "Embedded Systems",
    description: "Rust on microcontrollers and embedded devices",
    estimatedReadTime: 90,
    difficulty: "advanced",
    sections: [
      {
        id: "no-std",
        title: "No Standard Library",
        content:
          "## Bare Metal Rust\n\n### #![no_std]:\nDisable the standard library for embedded systems.\n\n### Core Library:\n- Basic types\n- Iterators\n- Option/Result\n- No heap allocation\n\n### Embedded-HAL:\nHardware abstraction layer for embedded devices.",
      },
      {
        id: "rtic-framework",
        title: "RTIC Framework",
        content:
          "## Real-Time Interrupt-driven Concurrency\n\n### Features:\n- Zero-cost abstractions\n- Task scheduling\n- Resource sharing\n- Lock-free\n\n### Example:\n```rust\n#[app(device = stm32f4xx_hal::pac)]\nmod app {\n    #[init]\n    fn init() {}\n    #[task]\n    fn task1() {}\n}\n```",
      },
      {
        id: "defmt",
        title: "Defmt Logging",
        content:
          "## Embedded Logging\n\n### defmt:\nHighly efficient logging for embedded systems.\n\n### Benefits:\n- Minimal overhead\n- Host-side formatting\n- Binary encoding\n- Conditional compilation",
      },
      {
        id: "probe-rs",
        title: "Probe.rs Debugging",
        content:
          "## Debugging Embedded Rust\n\n### probe-rs:\nUniversal debugging tool for embedded.\n\n### Features:\n- Flash programming\n- RTT (Real-Time Transfer)\n- GDB server\n- VS Code integration",
      },
    ],
  },
  {
    id: "game-development",
    title: "Game Development",
    description: "Building games with Rust",
    estimatedReadTime: 85,
    difficulty: "intermediate",
    sections: [
      {
        id: "bevy-intro",
        title: "Bevy Game Engine",
        content:
          "## Data-Driven Game Engine\n\n### Features:\n- Entity Component System (ECS)\n- 2D and 3D rendering\n- Cross-platform\n- Hot reloading\n\n### ECS Pattern:\n- Entities: IDs\n- Components: Data\n- Systems: Logic",
      },
      {
        id: "ecs-pattern",
        title: "Entity Component System",
        content:
          "## Modern Game Architecture\n\n### Entities:\nUnique identifiers for game objects.\n\n### Components:\nPlain data structures.\n\n### Systems:\nFunctions that process components.\n\n### Benefits:\n- Cache-friendly\n- Parallelizable\n- Composable",
      },
      {
        id: "ggez-framework",
        title: "GGEZ Framework",
        content:
          "## 2D Game Framework\n\n### Inspired by Love2D:\nSimple and lightweight.\n\n### Features:\n- 2D graphics\n- Sound\n- Input handling\n- Event loop\n\n### Good for:\n- Prototyping\n- Small games\n- Learning",
      },
      {
        id: "wgpu-graphics",
        title: "WGPU Graphics API",
        content:
          "## Modern GPU API\n\n### Features:\n- Cross-platform\n- WebGPU standard\n- Low-level control\n- Safe Rust API\n\n### Use Cases:\n- Custom renderers\n- Game engines\n- Scientific visualization",
      },
    ],
  },
  {
    id: "network-programming",
    title: "Network Programming",
    description: "Building networked applications",
    estimatedReadTime: 80,
    difficulty: "intermediate",
    sections: [
      {
        id: "tcp-udp",
        title: "TCP and UDP",
        content:
          "## Low-Level Networking\n\n### TCP:\n- Reliable\n- Ordered\n- Connection-based\n\n### UDP:\n- Unreliable\n- Fast\n- Connectionless\n\n### Rust std::net:\nBasic networking primitives.",
      },
      {
        id: "async-net",
        title: "Async Networking",
        content:
          "## Non-blocking I/O\n\n### tokio::net:\n- TcpListener\n- TcpStream\n- UdpSocket\n\n### Benefits:\n- Handle many connections\n- Efficient resource use\n- Scalable servers",
      },
      {
        id: "protocol-design",
        title: "Protocol Design",
        content:
          "## Custom Protocols\n\n### Serialization:\n- bincode (binary)\n- serde_json\n- protobuf\n- Cap'n Proto\n\n### Design Principles:\n- Version compatibility\n- Backwards compatibility\n- Error handling",
      },
      {
        id: "webhooks-websockets",
        title: "Webhooks and WebSockets",
        content:
          "## Real-Time Communication\n\n### WebSockets:\n- Bidirectional\n- Persistent connection\n- Good for: chat, games, live updates\n\n### Webhooks:\n- HTTP callbacks\n- Event-driven\n- Good for: integrations, notifications",
      },
    ],
  },
  {
    id: "database-access",
    title: "Database Access",
    description: "Working with databases in Rust",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "sqlx-orm",
        title: "SQLx",
        content:
          '## Compile-Time Checked SQL\n\n### Features:\n- SQL in Rust code\n- Compile-time verification\n- Async support\n- No runtime codegen\n\n### Example:\n```rust\nsqlx::query!("SELECT * FROM users WHERE id = $1", id)\n```',
      },
      {
        id: "diesel-orm",
        title: "Diesel ORM",
        content:
          "## Type-Safe SQL\n\n### Features:\n- Query builder\n- Migrations\n- Type-safe queries\n- Multiple backends\n\n### Benefits:\n- No SQL injection\n- Compile-time checks\n- IDE support",
      },
      {
        id: "redis",
        title: "Redis Integration",
        content:
          "## In-Memory Data Store\n\n### Use Cases:\n- Caching\n- Sessions\n- Pub/Sub\n- Rate limiting\n\n### Rust Clients:\n- redis (sync/async)\n- fred (async)\n- bb8-redis (connection pooling)",
      },
      {
        id: "mongodb",
        title: "MongoDB Driver",
        content:
          "## Document Database\n\n### Features:\n- BSON documents\n- Flexible schema\n- Async/await\n- Typed queries\n\n### mongodb crate:\nOfficial Rust driver with full feature support.",
      },
    ],
  },
  {
    id: "testing-strategies",
    title: "Testing Strategies",
    description: "Comprehensive testing approaches",
    estimatedReadTime: 70,
    difficulty: "intermediate",
    sections: [
      {
        id: "unit-testing",
        title: "Unit Testing",
        content:
          "## Testing Individual Components\n\n### Best Practices:\n- Test one thing at a time\n- Use descriptive names\n- Arrange, Act, Assert\n- Test edge cases\n\n### Mocking:\n- mockall\n- mockito (HTTP)\n- Custom mocks",
      },
      {
        id: "integration-testing",
        title: "Integration Testing",
        content:
          "## Testing Full Workflows\n\n### Structure:\n- tests/ directory\n- Test external API\n- Use real dependencies\n\n### Database Testing:\n- Test transactions\n- Rollback after tests\n- Use test databases",
      },
      {
        id: "property-testing",
        title: "Property-Based Testing",
        content:
          "## Testing with Random Data\n\n### proptest:\nGenerate test cases automatically.\n\n### Benefits:\n- Find edge cases\n- Reproducible failures\n- Shrinking (minimal failing case)\n\n### Example:\nProperty: reverse(reverse(x)) == x",
      },
      {
        id: "benchmarking",
        title: "Performance Testing",
        content:
          "## Measuring Performance\n\n### Criterion:\nStatistics-driven benchmarking.\n\n### Features:\n- Statistical analysis\n- Comparison with baseline\n- Graphs and reports\n- Stable measurements",
      },
    ],
  },
  {
    id: "cli-development",
    title: "CLI Development",
    description: "Building command-line tools",
    estimatedReadTime: 65,
    difficulty: "beginner",
    sections: [
      {
        id: "clap-parser",
        title: "Clap Argument Parser",
        content:
          "## Powerful CLI Framework\n\n### Features:\n- Derive macros\n- Subcommands\n- Shell completions\n- Help generation\n- Validation\n\n### Example:\n```rust\n#[derive(Parser)]\nstruct Args {\n    #[arg(short, long)]\n    verbose: bool,\n}\n```",
      },
      {
        id: "interactive-cli",
        title: "Interactive Prompts",
        content:
          "## User Interaction\n\n### inquire:\n- Text input\n- Select menus\n- Confirmations\n- Multi-select\n- Autocomplete\n\n### dialoguer:\nCross-platform interactive prompts.",
      },
      {
        id: "progress-indicators",
        title: "Progress Bars",
        content:
          "## Visual Feedback\n\n### indicatif:\n- Progress bars\n- Spinners\n- Multi-progress\n- ETA calculation\n- Custom styles\n\n### Use for:\n- Long operations\n- File processing\n- Downloads",
      },
      {
        id: "terminal-ui",
        title: "Terminal User Interface",
        content:
          "## Rich Terminal Apps\n\n### ratatui:\n- Widgets\n- Layouts\n- Event handling\n- Crossterm backend\n\n### Use Cases:\n- Dashboards\n- File managers\n- System monitors",
      },
    ],
  },
  {
    id: "cryptography",
    title: "Cryptography",
    description: "Secure programming with Rust",
    estimatedReadTime: 85,
    difficulty: "advanced",
    sections: [
      {
        id: "hashing",
        title: "Hashing Algorithms",
        content:
          "## Cryptographic Hashes\n\n### Algorithms:\n- SHA-2/SHA-3\n- Blake2/Blake3\n- Argon2 (passwords)\n\n### Crates:\n- sha2\n- blake3\n- argon2\n\n### Use Cases:\n- Data integrity\n- Password storage\n- Content addressing",
      },
      {
        id: "encryption",
        title: "Encryption",
        content:
          "## Symmetric and Asymmetric\n\n### AES (Symmetric):\nFast encryption with shared key.\n\n### RSA/ECIES (Asymmetric):\nPublic-key cryptography.\n\n### AEAD:\nAuthenticated encryption (ChaCha20-Poly1305).",
      },
      {
        id: "digital-signatures",
        title: "Digital Signatures",
        content:
          "## Verifying Authenticity\n\n### Ed25519:\nModern, fast signatures.\n\n### ECDSA:\nElliptic curve signatures.\n\n### Applications:\n- Code signing\n- Document verification\n- Blockchain\n- API authentication",
      },
      {
        id: "secure-coding",
        title: "Secure Coding Practices",
        content:
          "## Writing Secure Rust\n\n### Principles:\n- Minimize unsafe code\n- Validate all inputs\n- Use constant-time operations\n- Avoid side channels\n- Keep dependencies updated\n\n### Tools:\n- cargo-audit\n- cargo-deny\n- Miri (undefined behavior)",
      },
    ],
  },
  {
    id: "webassembly",
    title: "WebAssembly",
    description: "Rust in the browser and beyond",
    estimatedReadTime: 75,
    difficulty: "intermediate",
    sections: [
      {
        id: "wasm-intro",
        title: "WebAssembly Basics",
        content:
          "## wasm32 Target\n\n### Compilation:\n```bash\nrustup target add wasm32-unknown-unknown\ncargo build --target wasm32-unknown-unknown\n```\n\n### Use Cases:\n- Browser apps\n- Serverless functions\n- Plugin systems\n- Edge computing",
      },
      {
        id: "wasm-bindgen",
        title: "wasm-bindgen",
        content:
          '## Rust-JS Interop\n\n### Features:\n- Export Rust functions to JS\n- Import JS functions to Rust\n- Pass complex types\n- Work with DOM\n\n### Example:\n```rust\n#[wasm_bindgen]\npub fn greet(name: &str) -> String {\n    format!("Hello, {}!", name)\n}\n```',
      },
      {
        id: "web-sys",
        title: "web-sys Crate",
        content:
          "## Web API Bindings\n\n### Features:\n- DOM manipulation\n- Canvas API\n- WebGL\n- Fetch API\n- WebSockets\n\n### Access browser APIs safely from Rust.",
      },
      {
        id: "wasm-pack",
        title: "wasm-pack",
        content:
          "## Build Tool\n\n### Features:\n- Build and package\n- Publish to npm\n- Generate TypeScript definitions\n- Webpack integration\n\n### Workflow:\n```bash\nwasm-pack build --target web\n```",
      },
    ],
  },
  {
    id: "serialization",
    title: "Serialization",
    description: "Encoding and decoding data",
    estimatedReadTime: 60,
    difficulty: "beginner",
    sections: [
      {
        id: "serde-intro",
        title: "Serde Framework",
        content:
          "## Serialization Framework\n\n### Features:\n- Serialize to JSON, YAML, TOML, etc.\n- Derive macros\n- Custom serializers\n- Zero-copy deserialization\n\n### Derives:\n- #[derive(Serialize)]\n- #[derive(Deserialize)]",
      },
      {
        id: "json-handling",
        title: "JSON with serde_json",
        content:
          "## JSON Processing\n\n### Parsing:\n```rust\nlet data: Value = serde_json::from_str(json)?;\n```\n\n### Serialization:\n```rust\nlet json = serde_json::to_string(&data)?;\n```\n\n### Typed parsing for safety.",
      },
      {
        id: "binary-formats",
        title: "Binary Formats",
        content:
          "## Efficient Binary Encoding\n\n### Formats:\n- bincode (native Rust)\n- MessagePack\n- CBOR\n- Protobuf\n- FlatBuffers\n\n### Benefits:\n- Smaller size\n- Faster parsing\n- Schema evolution",
      },
      {
        id: "custom-serde",
        title: "Custom Serialization",
        content:
          "## Advanced Serde\n\n### Custom Serialize/Deserialize:\n```rust\nimpl Serialize for MyType {\n    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n    where S: Serializer {\n        // Custom logic\n    }\n}\n```",
      },
    ],
  },
  {
    id: "operating-systems",
    title: "Operating Systems",
    description: "Building OS kernels in Rust",
    estimatedReadTime: 100,
    difficulty: "advanced",
    sections: [
      {
        id: "os-basics",
        title: "OS Development Basics",
        content:
          "## Writing an OS in Rust\n\n### Blog OS:\nPhilipp Oppermann's excellent tutorial.\n\n### Key Concepts:\n- Boot process\n- Memory management\n- Interrupts\n- System calls\n\n### Tools:\n- QEMU for testing\n- GRUB for booting",
      },
      {
        id: "kernel-structs",
        title: "Kernel Data Structures",
        content:
          "## OS Primitives\n\n### Needed:\n- Page tables\n- Process control blocks\n- File descriptors\n- Interrupt vectors\n\n### Safety:\n- Careful with unsafe\n- Invariants matter\n- Memory barriers",
      },
      {
        id: "drivers",
        title: "Device Drivers",
        content:
          "## Hardware Interface\n\n### Writing Drivers:\n- Memory-mapped I/O\n- Port I/O (x86)\n- Interrupt handlers\n- DMA\n\n### Rust Benefits:\n- Type safety\n- Ownership prevents use-after-free",
      },
      {
        id: "scheduling",
        title: "Process Scheduling",
        content:
          "## CPU Management\n\n### Scheduling Algorithms:\n- Round-robin\n- Priority-based\n- Multi-level feedback queue\n\n### Context Switching:\nSave and restore CPU state.",
      },
    ],
  },
  {
    id: "functional-programming",
    title: "Functional Programming",
    description: "Rust's functional side",
    estimatedReadTime: 55,
    difficulty: "intermediate",
    sections: [
      {
        id: "immutability",
        title: "Immutability by Default",
        content:
          "## Immutable by Design\n\n### Variables:\n```rust\nlet x = 5; // immutable\nlet mut y = 5; // mutable\n```\n\n### Benefits:\n- Thread safety\n- Easier reasoning\n- Fewer bugs\n- Better optimizations",
      },
      {
        id: "higher-order",
        title: "Higher-Order Functions",
        content:
          "## Functions as Values\n\n### Passing Functions:\n```rust\nfn apply<F>(f: F, x: i32) -> i32\nwhere F: Fn(i32) -> i32 {\n    f(x)\n}\n```\n\n### Common Patterns:\n- map, filter, fold\n- Closures\n- Combinators",
      },
      {
        id: "algebraic-types",
        title: "Algebraic Data Types",
        content:
          "## Sum and Product Types\n\n### Enums (Sum Types):\n```rust\nenum Option<T> {\n    Some(T),\n    None,\n}\n```\n\n### Structs (Product Types):\nCombine multiple values.\n\n### Pattern matching unlocks their power.",
      },
      {
        id: "monads",
        title: "Monadic Patterns",
        content:
          "## Option and Result as Monads\n\n### Bind Operation:\n```rust\nopt.and_then(|x| Some(x * 2))\n```\n\n### ? Operator:\nMonadic bind for early returns.\n\n### Functional Composition:\nChain operations elegantly.",
      },
    ],
  },
];
