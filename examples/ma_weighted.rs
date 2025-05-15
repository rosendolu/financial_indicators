use financial_indicators::ma::weighted_moving_average;

fn main() {
    let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
    let weights = vec![1.0, 2.0, 3.0];
    let wma = weighted_moving_average(&prices, &weights);
    println!("Weighted Moving Average (weights = {:?}):", weights);
    for (i, value) in wma.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
