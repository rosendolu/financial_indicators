use financial_indicators::mfi::money_flow_index;

fn main() {
    // Edge case: empty input
    let high: Vec<f64> = vec![];
    let low: Vec<f64> = vec![];
    let close: Vec<f64> = vec![];
    let volume: Vec<f64> = vec![];
    let period = 14;
    let mfi_empty = money_flow_index(&high, &low, &close, &volume, period);
    println!("MFI with empty input: {:?}", mfi_empty);

    // Edge case: period longer than input
    let high = vec![127.01, 127.62, 126.59];
    let low = vec![125.36, 126.56, 125.07];
    let close = vec![126.82, 127.07, 125.95];
    let volume = vec![1000.0, 1100.0, 1200.0];
    let mfi_short = money_flow_index(&high, &low, &close, &volume, 10);
    println!("MFI with period > input length: {:?}", mfi_short);

    // Edge case: constant prices and volumes
    let high = vec![100.0; 20];
    let low = vec![100.0; 20];
    let close = vec![100.0; 20];
    let volume = vec![1000.0; 20];
    let mfi_constant = money_flow_index(&high, &low, &close, &volume, 14);
    println!("MFI with constant prices/volumes:");
    for (i, val) in mfi_constant.iter().enumerate() {
        println!("Idx {}: {:?}", i + 1, val);
    }
}
