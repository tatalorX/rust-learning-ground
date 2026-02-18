// Exercise 104: Linear Search Algorithm
// =====================================
//
// Learning Objective:
// Learn how to implement the Linear Search algorithm, which sequentially checks
// each element of the list until a match is found or the whole list has been
// searched. It's the simplest search algorithm.
//
// Time Complexity: O(n), Space Complexity: O(1)

fn main() {
    let numbers = vec![10, 20, 80, 30, 60, 50, 110, 100, 130, 170];
    
    // Test cases
    let test_cases = vec![(110, Some(6)), (200, None), (10, Some(0))];
    
    for (target, expected) in test_cases {
        let result = linear_search(&numbers, target);
        println!("Searching for {}: {:?} (expected: {:?})", target, result, expected);
        assert_eq!(result, expected);
    }
    
    println!("✓ Linear search completed successfully!");
}

/// Linear Search Implementation
///
/// Algorithm steps:
/// 1. Start from the leftmost element of the array
/// 2. One by one compare the target with each element
/// 3. If the target matches an element, return its index
/// 4. If the target doesn't match any element, return None
///
/// Hint: Use a simple loop to iterate through all elements.
/// Return the index as soon as you find a match.
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    // TODO: Implement linear search
    // Iterate through each element with its index
    for (index, &value) in arr.iter().enumerate() {
        // TODO: Check if current element matches target
        if value == target {
            return Some(index);
        }
    }
    
    // TODO: Return None if target not found
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_found() {
        let arr = vec![10, 20, 80, 30, 60, 50, 110, 100];
        assert_eq!(linear_search(&arr, 110), Some(6));
    }

    #[test]
    fn test_linear_search_not_found() {
        let arr = vec![10, 20, 80, 30, 60, 50];
        assert_eq!(linear_search(&arr, 200), None);
    }

    #[test]
    fn test_linear_search_first() {
        let arr = vec![10, 20, 30];
        assert_eq!(linear_search(&arr, 10), Some(0));
    }

    #[test]
    fn test_linear_search_last() {
        let arr = vec![10, 20, 30];
        assert_eq!(linear_search(&arr, 30), Some(2));
    }

    #[test]
    fn test_linear_search_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(linear_search(&arr, 10), None);
    }

    #[test]
    fn test_linear_search_single_found() {
        let arr = vec![42];
        assert_eq!(linear_search(&arr, 42), Some(0));
    }
}
