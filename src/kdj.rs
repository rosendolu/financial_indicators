//! The `kdj` module provides functionality to calculate the KDJ indicator for stock prices.
//!
//!  The KDJ indicator consists of three components:
//! - K: Stochastic indicator
//! - D: Oscillator
//! - J: Derived from K and D
//!
//! # Example
//!
//! ```
//! use financial_indicators::kdj::{calculate_kdj, OHLC};
//!
//! let prices = vec![
//!     OHLC { close: 10.0, high: 12.0, low: 8.0 },
//!     OHLC { close: 11.0, high: 13.0, low: 9.0 },
//!     OHLC { close: 12.0, high: 14.0, low: 10.0 },
//!     OHLC { close: 11.5, high: 13.5, low: 10.5 },
//!     OHLC { close: 13.0, high: 15.0, low: 11.0 },
//! ];
//!
//! let (k_values, d_values, j_values) = calculate_kdj(&prices, 3);
//! println!("K values: {:?}", k_values);
//! println!("D values: {:?}", d_values);
//! println!("J values: {:?}", j_values);
//! ```
//!

/// Represents the open-high-low-close (OHLC) data for a single trading period.
#[derive(Debug)]
pub struct OHLC {
    pub close: f64,
    pub high: f64,
    pub low: f64,
}

/// Calculates the KDJ indicator for a given set of OHLC data over a specified period.
///
/// # Parameters
///
/// - `prices`: A slice of OHLC data points.
/// - `period`: The number of periods to use for the calculation.
///
/// # Returns
///
/// A tuple containing three vectors: K values, D values, and J values.
///
/// # Example
///
/// ```
/// use financial_indicators::kdj::{calculate_kdj, OHLC};
///
/// let prices = vec![
///     OHLC { close: 10.0, high: 12.0, low: 8.0 },
///     OHLC { close: 11.0, high: 13.0, low: 9.0 },
///     OHLC { close: 12.0, high: 14.0, low: 10.0 },
///     OHLC { close: 11.5, high: 13.5, low: 10.5 },
///     OHLC { close: 13.0, high: 15.0, low: 11.0 },
/// ];
///
/// let (k_values, d_values, j_values) = calculate_kdj(&prices, 3);
/// assert_eq!(k_values.len(), 5);
/// assert_eq!(d_values.len(), 5);
/// assert_eq!(j_values.len(), 5);
/// ```

pub fn calculate_kdj(prices: &[OHLC], period: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut k_values = vec![];
    let mut d_values = vec![];
    let mut j_values = vec![];
    let mut k = 50.0;
    let mut d = 50.0;

    for i in 0..prices.len() {
        if i >= period - 1 {
            let low = prices[i + 1 - period..=i]
                .iter()
                .map(|p| p.low)
                .fold(f64::INFINITY, f64::min);
            let high = prices[i + 1 - period..=i]
                .iter()
                .map(|p| p.high)
                .fold(f64::NEG_INFINITY, f64::max);
            let close = prices[i].close;

            let rsv = if high != low {
                (close - low) / (high - low) * 100.0
            } else {
                50.0
            };

            k = (2.0 / 3.0) * k + (1.0 / 3.0) * rsv;
            d = (2.0 / 3.0) * d + (1.0 / 3.0) * k;
            let j = 3.0 * k - 2.0 * d;

            k_values.push(k);
            d_values.push(d);
            j_values.push(j);
        } else {
            k_values.push(k);
            d_values.push(d);
            j_values.push(3.0 * k - 2.0 * d);
        }
    }

    (k_values, d_values, j_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdj_calculation() {
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

        assert_eq!(k_values.len(), 5);
        assert_eq!(d_values.len(), 5);
        assert_eq!(j_values.len(), 5);

        println!("K values: {:?}", k_values);
        println!("D values: {:?}", d_values);
        println!("J values: {:?}", j_values);
    }
}
