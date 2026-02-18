// Exercise 137: Maximum Subarray (Kadane's Algorithm)
//
// Learning Objective:
// Find the contiguous subarray with the largest sum using Kadane's algorithm.
// Understand how to track local and global optima.
//
// Key Concepts:
// - Kadane's algorithm (greedy/DP hybrid)
// - Local maximum vs global maximum
// - Time complexity: O(n), Space complexity: O(1)

/// TODO: Find the maximum sum of any contiguous subarray
/// Returns the maximum sum value
/// 
/// Kadane's Algorithm:
/// Initialize: max_so_far = nums[0], max_ending_here = nums[0]
/// For each element:
///   max_ending_here = max(nums[i], max_ending_here + nums[i])
///   max_so_far = max(max_so_far, max_ending_here)
fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    // TODO: Initialize max_so_far and max_ending_here with first element
    
    // TODO: Iterate through remaining elements
    // Update max_ending_here: either start fresh at current or extend previous
    // Update max_so_far if max_ending_here is larger
    
    0
}

/// TODO: Find maximum subarray sum and return the subarray indices
/// Returns (max_sum, start_index, end_index)
fn max_subarray_with_indices(nums: &[i32]) -> (i32, usize, usize) {
    if nums.is_empty() {
        return (0, 0, 0);
    }
    
    // TODO: Similar to max_subarray_sum but track start/end indices
    // When starting fresh (nums[i] > max_ending_here + nums[i]),
    // update the start index
    // When finding new max, update end index
    
    (0, 0, 0)
}

/// TODO: Find the actual subarray with maximum sum
/// Returns a vector containing the subarray elements
fn max_subarray(nums: &[i32]) -> Vec<i32> {
    // TODO: Use max_subarray_with_indices to get indices
    // Return slice of original array
    
    vec![]
}

/// TODO: Maximum circular subarray sum
/// The subarray can wrap around the end to the beginning
/// 
/// Approach:
/// 1. Find max subarray sum using Kadane's (non-circular)
/// 2. Find total sum - min subarray sum (circular case)
/// 3. Return max of the two (handle all-negative case specially)
fn max_circular_subarray_sum(nums: &[i32]) -> i32 {
    // TODO: Implement using the approach above
    // Edge case: if all numbers are negative, return max element
    
    0
}

/// TODO: Maximum product subarray
/// Find contiguous subarray with maximum product
/// 
/// Challenge: Need to track both max and min because
/// multiplying by a negative flips the sign
fn max_product_subarray(nums: &[i32]) -> i32 {
    // TODO: Track max_ending_here and min_ending_here
    // Update both considering current element and products
    // Track global maximum
    
    0
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    
    println!("Array: {:?}", nums);
    
    let max_sum = max_subarray_sum(&nums);
    println!("\nMaximum subarray sum: {}", max_sum);
    
    let (sum, start, end) = max_subarray_with_indices(&nums);
    println!("Subarray indices: [{}, {}]", start, end);
    println!("Subarray: {:?}", &nums[start..=end]);
    println!("Sum: {}", sum);
    
    // Circular example
    let circular = vec![5, -3, 5];
    println!("\nCircular array: {:?}", circular);
    println!("Max circular sum: {}", max_circular_subarray_sum(&circular));
    
    // Product example
    let product_nums = vec![2, 3, -2, 4];
    println!("\nArray: {:?}", product_nums);
    println!("Max product: {}", max_product_subarray(&product_nums));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_subarray_basic() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_subarray_sum(&nums), 6); // [4, -1, 2, 1]
    }
    
    #[test]
    fn test_max_subarray_all_positive() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(max_subarray_sum(&nums), 15); // entire array
    }
    
    #[test]
    fn test_max_subarray_all_negative() {
        let nums = vec![-5, -4, -3, -2, -1];
        assert_eq!(max_subarray_sum(&nums), -1); // just [-1]
    }
    
    #[test]
    fn test_max_subarray_single_element() {
        let nums = vec![5];
        assert_eq!(max_subarray_sum(&nums), 5);
    }
    
    #[test]
    fn test_max_subarray_with_indices() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let (sum, start, end) = max_subarray_with_indices(&nums);
        assert_eq!(sum, 6);
        assert_eq!(&nums[start..=end], &[4, -1, 2, 1]);
    }
    
    #[test]
    fn test_max_circular() {
        let nums = vec![5, -3, 5];
        assert_eq!(max_circular_subarray_sum(&nums), 10); // [5, -3, 5] wraps
    }
    
    #[test]
    fn test_max_circular_all_negative() {
        let nums = vec![-3, -2, -1];
        assert_eq!(max_circular_subarray_sum(&nums), -1);
    }
    
    #[test]
    fn test_max_product() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(max_product_subarray(&nums), 6); // [2, 3]
    }
    
    #[test]
    fn test_max_product_with_negative() {
        let nums = vec![-2, 0, -1];
        assert_eq!(max_product_subarray(&nums), 0);
    }
}
