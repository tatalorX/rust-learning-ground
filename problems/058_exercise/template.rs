// Exercise 058: Error Propagation
//
// Error propagation is about passing errors up the call stack using the ? operator.
// This allows errors to be handled at the appropriate level of abstraction.
//
// Learning Objectives:
// - Propagate errors using the ? operator
// - Convert error types using From trait and map_err
// - Build higher-level operations that handle errors appropriately
//
// Your task: Implement a file processing system with proper error propagation.

use std::collections::HashMap;

/// A custom error type for file operations.
#[derive(Debug, PartialEq)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    InvalidFormat(String),
    EmptyFile,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::NotFound(s) => write!(f, "File not found: {}", s),
            FileError::PermissionDenied(s) => write!(f, "Permission denied: {}", s),
            FileError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            FileError::EmptyFile => write!(f, "File is empty"),
        }
    }
}

/// Simulates reading a file from disk.
fn read_file(filename: &str) -> Result<String, FileError> {
    // Simulated file database
    let files: HashMap<&str, &str> = [
        ("data.txt", "10\n20\n30"),
        ("empty.txt", ""),
        ("bad.txt", "10\nabc\n30"),
    ]
    .into_iter()
    .collect();
    
    match filename {
        "secret.txt" => Err(FileError::PermissionDenied(filename.to_string())),
        name => files
            .get(name)
            .map(|&content| content.to_string())
            .ok_or_else(|| FileError::NotFound(filename.to_string())),
    }
}

/// Parses a line as a number, converting errors to FileError.
fn parse_line(line: &str, line_num: usize) -> Result<i32, FileError> {
    // TODO: Parse line as i32
    // Return FileError::InvalidFormat with descriptive message if it fails
    todo!()
}

/// Reads and sums all numbers in a file.
/// Propagates errors using ? operator.
fn sum_file_numbers(filename: &str) -> Result<i32, FileError> {
    // TODO: Read the file using read_file and ?
    // TODO: Check if content is empty, return FileError::EmptyFile if so
    // TODO: Split by lines, parse each line using parse_line and ?
    // TODO: Return the sum of all numbers
    todo!()
}

/// Processes multiple files, collecting results.
fn process_files(filenames: &[&str]) -> Vec<Result<i32, FileError>> {
    // TODO: Iterate over filenames, call sum_file_number for each
    // Collect results into a vector
    filenames.iter().map(|&f| sum_file_numbers(f)).collect()
}

/// Attempts to sum all valid files, reporting errors for invalid ones.
fn sum_all_valid_files(filenames: &[&str]) -> (i32, Vec<String>) {
    // TODO: Process all files
    // Return tuple of (total_sum, error_messages)
    let results = process_files(filenames);
    let mut total = 0;
    let mut errors = Vec::new();
    
    for result in results {
        match result {
            Ok(sum) => total += sum,
            Err(e) => errors.push(e.to_string()),
        }
    }
    
    (total, errors)
}

fn main() {
    let files = vec!["data.txt", "empty.txt", "bad.txt", "secret.txt", "missing.txt"];
    
    println!("Processing files individually:\n");
    for filename in &files {
        match sum_file_numbers(filename) {
            Ok(sum) => println!("✓ {}: sum = {}", filename, sum),
            Err(e) => println!("✗ {}: {}", filename, e),
        }
    }
    
    println!("\n---\n");
    
    let (total, errors) = sum_all_valid_files(&files);
    println!("Batch processing:");
    println!("  Total sum: {}", total);
    println!("  Errors ({}):", errors.len());
    for error in errors {
        println!("    - {}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        assert!(read_file("data.txt").is_ok());
    }

    #[test]
    fn test_read_file_not_found() {
        assert!(matches!(read_file("nonexistent.txt"), Err(FileError::NotFound(_))));
    }

    #[test]
    fn test_read_file_permission_denied() {
        assert!(matches!(read_file("secret.txt"), Err(FileError::PermissionDenied(_))));
    }

    #[test]
    fn test_parse_line_valid() {
        assert_eq!(parse_line("42", 1), Ok(42));
    }

    #[test]
    fn test_parse_line_invalid() {
        assert!(matches!(parse_line("abc", 3), Err(FileError::InvalidFormat(_))));
    }

    #[test]
    fn test_sum_file_numbers_success() {
        assert_eq!(sum_file_numbers("data.txt"), Ok(60)); // 10 + 20 + 30
    }

    #[test]
    fn test_sum_file_numbers_empty() {
        assert_eq!(sum_file_numbers("empty.txt"), Err(FileError::EmptyFile));
    }

    #[test]
    fn test_sum_file_numbers_invalid_format() {
        assert!(matches!(sum_file_numbers("bad.txt"), Err(FileError::InvalidFormat(_))));
    }

    #[test]
    fn test_sum_all_valid_files() {
        let files = vec!["data.txt", "empty.txt"];
        let (total, errors) = sum_all_valid_files(&files);
        assert_eq!(total, 60);
        assert_eq!(errors.len(), 1);
    }
}
