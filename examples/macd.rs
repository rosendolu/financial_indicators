use financial_indicators::macd::MACD;
fn main() {
    let closing_prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let macd_values = MACD::new(&closing_prices, 12, 26, 9);

    for macd in macd_values {
        println!(
            "MACD: {}, Signal: {}, Histogram: {}",
            macd.macd, macd.signal, macd.histogram
        );
    }
}
