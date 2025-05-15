use financial_indicators::ema::exponential_moving_average;

fn main() {
    // Edge case: period is zero
    let prices = vec![10.0, 11.0, 12.0];
    let ema_zero = exponential_moving_average(&prices, 0);
    println!("EMA with period 0: {:?}", ema_zero);

    // Edge case: empty input
    let empty_prices: Vec<f64> = vec![];
    let ema_empty = exponential_moving_average(&empty_prices, 3);
    println!("EMA with empty input: {:?}", ema_empty);

    // Edge case: period longer than input (should still compute, as EMA does not require full window)
    let ema_long = exponential_moving_average(&prices, 5);
    println!("EMA with period > input length: {:?}", ema_long);
}
