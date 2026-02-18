// Exercise 065: Traits as Parameters
//
// Traits can be used to accept different types that share behavior.
// This is done using impl Trait syntax or trait objects.
//
// Learning Objectives:
// - Use impl Trait for function parameters
// - Use impl Trait for return types
// - Understand trait bounds vs impl Trait
//
// Your task: Write functions that accept and return trait implementations.

/// A trait for objects that can make noise.
trait SoundMaker {
    fn make_sound(&self) -> String;
    fn volume(&self) -> u8 {
        50 // default volume
    }
}

// Concrete types
struct Dog;
struct Cat;
struct Car {
    model: String,
}

impl SoundMaker for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
    fn volume(&self) -> u8 {
        80
    }
}

impl SoundMaker for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
    fn volume(&self) -> u8 {
        40
    }
}

impl SoundMaker for Car {
    fn make_sound(&self) -> String {
        "Vroom!".to_string()
    }
    fn volume(&self) -> u8 {
        90
    }
}

/// Takes anything that makes sound and prints its sound.
fn announce(sound_maker: &impl SoundMaker) {
    // TODO: Print "Making sound: {sound}" and "Volume: {volume}"
    todo!()
}

/// Takes two sound makers and returns the louder one.
fn loudest<'a>(a: &'a impl SoundMaker, b: &'a impl SoundMaker) -> &'a dyn SoundMaker {
    // TODO: Return a trait object reference to the louder sound maker
    todo!()
}

/// Takes a slice of sound makers and returns their sounds.
fn collect_sounds(items: &[impl SoundMaker]) -> Vec<String> {
    // TODO: Collect all sounds into a vector and return
    todo!()
}

/// Creates a sound maker based on input (simplified factory).
fn create_sound_maker(animal: &str) -> Box<dyn SoundMaker> {
    // TODO: Return Box<dyn SoundMaker> based on the animal string
    // "dog" -> Box::new(Dog)
    // "cat" -> Box::new(Cat)
    // otherwise -> Box::new(Car { model: animal.to_string() })
    todo!()
}

/// Filters sound makers by minimum volume.
fn loud_sounds<'a>(
    items: &'a [Box<dyn SoundMaker>],
    min_volume: u8,
) -> Vec<&'a dyn SoundMaker> {
    // TODO: Return references to items with volume >= min_volume
    todo!()
}

fn main() {
    // After implementing, uncomment and run:
    
    // let dog = Dog;
    // let cat = Cat;
    // let car = Car { model: "Ferrari".to_string() };
    
    // println!("Individual announcements:");
    // announce(&dog);
    // announce(&cat);
    // announce(&car);
    
    // println!("\nLoudest:");
    // let louder = loudest(&dog, &cat);
    // println!("Louder: {} (volume {})", louder.make_sound(), louder.volume());
    
    // println!("\nCollecting sounds:");
    // let animals: Vec<&dyn SoundMaker> = vec![&dog, &cat, &car];
    // let sounds = collect_sounds(&animals);
    // for sound in sounds {
    //     println!("  {}", sound);
    // }
    
    // println!("\nFactory pattern:");
    // let makers = vec![
    //     create_sound_maker("dog"),
    //     create_sound_maker("cat"),
    //     create_sound_maker("truck"),
    // ];
    // for maker in &makers {
    //     println!("  {} (volume: {})", maker.make_sound(), maker.volume());
    // }
    
    // println!("\nLoud sounds only (min 70):");
    // let loud = loud_sounds(&makers, 70);
    // for maker in loud {
    //     println!("  {} (volume: {})", maker.make_sound(), maker.volume());
    // }
    
    println!("Implement all TODOs to see the output!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_sound() {
        let dog = Dog;
        assert_eq!(dog.make_sound(), "Woof!");
        assert_eq!(dog.volume(), 80);
    }

    #[test]
    fn test_cat_sound() {
        let cat = Cat;
        assert_eq!(cat.make_sound(), "Meow!");
        assert_eq!(cat.volume(), 40);
    }

    #[test]
    fn test_car_sound() {
        let car = Car { model: "Test".to_string() };
        assert_eq!(car.make_sound(), "Vroom!");
        assert_eq!(car.volume(), 90);
    }

    #[test]
    fn test_create_sound_maker_dog() {
        let maker = create_sound_maker("dog");
        assert_eq!(maker.make_sound(), "Woof!");
    }

    #[test]
    fn test_create_sound_maker_cat() {
        let maker = create_sound_maker("cat");
        assert_eq!(maker.make_sound(), "Meow!");
    }

    #[test]
    fn test_create_sound_maker_other() {
        let maker = create_sound_maker("truck");
        assert_eq!(maker.make_sound(), "Vroom!");
    }

    #[test]
    fn test_loud_sounds_filtering() {
        let makers: Vec<Box<dyn SoundMaker>> = vec![
            create_sound_maker("dog"),   // vol 80
            create_sound_maker("cat"),   // vol 40
            create_sound_maker("car"),   // vol 90
        ];
        let loud = loud_sounds(&makers, 70);
        assert_eq!(loud.len(), 2);
    }
}
