# Financial Indicators ![Last Commit](https://badgen.net/github/last-commit/rosendolu/financial_indicators?label=🟣%20Updated&labelColor=black&color=448AFF&now=2024-09-05-13:49:47)

[![Crates.io](https://img.shields.io/crates/v/financial_indicators.svg)](https://crates.io/crates/financial_indicators)
[![Documentation](https://docs.rs/financial_indicators/badge.svg)](https://docs.rs/financial_indicators)

A Rust library providing various financial algorithms, including but not limited to KDJ and MACD indicators, for technical analysis.

## Features

- Easy integration with existing projects
- Well-documented and tested

### Indicators:

#### 1. Trend Indicators

- MA (Moving Average)
- EMA (Exponential Moving Average)
- MACD (Moving Average Convergence Divergence)
- DMI/ADX (Directional Movement Index / Average Directional Index)
- TRIX (Triple Exponential Average)

#### 2. Momentum Indicators

- RSI (Relative Strength Index)
- ROC (Rate of Change)
- CCI (Commodity Channel Index)
- Stochastic Oscillator (KDJ)
- MOM (Momentum)

#### 3. Volatility Indicators

- ATR (Average True Range)
- Bollinger Bands
- STD (Standard Deviation)

#### 4. Volume Indicators

- OBV (On-Balance Volume)
- VOL (Volume)
- MFI (Money Flow Index)
- VWAP (Volume Weighted Average Price)

#### 5. Other Common Indicators

- SAR (Parabolic SAR)
- PSY (Psychological Line)
- Williams %R
- CR (Cumulative Resistance)

## Installation

Add `financial_indicators` to your `Cargo.toml`:

```sh
cargo add financial_indicators
```

Or manual add to `cargo.toml`

```toml
[dependencies]
financial_indicators = "0.1.0"
```

## Usage

Here's an example of how to use the library to calculate KDJ indicators:

```rust
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
```

### Examples

[examples](https://github.com/rosendolu/financial_indicators/tree/main/examples)

### Indicators Docs

[indicator anatomy](https://github.com/rosendolu/financial_indicators/tree/main/docs)

## Contributing

We welcome contributions!👏

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
