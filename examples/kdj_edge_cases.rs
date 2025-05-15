use financial_indicators::kdj::KDJ;

fn main() {
    // Edge case: empty input
    let highs: Vec<f64> = vec![];
    let lows: Vec<f64> = vec![];
    let closes: Vec<f64> = vec![];
    let period = 3;
    let kdj_empty = KDJ::new(&highs, &lows, &closes, period);
    println!("KDJ with empty input: {:?}", kdj_empty);

    // Edge case: period longer than input
    let highs = vec![11.0, 12.0];
    let lows = vec![10.0, 9.0];
    let closes = vec![10.5, 11.0];
    let kdj_short = KDJ::new(&highs, &lows, &closes, 3);
    println!("KDJ with period > input length: {:?}", kdj_short);

    // Edge case: mismatched lengths (should panic)
    let highs = vec![11.0, 12.0];
    let lows = vec![10.0, 9.0];
    let closes = vec![10.5];
    println!("KDJ with mismatched lengths (should panic):");
    let result = std::panic::catch_unwind(|| {
        KDJ::new(&highs, &lows, &closes, 2);
    });
    println!("Panic occurred: {}", result.is_err());
}
