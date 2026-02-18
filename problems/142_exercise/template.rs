// Exercise 142: Cycle Detection in Undirected Graph
//
// Learning Objective:
// Implement cycle detection in undirected graphs using DFS and Union-Find.
// Understand different approaches to cycle detection.
//
// Key Concepts:
// - DFS-based cycle detection (check for back edges)
// - Union-Find based detection
// - Avoiding false positives from parent edge

use std::collections::{HashSet, VecDeque};

/// Graph using adjacency list
#[derive(Debug, Clone)]
struct Graph {
    vertices: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adj: vec![Vec::new(); vertices],
        }
    }
    
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
}

/// TODO: Detect cycle using DFS
/// Returns true if the graph contains any cycle
/// 
/// Approach:
/// For each unvisited vertex, do DFS
/// If we encounter a visited vertex that is not the parent, there's a cycle
fn has_cycle_dfs(graph: &Graph) -> bool {
    let mut visited = HashSet::new();
    
    // TODO: For each unvisited vertex, run DFS helper
    // If any DFS finds a cycle, return true
    
    false
}

/// TODO: DFS helper for cycle detection
/// Returns true if a cycle is found starting from vertex u
/// parent is the vertex we came from (to avoid false positive)
fn dfs_cycle_helper(graph: &Graph, u: usize, parent: Option<usize>, visited: &mut HashSet<usize>) -> bool {
    // TODO: Mark u as visited
    
    // TODO: For each neighbor v of u:
    //   If v is not visited: recursively check, if cycle found return true
    //   If v is visited AND v != parent: cycle found, return true
    
    false
}

/// TODO: Detect cycle using BFS
/// Alternative approach using BFS with parent tracking
fn has_cycle_bfs(graph: &Graph) -> bool {
    let mut visited = HashSet::new();
    
    // TODO: For each unvisited vertex, do BFS
    // Use queue of (vertex, parent) tuples
    // If we see a visited vertex that's not parent, cycle exists
    
    false
}

/// Union-Find data structure for cycle detection
#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    /// TODO: Find with path compression
    fn find(&mut self, x: usize) -> usize {
        // TODO: If parent[x] != x, set parent[x] = find(parent[x])
        // Return parent[x]
        
        self.parent[x]
    }
    
    /// TODO: Union by rank
    /// Returns true if union was performed (they were in different sets)
    /// Returns false if they were already in the same set (cycle detected!)
    fn union(&mut self, x: usize, y: usize) -> bool {
        // TODO: Find roots of x and y
        // If same root, return false (cycle!)
        // Otherwise, union by rank and return true
        
        true
    }
}

/// TODO: Detect cycle using Union-Find
fn has_cycle_union_find(graph: &Graph) -> bool {
    // TODO: Create UnionFind with graph.vertices elements
    
    // TODO: For each edge (u, v):
    //   If find(u) == find(v), cycle exists
    //   Otherwise, union(u, v)
    // Need to track edges to avoid processing same edge twice
    
    false
}

/// TODO: Find a specific cycle and return its vertices
/// Returns Some(cycle_vertices) if cycle exists, None otherwise
fn find_cycle(graph: &Graph) -> Option<Vec<usize>> {
    // BONUS CHALLENGE
    // Find and return the actual cycle path
    // Use DFS with parent tracking, when cycle found backtrack to construct path
    
    None
}

/// Check if graph is a tree (connected and acyclic)
fn is_tree(graph: &Graph) -> bool {
    // A tree has exactly n-1 edges and is connected
    // Or: acyclic and connected
    
    let edge_count: usize = graph.adj.iter().map(|v| v.len()).sum::<usize>() / 2;
    if edge_count != graph.vertices - 1 {
        return false;
    }
    
    // Check connectivity
    let mut visited = HashSet::new();
    dfs_connectivity(graph, 0, &mut visited);
    visited.len() == graph.vertices
}

fn dfs_connectivity(graph: &Graph, u: usize, visited: &mut HashSet<usize>) {
    visited.insert(u);
    for &v in &graph.adj[u] {
        if !visited.contains(&v) {
            dfs_connectivity(graph, v, visited);
        }
    }
}

fn main() {
    // Graph with cycle: 0 -- 1 -- 2 -- 0
    let mut graph1 = Graph::new(3);
    graph1.add_edge(0, 1);
    graph1.add_edge(1, 2);
    graph1.add_edge(2, 0);
    
    println!("Graph 1 (has cycle):");
    println!("  DFS: {}", has_cycle_dfs(&graph1));
    println!("  BFS: {}", has_cycle_bfs(&graph1));
    println!("  Union-Find: {}", has_cycle_union_find(&graph1));
    
    // Tree (no cycle): 0 -- 1 -- 2
    let mut graph2 = Graph::new(3);
    graph2.add_edge(0, 1);
    graph2.add_edge(1, 2);
    
    println!("\nGraph 2 (tree, no cycle):");
    println!("  DFS: {}", has_cycle_dfs(&graph2));
    println!("  BFS: {}", has_cycle_bfs(&graph2));
    println!("  Union-Find: {}", has_cycle_union_find(&graph2));
    println!("  Is tree: {}", is_tree(&graph2));
    
    // Disconnected graph with cycle in one component
    let mut graph3 = Graph::new(5);
    graph3.add_edge(0, 1);
    graph3.add_edge(1, 2);
    graph3.add_edge(2, 0); // Cycle in component 1
    graph3.add_edge(3, 4); // Component 2 (no cycle)
    
    println!("\nGraph 3 (disconnected, has cycle):");
    println!("  DFS: {}", has_cycle_dfs(&graph3));
    println!("  Is tree: {}", is_tree(&graph3));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cycle_triangle() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        
        assert!(has_cycle_dfs(&g));
        assert!(has_cycle_bfs(&g));
        assert!(has_cycle_union_find(&g));
    }
    
    #[test]
    fn test_no_cycle_tree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        
        assert!(!has_cycle_dfs(&g));
        assert!(!has_cycle_bfs(&g));
        assert!(!has_cycle_union_find(&g));
        assert!(is_tree(&g));
    }
    
    #[test]
    fn test_cycle_square() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        
        assert!(has_cycle_dfs(&g));
        assert!(has_cycle_union_find(&g));
    }
    
    #[test]
    fn test_disconnected_with_cycle() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0); // Cycle
        g.add_edge(3, 4); // No cycle here
        
        assert!(has_cycle_dfs(&g));
        assert!(!is_tree(&g));
    }
    
    #[test]
    fn test_single_edge() {
        let mut g = Graph::new(2);
        g.add_edge(0, 1);
        
        assert!(!has_cycle_dfs(&g));
        assert!(!has_cycle_union_find(&g));
    }
    
    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1);
        
        assert!(!has_cycle_dfs(&g));
        assert!(!has_cycle_union_find(&g));
    }
    
    #[test]
    fn test_all_methods_agree() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        g.add_edge(2, 4);
        
        let dfs_result = has_cycle_dfs(&g);
        let bfs_result = has_cycle_bfs(&g);
        let uf_result = has_cycle_union_find(&g);
        
        assert_eq!(dfs_result, bfs_result);
        assert_eq!(dfs_result, uf_result);
    }
}
