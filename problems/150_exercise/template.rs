// Exercise 150: Matrix Rotation (90 degrees)
//
// Learning Objective:
// Rotate a matrix 90 degrees clockwise and counter-clockwise in-place.
// Understand matrix transformation and in-place algorithms.
//
// Key Concepts:
// - Matrix transposition
// - Row/column reversal
// - Layer-by-layer rotation
// - In-place O(1) extra space rotation

/// TODO: Rotate matrix 90 degrees clockwise IN-PLACE
/// 
/// Algorithm:
/// 1. Transpose the matrix (swap elements across diagonal)
/// 2. Reverse each row
/// 
/// Example:
/// 1 2 3     1 4 7     7 4 1
/// 4 5 6  -> 2 5 8  -> 8 5 2
/// 7 8 9     3 6 9     9 6 3
fn rotate_clockwise(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix.len() == 1 {
        return;
    }
    
    let n = matrix.len();
    
    // TODO: Transpose - swap matrix[i][j] with matrix[j][i]
    // Only need to process upper triangle (i < j)
    
    // TODO: Reverse each row
}

/// TODO: Rotate matrix 90 degrees counter-clockwise IN-PLACE
/// 
/// Algorithm:
/// 1. Transpose the matrix
/// 2. Reverse each column
fn rotate_counter_clockwise(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix.len() == 1 {
        return;
    }
    
    let n = matrix.len();
    
    // TODO: Transpose the matrix
    
    // TODO: Reverse each column
}

/// TODO: Rotate matrix 180 degrees
fn rotate_180(matrix: &mut [Vec<i32>]) {
    // TODO: Either rotate clockwise twice
    // Or reverse all rows, then reverse each row
}

/// TODO: Rotate matrix clockwise and return new matrix
/// Does not modify original, creates new matrix
fn rotate_clockwise_new(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // TODO: Create new matrix where
    // new_matrix[i][j] = matrix[n-1-j][i]
    // This is the direct formula for 90-degree clockwise rotation
    
    vec![]
}

/// TODO: Rotate matrix layer by layer (alternative approach)
/// Rotate elements in each "onion layer" of the matrix
fn rotate_layer_by_layer(matrix: &mut [Vec<i32>]) {
    // TODO: Process each layer from outside to inside
    // For each layer, rotate elements in groups of 4
    // top-left -> top-right -> bottom-right -> bottom-left -> top-left
    
}

/// TODO: Check if matrix is square
fn is_square(matrix: &[Vec<i32>]) -> bool {
    // TODO: Check if number of rows equals number of columns
    // Also check that all rows have same length
    
    false
}

/// TODO: Print matrix in a readable format
fn print_matrix(matrix: &[Vec<i32>]) {
    // TODO: Print each row with proper formatting
    
}

/// TODO: Rotate a rectangular matrix (not square)
/// Returns a new matrix since rectangular can't be rotated in-place
fn rotate_rectangle(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // TODO: For m x n matrix, result is n x m
    // result[i][j] = matrix[m-1-j][i]
    
    vec![]
}

fn main() {
    // Test 3x3 matrix
    let mut matrix1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    println!("Original matrix:");
    print_matrix(&matrix1);
    
    rotate_clockwise(&mut matrix1);
    println!("\nRotated 90° clockwise:");
    print_matrix(&matrix1);
    
    rotate_counter_clockwise(&mut matrix1);
    println!("\nRotated 90° counter-clockwise (back to original):");
    print_matrix(&matrix1);
    
    rotate_180(&mut matrix1);
    println!("\nRotated 180°:");
    print_matrix(&matrix1);
    
    // Test with larger matrix
    let mut matrix2 = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    
    println!("\n\nOriginal 4x4 matrix:");
    print_matrix(&matrix2);
    
    let rotated_new = rotate_clockwise_new(&matrix2);
    println!("\nRotated 90° clockwise (new matrix):");
    print_matrix(&rotated_new);
    
    // Layer by layer rotation
    let mut matrix3 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    rotate_layer_by_layer(&mut matrix3);
    println!("\nRotated using layer-by-layer:");
    print_matrix(&matrix3);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rotate_3x3() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_clockwise(&mut matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }
    
    #[test]
    fn test_rotate_4x4() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate_clockwise(&mut matrix);
        assert_eq!(matrix, vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ]);
    }
    
    #[test]
    fn test_rotate_counter_clockwise() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_counter_clockwise(&mut matrix);
        assert_eq!(matrix, vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7],
        ]);
    }
    
    #[test]
    fn test_rotate_180() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_180(&mut matrix);
        assert_eq!(matrix, vec![
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1],
        ]);
    }
    
    #[test]
    fn test_rotate_new_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let rotated = rotate_clockwise_new(&matrix);
        assert_eq!(rotated, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
        // Original should be unchanged
        assert_eq!(matrix[0][0], 1);
    }
    
    #[test]
    fn test_layer_by_layer() {
        let mut m1 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let mut m2 = m1.clone();
        
        rotate_clockwise(&mut m1);
        rotate_layer_by_layer(&mut m2);
        
        assert_eq!(m1, m2);
    }
    
    #[test]
    fn test_single_element() {
        let mut matrix = vec![vec![42]];
        rotate_clockwise(&mut matrix);
        assert_eq!(matrix, vec![vec![42]]);
    }
    
    #[test]
    fn test_2x2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_clockwise(&mut matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }
    
    #[test]
    fn test_is_square() {
        assert!(is_square(&vec![vec![1, 2], vec![3, 4]]));
        assert!(!is_square(&vec![vec![1, 2, 3], vec![4, 5, 6]]));
        assert!(is_square(&vec![vec![1]]));
    }
    
    #[test]
    fn test_rotate_rectangle() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let rotated = rotate_rectangle(&matrix);
        // 2x3 becomes 3x2
        assert_eq!(rotated, vec![
            vec![4, 1],
            vec![5, 2],
            vec![6, 3],
        ]);
    }
    
    #[test]
    fn test_rotate_four_times() {
        let original = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let mut matrix = original.clone();
        
        for _ in 0..4 {
            rotate_clockwise(&mut matrix);
        }
        
        assert_eq!(matrix, original);
    }
}
