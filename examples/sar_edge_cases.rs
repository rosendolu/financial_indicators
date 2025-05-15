use financial_indicators::sar::parabolic_sar;

fn main() {
    // Edge case: empty input
    let high: Vec<f64> = vec![];
    let low: Vec<f64> = vec![];
    let sar_empty = parabolic_sar(&high, &low, 0.02, 0.2);
    println!("SAR with empty input: {:?}", sar_empty);

    // Edge case: single value
    let high = vec![10.0];
    let low = vec![9.7];
    let sar_single = parabolic_sar(&high, &low, 0.02, 0.2);
    println!("SAR with single value: {:?}", sar_single);

    // Edge case: constant prices
    let high = vec![10.0, 10.0, 10.0, 10.0];
    let low = vec![10.0, 10.0, 10.0, 10.0];
    let sar_constant = parabolic_sar(&high, &low, 0.02, 0.2);
    println!("SAR with constant prices: {:?}", sar_constant);
}
