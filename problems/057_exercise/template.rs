// Exercise 057: Option - unwrap_or and unwrap_or_else
//
// unwrap_or() returns the contained value or a provided default.
// unwrap_or_else() is similar but takes a closure for lazy evaluation.
//
// Learning Objectives:
// - Use unwrap_or() for simple default values
// - Use unwrap_or_else() for expensive default computations
// - Understand when to use each variant
//
// Your task: Implement functions using the appropriate unwrap variant.

/// Returns the config value or "default" if None.
fn get_config_or_default(config: Option<&str>) -> String {
    // TODO: Use unwrap_or() to return the config value or "default"
    // Note: You'll need to convert to String
    todo!()
}

/// Returns the number or computes a default based on input.
fn get_number_or_compute(input: Option<i32>, multiplier: i32) -> i32 {
    // TODO: Use unwrap_or_else() to return the number or compute default as multiplier * 10
    // This simulates an expensive computation
    todo!()
}

/// Returns the username or fetches from environment (simulated).
fn get_username(config: Option<String>) -> String {
    // TODO: Use unwrap_or_else() to return the username or "guest" (simulate expensive fetch)
    todo!()
}

/// Returns the max value or 0 if the iterator is empty.
fn find_max_or_default(numbers: &[i32]) -> i32 {
    // TODO: Use iter().max() which returns Option<&i32>
    // Use unwrap_or() with dereferencing to get the value
    todo!()
}

/// Simulates a cache lookup with expensive fallback computation.
struct Cache {
    data: std::collections::HashMap<String, i32>,
}

impl Cache {
    fn new() -> Self {
        let mut data = std::collections::HashMap::new();
        data.insert("key1".to_string(), 100);
        Self { data }
    }
    
    fn get(&self, key: &str) -> Option<i32> {
        self.data.get(key).copied()
    }
    
    /// Gets value from cache or computes it (simulated expensive operation).
    fn get_or_compute(&self, key: &str) -> i32 {
        // TODO: Use unwrap_or_else() to get from cache or compute (key.len() * 10 as i32)
        todo!()
    }
}

fn main() {
    // Test get_config_or_default
    println!("Config values:");
    println!("  Some('production'): {}", get_config_or_default(Some("production")));
    println!("  None: {}", get_config_or_default(None));
    
    // Test get_number_or_compute
    println!("\nNumber with compute fallback:");
    println!("  Some(5) with mult 3: {}", get_number_or_compute(Some(5), 3));
    println!("  None with mult 3: {}", get_number_or_compute(None, 3));
    
    // Test find_max_or_default
    let numbers = vec![3, 1, 4, 1, 5, 9];
    println!("\nMax values:");
    println!("  Max of {:?}: {}", numbers, find_max_or_default(&numbers));
    println!("  Max of []: {}", find_max_or_default(&[]));
    
    // Test cache
    let cache = Cache::new();
    println!("\nCache lookups:");
    println!("  key1: {}", cache.get_or_compute("key1"));
    println!("  key2 (computed as {} * 10): {}", "key2".len(), cache.get_or_compute("key2"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_or_default_some() {
        assert_eq!(get_config_or_default(Some("test")), "test".to_string());
    }

    #[test]
    fn test_get_config_or_default_none() {
        assert_eq!(get_config_or_default(None), "default".to_string());
    }

    #[test]
    fn test_get_number_or_compute_with_some() {
        assert_eq!(get_number_or_compute(Some(5), 3), 5);
    }

    #[test]
    fn test_get_number_or_compute_with_none() {
        assert_eq!(get_number_or_compute(None, 5), 50);
        assert_eq!(get_number_or_compute(None, 3), 30);
    }

    #[test]
    fn test_find_max_or_default_with_values() {
        assert_eq!(find_max_or_default(&[1, 5, 3]), 5);
    }

    #[test]
    fn test_find_max_or_default_empty() {
        assert_eq!(find_max_or_default(&[]), 0);
    }

    #[test]
    fn test_cache_hit() {
        let cache = Cache::new();
        assert_eq!(cache.get_or_compute("key1"), 100);
    }

    #[test]
    fn test_cache_miss_computes() {
        let cache = Cache::new();
        assert_eq!(cache.get_or_compute("abc"), 30); // "abc".len() * 10 = 30
        assert_eq!(cache.get_or_compute("longer"), 60); // "longer".len() * 10 = 60
    }
}
