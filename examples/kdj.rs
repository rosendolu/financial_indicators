use financial_indicators::kdj::KDJ;
fn main() {
    // Define example data for highs, lows, and closes
    let highs = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];
    let lows = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let closes = vec![10.5, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];

    // Calculate KDJ with a period of 3
    let kdj_values = KDJ::new(&highs, &lows, &closes, 3);

    // Print the KDJ values
    println!("KDJ Values for the given period:");
    for (i, kdj) in kdj_values.iter().enumerate() {
        println!(
            "Period {}: K: {:.2}, D: {:.2}, J: {:.2}",
            i + 1,
            kdj.k,
            kdj.d,
            kdj.j
        );
    }
}
