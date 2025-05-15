use financial_indicators::cci::commodity_channel_index;

fn main() {
    // Edge case: period is zero
    let high = vec![127.01, 127.62, 126.59];
    let low = vec![125.36, 126.56, 125.07];
    let close = vec![126.82, 127.07, 125.95];
    let cci_zero = commodity_channel_index(&high, &low, &close, 0);
    println!("CCI with period 0: {:?}", cci_zero);

    // Edge case: empty input
    let empty: Vec<f64> = vec![];
    let cci_empty = commodity_channel_index(&empty, &empty, &empty, 3);
    println!("CCI with empty input: {:?}", cci_empty);

    // Edge case: period longer than input
    let cci_long = commodity_channel_index(&high, &low, &close, 5);
    println!("CCI with period > input length: {:?}", cci_long);
}
