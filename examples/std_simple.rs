use financial_indicators::std_indicator::rolling_std;

fn main() {
    let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let period = 3;
    let std = rolling_std(&prices, period);
    println!("Rolling Standard Deviation (period = {}):", period);
    for (i, value) in std.iter().enumerate() {
        println!("Day {}: {:?}", i + 1, value);
    }
}
