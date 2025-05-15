use financial_indicators::mom::momentum;

fn main() {
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89,
    ];
    let period = 10;
    let mom = momentum(&prices, period);
    println!("Momentum (period = {}):", period);
    for (i, value) in mom.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
