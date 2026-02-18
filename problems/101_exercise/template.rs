// Exercise 101: Bubble Sort Algorithm
// ===================================
//
// Learning Objective:
// Learn how to implement the Bubble Sort algorithm, a simple comparison-based
// sorting algorithm. Understand how it repeatedly steps through the list,
// compares adjacent elements, and swaps them if they are in the wrong order.
//
// Bubble Sort has a time complexity of O(n²) and space complexity of O(1).

fn main() {
    // Test data
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    
    println!("Original array: {:?}", numbers);
    
    // Call the bubble sort function
    bubble_sort(&mut numbers);
    
    println!("Sorted array: {:?}", numbers);
    
    // Verify the result
    assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);
    println!("✓ Bubble sort completed successfully!");
}

/// Bubble Sort Implementation
/// 
/// Algorithm steps:
/// 1. Compare adjacent elements. If the first is greater than the second, swap them.
/// 2. Do this for each pair of adjacent elements, from the first pair to the last.
/// 3. After each pass, the largest unsorted element will "bubble up" to its correct position.
/// 4. Repeat the process for n-1 passes (where n is the length of the array).
///
/// Hint: Use nested loops - outer loop for passes, inner loop for comparisons.
/// Remember that after each pass, the last i elements are already sorted.
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    
    // TODO: Implement the outer loop for passes (0 to n-1)
    // The outer loop runs n-1 times because after n-1 passes,
    // the array will be completely sorted
    
    for i in 0..n {
        // Optional optimization: Track if any swaps occurred in this pass
        // If no swaps occurred, the array is already sorted
        let mut swapped = false;
        
        // TODO: Implement the inner loop for comparisons
        // The inner loop should go from 0 to n-i-1
        // (we don't need to check the last i elements as they're already sorted)
        for j in 0..(n - i - 1) {
            // TODO: Compare adjacent elements
            // If arr[j] > arr[j+1], swap them
            if arr[j] > arr[j + 1] {
                // TODO: Swap the elements
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // Optimization: If no swapping occurred, array is sorted
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_basic() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_bubble_sort_single() {
        let mut arr = vec![42];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_bubble_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
