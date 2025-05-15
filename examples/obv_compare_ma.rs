use financial_indicators::ma::simple_moving_average;
use financial_indicators::obv::on_balance_volume;

fn main() {
    let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8, 11.0, 10.7];
    let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0, 1500.0, 800.0];
    let period = 3;
    let sma = simple_moving_average(&closes, period);
    let obv = on_balance_volume(&closes, &volumes);
    println!("Day |  Close |   SMA   |   OBV");
    println!("----+--------+---------+---------");
    for i in 0..closes.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7?}",
            i + 1,
            closes[i],
            sma[i],
            obv[i],
        );
    }
}
