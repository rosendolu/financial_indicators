use financial_indicators::roc::rate_of_change;

fn main() {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61,
    ];
    let period = 12;
    let roc = rate_of_change(&prices, period);
    println!("Rate of Change (period = {}):", period);
    for (i, value) in roc.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
