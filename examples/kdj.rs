use financial_indicators::kdj::{calculate_kdj, OHLC};

fn main() {
    let prices = vec![
        OHLC {
            close: 10.0,
            high: 12.0,
            low: 8.0,
        },
        OHLC {
            close: 11.0,
            high: 13.0,
            low: 9.0,
        },
        OHLC {
            close: 12.0,
            high: 14.0,
            low: 10.0,
        },
        OHLC {
            close: 11.5,
            high: 13.5,
            low: 10.5,
        },
        OHLC {
            close: 13.0,
            high: 15.0,
            low: 11.0,
        },
    ];

    let (k_values, d_values, j_values) = calculate_kdj(&prices, 3);
    println!("K values: {:?}", k_values);
    println!("D values: {:?}", d_values);
    println!("J values: {:?}", j_values);
}
