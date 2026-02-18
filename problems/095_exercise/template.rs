// Exercise 095: Modules - Visibility Modifiers
//
// Learning objective: Understand Rust's visibility modifiers:
// pub, pub(crate), pub(super), and private (default).
//
// Visibility controls which items can be accessed from where.
// Default is private to the module and its children.

// This is the root module (crate root)

// TODO: Create a private function (default visibility)
fn private_function() {
    println!("I'm private to this module");
}

// TODO: Create a public function
pub fn public_function() {
    println!("I'm public");
    private_function(); // Can call private functions from same module
}

// TODO: Create a module with various visibility levels
mod outer {
    // Private to outer module
    fn outer_private() {
        println!("outer_private");
    }
    
    // Public within outer
    pub fn outer_public() {
        println!("outer_public");
        outer_private(); // Can access private items
    }
    
    // TODO: Create a nested module
    pub mod inner {
        // Private to inner
        fn inner_private() {
            println!("inner_private");
        }
        
        // Public
        pub fn inner_public() {
            println!("inner_public");
            inner_private();
        }
        
        // Visible to parent module only
        pub(super) fn super_visible() {
            println!("super_visible - visible to outer");
        }
        
        // Visible to the entire crate
        pub(crate) fn crate_visible() {
            println!("crate_visible - visible everywhere in crate");
        }
        
        // Can access parent's pub items
        pub fn access_parent() {
            // super::outer_public(); // Can call parent's public items
            println!("Accessed parent");
        }
    }
    
    pub fn access_inner() {
        inner::inner_public();
        inner::super_visible(); // Can access pub(super)
        inner::crate_visible(); // Can access pub(crate)
    }
}

// TODO: Create a struct with mixed visibility fields
pub struct MyStruct {
    pub public_field: i32,
    private_field: String, // Default: private
    pub(crate) crate_visible_field: bool,
}

impl MyStruct {
    // TODO: Create a public constructor
    pub fn new(value: i32, name: &str) -> Self {
        MyStruct {
            public_field: value,
            private_field: name.to_string(),
            crate_visible_field: true,
        }
    }
    
    // Public method
    pub fn get_private(&self) -> &str {
        &self.private_field
    }
    
    // Private method
    fn internal_helper(&self) -> i32 {
        self.public_field * 2
    }
    
    // Public method using private helper
    pub fn calculate(&self) -> i32 {
        self.internal_helper() + 10
    }
}

// TODO: Create an enum with visibility considerations
pub enum PublicEnum {
    VariantA,
    VariantB(i32),
    VariantC { name: String },
}

// Enum variants are public if the enum is public
// Private enum - variants are private to the module
enum PrivateEnum {
    Secret,
    Hidden,
}

// TODO: Create a trait with visibility
pub trait PublicTrait {
    fn public_method(&self);
    
    // Trait methods are public if trait is public
    fn default_implementation(&self) {
        println!("Default implementation");
    }
}

impl PublicTrait for MyStruct {
    fn public_method(&self) {
        println!("MyStruct implements PublicTrait: {}", self.public_field);
    }
}

fn main() {
    // TODO: Call public functions
    public_function();
    // private_function(); // Error: private_function is private
    
    // TODO: Access module items
    outer::outer_public();
    outer::access_inner();
    outer::inner::inner_public();
    outer::inner::crate_visible(); // Can access pub(crate) from same crate
    
    // TODO: Use the struct
    let s = MyStruct::new(42, "test");
    println!("Public field: {}", s.public_field);
    println!("Private field via getter: {}", s.get_private());
    println!("Calculated: {}", s.calculate());
    // println!("{}", s.private_field); // Error: field is private
    
    // TODO: Use the enum
    let e1 = PublicEnum::VariantA;
    let e2 = PublicEnum::VariantB(10);
    
    match e2 {
        PublicEnum::VariantA => println!("A"),
        PublicEnum::VariantB(n) => println!("B with {}", n),
        PublicEnum::VariantC { name } => println!("C with {}", name),
    }
    
    // TODO: Use the trait
    s.public_method();
    s.default_implementation();
}

// TODO: Complete this module that demonstrates visibility
mod utils {
    // Private helper
    fn helper() -> i32 {
        42
    }
    
    // Public function
    pub fn get_answer() -> i32 {
        helper()
    }
    
    // Public to crate
    pub(crate) fn internal_util() -> String {
        String::from("internal")
    }
    
    // Module for internal implementation details
    pub mod internal {
        pub fn implementation_detail() {
            println!("This is public in internal, but internal is public in utils");
        }
        
        // Only visible to utils module
        pub(super) fn utils_only() {
            println!("Only visible to utils and its descendants");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests can access crate-visible items
    #[test]
    fn test_struct() {
        let s = MyStruct::new(10, "test");
        assert_eq!(s.public_field, 10);
        assert_eq!(s.calculate(), 30);
    }
    
    #[test]
    fn test_utils() {
        assert_eq!(utils::get_answer(), 42);
        assert_eq!(utils::internal_util(), "internal");
    }
    
    #[test]
    fn test_visibility_chain() {
        outer::access_inner();
        outer::inner::crate_visible();
    }
}
