use financial_indicators::cci::commodity_channel_index;
use financial_indicators::ma::simple_moving_average;

fn main() {
    let high = vec![127.01, 127.62, 126.59, 127.35, 128.17];
    let low = vec![125.36, 126.56, 125.07, 126.50, 126.80];
    let close = vec![126.82, 127.07, 125.95, 127.29, 127.10];
    let period = 3;
    let cci = commodity_channel_index(&high, &low, &close, period);
    let sma = simple_moving_average(&close, period);
    println!("Day |   Close |    SMA   |    CCI");
    println!("----+---------+----------+----------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            close[i],
            sma[i],
            cci[i],
        );
    }
}
