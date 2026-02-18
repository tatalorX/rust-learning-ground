// Exercise 073: Iterators - map and filter
//
// Iterator adapters transform iterators into new iterators.
// map() transforms each element, filter() keeps only matching elements.
//
// Learning Objectives:
// - Use map() to transform iterator elements
// - Use filter() to select elements conditionally
// - Chain multiple iterator adapters
//
// Your task: Implement functions using map and filter.

/// Doubles all numbers in a vector.
fn double_all(numbers: &[i32]) -> Vec<i32> {
    // TODO: Use iter().map() to double each number
    todo!()
}

/// Returns only the even numbers from a vector.
fn get_evens(numbers: &[i32]) -> Vec<i32> {
    // TODO: Use iter().filter() to keep only even numbers
    todo!()
}

/// Returns the lengths of all strings in a vector.
fn string_lengths(strings: &[&str]) -> Vec<usize> {
    // TODO: Use map to get the length of each string
    todo!()
}

/// Returns only strings longer than a given length.
fn long_strings(strings: &[&str], min_length: usize) -> Vec<&str> {
    // TODO: Use filter to keep strings with length > min_length
    todo!()
}

/// Squares all odd numbers and returns them.
fn square_odds(numbers: &[i32]) -> Vec<i32> {
    // TODO: Chain filter and map: filter odd numbers, then square them
    todo!()
}

/// Converts a list of optional numbers to their values, filtering out None.
fn filter_map_optionals(optionals: &[Option<i32>]) -> Vec<i32> {
    // TODO: Use filter_map to extract values from Some variants
    todo!()
}

/// Returns the first letter of each name, uppercased.
fn first_letters(names: &[&str]) -> Vec<char> {
    // TODO: Use filter_map to get first char and uppercase it
    todo!()
}

/// Doubles positive numbers, filters out non-positive.
fn double_positives(numbers: &[i32]) -> Vec<i32> {
    // TODO: Filter positive numbers, then map to double them
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // let numbers = vec![1, 2, 3, 4, 5, 6];
    // println!("Original: {:?}", numbers);
    // println!("Doubled: {:?}", double_all(&numbers));
    // println!("Evens: {:?}", get_evens(&numbers));
    // println!("Square odds: {:?}", square_odds(&numbers));
    // 
    // let words = vec!["hello", "hi", "world", "x", "rust"];
    // println!("\nWords: {:?}", words);
    // println!("Lengths: {:?}", string_lengths(&words));
    // println!("Long strings (>2): {:?}", long_strings(&words, 2));
    // 
    // let maybe_numbers = vec![Some(1), None, Some(2), None, Some(3)];
    // println!("\nOptionals: {:?}", maybe_numbers);
    // println!("Filtered: {:?}", filter_map_optionals(&maybe_numbers));
    // 
    // let names = vec!["alice", "bob", "charlie"];
    // println!("\nNames: {:?}", names);
    // println!("First letters: {:?}", first_letters(&names));
    // 
    // let mixed = vec![-2, -1, 0, 1, 2, 3];
    // println!("\nMixed numbers: {:?}", mixed);
    // println!("Double positives: {:?}", double_positives(&mixed));
    
    println!("Implement all TODOs to see iterators in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_all(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_get_evens() {
        assert_eq!(get_evens(&[1, 2, 3, 4, 5]), vec![2, 4]);
        assert_eq!(get_evens(&[1, 3, 5]), Vec::<i32>::new());
    }

    #[test]
    fn test_string_lengths() {
        assert_eq!(string_lengths(&["hi", "hello", ""]), vec![2, 5, 0]);
    }

    #[test]
    fn test_long_strings() {
        assert_eq!(long_strings(&["a", "bb", "ccc", "dddd"], 2), vec!["ccc", "dddd"]);
    }

    #[test]
    fn test_square_odds() {
        assert_eq!(square_odds(&[1, 2, 3, 4, 5]), vec![1, 9, 25]);
    }

    #[test]
    fn test_filter_map_optionals() {
        let input = vec![Some(1), None, Some(2), None, Some(3)];
        assert_eq!(filter_map_optionals(&input), vec![1, 2, 3]);
    }

    #[test]
    fn test_first_letters() {
        assert_eq!(first_letters(&["alice", "bob", "charlie"]), vec!['A', 'B', 'C']);
    }

    #[test]
    fn test_double_positives() {
        assert_eq!(double_positives(&[-1, 0, 1, 2, 3]), vec![2, 4, 6]);
    }
}
