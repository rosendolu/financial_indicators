use financial_indicators::std_indicator::rolling_std;

fn main() {
    // Edge case: period is zero
    let prices = vec![1.0, 2.0, 3.0];
    let std_zero = rolling_std(&prices, 0);
    println!("STD with period 0: {:?}", std_zero);

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let std_empty = rolling_std(&empty, 3);
    println!("STD with empty input: {:?}", std_empty);

    // Edge case: period longer than input
    let std_long = rolling_std(&prices, 5);
    println!("STD with period > input length: {:?}", std_long);
}
