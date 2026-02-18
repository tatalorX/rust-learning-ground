// Exercise 128: Heap Sort Algorithm
//
// Learning Objective:
// Understand and implement heap sort using binary heap data structure.
// Learn about heap properties and how to build/maintain a heap.
//
// Key Concepts:
// - Binary heap (max-heap)
// - Heapify operation
// - In-place sorting
// - Time complexity: O(n log n) for all cases
// - Space complexity: O(1)

/// TODO: Implement the heap_sort function
/// Sorts a vector of i32 using heap sort algorithm
/// 
/// Algorithm:
/// 1. Build a max heap from the input data
/// 2. At this point, the largest item is stored at the root
/// 3. Replace the root with the last item, reduce heap size by 1, and heapify
/// 4. Repeat step 3 while heap size is greater than 1
fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // TODO: Build max heap
    // Start from the last non-leaf node and heapify each node
    // Last non-leaf node is at index (n/2 - 1)
    
    // TODO: Extract elements from heap one by one
    // Move current root to end, then heapify reduced heap
}

/// TODO: Implement the heapify function
/// 
/// Maintains the max-heap property for a subtree rooted at index i
/// Assumes the binary trees rooted at left(i) and right(i) are max-heaps
/// 
/// Parameters:
/// - arr: the array representing the heap
/// - n: size of the heap
/// - i: index of the root to heapify
fn heapify(arr: &mut [i32], n: usize, i: usize) {
    // TODO: Initialize largest as root
    
    // TODO: Calculate left child index: 2*i + 1
    
    // TODO: Calculate right child index: 2*i + 2
    
    // TODO: If left child exists and is greater than root, update largest
    
    // TODO: If right child exists and is greater than largest, update largest
    
    // TODO: If largest is not root, swap and recursively heapify the affected sub-tree
}

fn main() {
    // Test cases
    let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr1);
    heap_sort(&mut arr1);
    println!("Sorted array: {:?}", arr1);
    
    let mut arr2 = vec![3, -1, 0, 5, -2];
    println!("\nOriginal array: {:?}", arr2);
    heap_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
    
    let mut arr3 = vec![5, 4, 3, 2, 1];
    println!("\nOriginal array: {:?}", arr3);
    heap_sort(&mut arr3);
    println!("Sorted array: {:?}", arr3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_heap_sort_basic() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_heap_sort_with_negatives() {
        let mut arr = vec![3, -1, 0, 5, -2];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![-2, -1, 0, 3, 5]);
    }
    
    #[test]
    fn test_heap_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
    
    #[test]
    fn test_heap_sort_single_element() {
        let mut arr = vec![42];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
    
    #[test]
    fn test_heap_sort_already_heap() {
        let mut arr = vec![90, 34, 64, 12, 22, 11, 25]; // Already a max heap
        heap_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
}
