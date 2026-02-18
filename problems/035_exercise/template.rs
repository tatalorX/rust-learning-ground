// Exercise 035: Structs - Tuple Structs
//
// Learning Objective: Learn about tuple structs - named tuples that have
// the meaning of a struct but without named fields.
//
// Topics covered:
// - Defining tuple structs
// - Accessing fields by index (0, 1, 2, etc.)
// - Destructuring tuple structs
// - When to use tuple structs vs regular structs

fn main() {
    // TODO: Define a Color tuple struct with three u8 values (R, G, B)
    // struct Color(u8, u8, u8);

    // TODO: Define a Point tuple struct with three f64 values (x, y, z)
    // struct Point(f64, f64, f64);

    // TODO: Create a red color (255, 0, 0)
    // let red = Color(255, 0, 0);

    // TODO: Create a point at origin (0.0, 0.0, 0.0)
    // let origin = Point(0.0, 0.0, 0.0);

    // Access tuple struct fields by index
    // TODO: Print red's RGB values
    // println!("Red: R={}, G={}, B={}", red.0, red.1, red.2);

    // TODO: Print origin's coordinates
    // println!("Origin: x={}, y={}, z={}", origin.0, origin.1, origin.2);

    // Destructuring tuple structs
    // TODO: Destructure red into r, g, b variables
    // let Color(r, g, b) = red;
    // println!("Destructured: R={}, G={}, B={}", r, g, b);

    // Tuple structs with different types are distinct!
    // TODO: Define a Meters and Kilometers tuple struct
    // struct Meters(f64);
    // struct Kilometers(f64);

    // TODO: Create instances
    // let distance_m = Meters(1000.0);
    // let distance_km = Kilometers(1.0);

    // TODO: Print both
    // println!("Distance in meters: {}", distance_m.0);
    // println!("Distance in km: {}", distance_km.0);

    // These are different types - can't mix them!
    // let sum = distance_m.0 + distance_km.0; // This works (both f64 inside)
    // But you can't do: let x: Meters = distance_km; // Type mismatch!

    // Tuple structs are useful for wrapper types
    // TODO: Define a Wrapper around String
    // struct Wrapper(String);

    // TODO: Create a wrapped string
    // let wrapped = Wrapper(String::from("Hello"));
    // println!("Wrapped: {}", wrapped.0);

    // TODO: Destructure to get the inner String
    // let Wrapper(inner) = wrapped;
    // println!("Inner: {}", inner);
}

// Define tuple structs here:
// struct Color(u8, u8, u8);
// struct Point(f64, f64, f64);
// struct Meters(f64);
// struct Kilometers(f64);
// struct Wrapper(String);

// COMPLETION CHECKLIST:
// [ ] Define Color tuple struct
// [ ] Define Point tuple struct
// [ ] Create red Color instance
// [ ] Create origin Point instance
// [ ] Print values using index notation
// [ ] Destructure red into r, g, b
// [ ] Define Meters and Kilometers
// [ ] Create instances and print them
// [ ] Define Wrapper and demonstrate usage
// [ ] Verify the program compiles and runs
