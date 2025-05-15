use financial_indicators::atr::average_true_range;

fn main() {
    // Edge case: period is zero
    let high = vec![48.70, 48.72, 48.90];
    let low = vec![47.79, 48.14, 48.39];
    let close = vec![48.16, 48.61, 48.75];
    let atr_zero = average_true_range(&high, &low, &close, 0);
    println!("ATR with period 0: {:?}", atr_zero);

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let atr_empty = average_true_range(&empty, &empty, &empty, 3);
    println!("ATR with empty input: {:?}", atr_empty);

    // Edge case: period longer than input
    let atr_long = average_true_range(&high, &low, &close, 5);
    println!("ATR with period > input length: {:?}", atr_long);
}
