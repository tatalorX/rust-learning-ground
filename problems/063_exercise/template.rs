// Exercise 063: Traits - Implementing
//
// Traits are useless without implementations. This exercise focuses on
// implementing traits for various types, including foreign types and generics.
//
// Learning Objectives:
// - Implement traits for custom types
// - Implement traits for external types
// - Implement traits for generic types
//
// Your task: Implement the provided traits for the given types.

/// A trait for objects that have an area.
trait Area {
    fn area(&self) -> f64;
}

/// A trait for objects that have a perimeter.
trait Perimeter {
    fn perimeter(&self) -> f64;
}

/// A trait for displayable objects.
trait Displayable {
    fn display(&self) -> String;
}

// Shape definitions
struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// TODO: Implement Area for Rectangle
// Area = width * height

// TODO: Implement Perimeter for Rectangle  
// Perimeter = 2 * (width + height)

// TODO: Implement Area for Circle
// Area = π * r²

// TODO: Implement Perimeter for Circle (circumference)
// Circumference = 2 * π * r

// TODO: Implement Area for Triangle using Heron's formula
// s = (a + b + c) / 2
// Area = √(s * (s-a) * (s-b) * (s-c))

// TODO: Implement Perimeter for Triangle
// Perimeter = a + b + c

// TODO: Implement Displayable for Rectangle
// Format: "Rectangle: {width} x {height}"

// TODO: Implement Displayable for Circle
// Format: "Circle: radius = {radius}"

// TODO: Implement Displayable for Triangle
// Format: "Triangle: sides = {a}, {b}, {c}"

// Generic wrapper struct
struct Measured<T> {
    value: T,
    unit: String,
}

impl<T> Measured<T> {
    fn new(value: T, unit: &str) -> Self {
        Self {
            value,
            unit: unit.to_string(),
        }
    }
}

// TODO: Implement Displayable for Measured<T> where T: Displayable
// Format: "{display} ({unit})"

fn main() {
    // After implementing the traits, uncomment and run:
    
    // let rect = Rectangle { width: 10.0, height: 5.0 };
    // let circle = Circle { radius: 3.0 };
    // let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    
    // println!("Shapes and their properties:\n");
    
    // println!("Rectangle:");
    // println!("  Display: {}", rect.display());
    // println!("  Area: {:.2}", rect.area());
    // println!("  Perimeter: {:.2}", rect.perimeter());
    
    // println!("\nCircle:");
    // println!("  Display: {}", circle.display());
    // println!("  Area: {:.2}", circle.area());
    // println!("  Perimeter (Circumference): {:.2}", circle.perimeter());
    
    // println!("\nTriangle:");
    // println!("  Display: {}", triangle.display());
    // println!("  Area: {:.2}", triangle.area());
    // println!("  Perimeter: {:.2}", triangle.perimeter());
    
    // println!("\nMeasured values:");
    // let measured_rect = Measured::new(rect, "cm");
    // println!("  {}", measured_rect.display());
    
    println!("Implement all TODOs to see the shapes in action!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert!((rect.area() - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert!((rect.perimeter() - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 3.0 };
        assert!((circle.area() - 28.274).abs() < 0.001);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle { radius: 3.0 };
        assert!((circle.perimeter() - 18.850).abs() < 0.001);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert!((triangle.area() - 6.0).abs() < 0.001);
    }

    #[test]
    fn test_triangle_perimeter() {
        let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert!((triangle.perimeter() - 12.0).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_display() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert!(rect.display().contains("Rectangle"));
        assert!(rect.display().contains("10"));
        assert!(rect.display().contains("5"));
    }

    #[test]
    fn test_circle_display() {
        let circle = Circle { radius: 3.0 };
        assert!(circle.display().contains("Circle"));
        assert!(circle.display().contains("3"));
    }
}
