// Exercise 122: Binary Tree - Basic Structure
// ============================================
//
// Learning Objective:
// Learn how to implement a basic binary tree structure in Rust.
// A binary tree is a tree where each node has at most two children.
//
// This exercise covers:
// - Tree node structure with Box for heap allocation
// - Basic tree operations (insertion, traversal)
// - Recursive tree operations

fn main() {
    println!("=== Binary Tree - Basic Structure ===\n");
    
    // Create a binary tree manually
    let mut tree = BinaryTree::new();
    
    println!("Inserting values: 50, 30, 70, 20, 40, 60, 80");
    tree.insert(50);
    tree.insert(30);
    tree.insert(70);
    tree.insert(20);
    tree.insert(40);
    tree.insert(60);
    tree.insert(80);
    
    println!("Tree size: {}", tree.size());
    println!("Contains 40: {}", tree.contains(40));
    println!("Contains 100: {}", tree.contains(100));
    
    println!("\n✓ Binary tree basic structure completed successfully!");
}

/// Binary Tree Node
///
/// Each node contains:
/// - value: the data stored in this node
/// - left: left child (smaller values)
/// - right: right child (larger values)
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    /// Create a new node with the given value
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    
    /// Insert a value into the subtree rooted at this node
    fn insert(&mut self, value: T) {
        // TODO: Implement recursive insertion
        if value < self.value {
            // TODO: Insert into left subtree
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
        } else if value > self.value {
            // TODO: Insert into right subtree
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        }
        // If value == self.value, do nothing (no duplicates)
    }
    
    /// Check if the subtree contains a value
    fn contains(&self, value: T) -> bool {
        // TODO: Implement recursive search
        if value == self.value {
            return true;
        }
        
        if value < self.value {
            // TODO: Search left subtree
            match &self.left {
                Some(left) => left.contains(value),
                None => false,
            }
        } else {
            // TODO: Search right subtree
            match &self.right {
                Some(right) => right.contains(value),
                None => false,
            }
        }
    }
    
    /// Count nodes in the subtree
    fn size(&self) -> usize {
        // TODO: Implement size calculation
        let left_size = self.left.as_ref().map_or(0, |n| n.size());
        let right_size = self.right.as_ref().map_or(0, |n| n.size());
        1 + left_size + right_size
    }
}

/// Binary Tree wrapper
///
/// Provides a clean public interface and handles empty tree case.
#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    /// Create a new empty binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }
    
    /// Insert a value into the tree
    fn insert(&mut self, value: T) {
        // TODO: Handle insertion into empty tree or delegate to root
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }
    
    /// Check if the tree contains a value
    fn contains(&self, value: T) -> bool {
        // TODO: Delegate to root if it exists
        match &self.root {
            Some(root) => root.contains(value),
            None => false,
        }
    }
    
    /// Get the size of the tree
    fn size(&self) -> usize {
        // TODO: Delegate to root if it exists
        match &self.root {
            Some(root) => root.size(),
            None => 0,
        }
    }
    
    /// Check if the tree is empty
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.size(), 0);
        assert!(!tree.contains(5));
    }

    #[test]
    fn test_insert_and_contains() {
        let mut tree = BinaryTree::new();
        tree.insert(50);
        tree.insert(30);
        tree.insert(70);
        
        assert!(!tree.is_empty());
        assert!(tree.contains(50));
        assert!(tree.contains(30));
        assert!(tree.contains(70));
        assert!(!tree.contains(40));
    }

    #[test]
    fn test_no_duplicates() {
        let mut tree = BinaryTree::new();
        tree.insert(10);
        tree.insert(10);
        tree.insert(10);
        
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn test_size() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.size(), 0);
        
        tree.insert(50);
        assert_eq!(tree.size(), 1);
        
        tree.insert(30);
        tree.insert(70);
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn test_string_tree() {
        let mut tree = BinaryTree::new();
        tree.insert("hello");
        tree.insert("world");
        tree.insert("abc");
        
        assert!(tree.contains("hello"));
        assert!(!tree.contains("xyz"));
    }
}
