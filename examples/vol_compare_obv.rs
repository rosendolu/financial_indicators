use financial_indicators::obv::on_balance_volume;
use financial_indicators::vol::volume_indicator;

fn main() {
    let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8, 11.0, 10.7];
    let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0, 1500.0, 800.0];
    let vol = volume_indicator(&volumes);
    let obv = on_balance_volume(&closes, &volumes);
    println!("Day | Volume |   OBV");
    println!("----+--------+---------");
    for i in 0..volumes.len() {
        println!("{:>3} | {:>6.1} | {:>7.1}", i + 1, vol[i], obv[i]);
    }
}
