// Exercise 089: Pattern Matching - @ Bindings
//
// Learning objective: Learn the @ binding pattern which allows
// binding a value to a name while also pattern matching on it.
//
// The @ binding lets you test a value against a pattern AND
// bind it to a variable at the same time. Useful when you need
// both the whole value and its parts.

enum Message {
    Hello { id: i32 },
    Goodbye,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // TODO: Use @ binding to match and capture a value
    let x = 5;
    match x {
        // Bind the matched value to 'n' while also testing it's in range
        n @ 1..=10 => println!("Got a value in range: {}", n),
        n => println!("Got some other value: {}", n),
    }

    // TODO: Use @ binding with enum destructuring
    let msg = Message::Hello { id: 5 };
    match msg {
        // Bind the whole Message while also extracting id
        m @ Message::Hello { id: 3..=7 } => {
            println!("Found message in range: {:?}", m);
        }
        Message::Hello { id } => {
            println!("Other hello with id: {}", id);
        }
        Message::Goodbye => println!("Goodbye!"),
    }

    // TODO: Use @ binding with struct patterns
    let point = Point { x: 2, y: 3 };
    match point {
        // Bind the whole point while also extracting fields
        p @ Point { x: 0, y } => {
            println!("Point on y-axis: {:?}, y = {}", p, y);
        }
        p @ Point { x, y: 0 } => {
            println!("Point on x-axis: {:?}, x = {}", p, x);
        }
        p => {
            println!("Regular point: {:?}", p);
        }
    }

    // TODO: Use @ binding with nested patterns
    let points = vec![
        Point { x: 0, y: 5 },
        Point { x: 3, y: 0 },
        Point { x: 2, y: 2 },
    ];
    
    for point in points {
        classify_point(point);
    }

    // TODO: Use @ binding in function parameters
    let pairs = vec![(1, 2), (5, 5), (3, 4)];
    
    for pair in pairs {
        describe_pair(pair);
    }

    // TODO: Complex @ binding with Option
    let values = vec![Some(5), Some(15), None, Some(25)];
    
    for val in values {
        process_value(val);
    }
}

// TODO: Complete this function that uses @ binding to classify points
fn classify_point(p @ Point { x, y }: Point) {
    // Use p (the whole point), x, and y
    if x == y {
        println!("Point {} is on diagonal y=x", format_point(&p));
    } else if x > y {
        println!("Point {} is below diagonal", format_point(&p));
    } else {
        println!("Point {} is above diagonal", format_point(&p));
    }
}

fn format_point(p: &Point) -> String {
    format!("({}, {})", p.x, p.y)
}

// TODO: Complete this function using @ binding with tuples
fn describe_pair(pair @ (x, y): (i32, i32)) {
    match pair {
        // TODO: Use @ binding to match when x == y (same values)
        // and bind the whole pair
        p @ (a, b) if a == b => {
            println!("Equal pair {:?}: both are {}", p, a);
        }
        // Match when sum is even
        p @ (a, b) if (a + b) % 2 == 0 => {
            println!("Even sum pair {:?}: sum = {}", p, a + b);
        }
        // Everything else
        _ => {
            println!("Regular pair ({}, {}): sum = {}", x, y, x + y);
        }
    }
}

// TODO: Complete this function using @ binding with Option
fn process_value(val: Option<i32>) {
    match val {
        // Bind the option while checking range
        v @ Some(n) if n < 10 => {
            println!("Small value {:?}", v);
        }
        v @ Some(n) if n >= 10 && n < 20 => {
            println!("Medium value {:?}", v);
        }
        Some(n) => {
            println!("Large value: {}", n);
        }
        None => {
            println!("No value");
        }
    }
}

// TODO: Complete this function that extracts range info using @
fn get_range_info(n: i32) -> String {
    match n {
        // Use @ binding to capture the value and classify it
        v @ 0..=10 => format!("Small ({})", v),
        v @ 11..=50 => format!("Medium ({})", v),
        v @ 51..=100 => format!("Large ({})", v),
        v => format!("Out of range ({})", v),
    }
}

// TODO: Complete this struct and function
struct Rectangle {
    width: u32,
    height: u32,
}

fn describe_rectangle(r @ Rectangle { width, height }: Rectangle) -> String {
    // TODO: Return a string describing the rectangle using @ binding
    // Format: "Rectangle WxH is square" if width == height
    // Format: "Rectangle WxH is landscape" if width > height
    // Format: "Rectangle WxH is portrait" if height > width
    // YOUR CODE HERE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_range_info() {
        assert_eq!(get_range_info(5), "Small (5)");
        assert_eq!(get_range_info(25), "Medium (25)");
        assert_eq!(get_range_info(75), "Large (75)");
        assert_eq!(get_range_info(150), "Out of range (150)");
    }

    #[test]
    fn test_describe_rectangle() {
        let square = Rectangle { width: 5, height: 5 };
        assert!(describe_rectangle(square).contains("square"));
        
        let landscape = Rectangle { width: 10, height: 5 };
        assert!(describe_rectangle(landscape).contains("landscape"));
        
        let portrait = Rectangle { width: 5, height: 10 };
        assert!(describe_rectangle(portrait).contains("portrait"));
    }
}
