// Exercise 140: Graph Representation - Adjacency List
//
// Learning Objective:
// Implement a graph using adjacency list representation.
// Understand when and why to use this representation.
//
// Key Concepts:
// - Adjacency list representation
// - Space efficient for sparse graphs: O(V + E)
// - Fast iteration over neighbors
// - Directed and undirected graphs

use std::collections::{HashMap, HashSet, VecDeque};

/// TODO: Implement a graph using adjacency list representation
/// The graph can be either directed or undirected
#[derive(Debug, Clone)]
struct Graph {
    /// Number of vertices
    vertices: usize,
    /// For directed graph: adj[u] contains all v where edge u->v exists
    /// For undirected graph: adj[u] contains all v where edge u-v exists
    adj: Vec<Vec<usize>>,
    /// Optional: track if graph is directed
    directed: bool,
    /// Optional: edge weights (for weighted graph extension)
    /// weights[u][i] = weight of edge from u to adj[u][i]
    weights: Option<Vec<Vec<i32>>>,
}

impl Graph {
    /// TODO: Create a new graph with given number of vertices
    fn new(vertices: usize, directed: bool) -> Self {
        // TODO: Initialize adjacency list with empty vectors for each vertex
        Graph {
            vertices,
            adj: vec![Vec::new(); vertices],
            directed,
            weights: None,
        }
    }
    
    /// TODO: Add an edge to the graph
    fn add_edge(&mut self, u: usize, v: usize) {
        // TODO: Validate vertices are in bounds
        // Add edge u -> v
        // If undirected, also add v -> u
        
        if u >= self.vertices || v >= self.vertices {
            panic!("Vertex out of bounds");
        }
        
        self.adj[u].push(v);
        if !self.directed {
            self.adj[v].push(u);
        }
    }
    
    /// TODO: Remove an edge from the graph
    fn remove_edge(&mut self, u: usize, v: usize) -> bool {
        // TODO: Remove edge u -> v if it exists
        // Return true if edge was found and removed
        // If undirected, also remove v -> u
        
        false
    }
    
    /// TODO: Check if edge exists
    fn has_edge(&self, u: usize, v: usize) -> bool {
        // TODO: Check if v is in adj[u]
        
        false
    }
    
    /// TODO: Get all neighbors of a vertex
    fn neighbors(&self, u: usize) -> &[usize] {
        // TODO: Return slice of neighbors
        // Handle out of bounds
        
        &[]
    }
    
    /// TODO: Get degree of a vertex
    /// For undirected: number of edges incident to vertex
    /// For directed: out-degree (number of outgoing edges)
    fn degree(&self, u: usize) -> usize {
        // TODO: Return appropriate degree
        
        0
    }
    
    /// TODO: Get in-degree of a vertex (for directed graphs)
    fn in_degree(&self, u: usize) -> usize {
        // TODO: Count how many vertices have an edge to u
        
        0
    }
    
    /// TODO: Count total number of edges
    fn edge_count(&self) -> usize {
        // TODO: Count edges
        // For undirected, each edge is stored twice
        
        0
    }
    
    /// TODO: Transpose the graph (reverse all edges)
    /// Returns a new graph with all edges reversed
    fn transpose(&self) -> Graph {
        // TODO: Create new graph
        // For each edge u -> v in original, add v -> u in transposed
        
        Graph::new(0, true)
    }
    
    /// TODO: Perform BFS and return traversal order
    fn bfs(&self, start: usize) -> Vec<usize> {
        // TODO: BFS implementation
        
        vec![]
    }
    
    /// TODO: Perform DFS and return traversal order
    fn dfs(&self, start: usize) -> Vec<usize> {
        // TODO: DFS implementation
        
        vec![]
    }
}

/// Alternative: Graph using HashMap for sparse/dynamic graphs
#[derive(Debug)]
struct HashGraph<T: std::hash::Hash + Eq + Clone> {
    adj: HashMap<T, Vec<T>>,
    directed: bool,
}

impl<T: std::hash::Hash + Eq + Clone> HashGraph<T> {
    fn new(directed: bool) -> Self {
        HashGraph {
            adj: HashMap::new(),
            directed,
        }
    }
    
    fn add_vertex(&mut self, v: T) {
        self.adj.entry(v).or_insert_with(Vec::new);
    }
    
    fn add_edge(&mut self, u: T, v: T) {
        self.adj.entry(u.clone()).or_insert_with(Vec::new).push(v.clone());
        self.adj.entry(v.clone()).or_insert_with(Vec::new);
        if !self.directed {
            self.adj.entry(v).or_insert_with(Vec::new).push(u);
        }
    }
}

fn main() {
    // Create an undirected graph
    let mut graph = Graph::new(5, false);
    
    // Create graph: 0 -- 1 -- 2
    //               |    |
    //               3 -- 4
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(0, 3);
    graph.add_edge(1, 4);
    graph.add_edge(3, 4);
    
    println!("Undirected Graph:");
    println!("Adjacency list: {:?}", graph.adj);
    println!("Degree of vertex 1: {}", graph.degree(1));
    println!("Neighbors of vertex 1: {:?}", graph.neighbors(1));
    println!("Total edges: {}", graph.edge_count());
    
    println!("\nBFS from 0: {:?}", graph.bfs(0));
    println!("DFS from 0: {:?}", graph.dfs(0));
    
    // Create a directed graph
    let mut digraph = Graph::new(4, true);
    digraph.add_edge(0, 1);
    digraph.add_edge(0, 2);
    digraph.add_edge(1, 2);
    digraph.add_edge(2, 3);
    
    println!("\nDirected Graph:");
    println!("Adjacency list: {:?}", digraph.adj);
    println!("Out-degree of 0: {}", digraph.degree(0));
    println!("In-degree of 2: {}", digraph.in_degree(2));
    
    let transposed = digraph.transpose();
    println!("Transposed graph: {:?}", transposed.adj);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_undirected_graph() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0)); // Undirected
        assert_eq!(g.degree(1), 2);
        assert_eq!(g.edge_count(), 2);
    }
    
    #[test]
    fn test_directed_graph() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        
        assert!(g.has_edge(0, 1));
        assert!(!g.has_edge(1, 0)); // Directed
        assert_eq!(g.degree(0), 1); // Out-degree
        assert_eq!(g.in_degree(1), 1);
        assert_eq!(g.edge_count(), 2);
    }
    
    #[test]
    fn test_remove_edge() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1);
        assert!(g.remove_edge(0, 1));
        assert!(!g.has_edge(0, 1));
        assert!(!g.has_edge(1, 0));
        assert!(!g.remove_edge(0, 1)); // Already removed
    }
    
    #[test]
    fn test_transpose() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        
        let t = g.transpose();
        assert!(t.has_edge(1, 0));
        assert!(t.has_edge(2, 1));
        assert!(!t.has_edge(0, 1));
    }
    
    #[test]
    fn test_bfs_dfs() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        
        let bfs_result = g.bfs(0);
        assert_eq!(bfs_result[0], 0);
        assert!(bfs_result.contains(&3));
        
        let dfs_result = g.dfs(0);
        assert_eq!(dfs_result[0], 0);
        assert!(dfs_result.contains(&3));
    }
}
