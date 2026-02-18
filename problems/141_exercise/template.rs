// Exercise 141: Graph Representation - Adjacency Matrix
//
// Learning Objective:
// Implement a graph using adjacency matrix representation.
// Understand tradeoffs between adjacency matrix and list.
//
// Key Concepts:
// - 2D matrix representation
// - Fast edge lookup: O(1)
// - Space: O(V²) regardless of edge count
// - Good for dense graphs

use std::collections::VecDeque;

/// TODO: Implement a graph using adjacency matrix representation
#[derive(Debug, Clone)]
struct Graph {
    /// Number of vertices
    vertices: usize,
    /// Adjacency matrix: matrix[u][v] = true if edge u->v exists
    matrix: Vec<Vec<bool>>,
    /// Whether the graph is directed
    directed: bool,
    /// Optional: for weighted graphs, store weights instead of bool
    /// Use Option<i32> where None means no edge
    weighted: Option<Vec<Vec<Option<i32>>>>,
}

impl Graph {
    /// TODO: Create a new graph with given number of vertices
    fn new(vertices: usize, directed: bool) -> Self {
        // TODO: Initialize V x V matrix with all false
        Graph {
            vertices,
            matrix: vec![vec![false; vertices]; vertices],
            directed,
            weighted: None,
        }
    }
    
    /// TODO: Create a weighted graph
    fn new_weighted(vertices: usize, directed: bool) -> Self {
        // TODO: Initialize with None for all entries
        Graph {
            vertices,
            matrix: vec![vec![false; vertices]; vertices],
            directed,
            weighted: Some(vec![vec![None; vertices]; vertices]),
        }
    }
    
    /// TODO: Add an edge to the graph
    fn add_edge(&mut self, u: usize, v: usize) {
        // TODO: Validate bounds
        // Set matrix[u][v] = true
        // If undirected, set matrix[v][u] = true
        
        if u >= self.vertices || v >= self.vertices {
            panic!("Vertex out of bounds");
        }
        
        self.matrix[u][v] = true;
        if !self.directed {
            self.matrix[v][u] = true;
        }
    }
    
    /// TODO: Add a weighted edge
    fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i32) {
        // TODO: Add edge with weight
        // Update both matrix and weighted
        
        self.add_edge(u, v);
        if let Some(ref mut w) = self.weighted {
            w[u][v] = Some(weight);
            if !self.directed {
                w[v][u] = Some(weight);
            }
        }
    }
    
    /// TODO: Remove an edge
    fn remove_edge(&mut self, u: usize, v: usize) {
        // TODO: Set matrix[u][v] = false
        // Also clear weight if weighted
        
        if u < self.vertices && v < self.vertices {
            self.matrix[u][v] = false;
            if !self.directed {
                self.matrix[v][u] = false;
            }
            if let Some(ref mut w) = self.weighted {
                w[u][v] = None;
                if !self.directed {
                    w[v][u] = None;
                }
            }
        }
    }
    
    /// TODO: Check if edge exists
    fn has_edge(&self, u: usize, v: usize) -> bool {
        // TODO: Return matrix[u][v] if in bounds
        
        if u < self.vertices && v < self.vertices {
            self.matrix[u][v]
        } else {
            false
        }
    }
    
    /// TODO: Get edge weight
    fn get_weight(&self, u: usize, v: usize) -> Option<i32> {
        // TODO: Return weight if edge exists and graph is weighted
        
        None
    }
    
    /// TODO: Get all neighbors of a vertex
    fn neighbors(&self, u: usize) -> Vec<usize> {
        // TODO: Return all v where matrix[u][v] is true
        
        vec![]
    }
    
    /// TODO: Get degree (out-degree for directed)
    fn degree(&self, u: usize) -> usize {
        // TODO: Count edges from u
        
        0
    }
    
    /// TODO: Get in-degree
    fn in_degree(&self, u: usize) -> usize {
        // TODO: Count edges to u
        
        0
    }
    
    /// TODO: Count total edges
    fn edge_count(&self) -> usize {
        // TODO: Count all true values
        // Account for undirected (don't double count)
        
        0
    }
    
    /// TODO: Get all edges as vector of tuples
    fn get_edges(&self) -> Vec<(usize, usize)> {
        // TODO: Return all edges as (u, v) tuples
        // For undirected, only return u < v to avoid duplicates
        
        vec![]
    }
    
    /// TODO: Convert to adjacency list representation
    fn to_adjacency_list(&self) -> Vec<Vec<usize>> {
        // TODO: Create adjacency list from matrix
        
        vec![]
    }
    
    /// TODO: Check if graph is complete
    fn is_complete(&self) -> bool {
        // TODO: Check if every pair of distinct vertices has an edge
        // For undirected: n*(n-1)/2 edges
        // For directed: n*(n-1) edges
        
        false
    }
}

fn main() {
    // Create undirected graph
    let mut graph = Graph::new(4, false);
    
    // Create: 0 -- 1
    //         |  / |
    //         2 -- 3
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    
    println!("Undirected Graph:");
    println!("Adjacency Matrix:");
    for row in &graph.matrix {
        println!("  {:?}", row.iter().map(|&b| if b { 1 } else { 0 }).collect::<Vec<_>>());
    }
    
    println!("\nNeighbors of 1: {:?}", graph.neighbors(1));
    println!("Degree of 1: {}", graph.degree(1));
    println!("Total edges: {}", graph.edge_count());
    println!("Is complete: {}", graph.is_complete());
    
    // Create weighted directed graph
    let mut wgraph = Graph::new_weighted(4, true);
    wgraph.add_weighted_edge(0, 1, 5);
    wgraph.add_weighted_edge(0, 2, 3);
    wgraph.add_weighted_edge(1, 2, 1);
    wgraph.add_weighted_edge(2, 3, 7);
    
    println!("\nWeighted Directed Graph:");
    if let Some(ref weights) = wgraph.weighted {
        for row in weights {
            println!("  {:?}", row);
        }
    }
    println!("Weight of edge 0->1: {:?}", wgraph.get_weight(0, 1));
    
    // Convert to adjacency list
    let adj_list = graph.to_adjacency_list();
    println!("\nAs adjacency list: {:?}", adj_list);
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
        assert!(!g.has_edge(0, 2));
        assert_eq!(g.degree(1), 2);
    }
    
    #[test]
    fn test_directed_graph() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        
        assert!(g.has_edge(0, 1));
        assert!(!g.has_edge(1, 0)); // Directed
        assert_eq!(g.degree(0), 1);
        assert_eq!(g.in_degree(1), 1);
    }
    
    #[test]
    fn test_weighted_graph() {
        let mut g = Graph::new_weighted(3, false);
        g.add_weighted_edge(0, 1, 10);
        g.add_weighted_edge(1, 2, 20);
        
        assert_eq!(g.get_weight(0, 1), Some(10));
        assert_eq!(g.get_weight(1, 0), Some(10)); // Undirected
        assert_eq!(g.get_weight(0, 2), None);
    }
    
    #[test]
    fn test_complete_graph() {
        let mut g = Graph::new(4, false);
        // Complete graph K4 has 6 edges
        for i in 0..4 {
            for j in (i+1)..4 {
                g.add_edge(i, j);
            }
        }
        assert!(g.is_complete());
        assert_eq!(g.edge_count(), 6);
    }
    
    #[test]
    fn test_remove_edge() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1);
        assert!(g.has_edge(0, 1));
        g.remove_edge(0, 1);
        assert!(!g.has_edge(0, 1));
        assert!(!g.has_edge(1, 0));
    }
    
    #[test]
    fn test_neighbors() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        
        let n = g.neighbors(0);
        assert_eq!(n.len(), 3);
        assert!(n.contains(&1));
        assert!(n.contains(&2));
        assert!(n.contains(&3));
    }
}
