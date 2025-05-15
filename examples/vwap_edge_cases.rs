use financial_indicators::vwap::vwap;

fn main() {
    // Edge case: empty input
    let high: Vec<f64> = vec![];
    let low: Vec<f64> = vec![];
    let close: Vec<f64> = vec![];
    let volume: Vec<f64> = vec![];
    let vwap_empty = vwap(&high, &low, &close, &volume);
    println!("VWAP with empty input: {:?}", vwap_empty);

    // Edge case: mismatched lengths (should use min length)
    let high = vec![10.0, 11.0, 12.0];
    let low = vec![9.0, 10.0];
    let close = vec![9.5, 10.5, 11.5];
    let volume = vec![100.0, 200.0, 150.0];
    let vwap_mismatch = vwap(&high, &low, &close, &volume);
    println!("VWAP with mismatched lengths: {:?}", vwap_mismatch);

    // Edge case: zero volume
    let high = vec![10.0, 11.0, 12.0];
    let low = vec![9.0, 10.0, 11.0];
    let close = vec![9.5, 10.5, 11.5];
    let volume = vec![0.0, 0.0, 0.0];
    let vwap_zero_vol = vwap(&high, &low, &close, &volume);
    println!("VWAP with zero volume: {:?}", vwap_zero_vol);
}
