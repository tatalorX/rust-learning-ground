// Exercise 132: Dijkstra's Algorithm (Shortest Path)
//
// Learning Objective:
// Implement Dijkstra's algorithm to find the shortest path in a weighted graph
// with non-negative edge weights.
//
// Key Concepts:
// - Greedy algorithm
// - Priority queue (min-heap)
// - Relaxation of edges
// - Time complexity: O((V + E) log V) with binary heap

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Represents a weighted edge in the graph
#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    weight: u32,
}

/// Graph represented using adjacency list
#[derive(Debug, Clone)]
struct Graph {
    vertices: usize,
    adj: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adj: vec![Vec::new(); vertices],
        }
    }
    
    /// Adds a directed edge from u to v with given weight
    fn add_edge(&mut self, u: usize, v: usize, weight: u32) {
        self.adj[u].push(Edge { to: v, weight });
    }
    
    /// Adds an undirected edge between u and v
    fn add_undirected_edge(&mut self, u: usize, v: usize, weight: u32) {
        self.add_edge(u, v, weight);
        self.add_edge(v, u, weight);
    }
}

/// State for priority queue: (distance, vertex)
/// Implements Ord so that BinaryHeap becomes a min-heap
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    distance: u32,
    vertex: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering for min-heap
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// TODO: Implement Dijkstra's algorithm
/// Returns the shortest distances from start to all vertices
/// Unreachable vertices have distance u32::MAX
fn dijkstra(graph: &Graph, start: usize) -> Vec<u32> {
    let mut distances = vec![u32::MAX; graph.vertices];
    let mut heap = BinaryHeap::new();
    
    // TODO: Initialize distance to start as 0 and push to heap
    
    // TODO: Process the priority queue
    // While heap is not empty:
    //   Pop vertex with minimum distance
    //   If this distance is greater than known distance, skip
    //   For each neighbor, relax the edge
    //   If shorter path found, update distance and push to heap
    
    distances
}

/// TODO: Implement Dijkstra's algorithm with path reconstruction
/// Returns (distances, parents) where parents can be used to reconstruct paths
fn dijkstra_with_path(graph: &Graph, start: usize) -> (Vec<u32>, Vec<Option<usize>>) {
    let mut distances = vec![u32::MAX; graph.vertices];
    let mut parents: Vec<Option<usize>> = vec![None; graph.vertices];
    let mut heap = BinaryHeap::new();
    
    // TODO: Similar to dijkstra but track parents for path reconstruction
    
    (distances, parents)
}

/// Reconstructs path from start to end using parent array
fn reconstruct_path(parents: &[Option<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
    // TODO: Reconstruct path by following parents from end to start
    // Reverse the path at the end
    
    None
}

fn main() {
    // Create a sample weighted graph
    //       4
    //   0 ---- 1
    //   |      |
    // 1 |      | 2
    //   |      |
    //   2 ---- 3
    //       1
    let mut graph = Graph::new(4);
    graph.add_undirected_edge(0, 1, 4);
    graph.add_undirected_edge(0, 2, 1);
    graph.add_undirected_edge(1, 3, 2);
    graph.add_undirected_edge(2, 3, 1);
    
    println!("Graph edges: {:?}", graph.adj);
    
    let distances = dijkstra(&graph, 0);
    println!("\nShortest distances from vertex 0: {:?}", distances);
    
    let (distances, parents) = dijkstra_with_path(&graph, 0);
    println!("Distances: {:?}", distances);
    
    if let Some(path) = reconstruct_path(&parents, 0, 3) {
        println!("Shortest path from 0 to 3: {:?}", path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dijkstra_simple() {
        let mut graph = Graph::new(4);
        graph.add_undirected_edge(0, 1, 4);
        graph.add_undirected_edge(0, 2, 1);
        graph.add_undirected_edge(1, 3, 2);
        graph.add_undirected_edge(2, 3, 1);
        
        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 4);
        assert_eq!(distances[2], 1);
        assert_eq!(distances[3], 2); // 0 -> 2 -> 3 is shortest
    }
    
    #[test]
    fn test_dijkstra_unreachable() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);
        
        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], u32::MAX);
        assert_eq!(distances[3], u32::MAX);
    }
    
    #[test]
    fn test_dijkstra_directed() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 2);
        graph.add_edge(0, 2, 5);
        
        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[2], 3); // 0 -> 1 -> 2 is shorter than 0 -> 2
    }
    
    #[test]
    fn test_path_reconstruction() {
        let mut graph = Graph::new(4);
        graph.add_undirected_edge(0, 1, 4);
        graph.add_undirected_edge(0, 2, 1);
        graph.add_undirected_edge(1, 3, 2);
        graph.add_undirected_edge(2, 3, 1);
        
        let (_, parents) = dijkstra_with_path(&graph, 0);
        let path = reconstruct_path(&parents, 0, 3).unwrap();
        assert_eq!(path, vec![0, 2, 3]);
    }
}
