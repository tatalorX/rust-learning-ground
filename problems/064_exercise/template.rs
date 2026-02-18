// Exercise 064: Traits - Default Implementations
//
// Traits can provide default implementations for methods.
// Implementors can use the default or override it.
//
// Learning Objectives:
// - Write default method implementations in traits
// - Override defaults when necessary
// - Use default implementations based on other trait methods
//
// Your task: Implement traits with smart default behaviors.

/// A trait for printable objects with default formatting.
trait Printable {
    /// Returns the content to print - must be implemented.
    fn content(&self) -> String;
    
    /// Returns a prefix - has default implementation.
    fn prefix(&self) -> String {
        String::from("[INFO]")
    }
    
    /// Formats the full output - uses other methods.
    fn format(&self) -> String {
        format!("{} {}", self.prefix(), self.content())
    }
    
    /// Prints to stdout - uses format().
    fn print(&self) {
        println!("{}", self.format());
    }
}

/// A trait for comparable objects with utility methods.
trait Comparable {
    /// Compare self with other - must be implemented.
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
    
    /// Returns true if self < other.
    fn less_than(&self, other: &Self) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Less)
    }
    
    /// Returns true if self > other.
    fn greater_than(&self, other: &Self) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Greater)
    }
    
    /// Returns true if self == other.
    fn equals(&self, other: &Self) -> bool {
        matches!(self.compare(other), std::cmp::Ordering::Equal)
    }
    
    /// Returns min and max of two values.
    fn min_max(self, other: Self) -> (Self, Self) 
    where 
        Self: Sized 
    {
        if self.less_than(&other) {
            (self, other)
        } else {
            (other, self)
        }
    }
}

/// A trait for timed operations.
trait Timed {
    /// Returns the timestamp - must be implemented.
    fn timestamp(&self) -> u64;
    
    /// Returns the current time as a formatted string.
    fn formatted_time(&self) -> String {
        // Default: just show raw timestamp
        format!("{}", self.timestamp())
    }
    
    /// Checks if this timestamp is older than another.
    fn is_older_than(&self, other: &Self) -> bool 
    where 
        Self: Sized 
    {
        self.timestamp() < other.timestamp()
    }
    
    /// Returns the age in seconds from a reference time.
    fn age_in_seconds(&self, current_time: u64) -> u64 {
        current_time.saturating_sub(self.timestamp())
    }
}

// Types to implement traits for
struct Message {
    text: String,
    level: LogLevel,
}

enum LogLevel {
    Info,
    Warning,
    Error,
}

struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

struct Event {
    name: String,
    time: u64, // Unix timestamp
}

// TODO: Implement Printable for Message
// - Override prefix() to return [INFO], [WARN], or [ERROR] based on level
// - content() should return the message text

// TODO: Implement Comparable for Version
// - Compare major, then minor, then patch versions
// - Use this to get all comparison methods for free

// TODO: Implement Timed for Event
// - Override formatted_time() to show a human-readable format
// - You can use simple formatting like "Day {}" (time / 86400)

fn main() {
    // After implementing, uncomment and run:
    
    // let messages = vec![
    //     Message { text: "System started".to_string(), level: LogLevel::Info },
    //     Message { text: "Low memory".to_string(), level: LogLevel::Warning },
    //     Message { text: "Disk full".to_string(), level: LogLevel::Error },
    // ];
    
    // println!("Messages:");
    // for msg in &messages {
    //     msg.print();
    // }
    
    // let v1 = Version { major: 1, minor: 0, patch: 0 };
    // let v2 = Version { major: 1, minor: 5, patch: 2 };
    // let v3 = Version { major: 2, minor: 0, patch: 0 };
    
    // println!("\nVersion comparisons:");
    // println!("v1 < v2: {}", v1.less_than(&v2));
    // println!("v2 == v2: {}", v2.equals(&v2));
    // println!("v3 > v1: {}", v3.greater_than(&v1));
    // 
    // let (min, max) = v1.min_max(v3);
    // println!("Min: {}.{}.{} Max: {}.{}.{}", 
    //     min.major, min.minor, min.patch,
    //     max.major, max.minor, max.patch);
    
    // let event = Event { name: "System boot".to_string(), time: 1000000 };
    // println!("\nEvent:");
    // println!("  Name: {}", event.name);
    // println!("  Time: {}", event.formatted_time());
    // println!("  Age: {} seconds", event.age_in_seconds(2000000));
    
    println!("Implement all TODOs to see the output!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_info_prefix() {
        let msg = Message { 
            text: "test".to_string(), 
            level: LogLevel::Info 
        };
        assert!(msg.prefix().contains("INFO"));
    }

    #[test]
    fn test_message_error_prefix() {
        let msg = Message { 
            text: "error".to_string(), 
            level: LogLevel::Error 
        };
        assert!(msg.prefix().contains("ERROR"));
    }

    #[test]
    fn test_version_compare_major() {
        let v1 = Version { major: 1, minor: 0, patch: 0 };
        let v2 = Version { major: 2, minor: 0, patch: 0 };
        assert!(v1.less_than(&v2));
    }

    #[test]
    fn test_version_compare_minor() {
        let v1 = Version { major: 1, minor: 0, patch: 0 };
        let v2 = Version { major: 1, minor: 5, patch: 0 };
        assert!(v1.less_than(&v2));
    }

    #[test]
    fn test_version_equals() {
        let v1 = Version { major: 1, minor: 2, patch: 3 };
        let v2 = Version { major: 1, minor: 2, patch: 3 };
        assert!(v1.equals(&v2));
    }

    #[test]
    fn test_event_age() {
        let event = Event { 
            name: "test".to_string(), 
            time: 1000 
        };
        assert_eq!(event.age_in_seconds(2000), 1000);
    }

    #[test]
    fn test_event_age_saturating() {
        let event = Event { 
            name: "test".to_string(), 
            time: 2000 
        };
        assert_eq!(event.age_in_seconds(1000), 0);
    }
}
