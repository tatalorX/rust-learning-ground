// Exercise 133: 0/1 Knapsack Problem
//
// Learning Objective:
// Solve the classic 0/1 knapsack problem using dynamic programming.
// Understand the DP approach to optimization problems.
//
// Key Concepts:
// - Dynamic programming
// - Memoization / Tabulation
// - Space optimization techniques
// - Time complexity: O(n * W), Space complexity: O(W)

/// Represents an item with weight and value
#[derive(Debug, Clone, Copy)]
struct Item {
    weight: usize,
    value: usize,
}

impl Item {
    fn new(weight: usize, value: usize) -> Self {
        Item { weight, value }
    }
}

/// TODO: Implement 0/1 Knapsack using dynamic programming (2D table)
/// Returns the maximum value that can be obtained
/// 
/// dp[i][w] = maximum value using first i items with capacity w
fn knapsack_01(items: &[Item], capacity: usize) -> usize {
    let n = items.len();
    
    // TODO: Create a 2D DP table
    // dp[i][w] represents max value using first i items with capacity w
    
    // TODO: Fill the DP table
    // For each item i and each capacity w:
    //   If item doesn't fit (weight > w): dp[i][w] = dp[i-1][w]
    //   Else: dp[i][w] = max(dp[i-1][w], dp[i-1][w-weight] + value)
    
    // TODO: Return dp[n][capacity]
    
    0
}

/// TODO: Implement space-optimized 0/1 Knapsack (1D array)
/// Uses only O(W) space instead of O(n * W)
fn knapsack_01_optimized(items: &[Item], capacity: usize) -> usize {
    // TODO: Create a 1D DP array of size (capacity + 1)
    // dp[w] = max value achievable with capacity w
    
    // TODO: Iterate through items
    // For each item, update dp array in reverse order (from capacity down to weight)
    // This ensures each item is used at most once
    
    // TODO: Return dp[capacity]
    
    0
}

/// TODO: Implement knapsack that also returns which items were selected
/// Returns (max_value, selected_items_indices)
fn knapsack_with_selection(items: &[Item], capacity: usize) -> (usize, Vec<usize>) {
    let n = items.len();
    
    // TODO: Create DP table
    
    // TODO: Backtrack to find which items were selected
    // Start from dp[n][capacity] and work backwards
    // If dp[i][w] != dp[i-1][w], then item i-1 was selected
    
    (0, vec![])
}

/// Recursive approach with memoization (top-down)
fn knapsack_recursive(items: &[Item], capacity: usize) -> usize {
    // TODO: Implement recursive solution with memoization
    // Use a HashMap or Vec to store computed states
    // State: (index, remaining_capacity) -> max_value
    
    0
}

fn main() {
    let items = vec![
        Item::new(2, 3),
        Item::new(3, 4),
        Item::new(4, 5),
        Item::new(5, 6),
    ];
    let capacity = 5;
    
    println!("Items: {:?}", items);
    println!("Capacity: {}", capacity);
    
    let max_value = knapsack_01(&items, capacity);
    println!("\nMax value (2D DP): {}", max_value);
    
    let max_value_opt = knapsack_01_optimized(&items, capacity);
    println!("Max value (optimized): {}", max_value_opt);
    
    let (value, selected) = knapsack_with_selection(&items, capacity);
    println!("Selected items: {:?} with total value: {}", selected, value);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_knapsack_basic() {
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
            Item::new(5, 6),
        ];
        assert_eq!(knapsack_01(&items, 5), 7); // Items 0 and 1
    }
    
    #[test]
    fn test_knapsack_empty() {
        let items: Vec<Item> = vec![];
        assert_eq!(knapsack_01(&items, 10), 0);
    }
    
    #[test]
    fn test_knapsack_zero_capacity() {
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
        ];
        assert_eq!(knapsack_01(&items, 0), 0);
    }
    
    #[test]
    fn test_knapsack_all_fit() {
        let items = vec![
            Item::new(1, 10),
            Item::new(2, 20),
            Item::new(3, 30),
        ];
        assert_eq!(knapsack_01(&items, 10), 60); // All items fit
    }
    
    #[test]
    fn test_knapsack_optimized_matches() {
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
            Item::new(5, 6),
        ];
        assert_eq!(knapsack_01(&items, 5), knapsack_01_optimized(&items, 5));
    }
    
    #[test]
    fn test_knapsack_selection() {
        let items = vec![
            Item::new(2, 3),
            Item::new(3, 4),
            Item::new(4, 5),
        ];
        let (value, selected) = knapsack_with_selection(&items, 5);
        assert_eq!(value, 7);
        assert!(selected.contains(&0)); // Item 0 selected
        assert!(selected.contains(&1)); // Item 1 selected
    }
}
