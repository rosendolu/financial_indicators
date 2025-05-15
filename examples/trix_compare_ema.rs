use financial_indicators::ema::exponential_moving_average;
use financial_indicators::trix::trix;

fn main() {
    let closes: Vec<f64> = (1..=30).map(|x| x as f64).collect();
    let period = 15;
    let trix_values = trix(&closes, period);
    let ema = exponential_moving_average(&closes, period);
    println!("Idx |  Close |   EMA   |   TRIX");
    println!("----+--------+---------+---------");
    for i in 0..closes.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7?}",
            i + 1,
            closes[i],
            ema[i],
            trix_values[i]
        );
    }
}
