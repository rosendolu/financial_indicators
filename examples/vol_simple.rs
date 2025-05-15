use financial_indicators::vol::volume_indicator;

fn main() {
    let volume = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
    let vol = volume_indicator(&volume);
    println!("Idx | Volume");
    println!("----+--------");
    for (i, v) in vol.iter().enumerate() {
        println!("{:>3} | {:>6.1}", i + 1, v);
    }
}
