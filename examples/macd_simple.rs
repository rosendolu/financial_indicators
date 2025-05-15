use financial_indicators::macd::MACD;

fn main() {
    let closes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let short_period = 3;
    let long_period = 6;
    let signal_period = 2;
    let macd_values = MACD::new(&closes, short_period, long_period, signal_period);
    println!("MACD Values:");
    for (i, macd) in macd_values.iter().enumerate() {
        println!(
            "Idx {}: MACD: {:.4}, Signal: {:.4}, Histogram: {:.4}",
            i + long_period,
            macd.macd,
            macd.signal,
            macd.histogram
        );
    }
}
