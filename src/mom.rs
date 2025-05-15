//! Momentum (MOM) Indicator
//!
//! This module provides a function to calculate the Momentum (MOM) indicator.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::mom::momentum;
//!
//! let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89];
//! let period = 10;
//! let mom = momentum(&prices, period);
//! assert_eq!(mom.len(), prices.len());
//! ```

/// Calculates the Momentum (MOM) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the MOM,
/// or `Some(value)` for the computed MOM.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The lookback period to use for the MOM (commonly 10 or 14).
///
/// # Example
/// ```
/// use financial_indicators::mom::momentum;
/// let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89];
/// let period = 10;
/// let mom = momentum(&prices, period);
/// assert_eq!(mom.len(), prices.len());
/// ```
pub fn momentum(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || prices.len() <= period {
        return vec![None; prices.len()];
    }
    let mut mom = vec![None; prices.len()];
    for i in period..prices.len() {
        mom[i] = Some(prices[i] - prices[i - period]);
    }
    mom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mom_basic() {
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89,
        ];
        let mom = momentum(&prices, 10);
        assert!((mom[10].unwrap() - 1.55).abs() < 1e-6);
    }

    #[test]
    fn test_mom_short_input() {
        let prices = vec![1.0, 2.0, 3.0];
        let mom = momentum(&prices, 10);
        assert_eq!(mom, vec![None, None, None]);
    }

    #[test]
    fn test_mom_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let mom = momentum(&prices, 0);
        assert_eq!(mom, vec![None, None, None]);
    }
}
