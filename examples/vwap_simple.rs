use financial_indicators::vwap::vwap;

fn main() {
    let high = vec![10.0, 11.0, 12.0, 13.0, 14.0];
    let low = vec![9.0, 10.0, 11.0, 12.0, 13.0];
    let close = vec![9.5, 10.5, 11.5, 12.5, 13.5];
    let volume = vec![100.0, 200.0, 150.0, 120.0, 180.0];
    let vwap_values = vwap(&high, &low, &close, &volume);
    println!("Idx |  VWAP");
    println!("----+---------");
    for i in 0..high.len() {
        println!("{:>3} | {:>7.3?}", i + 1, vwap_values[i]);
    }
}
