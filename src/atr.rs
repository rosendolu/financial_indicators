//! Average True Range (ATR) Indicator
//!
//! This module provides a function to calculate the Average True Range (ATR).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::atr::average_true_range;
//!
//! let high = vec![48.70, 48.72, 48.90, 48.87, 48.82];
//! let low = vec![47.79, 48.14, 48.39, 48.37, 48.24];
//! let close = vec![48.16, 48.61, 48.75, 48.63, 48.74];
//! let period = 3;
//! let atr = average_true_range(&high, &low, &close, period);
//! assert_eq!(atr.len(), high.len());
//! ```

/// Calculates the Average True Range (ATR) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the ATR,
/// or `Some(value)` for the computed ATR.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `close` - A slice of f64 close prices.
/// * `period` - The number of periods to use for the ATR (commonly 14).
///
/// # Example
/// ```
/// use financial_indicators::atr::average_true_range;
/// let high = vec![48.70, 48.72, 48.90, 48.87, 48.82];
/// let low = vec![47.79, 48.14, 48.39, 48.37, 48.24];
/// let close = vec![48.16, 48.61, 48.75, 48.63, 48.74];
/// let atr = average_true_range(&high, &low, &close, 3);
/// assert_eq!(atr.len(), 5);
/// ```
pub fn average_true_range(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> Vec<Option<f64>> {
    let len = high.len().min(low.len()).min(close.len());
    if period == 0 || len < period {
        return vec![None; len];
    }
    let mut atr = vec![None; len];
    let mut tr = Vec::with_capacity(len);
    for i in 0..len {
        let prev_close = if i == 0 { close[0] } else { close[i - 1] };
        let tr_val = (high[i] - low[i])
            .max((high[i] - prev_close).abs())
            .max((low[i] - prev_close).abs());
        tr.push(tr_val);
    }
    // First ATR value: simple average of first `period` TRs
    let first_atr = tr[..period].iter().sum::<f64>() / period as f64;
    atr[period - 1] = Some(first_atr);
    // Subsequent ATR values: smoothed
    for i in period..len {
        let prev_atr = atr[i - 1].unwrap();
        let curr_atr = (prev_atr * (period as f64 - 1.0) + tr[i]) / period as f64;
        atr[i] = Some(curr_atr);
    }
    atr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atr_basic() {
        let high = vec![48.70, 48.72, 48.90, 48.87, 48.82];
        let low = vec![47.79, 48.14, 48.39, 48.37, 48.24];
        let close = vec![48.16, 48.61, 48.75, 48.63, 48.74];
        let atr = average_true_range(&high, &low, &close, 3);
        assert_eq!(atr.len(), 5);
        assert!(atr[0].is_none() && atr[1].is_none());
        assert!(atr[2].is_some() && atr[3].is_some() && atr[4].is_some());
    }

    #[test]
    fn test_atr_short_input() {
        let high = vec![1.0, 2.0];
        let low = vec![1.0, 2.0];
        let close = vec![1.0, 2.0];
        let atr = average_true_range(&high, &low, &close, 3);
        assert_eq!(atr, vec![None, None]);
    }

    #[test]
    fn test_atr_zero_period() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![1.0, 2.0, 3.0];
        let close = vec![1.0, 2.0, 3.0];
        let atr = average_true_range(&high, &low, &close, 0);
        assert_eq!(atr, vec![None, None, None]);
    }
}
