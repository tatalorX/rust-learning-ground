// Exercise 136: Two Sum Problem
//
// Learning Objective:
// Solve the classic two-sum problem efficiently using a hash map.
// Understand time-space tradeoffs.
//
// Key Concepts:
// - Hash map for O(1) lookup
// - Complement approach
// - Time complexity: O(n), Space complexity: O(n)
// - Multiple solution approaches

use std::collections::HashMap;

/// TODO: Find two indices whose values sum to target
/// Returns Some((i, j)) where nums[i] + nums[j] == target and i < j
/// Returns None if no such pair exists
/// 
/// Approach: Hash Map
/// For each element, check if (target - current) exists in the map
/// Store each element's index as we iterate
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // TODO: Create a HashMap to store value -> index
    
    // TODO: Iterate through nums with index
    // For each num, calculate complement = target - num
    // If complement exists in map, return (map[complement], index)
    // Otherwise, insert (num, index) into map
    
    None
}

/// TODO: Find all unique pairs that sum to target
/// Returns a vector of (i, j) tuples where i < j
fn two_sum_all(nums: &[i32], target: i32) -> Vec<(usize, usize)> {
    // TODO: Find all pairs, not just the first one
    // Be careful not to return duplicate pairs
    
    vec![]
}

/// TODO: Two sum using two-pointer approach (requires sorted array)
/// Returns Some((i, j)) where nums[i] + nums[j] == target
/// 
/// Note: This returns indices in the sorted array, not original indices
/// Time: O(n log n) due to sorting, Space: O(1) extra
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // TODO: Sort the array with indices
    // Use two pointers: one at start, one at end
    // If sum < target, move left pointer right
    // If sum > target, move right pointer left
    // If sum == target, return indices
    
    None
}

/// TODO: Three sum - find three numbers that sum to target
/// Returns a vector of tuples (i, j, k) where nums[i] + nums[j] + nums[k] == target
fn three_sum(nums: &[i32], target: i32) -> Vec<(usize, usize, usize)> {
    // BONUS CHALLENGE
    // Approach: For each element, solve two-sum on the remaining array
    // for target - current_element
    
    vec![]
}

/// Check if array has a contiguous subarray summing to target
fn has_subarray_sum(nums: &[i32], target: i32) -> bool {
    // BONUS CHALLENGE
    // Use prefix sums and hash set
    // prefix[j] - prefix[i] = sum(i+1..j)
    // Check if (current_prefix - target) exists in set
    
    false
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    println!("Array: {:?}", nums);
    println!("Target: {}", target);
    
    if let Some((i, j)) = two_sum(&nums, target) {
        println!("\nTwo sum found: indices ({}, {})", i, j);
        println!("Values: {} + {} = {}", nums[i], nums[j], target);
    } else {
        println!("\nNo solution found");
    }
    
    // Test with duplicates
    let nums2 = vec![3, 2, 4, 3, 3];
    let all_pairs = two_sum_all(&nums2, 6);
    println!("\nArray: {:?}", nums2);
    println!("All pairs summing to 6: {:?}", all_pairs);
    
    // Test sorted approach
    let nums3 = vec![1, 3, 4, 8, 10];
    if let Some((i, j)) = two_sum_sorted(&nums3, 11) {
        println!("\nTwo sum in sorted array: indices ({}, {})", i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum_basic() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 9), Some((0, 1)));
    }
    
    #[test]
    fn test_two_sum_not_first() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 18), Some((1, 2)));
    }
    
    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 100), None);
    }
    
    #[test]
    fn test_two_sum_negatives() {
        let nums = vec![-1, -2, -3, -4, -5];
        assert_eq!(two_sum(&nums, -8), Some((2, 4)));
    }
    
    #[test]
    fn test_two_sum_mixed() {
        let nums = vec![-1, 2, 3, 4, 5];
        assert_eq!(two_sum(&nums, 4), Some((0, 4))); // -1 + 5 = 4
    }
    
    #[test]
    fn test_two_sum_with_duplicates() {
        let nums = vec![3, 3];
        assert_eq!(two_sum(&nums, 6), Some((0, 1)));
    }
    
    #[test]
    fn test_two_sum_all() {
        let nums = vec![1, 2, 3, 4, 3];
        let pairs = two_sum_all(&nums, 6);
        assert_eq!(pairs.len(), 2); // (1,3) and (2,4)
    }
    
    #[test]
    fn test_two_sum_sorted() {
        let nums = vec![1, 3, 4, 8, 10];
        assert_eq!(two_sum_sorted(&nums, 11), Some((0, 3)));
    }
}
