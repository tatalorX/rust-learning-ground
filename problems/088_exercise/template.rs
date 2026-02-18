// Exercise 088: Pattern Matching - Guards
//
// Learning objective: Learn pattern guards (if conditions in match arms)
// for more expressive pattern matching with additional conditions.
//
// Guards allow you to add boolean conditions to match arms,
// refining when a pattern matches beyond just the structure.

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

struct Person {
    age: u32,
    name: String,
}

fn main() {
    // TODO: Match on temperature with guards
    let temps = vec![
        Temperature::Celsius(25.0),
        Temperature::Celsius(-5.0),
        Temperature::Fahrenheit(98.6),
        Temperature::Fahrenheit(32.0),
        Temperature::Kelvin(273.15),
    ];
    
    for temp in temps {
        describe_temperature(temp);
    }

    // TODO: Match with guards on integers
    let scores = vec![95, 82, 75, 65, 55, 45];
    
    for score in scores {
        let grade = match score {
            // YOUR CODE HERE - use guards for letter grades
            // A: >= 90, B: >= 80, C: >= 70, D: >= 60, F: < 60
            n if n >= 90 => 'A',
            n if n >= 80 => 'B',
            n if n >= 70 => 'C',
            n if n >= 60 => 'D',
            _ => 'F',
        };
        println!("Score {} => Grade {}", score, grade);
    }

    // TODO: Use guards with struct patterns
    let people = vec![
        Person { age: 5, name: String::from("Tommy") },
        Person { age: 17, name: String::from("Alex") },
        Person { age: 25, name: String::from("Sarah") },
        Person { age: 70, name: String::from("Bob") },
    ];
    
    for person in people {
        categorize_person(person);
    }

    // TODO: Multiple conditions with guards
    let pairs = vec![(1, 2), (2, 1), (2, 2), (3, 4), (5, 5)];
    
    for (x, y) in pairs {
        let result = match (x, y) {
            // YOUR CODE HERE - match different relationships
            (a, b) if a == b => "equal",
            (a, b) if a > b => "first is larger",
            (a, b) if a < b => "second is larger",
            _ => "other",
        };
        println!("({}, {}) => {}", x, y, result);
    }

    // TODO: Complex guards with multiple conditions
    let numbers = vec![Some(5), Some(15), Some(25), None];
    
    for num in numbers {
        match num {
            // YOUR CODE HERE - use guards to categorize
            Some(n) if n < 10 => println!("Small: {}", n),
            Some(n) if n >= 10 && n < 20 => println!("Medium: {}", n),
            Some(n) => println!("Large: {}", n),
            None => println!("No value"),
        }
    }
}

// TODO: Complete this function using guards to describe temperatures
fn describe_temperature(temp: Temperature) {
    match temp {
        // Freezing: <= 0°C, Cold: 0-15°C, Mild: 15-25°C, Hot: > 25°C
        // For Fahrenheit: <= 32°F, 32-59°F, 59-77°F, > 77°F
        // For Kelvin: <= 273.15K freezing, etc.
        Temperature::Celsius(c) if c <= 0.0 => println!("Freezing ({}°C)", c),
        Temperature::Celsius(c) if c <= 15.0 => println!("Cold ({}°C)", c),
        Temperature::Celsius(c) if c <= 25.0 => println!("Mild ({}°C)", c),
        Temperature::Celsius(c) => println!("Hot ({}°C)", c),
        
        Temperature::Fahrenheit(f) if f <= 32.0 => println!("Freezing ({}°F)", f),
        Temperature::Fahrenheit(f) if f <= 59.0 => println!("Cold ({}°F)", f),
        Temperature::Fahrenheit(f) if f <= 77.0 => println!("Mild ({}°F)", f),
        Temperature::Fahrenheit(f) => println!("Hot ({}°F)", f),
        
        Temperature::Kelvin(k) if k <= 273.15 => println!("Freezing ({}K)", k),
        Temperature::Kelvin(k) if k <= 288.15 => println!("Cold ({}K)", k),
        Temperature::Kelvin(k) if k <= 298.15 => println!("Mild ({}K)", k),
        Temperature::Kelvin(k) => println!("Hot ({}K)", k),
    }
}

// TODO: Complete this function using guards to categorize by age
fn categorize_person(Person { age, name }: Person) {
    match age {
        // YOUR CODE HERE - Baby: 0-2, Child: 3-12, Teen: 13-19, Adult: 20-64, Senior: 65+
        a if a <= 2 => println!("{} is a baby ({})", name, age),
        a if a <= 12 => println!("{} is a child ({})", name, age),
        a if a <= 19 => println!("{} is a teen ({})", name, age),
        a if a <= 64 => println!("{} is an adult ({})", name, age),
        _ => println!("{} is a senior ({})", name, age),
    }
}

// TODO: Complete this function that categorizes a number range
fn categorize_range(start: i32, end: i32) -> &'static str {
    match (start, end) {
        // Both negative
        (s, e) if s < 0 && e < 0 => "both negative",
        // Both positive
        (s, e) if s > 0 && e > 0 => "both positive",
        // Spans zero
        (s, e) if s <= 0 && e >= 0 => "spans zero",
        // Otherwise
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_range() {
        assert_eq!(categorize_range(-5, -1), "both negative");
        assert_eq!(categorize_range(1, 5), "both positive");
        assert_eq!(categorize_range(-3, 3), "spans zero");
    }

    #[test]
    fn test_grade_calculation() {
        let scores = vec![(95, 'A'), (85, 'B'), (75, 'C'), (65, 'D'), (55, 'F')];
        for (score, expected) in scores {
            let grade = match score {
                n if n >= 90 => 'A',
                n if n >= 80 => 'B',
                n if n >= 70 => 'C',
                n if n >= 60 => 'D',
                _ => 'F',
            };
            assert_eq!(grade, expected);
        }
    }
}
