// Exercise 130: Depth First Search (DFS) on Graph
//
// Learning Objective:
// Implement DFS traversal on a graph using both recursive and iterative approaches.
// Understand graph traversal and its applications.
//
// Key Concepts:
// - Graph traversal
// - Stack (explicit or implicit via recursion)
// - Visited set to avoid cycles
// - Time complexity: O(V + E)

use std::collections::{HashSet, VecDeque};

/// Graph represented using adjacency list
#[derive(Debug, Clone)]
struct Graph {
    /// Number of vertices
    vertices: usize,
    /// Adjacency list: graph[vertex] = list of neighbors
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates a new graph with given number of vertices
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adj: vec![Vec::new(); vertices],
        }
    }
    
    /// Adds an undirected edge between u and v
    fn add_edge(&mut self, u: usize, v: usize) {
        // TODO: Add v to adj[u] and u to adj[v]
        // Remember to check bounds
    }
    
    /// TODO: Implement recursive DFS
    /// Performs DFS starting from the given vertex
    /// Returns a vector containing vertices in DFS order
    fn dfs_recursive(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        
        // TODO: Call the helper function
        
        result
    }
    
    /// TODO: Helper function for recursive DFS
    fn dfs_helper(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        // TODO: Mark current vertex as visited and add to result
        
        // TODO: Recursively visit all unvisited neighbors
    }
    
    /// TODO: Implement iterative DFS using an explicit stack
    /// Performs DFS starting from the given vertex using a stack
    fn dfs_iterative(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = VecDeque::new();
        
        // TODO: Push start vertex to stack and mark as visited
        
        // TODO: Process stack until empty
        // Pop vertex, add to result, push unvisited neighbors
        
        result
    }
}

fn main() {
    // Create a sample graph
    //      0
    //     / \
    //    1   2
    //   /   / \
    //  3   4   5
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);
    
    println!("Graph adjacency list: {:?}", graph.adj);
    
    println!("\nDFS Recursive from vertex 0: {:?}", graph.dfs_recursive(0));
    println!("DFS Iterative from vertex 0: {:?}", graph.dfs_iterative(0));
    
    // Test with disconnected graph
    let mut graph2 = Graph::new(5);
    graph2.add_edge(0, 1);
    graph2.add_edge(2, 3); // Disconnected component
    
    println!("\nDisconnected graph - DFS from 0: {:?}", graph2.dfs_recursive(0));
    println!("Disconnected graph - DFS from 2: {:?}", graph2.dfs_recursive(2));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        
        let result = graph.dfs_recursive(0);
        assert_eq!(result[0], 0); // First vertex should be start
        assert!(result.contains(&3)); // Should reach vertex 3
    }
    
    #[test]
    fn test_dfs_disconnected() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);
        
        let result = graph.dfs_recursive(0);
        assert_eq!(result.len(), 2); // Should only visit 0 and 1
        assert!(!result.contains(&2));
        assert!(!result.contains(&3));
    }
    
    #[test]
    fn test_dfs_single_vertex() {
        let graph = Graph::new(1);
        let result = graph.dfs_recursive(0);
        assert_eq!(result, vec![0]);
    }
    
    #[test]
    fn test_dfs_iterative_matches_recursive() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        
        let rec_result = graph.dfs_recursive(0);
        let iter_result = graph.dfs_iterative(0);
        
        // Both should contain same vertices
        assert_eq!(rec_result.len(), iter_result.len());
        for v in &rec_result {
            assert!(iter_result.contains(v));
        }
    }
}
