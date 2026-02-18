// Exercise 125: Min/Max Finding in Array
// ======================================
//
// Learning Objective:
// Learn different approaches to find minimum and maximum values
// in an array. This includes single pass algorithms and tournament
// methods for optimization.

fn main() {
    let numbers = vec![3, 7, 2, 9, 1, 5, 8, 4, 6];
    
    println!("Array: {:?}\n", numbers);
    
    // Method 1: Simple approach
    let (min1, max1) = find_min_max_simple(&numbers);
    println!("Method 1 - Simple:");
    println!("  Min: {}, Max: {}", min1, max1);
    
    // Method 2: Single pass
    let (min2, max2) = find_min_max_single_pass(&numbers);
    println!("\nMethod 2 - Single Pass:");
    println!("  Min: {}, Max: {}", min2, max2);
    
    // Method 3: Tournament method (fewer comparisons)
    let (min3, max3) = find_min_max_tournament(&numbers);
    println!("\nMethod 3 - Tournament:");
    println!("  Min: {}, Max: {}", min3, max3);
    
    // Method 4: Using iterators
    let (min4, max4) = find_min_max_iter(&numbers);
    println!("\nMethod 4 - Iterators:");
    println!("  Min: {}, Max: {}", min4, max4);
    
    // Verify all methods give same results
    assert_eq!(min1, min2);
    assert_eq!(min2, min3);
    assert_eq!(min3, min4);
    assert_eq!(max1, max2);
    assert_eq!(max2, max3);
    assert_eq!(max3, max4);
    
    // Find indices
    println!("\n=== Finding Indices ===");
    if let Some((min_idx, max_idx)) = find_min_max_indices(&numbers) {
        println!("Min at index {}: {}", min_idx, numbers[min_idx]);
        println!("Max at index {}: {}", max_idx, numbers[max_idx]);
    }
    
    // Find second min/max
    println!("\n=== Second Min/Max ===");
    if let Some((second_min, second_max)) = find_second_min_max(&numbers) {
        println!("Second min: {}, Second max: {}", second_min, second_max);
    }
    
    println!("\n✓ Min/Max finding completed successfully!");
}

/// Find min and max using simple approach
///
/// Method: Use built-in min/max or iterate separately
/// Comparisons: 2n (worst case)
fn find_min_max_simple(arr: &[i32]) -> (i32, i32) {
    // TODO: Find min and max using simple iteration
    // Initialize with first element
    let mut min = arr[0];
    let mut max = arr[0];
    
    // TODO: Find min
    for &x in arr.iter().skip(1) {
        if x < min {
            min = x;
        }
    }
    
    // TODO: Find max
    for &x in arr.iter().skip(1) {
        if x > max {
            max = x;
        }
    }
    
    (min, max)
}

/// Find min and max in single pass
///
/// Method: Check each element against both min and max
/// Comparisons: ~2n (but only one pass through array)
fn find_min_max_single_pass(arr: &[i32]) -> (i32, i32) {
    // TODO: Find min and max in a single loop
    let mut min = arr[0];
    let mut max = arr[0];
    
    for &x in arr.iter().skip(1) {
        // TODO: Update min and max in one pass
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    
    (min, max)
}

/// Find min and max using tournament method
///
/// Method: Process elements in pairs
/// - Compare pair elements: 1 comparison
/// - Compare winner with max, loser with min: 2 comparisons
/// - Total: 3 comparisons per 2 elements = 3n/2 (better than 2n)
///
/// Best for large arrays where comparisons are expensive.
fn find_min_max_tournament(arr: &[i32]) -> (i32, i32) {
    // TODO: Implement tournament method
    if arr.is_empty() {
        panic!("Empty array");
    }
    
    if arr.len() == 1 {
        return (arr[0], arr[0]);
    }
    
    let mut min: i32;
    let mut max: i32;
    let mut i = 0;
    
    // TODO: Initialize min and max based on first pair
    if arr.len() % 2 == 0 {
        // Even length: compare first two
        if arr[0] < arr[1] {
            min = arr[0];
            max = arr[1];
        } else {
            min = arr[1];
            max = arr[0];
        }
        i = 2;
    } else {
        // Odd length: start with first element
        min = arr[0];
        max = arr[0];
        i = 1;
    }
    
    // TODO: Process remaining elements in pairs
    while i < arr.len() - 1 {
        let (local_min, local_max) = if arr[i] < arr[i + 1] {
            (arr[i], arr[i + 1])
        } else {
            (arr[i + 1], arr[i])
        };
        
        if local_min < min {
            min = local_min;
        }
        if local_max > max {
            max = local_max;
        }
        
        i += 2;
    }
    
    (min, max)
}

/// Find min and max using Rust iterators
///
/// Most idiomatic Rust approach using built-in methods.
fn find_min_max_iter(arr: &[i32]) -> (i32, i32) {
    // TODO: Use iterator methods to find min and max
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    (min, max)
}

/// Find indices of min and max elements
fn find_min_max_indices(arr: &[i32]) -> Option<(usize, usize)> {
    // TODO: Find indices of min and max
    if arr.is_empty() {
        return None;
    }
    
    let mut min_idx = 0;
    let mut max_idx = 0;
    
    for (i, &x) in arr.iter().enumerate().skip(1) {
        if x < arr[min_idx] {
            min_idx = i;
        }
        if x > arr[max_idx] {
            max_idx = i;
        }
    }
    
    Some((min_idx, max_idx))
}

/// Find second smallest and second largest elements
fn find_second_min_max(arr: &[i32]) -> Option<(i32, i32)> {
    // TODO: Bonus - find second min and second max
    if arr.len() < 2 {
        return None;
    }
    
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    
    for &x in arr {
        // Update mins
        if x < min1 {
            min2 = min1;
            min1 = x;
        } else if x < min2 && x != min1 {
            min2 = x;
        }
        
        // Update maxs
        if x > max1 {
            max2 = max1;
            max1 = x;
        } else if x > max2 && x != max1 {
            max2 = x;
        }
    }
    
    if min2 == i32::MAX || max2 == i32::MIN {
        None
    } else {
        Some((min2, max2))
    }
}

/// Generic version (Bonus)
fn find_min_max_generic<T: Ord + Clone>(arr: &[T]) -> Option<(T, T)> {
    // TODO: Bonus - make it generic
    if arr.is_empty() {
        return None;
    }
    
    let mut min = arr[0].clone();
    let mut max = arr[0].clone();
    
    for x in arr.iter().skip(1) {
        if *x < min {
            min = x.clone();
        }
        if *x > max {
            max = x.clone();
        }
    }
    
    Some((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_max() {
        let arr = vec![3, 7, 2, 9, 1, 5, 8, 4, 6];
        
        let (min, max) = find_min_max_simple(&arr);
        assert_eq!(min, 1);
        assert_eq!(max, 9);
        
        let (min2, max2) = find_min_max_single_pass(&arr);
        assert_eq!(min2, 1);
        assert_eq!(max2, 9);
        
        let (min3, max3) = find_min_max_tournament(&arr);
        assert_eq!(min3, 1);
        assert_eq!(max3, 9);
    }

    #[test]
    fn test_two_elements() {
        let arr = vec![5, 3];
        assert_eq!(find_min_max_simple(&arr), (3, 5));
        assert_eq!(find_min_max_tournament(&arr), (3, 5));
    }

    #[test]
    fn test_single_element() {
        let arr = vec![42];
        assert_eq!(find_min_max_simple(&arr), (42, 42));
        assert_eq!(find_min_max_tournament(&arr), (42, 42));
    }

    #[test]
    fn test_with_negatives() {
        let arr = vec![-5, 3, -10, 8, 0];
        let (min, max) = find_min_max_single_pass(&arr);
        assert_eq!(min, -10);
        assert_eq!(max, 8);
    }

    #[test]
    fn test_find_indices() {
        let arr = vec![3, 7, 2, 9, 1, 5, 8, 4, 6];
        let (min_idx, max_idx) = find_min_max_indices(&arr).unwrap();
        assert_eq!(arr[min_idx], 1);
        assert_eq!(arr[max_idx], 9);
    }

    #[test]
    fn test_second_min_max() {
        let arr = vec![3, 7, 2, 9, 1, 5, 8, 4, 6];
        let (second_min, second_max) = find_second_min_max(&arr).unwrap();
        assert_eq!(second_min, 2);
        assert_eq!(second_max, 8);
    }

    #[test]
    fn test_generic() {
        let arr = vec![3, 7, 2, 9, 1];
        let (min, max) = find_min_max_generic(&arr).unwrap();
        assert_eq!(min, 1);
        assert_eq!(max, 9);
        
        let str_arr = vec!["cherry", "apple", "banana"];
        let (min, max) = find_min_max_generic(&str_arr).unwrap();
        assert_eq!(min, "apple");
        assert_eq!(max, "cherry");
    }
}
