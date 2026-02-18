// Exercise 072: Closures - Capturing Environment
//
// Closures can capture variables from their environment in different ways:
// by reference (&T), by mutable reference (&mut T), or by value (T).
//
// Learning Objectives:
// - Understand capture modes: by reference, mutable reference, and by value
// - Use the move keyword
// - Understand Fn, FnMut, and FnOnce traits
//
// Your task: Create closures that capture their environment in different ways.

/// Creates a closure that captures by immutable reference.
fn capture_by_reference(items: &Vec<i32>) -> impl Fn() -> usize + '_ {
    // TODO: Return a closure that returns the length of items
    // This captures items by reference
    todo!()
}

/// Creates a closure that captures by mutable reference.
fn capture_by_mutable_reference(count: &mut i32) -> impl FnMut() + '_ {
    // TODO: Return a closure that increments count and prints it
    // This captures count by mutable reference
    todo!()
}

/// Creates a closure that captures by value (takes ownership).
fn capture_by_value(name: String) -> impl FnOnce() -> String {
    // TODO: Return a closure that consumes and returns name
    // Use move keyword to capture by value
    todo!()
}

/// Creates a closure that sorts a vector in place.
fn make_sorter<T: Ord>(ascending: bool) -> impl FnMut(&mut Vec<T>) {
    // TODO: Return a closure that sorts the vector
    // If ascending, sort normally; if descending, sort in reverse
    todo!()
}

/// Creates a filter function based on a threshold.
fn make_filter(min_value: i32) -> impl Fn(&i32) -> bool {
    // TODO: Return a closure that returns true if value >= min_value
    todo!()
}

/// Creates a closure that formats strings with a prefix and suffix.
fn make_formatter(prefix: String, suffix: String) -> impl Fn(&str) -> String {
    // TODO: Return a closure that formats: "{prefix}{text}{suffix}"
    // Use move to capture both strings by value
    todo!()
}

/// Demonstrates FnOnce by consuming a value.
fn create_consumer<T>(data: Vec<T>) -> impl FnOnce() -> Vec<T> {
    // TODO: Return a closure that takes ownership of data and returns it
    todo!()
}

/// Creates a closure that modifies captured state on each call.
fn make_stateful_processor() -> impl FnMut(i32) -> i32 {
    // TODO: Create a closure with internal state (sum and count)
    // On each call: add input to sum, increment count, return average
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // println!("Capture by reference:");
    // let items = vec![1, 2, 3, 4, 5];
    // let get_len = capture_by_reference(&items);
    // println!("  Length: {}", get_len());
    // println!("  Length again: {}", get_len());
    // // items is still usable here!
    // println!("  Items still available: {:?}", items);
    // 
    // println!("\nCapture by mutable reference:");
    // let mut count = 0;
    // let mut increment = capture_by_mutable_reference(&mut count);
    // increment();
    // increment();
    // increment();
    // println!("  Final count: {}", count);
    // 
    // println!("\nCapture by value:");
    // let name = String::from("Alice");
    // let consume = capture_by_value(name);
    // // name is no longer usable here!
    // println!("  Consumed: {}", consume());
    // 
    // println!("\nSorter closure:");
    // let mut sorter: Box<dyn FnMut(&mut Vec<i32>)> = Box::new(make_sorter(true));
    // let mut nums = vec![3, 1, 4, 1, 5];
    // sorter(&mut nums);
    // println!("  Ascending: {:?}", nums);
    // let mut reverse_sorter = make_sorter(false);
    // reverse_sorter(&mut nums);
    // println!("  Descending: {:?}", nums);
    // 
    // println!("\nFilter closure:");
    // let is_positive = make_filter(0);
    // let numbers = vec![-5, 3, -2, 8, 0, 10];
    // let positive: Vec<_> = numbers.iter().filter(|&&n| is_positive(&n)).collect();
    // println!("  Positive numbers: {:?}", positive);
    // 
    // println!("\nFormatter closure:");
    // let bracket_formatter = make_formatter("[".to_string(), "]".to_string());
    // println!("  {}", bracket_formatter("hello"));
    // 
    // println!("\nStateful processor:");
    // let mut processor = make_stateful_processor();
    // println!("  Process 10: {}", processor(10));
    // println!("  Process 20: {}", processor(20));
    // println!("  Process 30: {}", processor(30));
    
    println!("Implement all TODOs to see closure captures in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_by_reference() {
        let items = vec![1, 2, 3];
        let get_len = capture_by_reference(&items);
        assert_eq!(get_len(), 3);
        assert_eq!(get_len(), 3); // Can call multiple times
        drop(get_len);
        // items is still valid
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_capture_by_mutable_reference() {
        let mut count = 0;
        {
            let mut increment = capture_by_mutable_reference(&mut count);
            increment();
            increment();
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_capture_by_value() {
        let name = String::from("test");
        let consume = capture_by_value(name);
        assert_eq!(consume(), "test");
    }

    #[test]
    fn test_make_sorter_ascending() {
        let mut sorter = make_sorter::<i32>(true);
        let mut v = vec![3, 1, 2];
        sorter(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_make_sorter_descending() {
        let mut sorter = make_sorter::<i32>(false);
        let mut v = vec![1, 3, 2];
        sorter(&mut v);
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn test_make_filter() {
        let filter = make_filter(5);
        assert!(filter(&10));
        assert!(filter(&5));
        assert!(!filter(&3));
        assert!(!filter(&-1));
    }

    #[test]
    fn test_make_formatter() {
        let fmt = make_formatter("(".to_string(), ")".to_string());
        assert_eq!(fmt("test"), "(test)");
    }

    #[test]
    fn test_create_consumer() {
        let data = vec![1, 2, 3];
        let consumer = create_consumer(data);
        assert_eq!(consumer(), vec![1, 2, 3]);
    }

    #[test]
    fn test_make_stateful_processor() {
        let mut proc = make_stateful_processor();
        assert_eq!(proc(10), 10); // average of [10]
        assert_eq!(proc(20), 15); // average of [10, 20]
        assert_eq!(proc(30), 20); // average of [10, 20, 30]
    }
}
