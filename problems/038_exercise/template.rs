// Exercise 038: Structs - Associated Functions
//
// Learning Objective: Learn about associated functions (often called static
// methods in other languages) - functions in impl blocks that don't take self.
//
// Topics covered:
// - Associated functions vs methods
// - Using :: syntax to call associated functions
// - Constructor patterns
// - Namespaces with multiple impl blocks

fn main() {
    // TODO: Define a Point struct with x and y (f64)
    // (Define below main)

    // Using associated functions as constructors
    // TODO: Create origin using Point::origin()
    // let origin = Point::origin();
    // println!("Origin: ({}, {})", origin.x, origin.y);

    // TODO: Create a point using Point::new()
    // let p1 = Point::new(3.0, 4.0);
    // println!("p1: ({}, {})", p1.x, p1.y);

    // TODO: Create a point on the x-axis using Point::on_x()
    // let p2 = Point::on_x(5.0);
    // println!("p2 on x-axis: ({}, {})", p2.x, p2.y);

    // Using associated functions for utility functions
    // TODO: Calculate distance between two points using Point::distance()
    // let a = Point::new(0.0, 0.0);
    // let b = Point::new(3.0, 4.0);
    // let dist = Point::distance(&a, &b);
    // println!("Distance between a and b: {}", dist);

    // Multiple impl blocks are allowed!
    // TODO: Add a second impl block for conversion methods

    // Associated constants
    // TODO: Define associated constants for special points
    // println!("ORIGIN: ({}, {})", Point::ORIGIN.x, Point::ORIGIN.y);

    // Builder pattern example
    // TODO: Create a Config using a builder pattern
    // let config = Config::new()
    //     .with_host("localhost")
    //     .with_port(8080)
    //     .with_debug(true);
    // println!("Config: {}:{}, debug={}", config.host, config.port, config.debug);
}

// TODO: Define Point struct
// struct Point {
//     x: f64,
//     y: f64,
// }

// TODO: Implement first impl block with associated functions
// impl Point {
//     // Constructor for origin (0, 0)
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }
//
//     // General constructor
//     fn new(x: f64, y: f64) -> Point {
//         Point { x, y }
//     }
//
//     // Constructor for points on x-axis
//     fn on_x(x: f64) -> Point {
//         Point { x, y: 0.0 }
//     }
//
//     // Utility function (distance between two points)
//     fn distance(a: &Point, b: &Point) -> f64 {
//         let dx = b.x - a.x;
//         let dy = b.y - a.y;
//         (dx * dx + dy * dy).sqrt()
//     }
// }

// TODO: Second impl block is allowed!
// impl Point {
//     // Convert to tuple
//     fn to_tuple(&self) -> (f64, f64) {
//         (self.x, self.y)
//     }
// }

// TODO: Define Config struct for builder pattern
// struct Config {
//     host: String,
//     port: u16,
//     debug: bool,
// }

// TODO: Implement Config with builder pattern
// impl Config {
//     fn new() -> Config {
//         Config {
//             host: String::from("localhost"),
//             port: 80,
//             debug: false,
//         }
//     }
//
//     fn with_host(mut self, host: &str) -> Config {
//         self.host = host.to_string();
//         self
//     }
//
//     fn with_port(mut self, port: u16) -> Config {
//         self.port = port;
//         self
//     }
//
//     fn with_debug(mut self, debug: bool) -> Config {
//         self.debug = debug;
//         self
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Define Point struct
// [ ] Implement Point::origin()
// [ ] Implement Point::new(x, y)
// [ ] Implement Point::on_x(x)
// [ ] Implement Point::distance(&a, &b)
// [ ] Create points and test all associated functions
// [ ] Add second impl block with to_tuple()
// [ ] Define Config struct
// [ ] Implement Config with builder pattern
// [ ] Test the builder pattern
// [ ] Verify the program compiles and runs
