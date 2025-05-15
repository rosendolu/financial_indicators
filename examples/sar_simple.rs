use financial_indicators::sar::parabolic_sar;

fn main() {
    let high = vec![10.0, 10.2, 10.4, 10.3, 10.5, 10.7, 10.8];
    let low = vec![9.7, 9.9, 10.0, 10.1, 10.2, 10.4, 10.6];
    let sar = parabolic_sar(&high, &low, 0.02, 0.2);
    println!("Idx |   High |    Low |    SAR");
    println!("----+--------+--------+--------");
    for i in 0..high.len() {
        println!(
            "{:>3} | {:>6.2} | {:>6.2} | {:>6.3?}",
            i + 1,
            high[i],
            low[i],
            sar[i]
        );
    }
}
