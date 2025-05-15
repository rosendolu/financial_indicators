use financial_indicators::dmi::dmi_adx;

fn main() {
    // Edge case: empty input
    let high: Vec<f64> = vec![];
    let low: Vec<f64> = vec![];
    let close: Vec<f64> = vec![];
    let period = 14;
    let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, period);
    println!(
        "DMI/ADX with empty input: {:?} {:?} {:?}",
        plus_di, minus_di, adx
    );

    // Edge case: period longer than input
    let high = vec![30.0, 32.0];
    let low = vec![28.0, 29.0];
    let close = vec![29.0, 31.0];
    let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, 5);
    println!(
        "DMI/ADX with period > input length: {:?} {:?} {:?}",
        plus_di, minus_di, adx
    );

    // Edge case: mismatched lengths (should not panic, will use min length)
    let high = vec![30.0, 32.0, 31.0];
    let low = vec![28.0, 29.0];
    let close = vec![29.0, 31.0, 30.5];
    let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, 2);
    println!(
        "DMI/ADX with mismatched lengths: {:?} {:?} {:?}",
        plus_di, minus_di, adx
    );
}
