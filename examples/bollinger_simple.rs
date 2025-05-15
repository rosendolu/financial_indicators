use financial_indicators::bollinger::bollinger_bands;

fn main() {
    let prices = vec![
        22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39, 22.38,
        22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63, 23.82, 23.87, 23.65, 23.19, 23.10, 23.33,
        22.68, 23.10, 22.40, 22.17,
    ];
    let period = 20;
    let k = 2.0;
    let (upper, middle, lower) = bollinger_bands(&prices, period, k);
    println!("Bollinger Bands (period = {}, k = {}):", period, k);
    for i in 0..prices.len() {
        println!(
            "Day {}: upper={:?}, middle={:?}, lower={:?}",
            i + 1,
            upper[i],
            middle[i],
            lower[i]
        );
    }
}
