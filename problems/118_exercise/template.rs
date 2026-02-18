// Exercise 118: Tower of Hanoi
// ============================
//
// Learning Objective:
// Learn the Tower of Hanoi puzzle and its recursive solution.
// This classic problem demonstrates the power of recursive thinking.
//
// Rules:
// 1. Only one disk can be moved at a time
// 2. A disk can only be placed on top of a larger disk or empty rod
// 3. All disks start on the source rod, goal is to move to destination rod

fn main() {
    println!("=== Tower of Hanoi ===\n");
    
    // Solve for different numbers of disks
    for num_disks in 1..=4 {
        println!("Solving for {} disk(s):", num_disks);
        
        let moves = solve_hanoi(num_disks, 'A', 'C', 'B');
        
        println!("Total moves: {}", moves);
        println!("Minimum required: {}\n", (1u64 << num_disks) - 1);
        
        assert_eq!(moves as u64, (1u64 << num_disks) - 1);
    }
    
    println!("✓ Tower of Hanoi completed successfully!");
}

/// Tower of Hanoi Solver
///
/// Algorithm:
/// To move n disks from source to destination:
/// 1. Move n-1 disks from source to auxiliary (using destination as helper)
/// 2. Move the nth (largest) disk from source to destination
/// 3. Move n-1 disks from auxiliary to destination (using source as helper)
///
/// Base case: If n = 1, just move the disk directly
///
/// Parameters:
/// - n: number of disks
/// - source: source rod (e.g., 'A')
/// - destination: destination rod (e.g., 'C')
/// - auxiliary: auxiliary rod (e.g., 'B')
///
/// Returns: total number of moves made
fn solve_hanoi(n: u32, source: char, destination: char, auxiliary: char) -> u32 {
    // TODO: Handle base case
    // If there's only 1 disk, just move it directly
    if n == 1 {
        println!("  Move disk 1 from {} to {}", source, destination);
        return 1;
    }
    
    let mut moves = 0;
    
    // TODO: Step 1: Move n-1 disks from source to auxiliary
    // Use destination as the auxiliary rod
    moves += solve_hanoi(n - 1, source, auxiliary, destination);
    
    // TODO: Step 2: Move the nth disk from source to destination
    println!("  Move disk {} from {} to {}", n, source, destination);
    moves += 1;
    
    // TODO: Step 3: Move n-1 disks from auxiliary to destination
    // Use source as the auxiliary rod
    moves += solve_hanoi(n - 1, auxiliary, destination, source);
    
    moves
}

/// Non-recursive move counter (Bonus)
///
/// The minimum number of moves for n disks is always 2^n - 1
/// This can be computed directly without recursion.
fn min_moves(n: u32) -> u64 {
    // TODO: Bonus - calculate minimum moves mathematically
    (1u64 << n) - 1  // 2^n - 1
}

/// Iterative solution using bit manipulation (Bonus)
///
/// For a given move number m (0-indexed):
/// - Disk number = position of least significant 1-bit in (m + 1)
/// - Direction depends on parity of n
fn hanoi_iterative(n: u32) {
    // TODO: Bonus - implement iterative solution
    let total_moves = (1u64 << n) - 1;
    
    for m in 1..=total_moves {
        // Find which disk to move
        let disk = (m & (m + 1)).trailing_zeros() + 1;
        println!("Move {}", disk);
    }
}

/// Print state representation (Bonus)
///
/// Displays the current state of all three rods.
fn print_state(rod_a: &[u32], rod_b: &[u32], rod_c: &[u32]) {
    // TODO: Bonus - implement state visualization
    println!("A: {:?}", rod_a);
    println!("B: {:?}", rod_b);
    println!("C: {:?}", rod_c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves() {
        assert_eq!(min_moves(1), 1);      // 2^1 - 1 = 1
        assert_eq!(min_moves(2), 3);      // 2^2 - 1 = 3
        assert_eq!(min_moves(3), 7);      // 2^3 - 1 = 7
        assert_eq!(min_moves(4), 15);     // 2^4 - 1 = 15
        assert_eq!(min_moves(64), u64::MAX); // 2^64 - 1
    }

    #[test]
    fn test_hanoi_1_disk() {
        // This would print, so we just verify it doesn't panic
        // In a real test, you might capture stdout
        let moves = solve_hanoi(1, 'A', 'C', 'B');
        assert_eq!(moves, 1);
    }

    #[test]
    fn test_hanoi_3_disks() {
        let moves = solve_hanoi(3, 'A', 'C', 'B');
        assert_eq!(moves, 7);
    }
}
