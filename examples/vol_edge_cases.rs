use financial_indicators::vol::volume_indicator;

fn main() {
    // Edge case: empty input
    let volume: Vec<f64> = vec![];
    let vol_empty = volume_indicator(&volume);
    println!("VOL with empty input: {:?}", vol_empty);

    // Edge case: all zero volume
    let volume = vec![0.0, 0.0, 0.0, 0.0];
    let vol_zero = volume_indicator(&volume);
    println!("VOL with all zero volume: {:?}", vol_zero);
}
