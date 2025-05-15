//! Exponential Moving Average (EMA) Indicator
//!
//! This module provides a function to calculate the Exponential Moving Average (EMA).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::ema::exponential_moving_average;
//!
//! let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
//! let period = 3;
//! let ema = exponential_moving_average(&prices, period);
//! assert_eq!(ema.len(), prices.len());
//! ```

/// Calculates the Exponential Moving Average (EMA) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the average,
/// or `Some(value)` for the computed EMA.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The number of periods to use for the EMA.
///
/// # Example
/// ```
/// use financial_indicators::ema::exponential_moving_average;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let ema = exponential_moving_average(&prices, 3);
/// assert_eq!(ema, vec![Some(10.0), Some(10.5), Some(11.25), Some(12.125), Some(13.0625)]);
/// ```
pub fn exponential_moving_average(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || prices.is_empty() {
        return vec![None; prices.len()];
    }
    let alpha = 2.0 / (period as f64 + 1.0);
    let mut ema = Vec::with_capacity(prices.len());
    let mut prev_ema = None;
    for (i, &price) in prices.iter().enumerate() {
        let value = if i == 0 {
            price
        } else {
            let prev = prev_ema.unwrap();
            alpha * price + (1.0 - alpha) * prev
        };
        prev_ema = Some(value);
        ema.push(Some(value));
    }
    ema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_moving_average() {
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let ema = exponential_moving_average(&prices, 3);
        let expected = vec![
            Some(10.0),
            Some(10.5),
            Some(11.25),
            Some(12.125),
            Some(13.0625),
        ];
        for (a, b) in ema.iter().zip(expected.iter()) {
            match (a, b) {
                (Some(x), Some(y)) => assert!((x - y).abs() < 1e-8),
                (None, None) => {}
                _ => panic!("Mismatch in EMA output"),
            }
        }
    }

    #[test]
    fn test_ema_empty() {
        let prices: Vec<f64> = vec![];
        let ema = exponential_moving_average(&prices, 3);
        assert_eq!(ema, vec![]);
    }

    #[test]
    fn test_ema_zero_period() {
        let prices = vec![10.0, 11.0, 12.0];
        let ema = exponential_moving_average(&prices, 0);
        assert_eq!(ema, vec![None, None, None]);
    }
}
