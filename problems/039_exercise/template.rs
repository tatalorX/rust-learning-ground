// Exercise 039: Enums - Basic Definition
//
// Learning Objective: Learn how to define and use enums to represent
// a value that can be one of several variants.
//
// Topics covered:
// - Defining enums with the enum keyword
// - Enum variants
// - Using enums to represent state or choice

fn main() {
    // TODO: Define a Direction enum with variants: North, South, East, West
    // (Define below main)

    // TODO: Create enum values
    // let heading = Direction::North;
    // let opposite = Direction::South;

    // TODO: Use the describe_direction function
    // describe_direction(heading);
    // describe_direction(opposite);
    // describe_direction(Direction::East);
    // describe_direction(Direction::West);

    // Enums in function returns
    // TODO: Create a function that returns the opposite direction
    // let d = Direction::North;
    // let opp = opposite_direction(d);
    // println!("Opposite of {:?} is {:?}", d, opp);

    // Enums for game states
    // TODO: Define a GameState enum: Menu, Playing, Paused, GameOver
    // let state = GameState::Menu;
    // println!("Current game state: {:?}", state);

    // Change state
    // let state = GameState::Playing;
    // println!("Now playing: {:?}", state);

    // Enums for message types
    // TODO: Define MessageType: Text, Image, Video, Audio
    // let msg_type = MessageType::Text;
    // println!("Message type: {:?}", msg_type);

    // Enums can derive traits like Debug, Clone, Copy, PartialEq
    // TODO: Compare enum values
    // let a = Direction::North;
    // let b = Direction::North;
    // let c = Direction::South;
    // println!("a == b: {}", a == b);  // true
    // println!("a == c: {}", a == c);  // false
}

// TODO: Define Direction enum with Debug and PartialEq derives
// #[derive(Debug, PartialEq)]
// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// TODO: Define GameState enum
// #[derive(Debug)]
// enum GameState {
//     Menu,
//     Playing,
//     Paused,
//     GameOver,
// }

// TODO: Define MessageType enum
// #[derive(Debug)]
// enum MessageType {
//     Text,
//     Image,
//     Video,
//     Audio,
// }

// TODO: Implement describe_direction function
// fn describe_direction(dir: Direction) {
//     match dir {
//         Direction::North => println!("Going up!"),
//         Direction::South => println!("Going down!"),
//         Direction::East => println!("Going right!"),
//         Direction::West => println!("Going left!"),
//     }
// }

// TODO: Implement opposite_direction function
// fn opposite_direction(dir: Direction) -> Direction {
//     match dir {
//         Direction::North => Direction::South,
//         Direction::South => Direction::North,
//         Direction::East => Direction::West,
//         Direction::West => Direction::East,
//     }
// }

// COMPLETION CHECKLIST:
// [ ] Define Direction enum with 4 variants
// [ ] Create Direction values
// [ ] Implement describe_direction function
// [ ] Call describe_direction for all variants
// [ ] Define GameState enum
// [ ] Create and print GameState values
// [ ] Define MessageType enum
// [ ] Create and print MessageType
// [ ] Implement opposite_direction function
// [ ] Add derive macros (Debug, PartialEq)
// [ ] Compare enum values
// [ ] Verify the program compiles and runs
