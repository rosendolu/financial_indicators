use financial_indicators::ma::simple_moving_average;
use financial_indicators::vwap::vwap;

fn main() {
    let high = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
    let low = vec![9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
    let close = vec![9.5, 10.5, 11.5, 12.5, 13.5, 14.5, 15.5];
    let volume = vec![100.0, 200.0, 150.0, 120.0, 180.0, 160.0, 140.0];
    let period = 3;
    let vwap_values = vwap(&high, &low, &close, &volume);
    let sma = simple_moving_average(&close, period);
    println!("Idx |  Close |   SMA   |   VWAP");
    println!("----+--------+---------+---------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7.3?}",
            i + 1,
            close[i],
            sma[i],
            vwap_values[i]
        );
    }
}
