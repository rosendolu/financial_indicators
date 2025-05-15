use financial_indicators::trix::trix;

fn main() {
    let closes: Vec<f64> = (1..=30).map(|x| x as f64).collect();
    let period = 15;
    let trix_values = trix(&closes, period);
    println!("Idx |  Close |   TRIX");
    println!("----+--------+---------");
    for i in 0..closes.len() {
        println!("{:>3} | {:>6.2} | {:>7?}", i + 1, closes[i], trix_values[i]);
    }
}
