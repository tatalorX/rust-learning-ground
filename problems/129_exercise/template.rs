// Exercise 129: Counting Sort Algorithm
//
// Learning Objective:
// Implement counting sort, a non-comparison based sorting algorithm
// efficient for sorting integers with a known range.
//
// Key Concepts:
// - Non-comparison based sorting
// - Counting occurrences
// - Stable sorting
// - Time complexity: O(n + k) where k is the range of input
// - Space complexity: O(k)

/// TODO: Implement the counting_sort function
/// Sorts a vector of non-negative i32 using counting sort
/// 
/// Algorithm:
/// 1. Find the maximum element to determine the count array size
/// 2. Initialize a count array with zeros
/// 3. Store the count of each element
/// 4. Modify count array to store actual positions
/// 5. Build the output array using the count array
/// 
/// Note: For this exercise, assume all input values are non-negative
fn counting_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    // TODO: Find the maximum value in the array
    
    // TODO: Create a count array of size (max + 1) initialized with zeros
    
    // TODO: Count occurrences of each element
    
    // TODO: Modify count array to store cumulative counts
    // This gives us the actual position of elements in output
    
    // TODO: Create a temporary output array
    
    // TODO: Build the output array (iterate in reverse for stability)
    
    // TODO: Copy the output array back to the original array
}

/// Alternative: In-place counting sort that modifies the original array directly
/// This version modifies the array in-place without extra output array
fn counting_sort_in_place(arr: &mut [i32]) {
    // TODO: Implement an in-place version
    // Hint: Use the count array to determine how many times each element appears,
    // then overwrite the original array with sorted values
}

fn main() {
    // Test cases
    let mut arr1 = vec![4, 2, 2, 8, 3, 3, 1];
    println!("Original array: {:?}", arr1);
    counting_sort(&mut arr1);
    println!("Sorted array: {:?}", arr1);
    
    let mut arr2 = vec![1, 4, 1, 2, 7, 5, 2];
    println!("\nOriginal array: {:?}", arr2);
    counting_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
    
    let mut arr3 = vec![5, 5, 5, 1, 1, 1, 3];
    println!("\nOriginal array: {:?}", arr3);
    counting_sort(&mut arr3);
    println!("Sorted array: {:?}", arr3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_counting_sort_basic() {
        let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }
    
    #[test]
    fn test_counting_sort_with_duplicates() {
        let mut arr = vec![1, 4, 1, 2, 7, 5, 2];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 4, 5, 7]);
    }
    
    #[test]
    fn test_counting_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_counting_sort_single_element() {
        let mut arr = vec![42];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_counting_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_counting_sort_zeros() {
        let mut arr = vec![0, 0, 0, 1, 0, 2, 0];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![0, 0, 0, 0, 0, 1, 2]);
    }
}
