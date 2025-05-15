use financial_indicators::ma::simple_moving_average;
use financial_indicators::mom::momentum;

fn main() {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89,
    ];
    let period = 10;
    let mom = momentum(&prices, period);
    let sma = simple_moving_average(&prices, period);
    println!("Day |   Price |    SMA   |    MOM");
    println!("----+---------+----------+----------");
    for i in 0..prices.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            prices[i],
            sma[i],
            mom[i],
        );
    }
}
