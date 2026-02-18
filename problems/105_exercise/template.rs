// Exercise 105: Binary Search Algorithm
// =====================================
//
// Learning Objective:
// Learn how to implement the Binary Search algorithm, which efficiently finds
// an item from a sorted list of items. It works by repeatedly dividing in half
// the portion of the list that could contain the item.
//
// Time Complexity: O(log n), Space Complexity: O(1)
//
// IMPORTANT: Binary Search only works on sorted arrays!

fn main() {
    let numbers = vec![2, 3, 4, 10, 40, 50, 70, 80, 90, 100];
    
    println!("Sorted array: {:?}", numbers);
    
    // Test cases
    let test_cases = vec![(10, Some(3)), (100, Some(9)), (5, None), (2, Some(0))];
    
    for (target, expected) in test_cases {
        let result = binary_search(&numbers, target);
        println!("Searching for {}: {:?} (expected: {:?})", target, result, expected);
        assert_eq!(result, expected);
    }
    
    println!("✓ Binary search completed successfully!");
}

/// Binary Search Implementation (Iterative)
///
/// Algorithm steps:
/// 1. Compare the target with the middle element
/// 2. If target matches middle, return the middle index
/// 3. If target is greater, search in the right half
/// 4. If target is smaller, search in the left half
/// 5. Repeat until found or subarray size becomes 0
///
/// Hint: Use two pointers (left and right) to track the search range.
/// Calculate mid as left + (right - left) / 2 to avoid overflow.
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    // TODO: Initialize left and right pointers
    let mut left = 0;
    let mut right = arr.len() as isize - 1;
    
    // TODO: Implement the search loop
    // Continue while left pointer is less than or equal to right
    while left <= right {
        // TODO: Calculate middle index
        // Use this formula to prevent overflow: left + (right - left) / 2
        let mid = left + (right - left) / 2;
        let mid_idx = mid as usize;
        
        // TODO: Check if target is at mid
        if arr[mid_idx] == target {
            return Some(mid_idx);
        }
        
        // TODO: If target is greater, ignore left half
        if arr[mid_idx] < target {
            left = mid + 1;
        } else {
            // TODO: If target is smaller, ignore right half
            right = mid - 1;
        }
    }
    
    // TODO: Return None if target not found
    None
}

/// Binary Search Implementation (Recursive) - Bonus!
///
/// This is an optional recursive version of binary search.
/// Try implementing this after completing the iterative version.
fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: isize) -> Option<usize> {
    // TODO: Bonus - implement recursive binary search
    // Base case: if left > right, target not found
    if left as isize > right {
        return None;
    }
    
    // TODO: Calculate mid
    let mid = left + ((right - left as isize) / 2) as usize;
    
    // TODO: Check if target is at mid
    if arr[mid] == target {
        return Some(mid);
    }
    
    // TODO: Recursively search left or right half
    if arr[mid] > target {
        // Search left half
        if mid == 0 {
            return None;
        }
        return binary_search_recursive(arr, target, left, mid as isize - 1);
    } else {
        // Search right half
        return binary_search_recursive(arr, target, mid + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = vec![2, 3, 4, 10, 40];
        assert_eq!(binary_search(&arr, 10), Some(3));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = vec![2, 3, 4, 10, 40];
        assert_eq!(binary_search(&arr, 5), None);
    }

    #[test]
    fn test_binary_search_first() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 1), Some(0));
    }

    #[test]
    fn test_binary_search_last() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 5), Some(4));
    }

    #[test]
    fn test_binary_search_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(binary_search(&arr, 10), None);
    }

    #[test]
    fn test_binary_search_recursive() {
        let arr = vec![2, 3, 4, 10, 40, 50, 70];
        assert_eq!(binary_search_recursive(&arr, 10, 0, arr.len() as isize - 1), Some(3));
        assert_eq!(binary_search_recursive(&arr, 100, 0, arr.len() as isize - 1), None);
    }
}
