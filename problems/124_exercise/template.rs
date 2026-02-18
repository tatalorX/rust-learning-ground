// Exercise 124: Binary Tree - Height Calculation
// ==============================================
//
// Learning Objective:
// Learn how to calculate the height/depth of a binary tree.
// Height is the longest path from root to any leaf.
// This is important for understanding tree balancing and complexity.

fn main() {
    println!("=== Binary Tree Height ===\n");
    
    // Test with different tree shapes
    println!("Test 1: Empty tree");
    let empty_tree: BinaryTree<i32> = BinaryTree::new();
    println!("Height: {}", empty_tree.height());
    assert_eq!(empty_tree.height(), 0);
    
    println!("\nTest 2: Single node");
    let mut single = BinaryTree::new();
    single.insert(50);
    println!("Height: {}", single.height());
    assert_eq!(single.height(), 1);
    
    println!("\nTest 3: Balanced tree");
    //       50
    //      /  \
    //     30   70
    //    / \   / \
    //   20 40 60 80
    let mut balanced = BinaryTree::new();
    for &val in &[50, 30, 70, 20, 40, 60, 80] {
        balanced.insert(val);
    }
    println!("Height: {}", balanced.height());
    assert_eq!(balanced.height(), 3);
    
    println!("\nTest 4: Skewed tree (left)");
    //   50
    //  /
    // 40
    // /
    //30
    let mut left_skewed = BinaryTree::new();
    for &val in &[50, 40, 30, 20, 10] {
        left_skewed.insert(val);
    }
    println!("Height: {}", left_skewed.height());
    assert_eq!(left_skewed.height(), 5);
    
    println!("\nTest 5: Calculate diameter");
    println!("Diameter of balanced tree: {}", balanced.diameter());
    
    println!("\n✓ Binary tree height calculation completed successfully!");
}

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    
    fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
    
    /// Calculate height of this node
    ///
    /// Height = 1 + max(height of left subtree, height of right subtree)
    /// Base case: empty tree has height 0
    fn height(&self) -> usize {
        // TODO: Calculate height recursively
        let left_height = self.left.as_ref().map_or(0, |n| n.height());
        let right_height = self.right.as_ref().map_or(0, |n| n.height());
        
        1 + left_height.max(right_height)
    }
    
    /// Calculate diameter and height together (Bonus)
    ///
    /// Diameter is the longest path between any two nodes in the tree.
    /// It may or may not pass through the root.
    fn diameter_and_height(&self) -> (usize, usize) {
        // TODO: Bonus - calculate diameter
        // Returns (diameter, height)
        let (left_dia, left_height) = self.left.as_ref()
            .map_or((0, 0), |n| n.diameter_and_height());
        
        let (right_dia, right_height) = self.right.as_ref()
            .map_or((0, 0), |n| n.diameter_and_height());
        
        // Height of current node
        let height = 1 + left_height.max(right_height);
        
        // Diameter passing through root
        let through_root = left_height + right_height + 1;
        
        // Maximum diameter
        let diameter = through_root.max(left_dia.max(right_dia));
        
        (diameter, height)
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }
    
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }
    
    /// Calculate height of the tree
    ///
    /// Height is the number of nodes on the longest path from root to leaf.
    /// Empty tree has height 0.
    /// Single node tree has height 1.
    fn height(&self) -> usize {
        // TODO: Calculate tree height
        match &self.root {
            Some(root) => root.height(),
            None => 0,
        }
    }
    
    /// Check if the tree is balanced (Bonus)
    ///
    /// A tree is balanced if the height difference between left and right
    /// subtrees is at most 1 for all nodes.
    fn is_balanced(&self) -> bool {
        // TODO: Bonus - check if tree is balanced
        self.check_balanced().is_some()
    }
    
    fn check_balanced(&self) -> Option<usize> {
        // Helper that returns height if balanced, None otherwise
        self.root.as_ref()?.check_balanced_helper()
    }
    
    /// Calculate diameter of the tree (Bonus)
    fn diameter(&self) -> usize {
        // TODO: Bonus - calculate tree diameter
        match &self.root {
            Some(root) => root.diameter_and_height().0,
            None => 0,
        }
    }
}

impl<T: Ord> TreeNode<T> {
    fn check_balanced_helper(&self) -> Option<usize> {
        let left_height = self.left.as_ref()
            .and_then(|n| n.check_balanced_helper())
            .unwrap_or(0);
        
        let right_height = self.right.as_ref()
            .and_then(|n| n.check_balanced_helper())
            .unwrap_or(0);
        
        // Check if current node is balanced
        if (left_height as i32 - right_height as i32).abs() > 1 {
            return None;
        }
        
        Some(1 + left_height.max(right_height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree_height() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_single_node_height() {
        let mut tree = BinaryTree::new();
        tree.insert(50);
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn test_balanced_tree_height() {
        let mut tree = BinaryTree::new();
        //       50
        //      /  \
        //     30   70
        //    /      \
        //   20       80
        tree.insert(50);
        tree.insert(30);
        tree.insert(70);
        tree.insert(20);
        tree.insert(80);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_skewed_tree_height() {
        let mut tree = BinaryTree::new();
        for val in vec![10, 20, 30, 40, 50] {
            tree.insert(val);
        }
        // Chain: 10 -> 20 -> 30 -> 40 -> 50
        assert_eq!(tree.height(), 5);
    }

    #[test]
    fn test_is_balanced() {
        let mut balanced = BinaryTree::new();
        //       50
        //      /  \
        //     30   70
        balanced.insert(50);
        balanced.insert(30);
        balanced.insert(70);
        assert!(balanced.is_balanced());
        
        let mut unbalanced = BinaryTree::new();
        // Chain creates unbalanced tree
        for val in vec![10, 20, 30, 40, 50] {
            unbalanced.insert(val);
        }
        assert!(!unbalanced.is_balanced());
    }

    #[test]
    fn test_diameter() {
        let mut tree = BinaryTree::new();
        //       50
        //      /  \
        //     30   70
        //    /      \
        //   20       80
        tree.insert(50);
        tree.insert(30);
        tree.insert(70);
        tree.insert(20);
        tree.insert(80);
        // Diameter: 20 -> 30 -> 50 -> 70 -> 80 (5 nodes)
        assert_eq!(tree.diameter(), 5);
    }
}
