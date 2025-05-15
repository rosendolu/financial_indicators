use financial_indicators::kdj::KDJ;
use financial_indicators::ma::simple_moving_average;

fn main() {
    let highs = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];
    let lows = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let closes = vec![10.5, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
    let period = 3;
    let kdj_values = KDJ::new(&highs, &lows, &closes, period);
    let sma = simple_moving_average(&closes, period);
    println!("Day |  Close |   SMA   |    J");
    println!("----+--------+---------+---------");
    for i in 0..kdj_values.len() {
        let idx = i + period;
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7.2}",
            idx + 1,
            closes[idx],
            sma[idx],
            kdj_values[i].j
        );
    }
}
