use financial_indicators::obv::on_balance_volume;

fn main() {
    let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8];
    let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
    let obv = on_balance_volume(&closes, &volumes);
    println!("On-Balance Volume:");
    for (i, value) in obv.iter().enumerate() {
        println!("Day {}: {}", i + 1, value);
    }
}
