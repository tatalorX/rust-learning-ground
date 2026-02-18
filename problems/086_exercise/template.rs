// Exercise 086: Pattern Matching - Destructuring Structs
//
// Learning objective: Master destructuring structs in pattern matching
// to extract fields concisely and elegantly.
//
// Pattern matching can decompose structs, allowing you to bind
// individual fields to variables directly in the pattern.

struct Point {
    x: f64,
    y: f64,
}

struct User {
    name: String,
    email: String,
    age: u32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let point = Point { x: 10.0, y: 20.0 };
    
    // TODO: Destructure the point into x and y variables
    let Point { x, y } = // YOUR CODE HERE
    println!("Point: x = {}, y = {}", x, y);

    // TODO: Destructure with renaming (bind x to x_coord)
    let Point { x: x_coord, y: y_coord } = point;
    println!("Coordinates: ({}, {})", x_coord, y_coord);

    // TODO: Destructure only the fields you need
    let Point { x, .. } = point; // This gets only x
    println!("Just x: {}", x);

    // TODO: Destructure a User struct
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        age: 30,
    };
    
    // Extract all fields
    let User { name, email, age } = // YOUR CODE HERE
    println!("User: {} ({}), email: {}", name, age, email);

    // TODO: Destructure nested structs
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    
    // Destructure all the way down
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    
    println!("Rectangle: ({}, {}) to ({}, {})", x1, y1, x2, y2);

    // TODO: Use destructuring in function parameters
    describe_point(point);
    
    // TODO: Destructure in match patterns
    classify_point(point);
}

// TODO: Complete this function that destructures Point in the parameter
fn describe_point(Point { x, y }: Point) {
    // YOUR CODE HERE - print a description of the point
    println!("The point is at coordinates ({}, {})", x, y);
}

// TODO: Complete this function that uses pattern matching to classify points
fn classify_point(point: Point) {
    match point {
        // Match when x is 0 (on y-axis)
        Point { x: 0.0, y } => println!("On y-axis at y = {}", y),
        // Match when y is 0 (on x-axis)
        Point { x, y: 0.0 } => println!("On x-axis at x = {}", x),
        // Match origin
        Point { x: 0.0, y: 0.0 } => println!("At origin!"),
        // Match any other point
        Point { x, y } => println!("Regular point at ({}, {})", x, y),
    }
}

// TODO: Complete this function that calculates area using destructuring
fn rectangle_area(Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } }: Rectangle) -> f64 {
    // YOUR CODE HERE - calculate width * height
    ((x2 - x1) * (y1 - y2)).abs()
}

// TODO: Complete this function that extracts name from User
fn get_user_name(user: User) -> String {
    // YOUR CODE HERE - use destructuring to get just the name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            top_left: Point { x: 0.0, y: 10.0 },
            bottom_right: Point { x: 5.0, y: 0.0 },
        };
        assert_eq!(rectangle_area(rect), 50.0);
    }

    #[test]
    fn test_get_user_name() {
        let user = User {
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
            age: 25,
        };
        assert_eq!(get_user_name(user), "Bob");
    }
}
