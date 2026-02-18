// Exercise 037: Structs - Methods (impl block)
//
// Learning Objective: Learn how to define methods on structs using impl blocks.
// Methods are functions associated with a type and take self as first parameter.
//
// Topics covered:
// - impl blocks for methods
// - self, &self, &mut self receivers
// - Method syntax (dot notation)

fn main() {
    // TODO: Define a Rectangle struct with width and height (u32)
    // (Define below main)

    // TODO: Create a Rectangle instance
    // let rect = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // TODO: Call the area method on rect
    // println!("Area: {}", rect.area());

    // TODO: Call the perimeter method
    // println!("Perimeter: {}", rect.perimeter());

    // TODO: Call can_hold to check if rect can hold another rectangle
    // let small = Rectangle {
    //     width: 10,
    //     height: 20,
    // };
    // println!("Can rect hold small? {}", rect.can_hold(&small));

    // Mutable methods
    // TODO: Create a mutable rectangle
    // let mut square = Rectangle {
    //     width: 10,
    //     height: 10,
    // };
    // println!("Before: {}x{}", square.width, square.height);

    // TODO: Call the scale method to double the size
    // square.scale(2);
    // println!("After scaling: {}x{}", square.width, square.height);

    // Method chaining
    // TODO: Create a Square, scale it, and check its area
    // let mut sq = Rectangle::square(5);
    // sq.scale(3);
    // println!("Square area after scaling: {}", sq.area());
}

// TODO: Define Rectangle struct
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// TODO: Implement methods for Rectangle
// impl Rectangle {
//     // area takes &self (immutable borrow)
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     // perimeter takes &self
//     fn perimeter(&self) -> u32 {
//         2 * (self.width + self.height)
//     }
//
//     // can_hold takes &self and &Rectangle
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width >= other.width && self.height >= other.height
//     }
//
//     // scale takes &mut self (mutable borrow)
//     fn scale(&mut self, factor: u32) {
//         self.width *= factor;
//         self.height *= factor;
//     }
//
//     // square is an associated function (no self), see next exercise
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Define Rectangle struct
// [ ] Implement area(&self) method
// [ ] Implement perimeter(&self) method
// [ ] Implement can_hold(&self, other) method
// [ ] Implement scale(&mut self, factor) method
// [ ] Create Rectangle and call area()
// [ ] Call perimeter()
// [ ] Create small rect and test can_hold()
// [ ] Create mutable rect and call scale()
// [ ] Define square() associated function
// [ ] Verify the program compiles and runs
