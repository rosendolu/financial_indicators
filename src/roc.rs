//! Rate of Change (ROC) Indicator
//!
//! This module provides a function to calculate the Rate of Change (ROC).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::roc::rate_of_change;
//!
//! let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61];
//! let period = 12;
//! let roc = rate_of_change(&prices, period);
//! assert_eq!(roc.len(), prices.len());
//! ```

/// Calculates the Rate of Change (ROC) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the ROC,
/// or `Some(value)` for the computed ROC.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The lookback period to use for the ROC (commonly 12 or 25).
///
/// # Example
/// ```
/// use financial_indicators::roc::rate_of_change;
/// let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61];
/// let period = 12;
/// let roc = rate_of_change(&prices, period);
/// assert_eq!(roc.len(), prices.len());
/// ```
pub fn rate_of_change(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || prices.len() <= period {
        return vec![None; prices.len()];
    }
    let mut roc = vec![None; prices.len()];
    for i in period..prices.len() {
        let prev = prices[i - period];
        if prev == 0.0 {
            roc[i] = None;
        } else {
            roc[i] = Some((prices[i] - prev) / prev * 100.0);
        }
    }
    roc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roc_basic() {
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61,
        ];
        let roc = rate_of_change(&prices, 12);
        assert!((roc[12].unwrap() - 2.864230942715372).abs() < 1e-6);
    }

    #[test]
    fn test_roc_short_input() {
        let prices = vec![1.0, 2.0, 3.0];
        let roc = rate_of_change(&prices, 12);
        assert_eq!(roc, vec![None, None, None]);
    }

    #[test]
    fn test_roc_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let roc = rate_of_change(&prices, 0);
        assert_eq!(roc, vec![None, None, None]);
    }

    #[test]
    fn test_roc_zero_prev() {
        let prices = vec![0.0, 1.0, 2.0, 3.0];
        let roc = rate_of_change(&prices, 2);
        assert_eq!(roc, vec![None, None, None, Some(200.0)]);
    }
}
