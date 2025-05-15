//! Commodity Channel Index (CCI) Indicator
//!
//! This module provides a function to calculate the Commodity Channel Index (CCI).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::cci::commodity_channel_index;
//!
//! let high = vec![127.01, 127.62, 126.59, 127.35, 128.17];
//! let low = vec![125.36, 126.56, 125.07, 126.50, 126.80];
//! let close = vec![126.82, 127.07, 125.95, 127.29, 127.10];
//! let period = 3;
//! let cci = commodity_channel_index(&high, &low, &close, period);
//! assert_eq!(cci.len(), high.len());
//! ```

/// Calculates the Commodity Channel Index (CCI) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the CCI,
/// or `Some(value)` for the computed CCI.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `close` - A slice of f64 close prices.
/// * `period` - The number of periods to use for the CCI (commonly 20).
///
/// # Example
/// ```
/// use financial_indicators::cci::commodity_channel_index;
/// let high = vec![127.01, 127.62, 126.59, 127.35, 128.17];
/// let low = vec![125.36, 126.56, 125.07, 126.50, 126.80];
/// let close = vec![126.82, 127.07, 125.95, 127.29, 127.10];
/// let cci = commodity_channel_index(&high, &low, &close, 3);
/// assert_eq!(cci.len(), 5);
/// ```
pub fn commodity_channel_index(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> Vec<Option<f64>> {
    let len = high.len().min(low.len()).min(close.len());
    if period == 0 || len < period {
        return vec![None; len];
    }
    let mut cci = vec![None; len];
    let mut typical_prices = Vec::with_capacity(len);
    for i in 0..len {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        typical_prices.push(tp);
    }
    for i in (period - 1)..len {
        let window = &typical_prices[i + 1 - period..=i];
        let sma_tp: f64 = window.iter().sum::<f64>() / period as f64;
        let mean_dev: f64 =
            window.iter().map(|tp| (tp - sma_tp).abs()).sum::<f64>() / period as f64;
        if mean_dev == 0.0 {
            cci[i] = Some(0.0);
        } else {
            cci[i] = Some((typical_prices[i] - sma_tp) / (0.015 * mean_dev));
        }
    }
    cci
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cci_basic() {
        let high = vec![127.01, 127.62, 126.59, 127.35, 128.17];
        let low = vec![125.36, 126.56, 125.07, 126.50, 126.80];
        let close = vec![126.82, 127.07, 125.95, 127.29, 127.10];
        let cci = commodity_channel_index(&high, &low, &close, 3);
        assert_eq!(cci.len(), 5);
        // Only the last 3 values should be Some
        assert!(cci[0].is_none() && cci[1].is_none());
        assert!(cci[2].is_some() && cci[3].is_some() && cci[4].is_some());
    }

    #[test]
    fn test_cci_short_input() {
        let high = vec![1.0, 2.0];
        let low = vec![1.0, 2.0];
        let close = vec![1.0, 2.0];
        let cci = commodity_channel_index(&high, &low, &close, 3);
        assert_eq!(cci, vec![None, None]);
    }

    #[test]
    fn test_cci_zero_period() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![1.0, 2.0, 3.0];
        let close = vec![1.0, 2.0, 3.0];
        let cci = commodity_channel_index(&high, &low, &close, 0);
        assert_eq!(cci, vec![None, None, None]);
    }
}
