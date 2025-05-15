use financial_indicators::ema::exponential_moving_average;
use financial_indicators::macd::MACD;

fn main() {
    let closes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let short_period = 3;
    let long_period = 6;
    let signal_period = 2;
    let ema = exponential_moving_average(&closes, short_period);
    let macd_values = MACD::new(&closes, short_period, long_period, signal_period);
    println!("Idx |  Close |   EMA   |   MACD");
    println!("----+--------+---------+---------");
    for i in 0..macd_values.len() {
        let idx = i + long_period - 1;
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7.4}",
            idx + 1,
            closes[idx],
            ema[idx],
            macd_values[i].macd
        );
    }
}
