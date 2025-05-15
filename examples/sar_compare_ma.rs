use financial_indicators::ma::simple_moving_average;
use financial_indicators::sar::parabolic_sar;

fn main() {
    let high = vec![10.0, 10.2, 10.4, 10.3, 10.5, 10.7, 10.8, 10.9, 11.0];
    let low = vec![9.7, 9.9, 10.0, 10.1, 10.2, 10.4, 10.6, 10.7, 10.8];
    let close: Vec<f64> = high.iter().zip(&low).map(|(h, l)| (h + l) / 2.0).collect();
    let sar = parabolic_sar(&high, &low, 0.02, 0.2);
    let sma = simple_moving_average(&close, 3);
    println!("Idx |  Close |   SMA   |   SAR");
    println!("----+--------+---------+---------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7.3?}",
            i + 1,
            close[i],
            sma[i],
            sar[i]
        );
    }
}
