// Exercise 103: Insertion Sort Algorithm
// ======================================
//
// Learning Objective:
// Learn how to implement the Insertion Sort algorithm, which builds the final
// sorted array one item at a time. It's similar to how you might sort playing
// cards in your hands.
//
// Time Complexity: O(n²) worst case, O(n) best case (nearly sorted)
// Space Complexity: O(1)

fn main() {
    let mut numbers = vec![12, 11, 13, 5, 6];
    
    println!("Original array: {:?}", numbers);
    
    insertion_sort(&mut numbers);
    
    println!("Sorted array: {:?}", numbers);
    
    assert_eq!(numbers, vec![5, 6, 11, 12, 13]);
    println!("✓ Insertion sort completed successfully!");
}

/// Insertion Sort Implementation
///
/// Algorithm steps:
/// 1. Start from the second element (index 1), assuming the first is sorted
/// 2. Pick the current element and compare it with elements in the sorted portion
/// 3. Shift all larger elements one position to the right
/// 4. Insert the current element in its correct position
/// 5. Repeat for all elements
///
/// Hint: Think of the array as having a sorted left part and unsorted right part.
/// Pick each element from the unsorted part and insert it into the correct
/// position in the sorted part.
fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    
    // TODO: Implement insertion sort
    // Start from index 1 (second element)
    for i in 1..n {
        // TODO: Store the current element to be inserted
        let key = arr[i];
        
        // TODO: Initialize j to the index before i
        // j will track where we need to insert key
        let mut j = i;
        
        // TODO: Shift elements greater than key to the right
        // Move backwards through the sorted portion
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        
        // TODO: Place key at its correct position
        arr[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_basic() {
        let mut arr = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn test_insertion_sort_nearly_sorted() {
        let mut arr = vec![1, 2, 3, 5, 4];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_insertion_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
