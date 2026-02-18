// Exercise 092: Advanced Traits - Supertraits
//
// Learning objective: Understand supertraits (trait bounds on traits)
// and how they create trait hierarchies.
//
// A supertrait is a trait that another trait depends on.
// If trait B: A, then anything implementing B must also implement A.
// This creates a hierarchy similar to inheritance.

// TODO: Define a base trait Shape with area() method
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// TODO: Define a Drawable trait that requires Shape
// Drawable is a subtrait of Shape
pub trait Drawable: Shape {
    fn draw(&self);
    fn describe(&self) {
        println!("Shape with area {:.2}", self.area());
    }
}

// TODO: Define a Printable trait
pub trait Printable {
    fn print(&self);
}

// TODO: Define a Document trait that requires both Drawable and Printable
pub trait Document: Drawable + Printable {
    fn render(&self) {
        self.draw();
        self.print();
    }
}

// TODO: Implement a Circle that implements Shape
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// TODO: Implement Drawable for Circle
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

// TODO: Implement Printable for Circle
impl Printable for Circle {
    fn print(&self) {
        println!("Circle(radius={})", self.radius);
    }
}

// TODO: Implement Document for Circle (it already has Drawable and Printable!)
impl Document for Circle {}

// TODO: Implement Rectangle
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

// TODO: Implement Printable for Rectangle
impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle(width={}, height={})", self.width, self.height);
    }
}

// TODO: Define a trait hierarchy for animals
pub trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

pub trait Pet: Animal {
    fn owner(&self) -> &str;
    fn pet(&self) {
        println!("You pet {} the {}", self.name(), self.name());
    }
}

pub trait Dog: Pet {
    fn fetch(&self);
    fn bark(&self) {
        println!("Woof! Woof!");
        self.make_sound();
    }
}

// TODO: Implement the animal hierarchy
struct GoldenRetriever {
    name: String,
    owner: String,
}

impl GoldenRetriever {
    fn new(name: &str, owner: &str) -> Self {
        GoldenRetriever {
            name: name.to_string(),
            owner: owner.to_string(),
        }
    }
}

impl Animal for GoldenRetriever {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Bark!");
    }
}

impl Pet for GoldenRetriever {
    fn owner(&self) -> &str {
        &self.owner
    }
}

impl Dog for GoldenRetriever {
    fn fetch(&self) {
        println!("{} runs to fetch the ball!", self.name);
    }
}

fn main() {
    // TODO: Test Circle with all trait levels
    let circle = Circle::new(5.0);
    println!("Circle area: {:.2}", circle.area());
    circle.draw();
    circle.print();
    circle.describe();
    circle.render();
    
    println!();
    
    // TODO: Test Rectangle
    let rect = Rectangle::new(10.0, 20.0);
    println!("Rectangle area: {:.2}", rect.area());
    rect.draw();
    rect.print();
    
    println!();
    
    // TODO: Test Dog hierarchy
    let dog = GoldenRetriever::new("Buddy", "Alice");
    println!("Dog name: {}", dog.name());
    println!("Dog owner: {}", dog.owner());
    dog.make_sound();
    dog.pet();
    dog.bark();
    dog.fetch();

    // TODO: Use trait bounds with supertraits
    process_drawable(&circle);
    process_shape(&rect);
}

// TODO: Complete this function that requires Drawable (which requires Shape)
fn process_drawable<T: Drawable>(item: &T) {
    // YOUR CODE HERE - print area, perimeter, and draw
    println!("Area: {:.2}", item.area());
    println!("Perimeter: {:.2}", item.perimeter());
    item.draw();
}

// TODO: Complete this generic function that works with any Shape
fn process_shape<T: Shape>(shape: &T) {
    // YOUR CODE HERE
    println!("Processing shape with area {:.2} and perimeter {:.2}",
             shape.area(), shape.perimeter());
}

// TODO: Complete this function that requires Document (which requires Drawable + Printable)
fn render_document<T: Document>(doc: &T) {
    // YOUR CODE HERE
    doc.render();
}

// TODO: Complete this function for any Dog (which requires Pet which requires Animal)
fn play_with_dog<T: Dog>(dog: &T) {
    // YOUR CODE HERE
    dog.bark();
    dog.fetch();
    dog.pet();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_shape() {
        let c = Circle::new(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_shape() {
        let r = Rectangle::new(3.0, 4.0);
        assert_eq!(r.area(), 12.0);
        assert_eq!(r.perimeter(), 14.0);
    }

    #[test]
    fn test_dog_hierarchy() {
        let dog = GoldenRetriever::new("Rex", "Bob");
        assert_eq!(dog.name(), "Rex");
        assert_eq!(dog.owner(), "Bob");
    }
}
