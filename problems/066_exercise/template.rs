// Exercise 066: Trait Bounds
//
// Trait bounds restrict generic types to those that implement certain traits.
// This ensures the generic code can call methods on the type.
//
// Learning Objectives:
// - Write trait bounds on generic functions
// - Use multiple trait bounds
// - Use where clauses for complex bounds
//
// Your task: Implement generic functions with appropriate trait bounds.

use std::fmt::Debug;

/// Returns the maximum of two values.
fn generic_max<T: PartialOrd>(a: T, b: T) -> T {
    // TODO: Return the larger of a and b
    todo!()
}

/// Finds the largest element in a slice.
fn find_largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    // TODO: Return reference to the largest element, or None if empty
    todo!()
}

/// Sorts a slice of items in place.
fn sort_items<T: Ord>(items: &mut [T]) {
    // TODO: Sort the items (use items.sort() from std)
    todo!()
}

/// Compares two items and returns their ordering.
fn compare_items<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    // TODO: Return the ordering of a relative to b
    // Use a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
    todo!()
}

/// Formats a collection of items for display.
fn format_collection<T: Debug>(items: &[T]) -> String {
    // TODO: Return a formatted string like "[item1, item2, item3]"
    // Use format!("{:?}", items)
    todo!()
}

/// Clones a vector of items.
fn clone_vec<T: Clone>(items: &[T]) -> Vec<T> {
    // TODO: Return a cloned vector
    todo!()
}

/// A function requiring multiple trait bounds.
fn process_and_display<T>(items: &[T]) -> String
where
    T: PartialOrd + Debug + Clone,
{
    // TODO: Find the largest item
    // Clone it and return a formatted string like "Largest: {:?}"
    todo!()
}

/// Combines two values into a pair if they're equal.
fn pair_if_equal<T: PartialEq>(a: T, b: T) -> Option<(T, T)> {
    // TODO: Return Some((a, b)) if a == b, None otherwise
    todo!()
}

/// Sums a collection of numeric values.
fn generic_sum<T>(items: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Clone + Default,
{
    // TODO: Sum all items, starting with T::default()
    // Note: You may need to use fold or a loop due to ownership
    todo!()
}

/// A generic wrapper that requires Display.
struct DisplayWrapper<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> DisplayWrapper<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
    
    fn display(&self) -> String {
        // TODO: Return format!("Wrapped: {}", self.value)
        todo!()
    }
}

fn main() {
    // After implementing, uncomment and run:
    
    // println!("Generic max:");
    // println!("  max(5, 10) = {}", generic_max(5, 10));
    // println!("  max(3.14, 2.71) = {}", generic_max(3.14, 2.71));
    // println!("  max('a', 'z') = {}", generic_max('a', 'z'));
    
    // let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // println!("\nFind largest in {:?}: {:?}", numbers, find_largest(&numbers));
    // println!("Find largest in []: {:?}", find_largest(&[] as &[i32]));
    
    // let mut to_sort = vec![64, 34, 25, 12, 22, 11, 90];
    // println!("\nSorting {:?}", to_sort);
    // sort_items(&mut to_sort);
    // println!("Sorted: {:?}", to_sort);
    
    // println!("\nComparisons:");
    // println!("  compare(5, 10) = {:?}", compare_items(&5, &10));
    // println!("  compare(10, 5) = {:?}", compare_items(&10, &5));
    // println!("  compare(5, 5) = {:?}", compare_items(&5, &5));
    
    // let items = vec![1, 2, 3];
    // println!("\nFormat collection: {}", format_collection(&items));
    // 
    // let cloned = clone_vec(&items);
    // println!("Cloned: {:?}", cloned);
    
    // println!("\nProcess and display: {}", process_and_display(&numbers));
    
    // println!("\nPair if equal:");
    // println!("  pair_if_equal(5, 5) = {:?}", pair_if_equal(5, 5));
    // println!("  pair_if_equal(5, 6) = {:?}", pair_if_equal(5, 6));
    
    // let nums = vec![1, 2, 3, 4, 5];
    // println!("\nGeneric sum of {:?} = {}", nums, generic_sum(&nums));
    
    // let wrapper = DisplayWrapper::new(42);
    // println!("\nDisplay wrapper: {}", wrapper.display());
    
    println!("Implement all TODOs to see the output!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_max() {
        assert_eq!(generic_max(5, 10), 10);
        assert_eq!(generic_max(10, 5), 10);
    }

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&[1, 5, 3]), Some(&5));
        assert_eq!(find_largest(&[5]), Some(&5));
    }

    #[test]
    fn test_find_largest_empty() {
        let empty: &[i32] = &[];
        assert_eq!(find_largest(empty), None);
    }

    #[test]
    fn test_sort_items() {
        let mut items = vec![3, 1, 4, 1, 5];
        sort_items(&mut items);
        assert_eq!(items, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_clone_vec() {
        let original = vec![1, 2, 3];
        let cloned = clone_vec(&original);
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_pair_if_equal() {
        assert_eq!(pair_if_equal(5, 5), Some((5, 5)));
        assert_eq!(pair_if_equal(5, 6), None);
    }

    #[test]
    fn test_generic_sum() {
        assert_eq!(generic_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(generic_sum(&[10, 20]), 30);
    }

    #[test]
    fn test_display_wrapper() {
        let w = DisplayWrapper::new(42);
        assert!(w.display().contains("42"));
    }
}
