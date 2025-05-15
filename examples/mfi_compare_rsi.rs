use financial_indicators::mfi::money_flow_index;
use financial_indicators::rsi::relative_strength_index;

fn main() {
    let high = vec![
        127.01, 127.62, 126.59, 127.35, 128.17, 127.75, 128.09, 127.29, 128.47, 128.09, 127.60,
        128.21, 127.70, 128.45, 128.73,
    ];
    let low = vec![
        125.36, 126.56, 125.07, 126.50, 126.80, 126.09, 126.32, 125.00, 126.80, 126.32, 126.09,
        126.80, 126.32, 126.80, 127.00,
    ];
    let close = vec![
        126.82, 127.07, 125.95, 127.29, 127.10, 127.29, 127.93, 126.82, 127.60, 127.93, 127.60,
        127.93, 127.60, 127.93, 128.23,
    ];
    let volume = vec![
        1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0,
        2100.0, 2200.0, 2300.0, 2400.0,
    ];
    let period = 14;
    let mfi = money_flow_index(&high, &low, &close, &volume, period);
    let rsi = relative_strength_index(&close, period);
    println!("Idx |  Close |   RSI   |   MFI");
    println!("----+--------+---------+---------");
    for i in 0..close.len() {
        println!(
            "{:>3} | {:>6.2} | {:>7?} | {:>7?}",
            i + 1,
            close[i],
            rsi[i],
            mfi[i]
        );
    }
}
