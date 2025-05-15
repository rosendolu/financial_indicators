use financial_indicators::mom::momentum;

fn main() {
    // Edge case: period is zero
    let prices = vec![44.34, 44.09, 44.15];
    let mom_zero = momentum(&prices, 0);
    println!("MOM with period 0: {:?}", mom_zero);

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let mom_empty = momentum(&empty, 10);
    println!("MOM with empty input: {:?}", mom_empty);

    // Edge case: period longer than input
    let mom_long = momentum(&prices, 10);
    println!("MOM with period > input length: {:?}", mom_long);
}
