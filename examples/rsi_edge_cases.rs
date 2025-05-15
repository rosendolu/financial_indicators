use financial_indicators::rsi::relative_strength_index;

fn main() {
    // Edge case: period is zero
    let prices = vec![44.34, 44.09, 44.15];
    let rsi_zero = relative_strength_index(&prices, 0);
    println!("RSI with period 0: {:?}", rsi_zero);

    // Edge case: empty input
    let empty_prices: Vec<f64> = vec![];
    let rsi_empty = relative_strength_index(&empty_prices, 14);
    println!("RSI with empty input: {:?}", rsi_empty);

    // Edge case: period longer than input
    let rsi_long = relative_strength_index(&prices, 14);
    println!("RSI with period > input length: {:?}", rsi_long);
}
