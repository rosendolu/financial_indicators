use financial_indicators::ema::exponential_moving_average;
use financial_indicators::ma::simple_moving_average;

fn main() {
    let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
    let period = 3;
    let ema = exponential_moving_average(&prices, period);
    let sma = simple_moving_average(&prices, period);
    println!("Day |   Price |    SMA   |    EMA");
    println!("----+---------+----------+----------");
    for i in 0..prices.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            prices[i],
            sma[i],
            ema[i],
        );
    }
}
