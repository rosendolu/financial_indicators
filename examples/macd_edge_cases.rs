use financial_indicators::macd::MACD;

fn main() {
    // Edge case: empty input
    let closes: Vec<f64> = vec![];
    let macd_empty = MACD::new(&closes, 3, 6, 2);
    println!("MACD with empty input: {:?}", macd_empty);

    // Edge case: single value
    let closes = vec![100.0];
    let macd_single = MACD::new(&closes, 3, 6, 2);
    println!("MACD with single value: {:?}", macd_single);

    // Edge case: constant prices
    let closes = vec![100.0, 100.0, 100.0, 100.0, 100.0, 100.0];
    let macd_constant = MACD::new(&closes, 3, 6, 2);
    println!("MACD with constant prices:");
    for (i, macd) in macd_constant.iter().enumerate() {
        println!(
            "Idx {}: MACD: {:.4}, Signal: {:.4}, Histogram: {:.4}",
            i + 6,
            macd.macd,
            macd.signal,
            macd.histogram
        );
    }
}
