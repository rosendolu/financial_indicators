use financial_indicators::dmi::dmi_adx;
use financial_indicators::ma::simple_moving_average;

fn main() {
    let high = vec![
        30.0, 32.0, 31.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0,
    ];
    let low = vec![
        28.0, 29.0, 29.5, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0,
    ];
    let close = vec![
        29.0, 31.0, 30.5, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0,
    ];
    let period = 14;
    let (_, _, adx) = dmi_adx(&high, &low, &close, period);
    let sma = simple_moving_average(&close, period);
    println!("Idx |  Close |   SMA   |   ADX");
    println!("----+--------+---------+---------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7?}",
            i + 1,
            close[i],
            sma[i],
            adx[i]
        );
    }
}
