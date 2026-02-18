// Exercise 087: Pattern Matching - Destructuring Enums
//
// Learning objective: Master destructuring enums with data,
// including nested enums and structs within variants.
//
// Enums with data can be destructured to extract the values
// inside each variant, making it easy to work with complex data.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

enum Status {
    Pending,
    Processing { progress: u8 },
    Completed { result: String },
    Failed { error: String, code: u32 },
}

fn main() {
    // TODO: Destructure a Message::Move
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        }
        _ => (),
    }

    // TODO: Destructure a Message::Write and extract the String
    let msg = Message::Write(String::from("Hello"));
    match msg {
        // YOUR CODE HERE
        Message::Write(text) => println!("Message: {}", text),
        _ => (),
    }

    // TODO: Destructure a Message::ChangeColor (tuple variant)
    let msg = Message::ChangeColor(255, 0, 128);
    match msg {
        // YOUR CODE HERE - extract r, g, b
        Message::ChangeColor(r, g, b) => {
            println!("RGB({}, {}, {})", r, g, b);
        }
        _ => (),
    }

    // TODO: Destructure Message::Quit (unit variant)
    let msg = Message::Quit;
    match msg {
        Message::Quit => println!("Quit message received"),
        _ => (),
    }

    // TODO: Match on different shapes and calculate area
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 20.0 },
        Shape::Triangle { base: 10.0, height: 5.0 },
    ];
    
    for shape in shapes {
        let area = match shape {
            // YOUR CODE HERE - calculate area for each shape
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        };
        println!("Area: {:.2}", area);
    }

    // TODO: Match on Status with nested destructuring
    let statuses = vec![
        Status::Pending,
        Status::Processing { progress: 45 },
        Status::Completed { result: String::from("Success") },
        Status::Failed { error: String::from("Network error"), code: 500 },
    ];
    
    for status in statuses {
        describe_status(status);
    }
}

// TODO: Complete this function that describes each status
fn describe_status(status: Status) {
    match status {
        Status::Pending => println!("Waiting to start..."),
        // YOUR CODE HERE - handle Processing, Completed, and Failed
        Status::Processing { progress } => {
            println!("Processing... {}% complete", progress);
        }
        Status::Completed { result } => {
            println!("Completed with result: {}", result);
        }
        Status::Failed { error, code } => {
            println!("Failed with error {} (code {})", error, code);
        }
    }
}

// TODO: Complete this function that extracts data from Message
fn extract_message_text(msg: Message) -> Option<String> {
    match msg {
        // YOUR CODE HERE - return Some(text) for Write, None otherwise
        Message::Write(text) => Some(text),
        _ => None,
    }
}

// TODO: Complete this function that processes different shape types
fn shape_perimeter(shape: Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
        // YOUR CODE HERE - handle Rectangle and Triangle
        Shape::Rectangle { width, height } => 2.0 * (width + height),
        Shape::Triangle { base, height } => {
            // Simplified - assumes right triangle
            let hypotenuse = (base * base + height * height).sqrt();
            base + height + hypotenuse
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_message_text() {
        let msg = Message::Write(String::from("test"));
        assert_eq!(extract_message_text(msg), Some(String::from("test")));
        
        let quit = Message::Quit;
        assert_eq!(extract_message_text(quit), None);
    }

    #[test]
    fn test_shape_perimeter() {
        let circle = Shape::Circle { radius: 1.0 };
        let perimeter = shape_perimeter(circle);
        assert!(perimeter > 6.28 && perimeter < 6.29);
        
        let rect = Shape::Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(shape_perimeter(rect), 14.0);
    }
}
