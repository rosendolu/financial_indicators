//! Moving Average (MA) Indicator
//!
//! This module provides functions to calculate Simple Moving Average (SMA) and Weighted Moving Average (WMA).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::ma::{simple_moving_average, weighted_moving_average};
//!
//! let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
//! let period = 3;
//! let sma = simple_moving_average(&prices, period);
//! assert_eq!(sma.len(), prices.len());
//!
//! let weights = vec![1.0, 2.0, 3.0];
//! let wma = weighted_moving_average(&prices, &weights);
//! assert_eq!(wma.len(), prices.len());
//! ```

/// Calculates the Simple Moving Average (SMA) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the average,
/// or `Some(value)` for the computed SMA.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The number of periods to use for the moving average.
///
/// # Example
/// ```
/// use financial_indicators::ma::simple_moving_average;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let sma = simple_moving_average(&prices, 3);
/// assert_eq!(sma, vec![None, None, Some(11.0), Some(12.0), Some(13.0)]);
/// ```
pub fn simple_moving_average(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 {
        return vec![None; prices.len()];
    }
    let mut sma = Vec::with_capacity(prices.len());
    for i in 0..prices.len() {
        if i + 1 < period {
            sma.push(None);
        } else {
            let sum: f64 = prices[i + 1 - period..=i].iter().sum();
            sma.push(Some(sum / period as f64));
        }
    }
    sma
}

/// Calculates the Weighted Moving Average (WMA) for a given set of weights.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the average,
/// or `Some(value)` for the computed WMA.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `weights` - A slice of f64 weights (most recent price gets the last weight).
///
/// # Example
/// ```
/// use financial_indicators::ma::weighted_moving_average;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let weights = vec![1.0, 2.0, 3.0];
/// let wma = weighted_moving_average(&prices, &weights);
/// assert_eq!(wma, vec![None, None, Some((10.0*1.0 + 11.0*2.0 + 12.0*3.0)/6.0), Some((11.0*1.0 + 12.0*2.0 + 13.0*3.0)/6.0), Some((12.0*1.0 + 13.0*2.0 + 14.0*3.0)/6.0)]);
/// ```
pub fn weighted_moving_average(prices: &[f64], weights: &[f64]) -> Vec<Option<f64>> {
    let period = weights.len();
    if period == 0 {
        return vec![None; prices.len()];
    }
    let weight_sum: f64 = weights.iter().sum();
    let mut wma = Vec::with_capacity(prices.len());
    for i in 0..prices.len() {
        if i + 1 < period {
            wma.push(None);
        } else {
            let mut sum = 0.0;
            for j in 0..period {
                sum += prices[i + 1 - period + j] * weights[j];
            }
            wma.push(Some(sum / weight_sum));
        }
    }
    wma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average() {
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let sma = simple_moving_average(&prices, 3);
        assert_eq!(sma, vec![None, None, Some(11.0), Some(12.0), Some(13.0)]);
    }

    #[test]
    fn test_weighted_moving_average() {
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let weights = vec![1.0, 2.0, 3.0];
        let wma = weighted_moving_average(&prices, &weights);
        let expected = vec![
            None,
            None,
            Some((10.0 * 1.0 + 11.0 * 2.0 + 12.0 * 3.0) / 6.0),
            Some((11.0 * 1.0 + 12.0 * 2.0 + 13.0 * 3.0) / 6.0),
            Some((12.0 * 1.0 + 13.0 * 2.0 + 14.0 * 3.0) / 6.0),
        ];
        for (a, b) in wma.iter().zip(expected.iter()) {
            match (a, b) {
                (Some(x), Some(y)) => assert!((x - y).abs() < 1e-8),
                (None, None) => {}
                _ => panic!("Mismatch in WMA output"),
            }
        }
    }
}
