// Exercise 149: Merge k Sorted Lists Concept
//
// Learning Objective:
// Understand and implement the merge k sorted lists problem,
// using multiple approaches: brute force, min-heap, and divide-and-conquer.
//
// Key Concepts:
// - Merging sorted sequences
// - Priority queue (min-heap) for efficient k-way merge
// - Divide and conquer merging
// - Time complexity: O(N log k) where N = total elements, k = number of lists

/// A node in a linked list (for conceptual understanding)
#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// For this exercise, we'll work with Vec<Vec<i32>> to represent k sorted lists
/// This is more practical for testing and understanding

/// TODO: Merge k sorted lists using brute force approach
/// Concatenate all, then sort
/// Time: O(N log N), Space: O(N)
fn merge_k_lists_brute_force(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Flatten all lists into one vector
    // Sort the result
    
    vec![]
}

/// TODO: Merge k sorted lists using min-heap (priority queue)
/// Time: O(N log k), Space: O(k)
/// 
/// Algorithm:
/// 1. Insert first element of each list into min-heap
/// 2. While heap not empty:
///    - Extract min element, add to result
///    - Insert next element from same list (if exists)
fn merge_k_lists_heap(lists: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    
    // TODO: Create a min-heap storing (value, list_index, element_index)
    // Use Reverse for min-heap (BinaryHeap is max-heap by default)
    
    // TODO: Initialize heap with first element from each non-empty list
    
    // TODO: Process heap until empty
    // Pop smallest, add to result
    // Push next element from same list if exists
    
    vec![]
}

/// TODO: Merge k sorted lists using divide and conquer
/// Recursively merge pairs of lists
/// Time: O(N log k), Space: O(log k) for recursion
fn merge_k_lists_divide_conquer(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: If lists is empty, return empty
    // If lists has 1 element, return it
    // Split lists into two halves
    // Recursively merge each half
    // Merge the two results
    
    vec![]
}

/// TODO: Merge two sorted lists (helper function)
/// Time: O(n + m), Space: O(n + m)
fn merge_two_lists(list1: &[i32], list2: &[i32]) -> Vec<i32> {
    // TODO: Standard merge of two sorted arrays
    // Use two pointers, compare elements, add smaller to result
    // Add remaining elements when one list is exhausted
    
    vec![]
}

/// TODO: Merge k sorted lists using sequential merging
/// Merge lists one by one
/// Time: O(k * N), Space: O(N)
fn merge_k_lists_sequential(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Start with empty result
    // For each list, merge it with current result
    
    vec![]
}

/// TODO: Find the kth smallest element in merged array without fully merging
fn find_kth_smallest(lists: &[Vec<i32>], k: usize) -> Option<i32> {
    // BONUS CHALLENGE
    // Use binary search on value range
    // Count how many elements <= mid in all lists
    
    None
}

fn main() {
    let lists = vec![
        vec![1, 4, 5],
        vec![1, 3, 4],
        vec![2, 6],
    ];
    
    println!("Input lists: {:?}\n", lists);
    
    println!("Brute force: {:?}", merge_k_lists_brute_force(lists.clone()));
    println!("Min heap:    {:?}", merge_k_lists_heap(lists.clone()));
    println!("Divide & Conquer: {:?}", merge_k_lists_divide_conquer(lists.clone()));
    println!("Sequential:  {:?}", merge_k_lists_sequential(lists.clone()));
    
    // Test with empty lists
    let lists2 = vec![
        vec![],
        vec![1],
        vec![],
    ];
    println!("\nWith empty lists: {:?}", merge_k_lists_heap(lists2));
    
    // Test merge two lists
    println!("\nMerge [1,3,5] and [2,4,6]: {:?}", 
        merge_two_lists(&[1, 3, 5], &[2, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merge_two_lists() {
        assert_eq!(merge_two_lists(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_two_lists(&[], &[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(merge_two_lists(&[1, 2], &[]), vec![1, 2]);
        assert_eq!(merge_two_lists(&[], &[]), Vec::<i32>::new());
    }
    
    #[test]
    fn test_merge_k_lists_basic() {
        let lists = vec![
            vec![1, 4, 5],
            vec![1, 3, 4],
            vec![2, 6],
        ];
        let expected = vec![1, 1, 2, 3, 4, 4, 5, 6];
        
        assert_eq!(merge_k_lists_brute_force(lists.clone()), expected);
        assert_eq!(merge_k_lists_heap(lists.clone()), expected);
        assert_eq!(merge_k_lists_divide_conquer(lists.clone()), expected);
    }
    
    #[test]
    fn test_empty_lists() {
        let lists: Vec<Vec<i32>> = vec![];
        assert!(merge_k_lists_heap(lists).is_empty());
        
        let lists = vec![vec![], vec![], vec![]];
        assert!(merge_k_lists_heap(lists).is_empty());
    }
    
    #[test]
    fn test_single_list() {
        let lists = vec![vec![1, 2, 3]];
        assert_eq!(merge_k_lists_heap(lists), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_single_element_lists() {
        let lists = vec![vec![3], vec![1], vec![2]];
        assert_eq!(merge_k_lists_heap(lists), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_with_negatives() {
        let lists = vec![
            vec![-5, -3, 0],
            vec![-2, 4],
            vec![1, 3],
        ];
        let expected = vec![-5, -3, -2, 0, 1, 3, 4];
        
        assert_eq!(merge_k_lists_heap(lists.clone()), expected);
        assert_eq!(merge_k_lists_divide_conquer(lists), expected);
    }
    
    #[test]
    fn test_all_methods_agree() {
        let lists = vec![
            vec![1, 4, 5],
            vec![1, 3, 4],
            vec![2, 6],
            vec![0, 7, 8],
        ];
        
        let brute = merge_k_lists_brute_force(lists.clone());
        let heap = merge_k_lists_heap(lists.clone());
        let dc = merge_k_lists_divide_conquer(lists.clone());
        let seq = merge_k_lists_sequential(lists);
        
        assert_eq!(brute, heap);
        assert_eq!(heap, dc);
        assert_eq!(dc, seq);
    }
}
