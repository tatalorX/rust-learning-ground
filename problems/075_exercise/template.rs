// Exercise 075: Iterators - fold and reduce
//
// fold() and reduce() are consuming adapters that process all elements
// to produce a single value. They're used for aggregation operations.
//
// Learning Objectives:
// - Use fold() with an initial accumulator value
// - Use reduce() when there's no natural initial value
// - Implement common operations using fold/reduce
//
// Your task: Implement aggregation functions using fold and reduce.

/// Calculates the sum of all numbers using fold.
fn fold_sum(numbers: &[i32]) -> i32 {
    // TODO: Use fold with initial value 0, add each number
    todo!()
}

/// Calculates the product of all numbers using fold.
fn fold_product(numbers: &[i32]) -> i32 {
    // TODO: Use fold with initial value 1, multiply each number
    todo!()
}

/// Finds the maximum value using fold.
fn fold_max(numbers: &[i32]) -> Option<i32> {
    // TODO: Use fold with first element as initial, or None if empty
    todo!()
}

/// Concatenates all strings using fold.
fn fold_concatenate(strings: &[&str]) -> String {
    // TODO: Use fold with initial empty string, append each string
    todo!()
}

/// Counts how many elements satisfy a predicate using fold.
fn fold_count_if<F>(numbers: &[i32], predicate: F) -> usize
where
    F: Fn(i32) -> bool,
{
    // TODO: Use fold to count elements where predicate returns true
    todo!()
}

/// Reverses a vector using fold.
fn fold_reverse<T: Clone>(items: &[T]) -> Vec<T> {
    // TODO: Use fold to build a reversed vector
    // Prepend each element to accumulator
    todo!()
}

/// Finds the longest string using reduce.
fn reduce_longest(strings: &[&str]) -> Option<&str> {
    // TODO: Use reduce to find the longest string
    todo!()
}

/// Calculates the average using fold (returns 0.0 for empty).
fn fold_average(numbers: &[f64]) -> f64 {
    // TODO: Use fold to compute (sum, count), then return sum/count
    // Return 0.0 if count is 0
    todo!()
}

/// Builds a frequency map using fold.
fn fold_frequency(items: &[char]) -> std::collections::HashMap<char, usize> {
    // TODO: Use fold to build a HashMap of frequencies
    todo!()
}

/// Flattens a nested vector using fold.
fn fold_flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    // TODO: Use fold to concatenate all inner vectors
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // let numbers = vec![1, 2, 3, 4, 5];
    // println!("Numbers: {:?}", numbers);
    // println!("Sum (fold): {}", fold_sum(&numbers));
    // println!("Product (fold): {}", fold_product(&numbers));
    // println!("Max (fold): {:?}", fold_max(&numbers));
    // 
    // let words = vec!["Hello", " ", "World", "!"];
    // println!("\nWords: {:?}", words);
    // println!("Concatenated: '{}'", fold_concatenate(&words));
    // 
    // let evens_count = fold_count_if(&numbers, |n| n % 2 == 0);
    // println!("\nEven numbers count: {}", evens_count);
    // 
    // let reversed = fold_reverse(&numbers);
    // println!("Reversed: {:?}", reversed);
    // 
    // let fruits = vec!["apple", "banana", "cherry", "date"];
    // match reduce_longest(&fruits) {
    //     Some(longest) => println!("\nLongest fruit: '{}'", longest),
    //     None => println!("\nNo fruits"),
    // }
    // 
    // let floats = vec![1.0, 2.0, 3.0, 4.0];
    // println!("\nFloats: {:?}", floats);
    // println!("Average: {:.2}", fold_average(&floats));
    // 
    // let chars = vec!['a', 'b', 'a', 'c', 'b', 'a'];
    // println!("\nChars: {:?}", chars);
    // println!("Frequency: {:?}", fold_frequency(&chars));
    // 
    // let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    // println!("\nNested: {:?}", nested);
    // println!("Flattened: {:?}", fold_flatten(&nested));
    
    println!("Implement all TODOs to see fold/reduce in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_sum() {
        assert_eq!(fold_sum(&[1, 2, 3, 4]), 10);
        assert_eq!(fold_sum(&[]), 0);
    }

    #[test]
    fn test_fold_product() {
        assert_eq!(fold_product(&[1, 2, 3, 4]), 24);
        assert_eq!(fold_product(&[]), 1);
    }

    #[test]
    fn test_fold_max() {
        assert_eq!(fold_max(&[1, 5, 3]), Some(5));
        assert_eq!(fold_max(&[5]), Some(5));
        assert_eq!(fold_max(&[]), None);
    }

    #[test]
    fn test_fold_concatenate() {
        assert_eq!(fold_concatenate(&["Hello", " ", "World"]), "Hello World");
    }

    #[test]
    fn test_fold_count_if() {
        assert_eq!(fold_count_if(&[1, 2, 3, 4, 5], |n| n > 2), 3);
        assert_eq!(fold_count_if(&[1, 2, 3], |n| n % 2 == 0), 1);
    }

    #[test]
    fn test_fold_reverse() {
        assert_eq!(fold_reverse(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_reduce_longest() {
        assert_eq!(reduce_longest(&["a", "bb", "ccc"]), Some("ccc"));
        assert_eq!(reduce_longest(&[]), None);
    }

    #[test]
    fn test_fold_average() {
        assert_eq!(fold_average(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(fold_average(&[]), 0.0);
    }

    #[test]
    fn test_fold_frequency() {
        let freq = fold_frequency(&['a', 'b', 'a']);
        assert_eq!(freq.get(&'a'), Some(&2));
        assert_eq!(freq.get(&'b'), Some(&1));
    }

    #[test]
    fn test_fold_flatten() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(fold_flatten(&nested), vec![1, 2, 3, 4]);
    }
}
