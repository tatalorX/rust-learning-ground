// Exercise 090: Advanced Traits - Associated Types
//
// Learning objective: Understand associated types in traits
// and how they differ from generic parameters.
//
// Associated types connect a type placeholder with a trait.
// Unlike generics, each implementation has exactly ONE associated type,
// making code cleaner when the type relationship is 1:1.

// TODO: Define a trait `Iterator` with an associated type `Item`
// and a method `next(&mut self) -> Option<Self::Item>`
pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// TODO: Implement MyIterator for a Counter struct
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl MyIterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// TODO: Define a trait `Container` with associated types
pub trait Container {
    type Item;
    type Error;
    
    fn get(&self, index: usize) -> Result<&Self::Item, Self::Error>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// TODO: Implement Container for Vec<T>
impl<T> Container for Vec<T> {
    type Item = T;
    type Error = &'static str;
    
    fn get(&self, index: usize) -> Result<&Self::Item, Self::Error> {
        if index < self.len() {
            Ok(&self[index])
        } else {
            Err("Index out of bounds")
        }
    }
    
    fn len(&self) -> usize {
        self.len()
    }
}

// TODO: Define a trait with multiple associated types
pub trait Graph {
    type Node;
    type Edge;
    
    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;
    fn has_edge(&self, from: &Self::Node, to: &Self::Node) -> bool;
}

// TODO: Implement a simple Graph
struct SimpleGraph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph for SimpleGraph {
    type Node = usize;
    type Edge = (usize, usize);
    
    fn nodes(&self) -> Vec<Self::Node> {
        (0..self.adjacency_list.len()).collect()
    }
    
    fn edges(&self) -> Vec<Self::Edge> {
        let mut edges = vec![];
        for (from, neighbors) in self.adjacency_list.iter().enumerate() {
            for &to in neighbors {
                edges.push((from, to));
            }
        }
        edges
    }
    
    fn has_edge(&self, from: &Self::Node, to: &Self::Node) -> bool {
        self.adjacency_list.get(*from)
            .map(|neighbors| neighbors.contains(to))
            .unwrap_or(false)
    }
}

fn main() {
    // TODO: Test the Counter iterator
    let mut counter = Counter::new(5);
    println!("Counter values:");
    while let Some(val) = counter.next() {
        println!("  {}", val);
    }

    // TODO: Test Container trait with Vec
    let vec = vec![10, 20, 30, 40, 50];
    println!("\nContainer operations:");
    println!("Length: {}", vec.len());
    println!("Item at 2: {:?}", vec.get(2));
    println!("Item at 10: {:?}", vec.get(10));

    // TODO: Test Graph trait
    let graph = SimpleGraph {
        adjacency_list: vec![
            vec![1, 2],    // 0 -> 1, 2
            vec![2],       // 1 -> 2
            vec![],        // 2 ->
        ],
    };
    
    println!("\nGraph operations:");
    println!("Nodes: {:?}", graph.nodes());
    println!("Edges: {:?}", graph.edges());
    println!("Has edge 0->1: {}", graph.has_edge(&0, &1));
    println!("Has edge 1->0: {}", graph.has_edge(&1, &0));

    // TODO: Demonstrate why associated types are useful
    // Compare: Iterator<T> (generic) vs Iterator with associated type
    // With associated types, you don't need to specify T every time
    process_iterator();
}

// TODO: Complete this generic function that works with any Container
fn print_first<C: Container>(container: &C)
where
    C::Item: std::fmt::Display,
{
    match container.get(0) {
        Ok(item) => println!("First item: {}", item),
        Err(e) => println!("Error: {}", e),
    }
}

// TODO: Complete this function that uses associated types
fn process_iterator() {
    let mut counter = Counter::new(3);
    
    // The type of item is known from the associated type
    // We don't need to specify it explicitly
    while let Some(item) = counter.next() {
        let _squared: u32 = item * item; // Type is inferred!
        println!("Squared: {}", _squared);
    }
}

// TODO: Define a trait for convertible types
pub trait Convertible {
    type Output;
    fn convert(&self) -> Self::Output;
}

impl Convertible for String {
    type Output = Vec<u8>;
    
    fn convert(&self) -> Self::Output {
        self.bytes().collect()
    }
}

impl Convertible for &[u8] {
    type Output = String;
    
    fn convert(&self) -> Self::Output {
        String::from_utf8_lossy(self).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_iterator() {
        let mut counter = Counter::new(3);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_container_vec() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.get(0), Ok(&1));
        assert_eq!(vec.get(5), Err("Index out of bounds"));
    }

    #[test]
    fn test_convertible() {
        let s = String::from("hello");
        let bytes: Vec<u8> = s.convert();
        assert_eq!(bytes, vec![104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_graph() {
        let graph = SimpleGraph {
            adjacency_list: vec![vec![1], vec![], vec![]],
        };
        assert!(graph.has_edge(&0, &1));
        assert!(!graph.has_edge(&1, &0));
    }
}
