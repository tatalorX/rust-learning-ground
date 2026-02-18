// Exercise 091: Advanced Traits - Operator Overloading
//
// Learning objective: Learn how to overload operators using
// traits from std::ops to make custom types behave like primitives.
//
// Operator overloading allows you to define how operators (+, -, *, etc.)
// work with your custom types by implementing standard traits.

use std::ops::{Add, Sub, Mul, Neg, Index, IndexMut};

// TODO: Define a Point struct and implement Add
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// TODO: Implement Sub for Point
impl Sub for Point {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        // YOUR CODE HERE
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// TODO: Implement Neg for Point (unary minus)
impl Neg for Point {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        // YOUR CODE HERE
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

// TODO: Implement Mul<f64> for Point (scalar multiplication)
impl Mul<f64> for Point {
    type Output = Self;
    
    fn mul(self, scalar: f64) -> Self::Output {
        // YOUR CODE HERE
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// TODO: Define a Matrix struct and implement Index
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }
    
    fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix { data, rows, cols }
    }
}

// TODO: Implement Index for Matrix (immutable access)
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

// TODO: Implement IndexMut for Matrix (mutable access)
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

// TODO: Define a Complex number struct
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
    
    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

// TODO: Implement Add for Complex
impl Add for Complex {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        // YOUR CODE HERE
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

// TODO: Implement Mul for Complex
impl Mul for Complex {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self::Output {
        // (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

fn main() {
    // TODO: Test Point operations
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);
    
    println!("Point operations:");
    println!("p1 + p2 = {:?}", p1 + p2);
    println!("p1 - p2 = {:?}", p1 - p2);
    println!("-p1 = {:?}", -p1);
    println!("p1 * 2.0 = {:?}", p1 * 2.0);

    // TODO: Test Matrix indexing
    let mut matrix = Matrix::new(3, 3);
    matrix[(0, 0)] = 1.0;
    matrix[(1, 1)] = 2.0;
    matrix[(2, 2)] = 3.0;
    
    println!("\nMatrix:");
    for row in 0..3 {
        for col in 0..3 {
            print!("{:4.1} ", matrix[(row, col)]);
        }
        println!();
    }

    // TODO: Test Complex operations
    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, 2.0);
    
    println!("\nComplex operations:");
    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);
    println!("c1 + c2 = {:?}", c1 + c2);
    println!("c1 * c2 = {:?}", c1 * c2);
    println!("|c1| = {}", c1.magnitude());

    // TODO: Create a vector and add points
    let points = vec![
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(3.0, 3.0),
    ];
    
    let sum: Point = points.iter().fold(Point::new(0.0, 0.0), |acc, &p| acc + p);
    println!("\nSum of points: {:?}", sum);
}

// TODO: Complete this function that adds two values implementing Add
fn generic_add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// TODO: Complete this function to calculate distance between points
fn distance(p1: Point, p2: Point) -> f64 {
    let diff = p1 - p2;
    (diff.x * diff.x + diff.y * diff.y).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1 + p2, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_point_sub() {
        let p1 = Point::new(5.0, 5.0);
        let p2 = Point::new(2.0, 3.0);
        assert_eq!(p1 - p2, Point::new(3.0, 2.0));
    }

    #[test]
    fn test_complex_mul() {
        let c1 = Complex::new(3.0, 2.0);
        let c2 = Complex::new(1.0, 7.0);
        let result = c1 * c2;
        // (3+2i)(1+7i) = 3 + 21i + 2i + 14i² = 3 + 23i - 14 = -11 + 23i
        assert!((result.real - (-11.0)).abs() < 0.001);
        assert!((result.imag - 23.0).abs() < 0.001);
    }

    #[test]
    fn test_matrix_index() {
        let mut m = Matrix::new(2, 2);
        m[(0, 1)] = 5.0;
        assert_eq!(m[(0, 1)], 5.0);
    }
}
