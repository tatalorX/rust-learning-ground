// Exercise 040: Enums - With Data
//
// Learning Objective: Learn about enums that carry data - each variant
// can have different types and amounts of associated data.
//
// Topics covered:
// - Enums with data (variant payloads)
// - Different data for each variant
// - Tuple vs struct-like variants

fn main() {
    // TODO: Define a Message enum with different data for each variant:
    // - Quit (no data)
    // - Move { x: i32, y: i32 } (named fields)
    // - Write(String) (single tuple field)
//     // - ChangeColor(u8, u8, u8) (multiple tuple fields)
    // (Define below main)

    // TODO: Create different message variants
    // let quit = Message::Quit;
    // let move_msg = Message::Move { x: 10, y: 20 };
    // let write = Message::Write(String::from("Hello"));
    // let color = Message::ChangeColor(255, 0, 0);

    // TODO: Print the messages (using debug format)
    // println!("{:?}", quit);
    // println!("{:?}", move_msg);
    // println!("{:?}", write);
    // println!("{:?}", color);

    // TODO: Call process_message for each
    // process_message(quit);
    // process_message(move_msg);
    // process_message(write);
    // process_message(color);

    // Enums with data for shapes
    // TODO: Define a Shape enum:
    // - Circle { radius: f64 }
    // - Rectangle { width: f64, height: f64 }
    // - Triangle { base: f64, height: f64 }

    // TODO: Create shapes and calculate areas
    // let circle = Shape::Circle { radius: 5.0 };
    // let rect = Shape::Rectangle { width: 10.0, height: 20.0 };
    // let tri = Shape::Triangle { base: 8.0, height: 4.0 };

    // TODO: Calculate and print areas
    // println!("Circle area: {:.2}", area(&circle));
    // println!("Rectangle area: {:.2}", area(&rect));
    // println!("Triangle area: {:.2}", area(&tri));

    // IP Address example (like in Rust book)
    // TODO: Define IpAddr enum with variants for different IP types
    // - V4 with four u8 octets
    // - V6 with a String

    // TODO: Create IP addresses
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // TODO: Print them
    // println!("Home: {:?}", home);
    // println!("Loopback: {:?}", loopback);
}

// TODO: Define Message enum
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(u8, u8, u8),
// }

// TODO: Define Shape enum
// enum Shape {
//     Circle { radius: f64 },
//     Rectangle { width: f64, height: f64 },
//     Triangle { base: f64, height: f64 },
// }

// TODO: Define IpAddr enum
// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// TODO: Implement process_message function
// fn process_message(msg: Message) {
//     match msg {
//         Message::Quit => println!("Quitting..."),
//         Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
//         Message::Write(text) => println!("Writing: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
//     }
// }

// TODO: Implement area function for shapes
// fn area(shape: &Shape) -> f64 {
//     match shape {
//         Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
//         Shape::Rectangle { width, height } => width * height,
//         Shape::Triangle { base, height } => 0.5 * base * height,
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Define Message enum with 4 variants (different data types)
// [ ] Create instances of each Message variant
// [ ] Print all messages
// [ ] Implement process_message function
// [ ] Call process_message for each variant
// [ ] Define Shape enum
// [ ] Implement area function for shapes
// [ ] Create shapes and print their areas
// [ ] Define IpAddr enum
// [ ] Create and print IP addresses
// [ ] Verify the program compiles and runs
