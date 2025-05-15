use financial_indicators::ma::{simple_moving_average, weighted_moving_average};

fn main() {
    // Edge case: period is zero
    let prices = vec![10.0, 11.0, 12.0];
    let sma_zero = simple_moving_average(&prices, 0);
    println!("SMA with period 0: {:?}", sma_zero);

    // Edge case: empty input
    let empty_prices: Vec<f64> = vec![];
    let sma_empty = simple_moving_average(&empty_prices, 3);
    println!("SMA with empty input: {:?}", sma_empty);

    // Edge case: period longer than input
    let sma_long = simple_moving_average(&prices, 5);
    println!("SMA with period > input length: {:?}", sma_long);

    // Edge case: WMA with empty weights
    let wma_empty_weights = weighted_moving_average(&prices, &[]);
    println!("WMA with empty weights: {:?}", wma_empty_weights);

    // Edge case: WMA with weights longer than input
    let weights = vec![1.0, 2.0, 3.0, 4.0];
    let wma_long_weights = weighted_moving_average(&prices, &weights);
    println!("WMA with weights > input length: {:?}", wma_long_weights);
}
