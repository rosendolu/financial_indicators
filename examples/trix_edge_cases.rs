use financial_indicators::trix::trix;

fn main() {
    // Edge case: empty input
    let closes: Vec<f64> = vec![];
    let period = 15;
    let trix_empty = trix(&closes, period);
    println!("TRIX with empty input: {:?}", trix_empty);

    // Edge case: period longer than input
    let closes = vec![1.0, 2.0, 3.0];
    let trix_short = trix(&closes, 10);
    println!("TRIX with period > input length: {:?}", trix_short);

    // Edge case: constant prices
    let closes = vec![100.0; 30];
    let trix_constant = trix(&closes, 15);
    println!("TRIX with constant prices:");
    for (i, val) in trix_constant.iter().enumerate() {
        println!("Idx {}: {:?}", i + 1, val);
    }
}
