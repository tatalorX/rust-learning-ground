// Exercise 139: Trie (Prefix Tree)
//
// Learning Objective:
// Implement a Trie data structure for efficient string prefix matching.
// Understand tree-based string storage.
//
// Key Concepts:
// - Prefix tree data structure
// - Node-based tree structure
// - Efficient prefix search: O(m) where m is key length
// - Applications: autocomplete, spell checker, IP routing

use std::collections::HashMap;

/// Node in the Trie
#[derive(Debug, Default)]
struct TrieNode {
    /// Whether this node marks the end of a word
    is_end_of_word: bool,
    /// Children nodes: character -> node
    children: HashMap<char, TrieNode>,
    /// Optional: store the complete word at end nodes for retrieval
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end_of_word: false,
            children: HashMap::new(),
            word: None,
        }
    }
}

/// Trie data structure
#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    /// TODO: Create a new empty Trie
    fn new() -> Self {
        // TODO: Initialize with new root node
        Trie { root: TrieNode::new() }
    }
    
    /// TODO: Insert a word into the Trie
    fn insert(&mut self, word: &str) {
        // TODO: Start from root
        // For each character in word:
        //   Create child if doesn't exist
        //   Move to child
        // Mark final node as end_of_word
        
        let mut current = &mut self.root;
        for ch in word.chars() {
            // TODO: Navigate/create nodes
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        current.is_end_of_word = true;
        current.word = Some(word.to_string());
    }
    
    /// TODO: Search for a word in the Trie
    /// Returns true if the exact word exists
    fn search(&self, word: &str) -> bool {
        // TODO: Navigate through nodes following characters
        // Return true only if end_of_word is true at final node
        
        false
    }
    
    /// TODO: Check if any word starts with given prefix
    fn starts_with(&self, prefix: &str) -> bool {
        // TODO: Navigate through nodes following prefix characters
        // Return true if we can traverse entire prefix
        
        false
    }
    
    /// TODO: Find the node corresponding to a prefix
    /// Returns Some(node) if prefix exists, None otherwise
    fn find_node(&self, prefix: &str) -> Option<&TrieNode> {
        // TODO: Navigate to the node at end of prefix
        
        None
    }
    
    /// TODO: Get all words with given prefix
    fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        // TODO: Find node for prefix
        // Collect all words from that node and its descendants
        
        vec![]
    }
    
    /// TODO: Helper: collect all words from a node
    fn collect_words(&self, node: &TrieNode, result: &mut Vec<String>) {
        // TODO: DFS to collect all words from this node
        // If node is end_of_word, add to result
        // Recurse on all children
    }
    
    /// TODO: Delete a word from the Trie
    /// Returns true if word was found and deleted
    fn delete(&mut self, word: &str) -> bool {
        // BONUS CHALLENGE
        // Delete word and cleanup unused nodes
        // Return false if word not found
        
        false
    }
    
    /// TODO: Count words in Trie
    fn count(&self) -> usize {
        // TODO: Count all end_of_word nodes
        
        0
    }
}

fn main() {
    let mut trie = Trie::new();
    
    let words = vec!["cat", "car", "card", "care", "careful", "dog", "dodge"];
    
    println!("Inserting words: {:?}\n", words);
    
    for word in &words {
        trie.insert(word);
    }
    
    println!("Search 'cat': {}", trie.search("cat"));
    println!("Search 'car': {}", trie.search("car"));
    println!("Search 'ca': {}", trie.search("ca")); // false, it's a prefix not a word
    println!("Search 'catz': {}", trie.search("catz"));
    
    println!("\nStarts with 'ca': {}", trie.starts_with("ca"));
    println!("Starts with 'dog': {}", trie.starts_with("dog"));
    println!("Starts with 'z': {}", trie.starts_with("z"));
    
    println!("\nWords with prefix 'car': {:?}", trie.words_with_prefix("car"));
    println!("Words with prefix 'do': {:?}", trie.words_with_prefix("do"));
    println!("Words with prefix 'care': {:?}", trie.words_with_prefix("care"));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trie_insert_search() {
        let mut trie = Trie::new();
        trie.insert("apple");
        assert!(trie.search("apple"));
        assert!(!trie.search("app"));
        assert!(!trie.search("apples"));
    }
    
    #[test]
    fn test_trie_starts_with() {
        let mut trie = Trie::new();
        trie.insert("apple");
        assert!(trie.starts_with("app"));
        assert!(trie.starts_with("apple"));
        assert!(!trie.starts_with("apl"));
    }
    
    #[test]
    fn test_trie_multiple_words() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("car");
        trie.insert("card");
        
        assert!(trie.search("cat"));
        assert!(trie.search("car"));
        assert!(trie.search("card"));
        assert!(!trie.search("ca"));
        assert!(!trie.search("cart"));
    }
    
    #[test]
    fn test_trie_words_with_prefix() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("car");
        trie.insert("card");
        trie.insert("care");
        
        let words = trie.words_with_prefix("ca");
        assert_eq!(words.len(), 4);
        assert!(words.contains(&"cat".to_string()));
        assert!(words.contains(&"car".to_string()));
        assert!(words.contains(&"card".to_string()));
        assert!(words.contains(&"care".to_string()));
    }
    
    #[test]
    fn test_trie_empty_prefix() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("dog");
        
        let words = trie.words_with_prefix("");
        assert_eq!(words.len(), 2);
    }
    
    #[test]
    fn test_trie_count() {
        let mut trie = Trie::new();
        assert_eq!(trie.count(), 0);
        
        trie.insert("a");
        trie.insert("ab");
        trie.insert("abc");
        assert_eq!(trie.count(), 3);
    }
}
