// Exercise 096: Modules - use Keyword
//
// Learning objective: Master the `use` keyword for bringing
// items into scope, including aliases, nested imports, and glob imports.
//
// `use` brings items into scope so you don't need full paths.
// It supports various patterns for different use cases.

// TODO: Create a module hierarchy to import from
mod network {
    pub mod tcp {
        pub struct Connection;
        impl Connection {
            pub fn new() -> Self { Connection }
            pub fn connect(&self) { println!("TCP connected"); }
        }
    }
    
    pub mod udp {
        pub struct Socket;
        impl Socket {
            pub fn new() -> Self { Socket }
            pub fn bind(&self) { println!("UDP bound"); }
        }
    }
}

mod data {
    pub struct Record {
        pub id: u32,
        pub name: String,
    }
    
    pub mod storage {
        pub fn save() { println!("Saved to storage"); }
        pub fn load() { println!("Loaded from storage"); }
        
        pub mod formats {
            pub fn json() { println!("JSON format"); }
            pub fn yaml() { println!("YAML format"); }
            pub fn xml() { println!("XML format"); }
        }
    }
}

// TODO: Basic use statements
use network::tcp::Connection;
use network::udp::Socket as UdpSocket;

// TODO: Nested imports
use data::storage::{save, load, formats::{json, yaml}};

// TODO: Glob import (use sparingly)
use std::collections::*;

// TODO: Self in use statements
use data::Record;

// TODO: Re-exporting
pub use network::tcp::Connection as TcpConnection;
pub use data::storage::formats;

fn main() {
    // TODO: Use the imported items
    println!("=== Basic use ===");
    let conn = Connection::new();
    conn.connect();
    
    let udp = UdpSocket::new();
    udp.bind();
    
    // TODO: Use nested imports
    println!("\n=== Nested imports ===");
    save();
    load();
    json();
    yaml();
    
    // TODO: Use glob import
    println!("\n=== Glob import ===");
    let _map: HashMap<String, i32> = HashMap::new();
    let _set: HashSet<i32> = HashSet::new();
    let _vec: Vec<i32> = Vec::new(); // Vec is in prelude, but HashMap/Set need import
    
    // TODO: Use the re-exported items
    println!("\n=== Re-exports ===");
    let tcp = TcpConnection::new();
    tcp.connect();
    formats::json();
    
    // TODO: Create and use data::Record
    println!("\n=== Using Record ===");
    let record = Record {
        id: 1,
        name: String::from("Test"),
    };
    println!("Record: id={}, name={}", record.id, record.name);
}

// TODO: Demonstrate use within a function
fn local_imports() {
    // Can use `use` inside functions too
    use std::io::{self, Write};
    
    // io and Write are now available
    let _stdout = io::stdout();
}

// TODO: Demonstrate shadowing with use
mod shadowing {
    pub fn action() {
        println!("Action from shadowing module");
    }
}

fn shadowing_demo() {
    use shadowing::action;
    action(); // Calls shadowing::action
}

// TODO: Complete this module with multiple use patterns
mod imports_demo {
    // Import multiple items from std::vec
    pub use std::vec::Vec;
    
    // Import with deep nesting
    pub use std::collections::hash_map::{HashMap, Entry};
    
    // Import enum variants
    pub use std::option::Option::{Some, None};
    
    // Type alias via use
    pub type StringVec = Vec<String>;
}

// TODO: Create a module that uses prelude-like pattern
mod my_prelude {
    pub use data::Record;
    pub use network::tcp::Connection;
}

fn use_prelude() {
    use my_prelude::*;
    
    let _record = Record { id: 1, name: String::from("test") };
    let _conn = Connection::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_imports() {
        let conn = Connection::new();
        conn.connect();
        
        let udp = UdpSocket::new();
        udp.bind();
    }
    
    #[test]
    fn test_nested_imports() {
        save();
        load();
        json();
    }
    
    #[test]
    fn test_reexports() {
        let tcp = TcpConnection::new();
        tcp.connect();
    }
    
    #[test]
    fn test_imports_demo() {
        use imports_demo::*;
        
        let _v: StringVec = Vec::new();
        let _map: HashMap<String, i32> = HashMap::new();
    }
}
