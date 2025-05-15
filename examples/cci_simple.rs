use financial_indicators::cci::commodity_channel_index;

fn main() {
    let high = vec![127.01, 127.62, 126.59, 127.35, 128.17];
    let low = vec![125.36, 126.56, 125.07, 126.50, 126.80];
    let close = vec![126.82, 127.07, 125.95, 127.29, 127.10];
    let period = 3;
    let cci = commodity_channel_index(&high, &low, &close, period);
    println!("Commodity Channel Index (period = {}):", period);
    for (i, value) in cci.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
