//! Standard Deviation (STD) Indicator
//!
//! This module provides a function to calculate the rolling standard deviation (STD) of a price series.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::std_indicator::rolling_std;
//!
//! let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
//! let period = 3;
//! let std = rolling_std(&prices, period);
//! assert_eq!(std.len(), prices.len());
//! ```

/// Calculates the rolling standard deviation (STD) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the STD,
/// or `Some(value)` for the computed STD.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The number of periods to use for the STD (commonly 20).
///
/// # Example
/// ```
/// use financial_indicators::std_indicator::rolling_std;
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let std = rolling_std(&prices, 3);
/// assert_eq!(std.len(), 5);
/// ```
pub fn rolling_std(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || prices.len() < period {
        return vec![None; prices.len()];
    }
    let mut stds = vec![None; prices.len()];
    for i in (period - 1)..prices.len() {
        let window = &prices[i + 1 - period..=i];
        let mean = window.iter().sum::<f64>() / period as f64;
        let variance = window.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / period as f64;
        stds[i] = Some(variance.sqrt());
    }
    stds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_std_basic() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let std = rolling_std(&prices, 3);
        assert_eq!(std.len(), 5);
        assert!(std[2].is_some() && std[3].is_some() && std[4].is_some());
    }

    #[test]
    fn test_rolling_std_short_input() {
        let prices = vec![1.0, 2.0];
        let std = rolling_std(&prices, 3);
        assert_eq!(std, vec![None, None]);
    }

    #[test]
    fn test_rolling_std_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let std = rolling_std(&prices, 0);
        assert_eq!(std, vec![None, None, None]);
    }
}
