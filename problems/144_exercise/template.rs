// Exercise 144: Union-Find (Disjoint Set Union - DSU)
//
// Learning Objective:
// Implement the Union-Find data structure with path compression
// and union by rank optimizations.
//
// Key Concepts:
// - Disjoint set data structure
// - Path compression (amortized nearly O(1))
// - Union by rank/size
// - Applications: Kruskal's MST, cycle detection, connected components

/// Union-Find (Disjoint Set Union) implementation
#[derive(Debug)]
struct UnionFind {
    /// Parent of each element
    parent: Vec<usize>,
    /// Rank (approximate tree height) for union by rank
    rank: Vec<usize>,
    /// Size of each set (for union by size)
    size: Vec<usize>,
    /// Number of disjoint sets
    count: usize,
}

impl UnionFind {
    /// TODO: Create a new Union-Find with n elements (0 to n-1)
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            count: n,
        }
    }
    
    /// TODO: Find the representative (root) of the set containing x
    /// Implements path compression: flatten the structure during find
    fn find(&mut self, x: usize) -> usize {
        // TODO: Implement find with path compression
        // If parent[x] != x, set parent[x] = find(parent[x])
        // Return parent[x]
        
        self.parent[x]
    }
    
    /// TODO: Union by rank
    fn union(&mut self, x: usize, y: usize) -> bool {
        // TODO: Find roots of x and y
        // If same root, return false
        // Otherwise, union by rank and return true
        // Decrement count if union performed
        
        false
    }
    
    /// TODO: Check if x and y are in the same set
    fn connected(&mut self, x: usize, y: usize) -> bool {
        // TODO: Two elements are connected if they have the same root
        
        false
    }
    
    /// Get the number of disjoint sets
    fn set_count(&self) -> usize {
        self.count
    }
}

/// TODO: Count number of connected components
fn count_components(n: usize, edges: &[(usize, usize)]) -> usize {
    // TODO: Create Union-Find with n elements
    // Union all edges
    // Return count
    
    0
}

fn main() {
    let mut uf = UnionFind::new(5);
    
    println!("Initial sets: {}", uf.set_count());
    
    uf.union(0, 1);
    uf.union(2, 3);
    
    println!("After unions: {}", uf.set_count());
    println!("0 and 1 connected: {}", uf.connected(0, 1));
    println!("0 and 2 connected: {}", uf.connected(0, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_union_find() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.set_count(), 5);
        
        assert!(uf.union(0, 1));
        assert_eq!(uf.set_count(), 4);
        
        assert!(uf.connected(0, 1));
        assert!(!uf.connected(0, 2));
    }
    
    #[test]
    fn test_same_set_union() {
        let mut uf = UnionFind::new(3);
        uf.union(0, 1);
        assert!(!uf.union(0, 1)); // Already in same set
        assert_eq!(uf.set_count(), 2);
    }
    
    #[test]
    fn test_connected_components() {
        let edges = vec![(0, 1), (1, 2), (3, 4)];
        assert_eq!(count_components(5, &edges), 2);
    }
}
