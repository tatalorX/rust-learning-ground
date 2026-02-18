// Exercise 126: Merge Sort Algorithm
//
// Learning Objective:
// Understand and implement the merge sort algorithm, a classic divide-and-conquer
// sorting algorithm with O(n log n) time complexity in all cases.
//
// Key Concepts:
// - Divide and conquer strategy
// - Recursion
// - Merging two sorted arrays
// - Time complexity: O(n log n)
// - Space complexity: O(n)

/// TODO: Implement the merge_sort function
/// This function should sort a vector of i32 using the merge sort algorithm
/// 
/// Algorithm:
/// 1. If the array has 0 or 1 elements, it's already sorted (base case)
/// 2. Divide the array into two halves
/// 3. Recursively sort both halves
/// 4. Merge the two sorted halves into a single sorted array
fn merge_sort(arr: &mut [i32]) {
    // TODO: Implement the base case - if len <= 1, return
    
    // TODO: Find the middle index to split the array
    
    // TODO: Recursively sort the left half
    
    // TODO: Recursively sort the right half
    
    // TODO: Merge the two sorted halves
}

/// TODO: Implement the merge function
/// Merges two sorted subarrays: arr[left..mid] and arr[mid..right]
/// 
/// Hint: Create temporary vectors to hold the left and right halves,
/// then merge them back into the original array in sorted order
fn merge(arr: &mut [i32], mid: usize) {
    // TODO: Create temporary vectors for left and right halves
    
    // TODO: Merge the temporary vectors back into arr
    // Use three indices: i for left, j for right, k for arr
    
    // TODO: Copy any remaining elements from left
    
    // TODO: Copy any remaining elements from right
}

fn main() {
    // Test cases
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr1);
    merge_sort(&mut arr1);
    println!("Sorted array: {:?}", arr1);
    
    let mut arr2 = vec![3, -1, 0, 5, -2];
    println!("\nOriginal array: {:?}", arr2);
    merge_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
    
    let mut arr3: Vec<i32> = vec![];
    println!("\nOriginal array: {:?}", arr3);
    merge_sort(&mut arr3);
    println!("Sorted array: {:?}", arr3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merge_sort_basic() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_merge_sort_with_negatives() {
        let mut arr = vec![3, -1, 0, 5, -2];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![-2, -1, 0, 3, 5]);
    }
    
    #[test]
    fn test_merge_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_merge_sort_single_element() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_merge_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_merge_sort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
