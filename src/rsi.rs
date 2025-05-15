//! Relative Strength Index (RSI) Indicator
//!
//! This module provides a function to calculate the Relative Strength Index (RSI).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::rsi::relative_strength_index;
//!
//! let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28, 46.28];
//! let period = 14;
//! let rsi = relative_strength_index(&prices, period);
//! assert_eq!(rsi.len(), prices.len());
//! ```

/// Calculates the Relative Strength Index (RSI) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the RSI,
/// or `Some(value)` for the computed RSI.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The number of periods to use for the RSI (commonly 14).
///
/// # Example
/// ```
/// use financial_indicators::rsi::relative_strength_index;
/// let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28, 46.28];
/// let rsi = relative_strength_index(&prices, 14);
/// assert_eq!(rsi[14], Some(70.46413502109705));
/// ```
pub fn relative_strength_index(prices: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || prices.len() <= period {
        return vec![None; prices.len()];
    }
    let mut rsi = vec![None; prices.len()];
    let mut gains = 0.0;
    let mut losses = 0.0;
    // Calculate initial average gain/loss
    for i in 1..=period {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains += change;
        } else {
            losses -= change;
        }
    }
    let mut avg_gain = gains / period as f64;
    let mut avg_loss = losses / period as f64;
    // First RSI value
    if avg_loss == 0.0 {
        rsi[period] = Some(100.0);
    } else {
        let rs = avg_gain / avg_loss;
        rsi[period] = Some(100.0 - 100.0 / (1.0 + rs));
    }
    // Subsequent RSI values
    for i in (period + 1)..prices.len() {
        let change = prices[i] - prices[i - 1];
        let gain = if change > 0.0 { change } else { 0.0 };
        let loss = if change < 0.0 { -change } else { 0.0 };
        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;
        if avg_loss == 0.0 {
            rsi[i] = Some(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi[i] = Some(100.0 - 100.0 / (1.0 + rs));
        }
    }
    rsi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_example() {
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28,
        ];
        let rsi = relative_strength_index(&prices, 14);
        assert!((rsi[14].unwrap() - 70.46413502109705).abs() < 1e-6);
    }

    #[test]
    fn test_rsi_short_input() {
        let prices = vec![1.0, 2.0, 3.0];
        let rsi = relative_strength_index(&prices, 14);
        assert_eq!(rsi, vec![None, None, None]);
    }

    #[test]
    fn test_rsi_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let rsi = relative_strength_index(&prices, 0);
        assert_eq!(rsi, vec![None, None, None]);
    }
}
