use financial_indicators::rsi::relative_strength_index;

fn main() {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61,
        46.28, 46.28,
    ];
    let period = 14;
    let rsi = relative_strength_index(&prices, period);
    println!("Relative Strength Index (period = {}):", period);
    for (i, value) in rsi.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
