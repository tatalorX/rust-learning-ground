// ═══════════════════════════════════════════════════════════════════════════════
// 🦀 EXERCISE 151: Iterator Adapters
// ═══════════════════════════════════════════════════════════════════════════════
//
// 📚 LEARNING OBJECTIVE:
//    Master iterator adapters: map, filter, fold, and collect
//
// 🏢 REAL-WORLD SCENARIO:
//    You're processing sales data. Filter out invalid entries, transform prices
//    with tax calculations, and aggregate totals by category.
//
// 🎯 YOUR TASK:
//    Implement the following functions using iterator adapters:
//    - filter_valid_sales: Keep only positive amounts
//    - apply_tax: Add 8% tax to each amount
//    - total_by_threshold: Sum amounts above a threshold
//    - categorize_sales: Group sales into small/medium/large
//
// ═══════════════════════════════════════════════════════════════════════════════

fn main() {
    let sales = vec![100.0, -50.0, 200.0, 0.0, 150.0, -25.0, 300.0];

    println!("Original sales: {:?}", sales);
    println!("Valid sales: {:?}", filter_valid_sales(&sales));
    println!("With tax: {:?}", apply_tax(&sales, 0.08));
    println!("Total above 100: {}", total_by_threshold(&sales, 100.0));
    println!("By category: {:?}", categorize_sales(&sales));
}

// Filter out non-positive sales amounts
fn filter_valid_sales(sales: &[f64]) -> Vec<f64> {
    // TODO: Use filter adapter
    todo!()
}

// Apply tax percentage to each sale
fn apply_tax(sales: &[f64], tax_rate: f64) -> Vec<f64> {
    // TODO: Use map adapter
    todo!()
}

// Sum only sales above threshold
fn total_by_threshold(sales: &[f64], threshold: f64) -> f64 {
    // TODO: Use filter + fold
    todo!()
}

// Categorize sales: small (<100), medium (100-200), large (>200)
fn categorize_sales(sales: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // TODO: Use filter with partition or collect
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_valid_sales() {
        let sales = vec![100.0, -50.0, 200.0, 0.0, 150.0];
        assert_eq!(filter_valid_sales(&sales), vec![100.0, 200.0, 150.0]);
    }

    #[test]
    fn test_apply_tax() {
        let sales = vec![100.0, 200.0];
        let result = apply_tax(&sales, 0.10);
        assert!((result[0] - 110.0).abs() < 0.001);
        assert!((result[1] - 220.0).abs() < 0.001);
    }

    #[test]
    fn test_total_by_threshold() {
        let sales = vec![50.0, 150.0, 200.0, 75.0];
        assert!((total_by_threshold(&sales, 100.0) - 350.0).abs() < 0.001);
    }

    #[test]
    fn test_categorize_sales() {
        let sales = vec![50.0, 150.0, 250.0, 100.0, 200.0];
        let (small, medium, large) = categorize_sales(&sales);
        assert_eq!(small, vec![50.0]);
        assert_eq!(medium, vec![150.0, 100.0, 200.0]);
        assert_eq!(large, vec![250.0]);
    }
}
