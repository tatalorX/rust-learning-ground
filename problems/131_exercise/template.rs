// Exercise 131: Breadth First Search (BFS) on Graph
//
// Learning Objective:
// Implement BFS traversal on a graph. Understand level-order traversal
// and its applications in finding shortest paths in unweighted graphs.
//
// Key Concepts:
// - Queue-based traversal
// - Level-order processing
// - Shortest path in unweighted graphs
// - Time complexity: O(V + E)

use std::collections::{HashSet, VecDeque};

/// Graph represented using adjacency list
#[derive(Debug, Clone)]
struct Graph {
    vertices: usize,
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
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
    
    /// TODO: Implement BFS traversal
    /// Performs BFS starting from the given vertex
    /// Returns a vector containing vertices in BFS order
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // TODO: Mark start as visited and enqueue it
        
        // TODO: Process queue until empty
        // Dequeue vertex, add to result, enqueue unvisited neighbors
        
        result
    }
    
    /// TODO: Implement BFS that returns distances from start
    /// Returns a vector where distances[i] is the shortest distance
    /// from start to vertex i (usize::MAX if unreachable)
    fn bfs_with_distance(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.vertices];
        let mut queue = VecDeque::new();
        
        // TODO: Set distance of start to 0 and enqueue it
        
        // TODO: Process queue, updating distances for each neighbor
        // If neighbor hasn't been visited (distance is MAX),
        // set its distance to current distance + 1
        
        distances
    }
    
    /// TODO: Find shortest path between two vertices using BFS
    /// Returns the path as a vector of vertices, or None if no path exists
    fn shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        // TODO: Implement using BFS with parent tracking
        // Use a HashMap or Vec to track parent of each vertex
        // After BFS, reconstruct path from end to start using parents
        
        None
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
    
    println!("\nBFS from vertex 0: {:?}", graph.bfs(0));
    
    let distances = graph.bfs_with_distance(0);
    println!("\nDistances from vertex 0: {:?}", distances);
    
    if let Some(path) = graph.shortest_path(3, 5) {
        println!("\nShortest path from 3 to 5: {:?}", path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bfs_simple() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        
        let result = graph.bfs(0);
        assert_eq!(result[0], 0);
        // Level 1: 1, 2
        assert!(result[1] == 1 || result[1] == 2);
        assert!(result[2] == 1 || result[2] == 2);
    }
    
    #[test]
    fn test_bfs_distances() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        
        let distances = graph.bfs_with_distance(0);
        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], 1);
        assert_eq!(distances[3], 2);
        assert_eq!(distances[4], 2);
        assert_eq!(distances[5], 2);
    }
    
    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 4); // Alternative path
        
        let path = graph.shortest_path(3, 5).unwrap();
        assert_eq!(path[0], 3);
        assert_eq!(path[path.len() - 1], 5);
        assert!(path.len() <= 4); // Should find shortest path
    }
    
    #[test]
    fn test_shortest_path_no_path() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);
        
        assert!(graph.shortest_path(0, 2).is_none());
    }
}
