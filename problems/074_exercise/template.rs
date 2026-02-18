// Exercise 074: Iterators - collect
//
// The collect() method transforms an iterator into a collection.
// It can create Vec, HashMap, HashSet, String, and many other types.
//
// Learning Objectives:
// - Use collect() to build collections from iterators
// - Collect into different container types
// - Use collect with Result types for error handling
//
// Your task: Implement functions using collect() effectively.

use std::collections::{HashMap, HashSet};

/// Collects numbers into a vector.
fn to_vector(numbers: &[i32]) -> Vec<i32> {
    // TODO: Use iter().cloned().collect() or iter().copied().collect()
    todo!()
}

/// Collects unique numbers into a HashSet.
fn to_hashset(numbers: &[i32]) -> HashSet<i32> {
    // TODO: Use iter().copied().collect()
    todo!()
}

/// Creates a HashMap from pairs of keys and values.
fn pairs_to_map(keys: &[&str], values: &[i32]) -> HashMap<String, i32> {
    // TODO: Zip keys and values together, collect into HashMap
    // Map keys to String using .map(|k| k.to_string())
    todo!()
}

/// Collects iterator results into a Result<Vec<T>, E>.
fn try_collect(numbers: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    // TODO: Parse strings to i32, collect into Result
    // If any parse fails, the whole result is Err
    todo!()
}

/// Collects valid parses only, filtering out errors.
fn collect_valid(numbers: &[&str]) -> Vec<i32> {
    // TODO: Filter map to parse valid numbers only
    todo!()
}

/// Joins strings into a single string separated by commas.
fn join_with_comma(strings: &[&str]) -> String {
    // TODO: Use .join(",") which uses collect internally
    // Or use .intersperse and collect (nightly)
    // Simple solution: strings.join(",")
    todo!()
}

/// Creates a frequency map of characters.
fn char_frequency(text: &str) -> HashMap<char, usize> {
    // TODO: Count occurrences of each character
    // Use chars().fold or a for loop to build the map
    todo!()
}

/// Groups numbers by even/odd into a HashMap.
fn group_by_parity(numbers: &[i32]) -> HashMap<bool, Vec<i32>> {
    // TODO: Group numbers: true -> even numbers, false -> odd numbers
    // Hint: Use fold to build the map
    todo!()
}

/// Collects iterator of Results, partitioning into Ok and Err values.
fn partition_results(strings: &[&str]) -> (Vec<i32>, Vec<String>) {
    // TODO: Parse strings, partition into (successes, failures)
    // Use iter().partition() with a predicate checking is_ok()
    // Then extract values from Ok and Err variants
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // let numbers = vec![1, 2, 3, 2, 1];
    // println!("To vector: {:?}", to_vector(&numbers));
    // println!("To hashset: {:?}", to_hashset(&numbers));
    // 
    // let keys = vec!["one", "two", "three"];
    // let values = vec![1, 2, 3];
    // println!("\nPairs to map: {:?}", pairs_to_map(&keys, &values));
    // 
    // let parse_attempts = vec!["10", "20", "30"];
    // match try_collect(&parse_attempts) {
    //     Ok(nums) => println!("\nParsed: {:?}", nums),
    //     Err(e) => println!("\nParse error: {}", e),
    // }
    // 
    // let mixed = vec!["1", "abc", "2", "xyz", "3"];
    // println!("Valid parses: {:?}", collect_valid(&mixed));
    // 
    // let words = vec!["apple", "banana", "cherry"];
    // println!("\nJoined: {}", join_with_comma(&words));
    // 
    // let text = "hello world";
    // println!("\nChar frequency in '{}': {:?}", text, char_frequency(text));
    // 
    // let nums = vec![1, 2, 3, 4, 5, 6];
    // println!("\nGrouped by parity: {:?}", group_by_parity(&nums));
    // 
    // let to_parse = vec!["10", "20", "abc", "30", "xyz"];
    // let (successes, failures) = partition_results(&to_parse);
    // println!("\nPartitioned results:");
    // println!("  Successes: {:?}", successes);
    // println!("  Failures: {:?}", failures);
    
    println!("Implement all TODOs to see collect in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vector() {
        assert_eq!(to_vector(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_to_hashset() {
        let set = to_hashset(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_pairs_to_map() {
        let keys = vec!["a", "b"];
        let values = vec![1, 2];
        let map = pairs_to_map(&keys, &values);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_try_collect_success() {
        let input = vec!["1", "2", "3"];
        assert_eq!(try_collect(&input), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_try_collect_failure() {
        let input = vec!["1", "abc", "3"];
        assert!(try_collect(&input).is_err());
    }

    #[test]
    fn test_collect_valid() {
        let input = vec!["1", "abc", "2", "xyz"];
        assert_eq!(collect_valid(&input), vec![1, 2]);
    }

    #[test]
    fn test_join_with_comma() {
        assert_eq!(join_with_comma(&["a", "b", "c"]), "a,b,c");
    }

    #[test]
    fn test_char_frequency() {
        let freq = char_frequency("aba");
        assert_eq!(freq.get(&'a'), Some(&2));
        assert_eq!(freq.get(&'b'), Some(&1));
    }

    #[test]
    fn test_group_by_parity() {
        let grouped = group_by_parity(&[1, 2, 3, 4]);
        assert_eq!(grouped.get(&true), Some(&vec![2, 4]));   // even
        assert_eq!(grouped.get(&false), Some(&vec![1, 3])); // odd
    }

    #[test]
    fn test_partition_results() {
        let (ok, err) = partition_results(&["1", "abc", "2", "xyz"]);
        assert_eq!(ok, vec![1, 2]);
        assert_eq!(err.len(), 2);
    }
}
