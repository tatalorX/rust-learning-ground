// Exercise 102: Selection Sort Algorithm
// ======================================
//
// Learning Objective:
// Learn how to implement the Selection Sort algorithm, which divides the input
// list into two parts: a sorted portion at the left end and an unsorted portion
// at the right end. Initially, the sorted portion is empty and the unsorted
// portion contains all elements.
//
// Time Complexity: O(n²), Space Complexity: O(1)

fn main() {
    let mut numbers = vec![64, 25, 12, 22, 11];
    
    println!("Original array: {:?}", numbers);
    
    selection_sort(&mut numbers);
    
    println!("Sorted array: {:?}", numbers);
    
    assert_eq!(numbers, vec![11, 12, 22, 25, 64]);
    println!("✓ Selection sort completed successfully!");
}

/// Selection Sort Implementation
///
/// Algorithm steps:
/// 1. Find the minimum element in the unsorted portion of the array
/// 2. Swap it with the first element of the unsorted portion
/// 3. Move the boundary between sorted and unsorted one element to the right
/// 4. Repeat until the entire array is sorted
///
/// Hint: The outer loop tracks where the sorted portion ends.
/// The inner loop finds the minimum in the remaining unsorted portion.
fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    
    // TODO: Implement the outer loop
    // One by one move the boundary of the unsorted subarray
    // The outer loop should go from 0 to n-1
    for i in 0..n {
        // TODO: Find the minimum element in the unsorted portion
        // Assume the first element is the minimum
        let mut min_idx = i;
        
        // TODO: Check the rest of the unsorted portion to find the actual minimum
        for j in (i + 1)..n {
            // TODO: If we find a smaller element, update min_idx
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        
        // TODO: Swap the found minimum element with the first element
        // Only swap if min_idx is different from i
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_basic() {
        let mut arr = vec![64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_selection_sort_single() {
        let mut arr = vec![42];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_selection_sort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
}
