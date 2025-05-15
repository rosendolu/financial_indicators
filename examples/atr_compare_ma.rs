use financial_indicators::atr::average_true_range;
use financial_indicators::ma::simple_moving_average;

fn main() {
    let high = vec![48.70, 48.72, 48.90, 48.87, 48.82];
    let low = vec![47.79, 48.14, 48.39, 48.37, 48.24];
    let close = vec![48.16, 48.61, 48.75, 48.63, 48.74];
    let period = 3;
    let atr = average_true_range(&high, &low, &close, period);
    let sma = simple_moving_average(&close, period);
    println!("Day |   Close |    SMA   |    ATR");
    println!("----+---------+----------+----------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>7.2} | {:>8?} | {:>8?}",
            i + 1,
            close[i],
            sma[i],
            atr[i],
        );
    }
}
