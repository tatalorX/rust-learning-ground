// Exercise 094: Smart Pointers - Drop Trait
//
// Learning objective: Learn how to implement the Drop trait
// for custom cleanup code when values go out of scope.
//
// Drop is called automatically when a value goes out of scope.
// It's used for resource cleanup (memory, files, locks, etc.).
// std::mem::drop can be used to drop early.

use std::ops::Drop;
use std::mem;

// TODO: Create a struct that implements Drop for logging
struct Logger {
    name: String,
}

impl Logger {
    fn new(name: &str) -> Self {
        println!("Logger '{}' created", name);
        Logger {
            name: name.to_string(),
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        println!("Logger '{}' dropped", self.name);
    }
}

// TODO: Create a resource manager that tracks allocations
struct Resource {
    id: u32,
    data: String,
}

impl Resource {
    fn new(id: u32, data: &str) -> Self {
        println!("Resource {} allocated with data: {}", id, data);
        Resource {
            id,
            data: data.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource {} deallocated, data was: {}", self.id, self.data);
    }
}

// TODO: Create a file handle simulator
struct FileHandle {
    filename: String,
    is_open: bool,
}

impl FileHandle {
    fn open(filename: &str) -> Self {
        println!("Opening file: {}", filename);
        FileHandle {
            filename: filename.to_string(),
            is_open: true,
        }
    }
    
    fn close(&mut self) {
        if self.is_open {
            println!("Explicitly closing file: {}", self.filename);
            self.is_open = false;
        }
    }
    
    fn read(&self) -> Option<String> {
        if self.is_open {
            Some(format!("Contents of {}", self.filename))
        } else {
            None
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if self.is_open {
            println!("Auto-closing file in Drop: {}", self.filename);
        }
    }
}

// TODO: Create a struct that can only be dropped once
struct UniquePtr<T> {
    ptr: *mut T,
    is_dropped: bool,
}

impl<T> UniquePtr<T> {
    fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        UniquePtr { ptr, is_dropped: false }
    }
    
    fn get(&self) -> Option<&T> {
        if self.is_dropped {
            None
        } else {
            Some(unsafe { &*self.ptr })
        }
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        if !self.is_dropped {
            unsafe {
                let _ = Box::from_raw(self.ptr);
            }
            println!("UniquePtr dropped, memory freed");
            self.is_dropped = true;
        }
    }
}

fn main() {
    // TODO: Demonstrate basic Drop behavior
    println!("=== Creating Logger ===");
    let logger = Logger::new("main");
    println!("About to exit block...");
    drop(logger);
    println!("After explicit drop");
    
    println!("\n=== Scope-based Drop ===");
    {
        let _inner = Logger::new("inner");
        println!("Inside block");
    } // _inner dropped here automatically
    println!("After block");
    
    println!("\n=== Multiple Resources ===");
    let res1 = Resource::new(1, "First");
    let res2 = Resource::new(2, "Second");
    drop(res1); // Explicitly drop first
    println!("After dropping res1");
    drop(res2);
    println!("After dropping res2");
    
    println!("\n=== File Handle ===");
    {
        let mut file = FileHandle::open("data.txt");
        println!("Reading: {:?}", file.read());
        // File will be auto-closed when dropped
    }
    
    println!("\n=== Early Drop with mem::drop ===");
    let file2 = FileHandle::open("temp.txt");
    mem::drop(file2);
    println!("After mem::drop");
    
    println!("\n=== Drop Order ===");
    let _a = Logger::new("A");
    let _b = Logger::new("B");
    let _c = Logger::new("C");
    // Dropped in reverse order: C, B, A
    println!("Exiting main...");
}

// TODO: Complete this function demonstrating RAII pattern
fn use_resource() {
    let resource = Resource::new(99, "temporary");
    println!("Using resource...");
    // Resource automatically cleaned up when function returns
}

// TODO: Complete this function that drops a value conditionally
fn conditional_drop<T>(item: T, should_drop: bool) {
    if should_drop {
        // YOUR CODE HERE - drop the item
        drop(item);
        println!("Item was dropped");
    } else {
        println!("Item not dropped, it will be dropped at end of scope");
        // Item will be dropped when function returns
        mem::forget(item); // Actually don't drop it - this is for demonstration
        // Note: mem::forget is usually not recommended, used here for demo
    }
}

// TODO: Create a struct that runs a callback on drop
struct OnDrop<F: FnOnce()> {
    callback: Option<F>,
}

impl<F: FnOnce()> OnDrop<F> {
    fn new(callback: F) -> Self {
        OnDrop {
            callback: Some(callback),
        }
    }
    
    fn cancel(&mut self) {
        self.callback = None;
    }
}

impl<F: FnOnce()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        if let Some(callback) = self.callback.take() {
            callback();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_handle() {
        let file = FileHandle::open("test.txt");
        assert!(file.read().is_some());
    }

    #[test]
    fn test_unique_ptr() {
        let ptr = UniquePtr::new(42);
        assert_eq!(*ptr.get().unwrap(), 42);
    }

    #[test]
    fn test_on_drop() {
        let mut dropped = false;
        {
            let _guard = OnDrop::new(|| {
                dropped = true;
            });
        }
        assert!(dropped);
    }

    #[test]
    fn test_on_drop_cancel() {
        let mut dropped = false;
        {
            let mut guard = OnDrop::new(|| {
                dropped = true;
            });
            guard.cancel();
        }
        assert!(!dropped);
    }
}
