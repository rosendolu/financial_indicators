//! Bollinger Bands Indicator
//!
//! This module provides a function to calculate Bollinger Bands.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::bollinger::bollinger_bands;
//!
//! let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63];
//! let period = 20;
//! let k = 2.0;
//! let (upper, middle, lower) = bollinger_bands(&prices, period, k);
//! assert_eq!(upper.len(), prices.len());
//! ```

/// Calculates Bollinger Bands for a given period and multiplier.
///
/// Returns three vectors: upper band, middle band (SMA), and lower band.
/// Each element is `None` if there is insufficient data to compute the bands,
/// or `Some(value)` for the computed band value.
///
/// # Arguments
/// * `prices` - A slice of f64 price values.
/// * `period` - The number of periods to use for the bands (commonly 20).
/// * `k` - The number of standard deviations for the bands (commonly 2.0).
///
/// # Example
/// ```
/// use financial_indicators::bollinger::bollinger_bands;
/// let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63, 23.82, 23.87, 23.65, 23.19, 23.10, 23.33, 22.68, 23.10, 22.40, 22.17];
/// let (upper, middle, lower) = bollinger_bands(&prices, 20, 2.0);
/// assert_eq!(upper.len(), prices.len());
/// ```
pub fn bollinger_bands(
    prices: &[f64],
    period: usize,
    k: f64,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    if period == 0 || prices.len() < period {
        let none_vec = vec![None; prices.len()];
        return (none_vec.clone(), none_vec.clone(), none_vec);
    }
    let mut upper = vec![None; prices.len()];
    let mut middle = vec![None; prices.len()];
    let mut lower = vec![None; prices.len()];
    for i in (period - 1)..prices.len() {
        let window = &prices[i + 1 - period..=i];
        let sma = window.iter().sum::<f64>() / period as f64;
        let std = (window.iter().map(|p| (p - sma).powi(2)).sum::<f64>() / period as f64).sqrt();
        upper[i] = Some(sma + k * std);
        middle[i] = Some(sma);
        lower[i] = Some(sma - k * std);
    }
    (upper, middle, lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bollinger_bands_basic() {
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39,
            22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63, 23.82, 23.87, 23.65, 23.19,
            23.10, 23.33, 22.68, 23.10, 22.40, 22.17,
        ];
        let (upper, middle, lower) = bollinger_bands(&prices, 20, 2.0);
        assert_eq!(upper.len(), prices.len());
        assert!(upper[19].is_some() && middle[19].is_some() && lower[19].is_some());
    }

    #[test]
    fn test_bollinger_bands_short_input() {
        let prices = vec![1.0, 2.0, 3.0];
        let (upper, middle, lower) = bollinger_bands(&prices, 20, 2.0);
        assert_eq!(upper, vec![None, None, None]);
        assert_eq!(middle, vec![None, None, None]);
        assert_eq!(lower, vec![None, None, None]);
    }

    #[test]
    fn test_bollinger_bands_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let (upper, middle, lower) = bollinger_bands(&prices, 0, 2.0);
        assert_eq!(upper, vec![None, None, None]);
        assert_eq!(middle, vec![None, None, None]);
        assert_eq!(lower, vec![None, None, None]);
    }
}
