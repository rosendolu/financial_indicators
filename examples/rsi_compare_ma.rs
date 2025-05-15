use financial_indicators::ma::simple_moving_average;
use financial_indicators::rsi::relative_strength_index;

fn main() {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61,
        46.28, 46.28,
    ];
    let period = 14;
    let rsi = relative_strength_index(&prices, period);
    let sma = simple_moving_average(&prices, period);
    println!("Day |   Price |    SMA   |    RSI");
    println!("----+---------+----------+----------");
    for i in 0..prices.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            prices[i],
            sma[i],
            rsi[i],
        );
    }
}
