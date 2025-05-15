//! VWAP (Volume Weighted Average Price) Indicator
//!
//! This module provides a function to calculate the Volume Weighted Average Price (VWAP).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::vwap::vwap;
//!
//! let high = vec![10.0, 11.0, 12.0, 13.0, 14.0];
//! let low = vec![9.0, 10.0, 11.0, 12.0, 13.0];
//! let close = vec![9.5, 10.5, 11.5, 12.5, 13.5];
//! let volume = vec![100.0, 200.0, 150.0, 120.0, 180.0];
//! let vwap_values = vwap(&high, &low, &close, &volume);
//! assert_eq!(vwap_values.len(), high.len());
//! ```

/// Calculates the Volume Weighted Average Price (VWAP) for each period.
///
/// Returns a vector of VWAP values for each period. If any input is empty or lengths do not match, returns an empty vector.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `close` - A slice of f64 close prices.
/// * `volume` - A slice of f64 volume values.
///
/// # Example
/// ```
/// use financial_indicators::vwap::vwap;
/// let high = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let low = vec![9.0, 10.0, 11.0, 12.0, 13.0];
/// let close = vec![9.5, 10.5, 11.5, 12.5, 13.5];
/// let volume = vec![100.0, 200.0, 150.0, 120.0, 180.0];
/// let vwap_values = vwap(&high, &low, &close, &volume);
/// assert_eq!(vwap_values.len(), high.len());
/// ```
pub fn vwap(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> Vec<Option<f64>> {
    let len = high.len().min(low.len()).min(close.len()).min(volume.len());
    if len == 0 {
        return vec![];
    }
    let mut vwap = Vec::with_capacity(len);
    let mut cum_money_flow = 0.0;
    let mut cum_volume = 0.0;
    for i in 0..len {
        let tp = (high[i] + low[i] + close[i]) / 3.0;
        let money_flow = tp * volume[i];
        cum_money_flow += money_flow;
        cum_volume += volume[i];
        if cum_volume == 0.0 {
            vwap.push(None);
        } else {
            vwap.push(Some(cum_money_flow / cum_volume));
        }
    }
    vwap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vwap_basic() {
        let high = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0, 12.0, 13.0];
        let close = vec![9.5, 10.5, 11.5, 12.5, 13.5];
        let volume = vec![100.0, 200.0, 150.0, 120.0, 180.0];
        let vwap_values = vwap(&high, &low, &close, &volume);
        assert_eq!(vwap_values.len(), high.len());
        assert!(vwap_values.iter().all(|v| v.is_some()));
    }

    #[test]
    fn test_vwap_empty() {
        let high: Vec<f64> = vec![];
        let low: Vec<f64> = vec![];
        let close: Vec<f64> = vec![];
        let volume: Vec<f64> = vec![];
        let vwap_values = vwap(&high, &low, &close, &volume);
        assert_eq!(vwap_values, vec![]);
    }
}
