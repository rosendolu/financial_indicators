use financial_indicators::bollinger::bollinger_bands;

fn main() {
    // Edge case: period is zero
    let prices = vec![22.27, 22.19, 22.08];
    let (upper_zero, middle_zero, lower_zero) = bollinger_bands(&prices, 0, 2.0);
    println!(
        "Bollinger Bands with period 0: upper={:?}, middle={:?}, lower={:?}",
        upper_zero, middle_zero, lower_zero
    );

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let (upper_empty, middle_empty, lower_empty) = bollinger_bands(&empty, 20, 2.0);
    println!(
        "Bollinger Bands with empty input: upper={:?}, middle={:?}, lower={:?}",
        upper_empty, middle_empty, lower_empty
    );

    // Edge case: period longer than input
    let (upper_long, middle_long, lower_long) = bollinger_bands(&prices, 20, 2.0);
    println!(
        "Bollinger Bands with period > input length: upper={:?}, middle={:?}, lower={:?}",
        upper_long, middle_long, lower_long
    );
}
