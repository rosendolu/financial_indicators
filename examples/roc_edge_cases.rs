use financial_indicators::roc::rate_of_change;

fn main() {
    // Edge case: period is zero
    let prices = vec![44.34, 44.09, 44.15];
    let roc_zero = rate_of_change(&prices, 0);
    println!("ROC with period 0: {:?}", roc_zero);

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let roc_empty = rate_of_change(&empty, 12);
    println!("ROC with empty input: {:?}", roc_empty);

    // Edge case: period longer than input
    let roc_long = rate_of_change(&prices, 12);
    println!("ROC with period > input length: {:?}", roc_long);

    // Edge case: previous price is zero
    let prices_with_zero = vec![0.0, 1.0, 2.0, 3.0];
    let roc_zero_prev = rate_of_change(&prices_with_zero, 2);
    println!("ROC with zero previous price: {:?}", roc_zero_prev);
}
