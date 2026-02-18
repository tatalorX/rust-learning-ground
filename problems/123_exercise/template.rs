// Exercise 123: Binary Tree - Inorder Traversal
// =============================================
//
// Learning Objective:
// Learn tree traversal algorithms. Inorder traversal visits nodes in:
// Left -> Root -> Right order, which gives sorted order for BSTs.
//
// Also covers: Preorder, Postorder, and Level order traversals

fn main() {
    println!("=== Binary Tree Traversals ===\n");
    
    // Create a binary tree
    //       50
    //      /  \
    //     30   70
    //    / \   / \
    //   20 40 60 80
    let mut tree = BinaryTree::new();
    for &val in &[50, 30, 70, 20, 40, 60, 80] {
        tree.insert(val);
    }
    
    println!("Tree structure:");
    println!("       50");
    println!("      /  \\");
    println!("     30   70");
    println!("    / \\   / \\");
    println!("   20 40 60 80\n");
    
    // Demonstrate all traversal types
    println!("Inorder (Left-Root-Right):   {:?}", tree.inorder());
    println!("Preorder (Root-Left-Right):  {:?}", tree.preorder());
    println!("Postorder (Left-Right-Root): {:?}", tree.postorder());
    println!("Level order:                 {:?}", tree.level_order());
    
    // Verify inorder gives sorted result
    assert_eq!(tree.inorder(), vec![20, 30, 40, 50, 60, 70, 80]);
    
    println!("\n✓ Binary tree traversals completed successfully!");
}

use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> TreeNode<T> {
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
    
    /// Inorder traversal: Left -> Root -> Right
    fn inorder(&self, result: &mut Vec<T>) {
        // TODO: Implement inorder traversal
        // 1. Traverse left subtree
        if let Some(ref left) = self.left {
            left.inorder(result);
        }
        
        // 2. Visit root
        result.push(self.value.clone());
        
        // 3. Traverse right subtree
        if let Some(ref right) = self.right {
            right.inorder(result);
        }
    }
    
    /// Preorder traversal: Root -> Left -> Right
    fn preorder(&self, result: &mut Vec<T>) {
        // TODO: Implement preorder traversal
        // 1. Visit root
        result.push(self.value.clone());
        
        // 2. Traverse left subtree
        if let Some(ref left) = self.left {
            left.preorder(result);
        }
        
        // 3. Traverse right subtree
        if let Some(ref right) = self.right {
            right.preorder(result);
        }
    }
    
    /// Postorder traversal: Left -> Right -> Root
    fn postorder(&self, result: &mut Vec<T>) {
        // TODO: Implement postorder traversal
        // 1. Traverse left subtree
        if let Some(ref left) = self.left {
            left.postorder(result);
        }
        
        // 2. Traverse right subtree
        if let Some(ref right) = self.right {
            right.postorder(result);
        }
        
        // 3. Visit root
        result.push(self.value.clone());
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }
    
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }
    
    /// Inorder traversal wrapper
    fn inorder(&self) -> Vec<T> {
        // TODO: Call recursive inorder on root
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.inorder(&mut result);
        }
        result
    }
    
    /// Preorder traversal wrapper
    fn preorder(&self) -> Vec<T> {
        // TODO: Call recursive preorder on root
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.preorder(&mut result);
        }
        result
    }
    
    /// Postorder traversal wrapper
    fn postorder(&self) -> Vec<T> {
        // TODO: Call recursive postorder on root
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            root.postorder(&mut result);
        }
        result
    }
    
    /// Level order traversal (BFS)
    ///
    /// Visit nodes level by level, from left to right
    fn level_order(&self) -> Vec<T> {
        // TODO: Implement level order using a queue
        let mut result = Vec::new();
        
        if self.root.is_none() {
            return result;
        }
        
        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap());
        
        while let Some(node) = queue.pop_front() {
            result.push(node.value.clone());
            
            if let Some(ref left) = node.left {
                queue.push_back(left);
            }
            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> BinaryTree<i32> {
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
        tree
    }

    #[test]
    fn test_inorder() {
        let tree = create_test_tree();
        // Inorder should give sorted order for BST
        assert_eq!(tree.inorder(), vec![20, 30, 50, 70, 80]);
    }

    #[test]
    fn test_preorder() {
        let tree = create_test_tree();
        // Root -> Left -> Right
        assert_eq!(tree.preorder(), vec![50, 30, 20, 70, 80]);
    }

    #[test]
    fn test_postorder() {
        let tree = create_test_tree();
        // Left -> Right -> Root
        assert_eq!(tree.postorder(), vec![20, 30, 80, 70, 50]);
    }

    #[test]
    fn test_level_order() {
        let tree = create_test_tree();
        // Level by level
        assert_eq!(tree.level_order(), vec![50, 30, 70, 20, 80]);
    }

    #[test]
    fn test_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(tree.inorder().is_empty());
        assert!(tree.preorder().is_empty());
        assert!(tree.postorder().is_empty());
        assert!(tree.level_order().is_empty());
    }
}
