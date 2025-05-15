use financial_indicators::ma::simple_moving_average;
use financial_indicators::std_indicator::rolling_std;

fn main() {
    let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let period = 3;
    let std = rolling_std(&prices, period);
    let sma = simple_moving_average(&prices, period);
    println!("Day |   Price |    SMA   |    STD");
    println!("----+---------+----------+----------");
    for i in 0..prices.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            prices[i],
            sma[i],
            std[i],
        );
    }
}
