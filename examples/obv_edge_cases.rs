use financial_indicators::obv::on_balance_volume;

fn main() {
    // Edge case: empty input
    let closes: Vec<f64> = vec![];
    let volumes: Vec<f64> = vec![];
    let obv_empty = on_balance_volume(&closes, &volumes);
    println!("OBV with empty input: {:?}", obv_empty);

    // Edge case: single value
    let closes = vec![10.0];
    let volumes = vec![1000.0];
    let obv_single = on_balance_volume(&closes, &volumes);
    println!("OBV with single value: {:?}", obv_single);

    // Edge case: mismatched lengths
    let closes = vec![10.0, 10.5, 10.2];
    let volumes = vec![1000.0, 1200.0];
    let obv_mismatch = on_balance_volume(&closes, &volumes);
    println!("OBV with mismatched lengths: {:?}", obv_mismatch);
}
