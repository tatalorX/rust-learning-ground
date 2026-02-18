// Exercise 143: Topological Sort
//
// Learning Objective:
// Implement topological sorting of a directed acyclic graph (DAG).
// Understand Kahn's algorithm and DFS-based approach.
//
// Key Concepts:
// - Topological ordering (linear ordering respecting edge directions)
// - Kahn's algorithm (BFS-based, using in-degrees)
// - DFS-based approach
// - Cycle detection in DAG verification

use std::collections::{HashSet, VecDeque};

/// Directed Graph
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
    }
    
    /// TODO: Calculate in-degree of all vertices
    fn calculate_in_degrees(&self) -> Vec<usize> {
        // TODO: Count incoming edges for each vertex
        
        vec![]
    }
}

/// TODO: Topological sort using Kahn's algorithm (BFS)
/// 
/// Algorithm:
/// 1. Calculate in-degree of all vertices
/// 2. Enqueue all vertices with in-degree 0
/// 3. While queue not empty:
///    - Dequeue vertex u, add to result
///    - For each neighbor v of u: decrease in-degree, if 0 enqueue
/// 4. If result size != vertices, graph has cycle
fn topological_sort_kahn(graph: &Graph) -> Option<Vec<usize>> {
    // TODO: Calculate in-degrees
    
    // TODO: Initialize queue with all vertices having in-degree 0
    
    // TODO: Process queue, building result
    
    // TODO: If result length equals vertices, return Some(result)
    // Otherwise, graph has cycle, return None
    
    None
}

/// TODO: Topological sort using DFS
/// 
/// Algorithm:
/// 1. Mark all vertices as unvisited
/// 2. For each unvisited vertex, call DFS helper
/// 3. DFS helper: mark as visiting, recurse on neighbors,
///    then push to result when done
/// 4. Reverse the result
fn topological_sort_dfs(graph: &Graph) -> Option<Vec<usize>> {
    // TODO: Implement DFS-based topological sort
    // Use three colors: White (unvisited), Gray (visiting), Black (visited)
    // If we encounter a Gray vertex, there's a cycle
    
    None
}

/// DFS helper for topological sort
/// Returns false if cycle detected
fn dfs_topological(
    graph: &Graph,
    u: usize,
    visited: &mut [i32], // 0=white, 1=gray, 2=black
    result: &mut Vec<usize>,
) -> bool {
    // TODO: Mark u as gray (visiting)
    
    // TODO: For each neighbor:
    //   If gray: cycle detected, return false
    //   If white: recursively visit, if false return false
    
    // TODO: Mark u as black (visited) and push to result
    
    true
}

/// TODO: Check if a given ordering is a valid topological sort
fn is_valid_topological_order(graph: &Graph, order: &[usize]) -> bool {
    // TODO: Verify that order is a valid topological ordering
    // For each edge u -> v in graph, u must appear before v in order
    // Also verify order contains all vertices exactly once
    
    false
}

/// TODO: Find all topological orderings
/// Returns vector of all valid topological sorts
fn all_topological_sorts(graph: &Graph) -> Vec<Vec<usize>> {
    // BONUS CHALLENGE
    // Use backtracking to find all possible orderings
    // At each step, choose any vertex with in-degree 0
    
    vec![]
}

/// Check if graph is a DAG (Directed Acyclic Graph)
fn is_dag(graph: &Graph) -> bool {
    topological_sort_kahn(graph).is_some()
}

fn main() {
    // Example: Course prerequisites
    // 0: Introduction to Programming
    // 1: Data Structures (requires 0)
    // 2: Algorithms (requires 1)
    // 3: Databases (requires 0)
    // 4: Web Development (requires 0, 3)
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1); // Intro -> Data Structures
    graph.add_edge(1, 2); // Data Structures -> Algorithms
    graph.add_edge(0, 3); // Intro -> Databases
    graph.add_edge(0, 4); // Intro -> Web Dev
    graph.add_edge(3, 4); // Databases -> Web Dev
    
    println!("Course Prerequisites DAG:");
    println!("Adjacency list: {:?}\n", graph.adj);
    
    if let Some(order) = topological_sort_kahn(&graph) {
        println!("Kahn's topological order: {:?}", order);
        println!("Valid: {}", is_valid_topological_order(&graph, &order));
    }
    
    if let Some(order) = topological_sort_dfs(&graph) {
        println!("DFS topological order: {:?}", order);
        println!("Valid: {}", is_valid_topological_order(&graph, &order));
    }
    
    // Graph with cycle
    let mut cyclic = Graph::new(3);
    cyclic.add_edge(0, 1);
    cyclic.add_edge(1, 2);
    cyclic.add_edge(2, 0);
    
    println!("\nCyclic graph:");
    println!("Is DAG: {}", is_dag(&cyclic));
    println!("Kahn's result: {:?}", topological_sort_kahn(&cyclic));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topological_sort_linear() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        
        let order = topological_sort_kahn(&g).unwrap();
        assert_eq!(order[0], 0);
        assert_eq!(order[3], 3);
        assert!(is_valid_topological_order(&g, &order));
    }
    
    #[test]
    fn test_topological_sort_dag() {
        let mut g = Graph::new(6);
        g.add_edge(5, 2);
        g.add_edge(5, 0);
        g.add_edge(4, 0);
        g.add_edge(4, 1);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
        
        let kahn_order = topological_sort_kahn(&g).unwrap();
        let dfs_order = topological_sort_dfs(&g).unwrap();
        
        assert!(is_valid_topological_order(&g, &kahn_order));
        assert!(is_valid_topological_order(&g, &dfs_order));
        assert!(is_dag(&g));
    }
    
    #[test]
    fn test_cyclic_graph() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        
        assert!(topological_sort_kahn(&g).is_none());
        assert!(topological_sort_dfs(&g).is_none());
        assert!(!is_dag(&g));
    }
    
    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1);
        let order = topological_sort_kahn(&g).unwrap();
        assert_eq!(order, vec![0]);
    }
    
    #[test]
    fn test_disconnected_dag() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        
        let order = topological_sort_kahn(&g).unwrap();
        assert!(is_valid_topological_order(&g, &order));
        assert_eq!(order.len(), 4);
    }
    
    #[test]
    fn test_multiple_edges() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        
        let order = topological_sort_kahn(&g).unwrap();
        assert!(is_valid_topological_order(&g, &order));
        assert_eq!(order[0], 0);
        assert_eq!(order[3], 3);
    }
    
    #[test]
    fn test_kahn_equals_dfs() {
        let mut g = Graph::new(5);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(2, 4);
        
        let kahn = topological_sort_kahn(&g);
        let dfs = topological_sort_dfs(&g);
        
        assert!(kahn.is_some());
        assert!(dfs.is_some());
        assert_eq!(kahn.unwrap().len(), dfs.unwrap().len());
    }
}
