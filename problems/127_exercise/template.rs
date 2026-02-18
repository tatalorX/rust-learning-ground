// Exercise 127: Quick Sort Algorithm
//
// Learning Objective:
// Understand and implement the quick sort algorithm, an efficient in-place
// sorting algorithm using the divide-and-conquer paradigm.
//
// Key Concepts:
// - Pivot selection
// - Partitioning
// - In-place sorting (O(log n) space for recursion)
// - Average time complexity: O(n log n)
// - Worst case time complexity: O(n²)

/// TODO: Implement the quick_sort function
/// Sorts a slice of i32 in-place using the quick sort algorithm
/// 
/// Algorithm:
/// 1. If the array has 0 or 1 elements, it's already sorted (base case)
/// 2. Select a pivot element
/// 3. Partition the array such that elements < pivot are on the left,
///    elements > pivot are on the right
/// 4. Recursively sort the left and right partitions
fn quick_sort(arr: &mut [i32]) {
    // TODO: Implement base case - if len <= 1, return
    
    // TODO: Partition the array and get the pivot index
    
    // TODO: Recursively sort elements before the pivot
    
    // TODO: Recursively sort elements after the pivot
}

/// TODO: Implement the partition function
/// 
/// Rearranges elements in arr such that:
/// - All elements less than pivot come before it
/// - All elements greater than pivot come after it
/// 
/// Returns the final index of the pivot element
/// 
/// Hint: Use the last element as pivot (Lomuto partition scheme)
fn partition(arr: &mut [i32]) -> usize {
    // TODO: Choose the last element as pivot
    
    // TODO: Initialize index of smaller element
    
    // TODO: Traverse through all elements
    // If current element is smaller than or equal to pivot,
    // increment index of smaller element and swap
    
    // TODO: Place pivot in its correct position and return its index
    
    0 // Placeholder return
}

fn main() {
    // Test cases
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr1);
    quick_sort(&mut arr1);
    println!("Sorted array: {:?}", arr1);
    
    let mut arr2 = vec![3, -1, 0, 5, -2];
    println!("\nOriginal array: {:?}", arr2);
    quick_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
    
    let mut arr3: Vec<i32> = vec![];
    println!("\nOriginal array: {:?}", arr3);
    quick_sort(&mut arr3);
    println!("Sorted array: {:?}", arr3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_sort_basic() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_quick_sort_with_negatives() {
        let mut arr = vec![3, -1, 0, 5, -2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![-2, -1, 0, 3, 5]);
    }
    
    #[test]
    fn test_quick_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_quick_sort_single_element() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_quick_sort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
