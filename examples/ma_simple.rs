use financial_indicators::ma::simple_moving_average;

fn main() {
    let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
    let period = 3;
    let sma = simple_moving_average(&prices, period);
    println!("Simple Moving Average (period = {}):", period);
    for (i, value) in sma.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
