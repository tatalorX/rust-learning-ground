// Exercise 028: Ownership - Copy Trait Types
//
// Learning Objective: Understand which types implement the Copy trait and
// are copied instead of moved. Simple scalar types are Copy.
//
// Topics covered:
// - Types that implement Copy: integers, floats, bool, char, tuples of Copy types
// - Stack-only data can be Copy
// - Types with heap data (like String, Vec) cannot be Copy

fn main() {
    // Integers are Copy - they are copied, not moved
    let x = 42;
    let y = x; // x is COPIED to y, not moved
    
    // TODO: Print both x and y to show both are valid
    // println!("x = {}, y = {}", ___, ___);

    // Floats are also Copy
    let a = 3.14;
    let b = a;
    
    // TODO: Print both a and b
    // println!("a = {}, b = {}", ___, ___);

    // Booleans are Copy
    let flag = true;
    let flag_copy = flag;
    
    // TODO: Print both
    // println!("flag = {}, flag_copy = {}", ___, ___);

    // Tuples of Copy types are also Copy
    let point = (10, 20);
    let point_copy = point;
    
    // TODO: Print both tuples
    // println!("point = {:?}, point_copy = {:?}", ___, ___);

    // But tuples containing non-Copy types are NOT Copy
    let data = (String::from("hello"), 42);
    // let data_copy = data; // This would MOVE data, not copy!
    
    // To copy a tuple with String, we need to clone the String
    // TODO: Create a copy of data by cloning the String part
    // let data_clone = (data.0.clone(), data.1);
    
    // TODO: Print data and data_clone
    // println!("data = {:?}", data);
    // println!("data_clone = {:?}", data_clone);

    // Demonstrate: passing Copy types to functions
    let num = 100;
    
    // TODO: Call use_number twice with the same variable (works because i32 is Copy!)
    // use_number(___);
    // use_number(___);
    
    // TODO: Print num to show it's still valid
    // println!("num is still: {}", num);
}

fn use_number(n: i32) {
    println!("Using number: {}", n);
}

// COMPLETION CHECKLIST:
// [ ] Print x and y (integers)
// [ ] Print a and b (floats)
// [ ] Print flag and flag_copy (booleans)
// [ ] Print point and point_copy (tuples of Copy types)
// [ ] Create data_clone with explicit String clone
// [ ] Print data and data_clone
// [ ] Call use_number twice with the same num variable
// [ ] Print num after the calls
