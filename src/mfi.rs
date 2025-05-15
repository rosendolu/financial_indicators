//! MFI (Money Flow Index) Indicator
//!
//! This module provides a function to calculate the Money Flow Index (MFI).
//!
//! # Examples
//!
//! ```
//! use financial_indicators::mfi::money_flow_index;
//!
//! let high = vec![127.01, 127.62, 126.59, 127.35, 128.17, 127.75, 128.09, 127.29, 128.47, 128.09, 127.60, 128.21, 127.70, 128.45, 128.73];
//! let low = vec![125.36, 126.56, 125.07, 126.50, 126.80, 126.09, 126.32, 125.00, 126.80, 126.32, 126.09, 126.80, 126.32, 126.80, 127.00];
//! let close = vec![126.82, 127.07, 125.95, 127.29, 127.10, 127.29, 127.93, 126.82, 127.60, 127.93, 127.60, 127.93, 127.60, 127.93, 128.23];
//! let volume = vec![1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0, 2100.0, 2200.0, 2300.0, 2400.0];
//! let period = 14;
//! let mfi = money_flow_index(&high, &low, &close, &volume, period);
//! assert_eq!(mfi.len(), high.len());
//! ```

/// Calculates the Money Flow Index (MFI) for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the MFI,
/// or `Some(value)` for the computed MFI.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `close` - A slice of f64 close prices.
/// * `volume` - A slice of f64 volume values.
/// * `period` - The number of periods to use for the MFI (commonly 14).
///
/// # Example
/// ```
/// use financial_indicators::mfi::money_flow_index;
/// let high = vec![127.01, 127.62, 126.59, 127.35, 128.17, 127.75, 128.09, 127.29, 128.47, 128.09, 127.60, 128.21, 127.70, 128.45, 128.73];
/// let low = vec![125.36, 126.56, 125.07, 126.50, 126.80, 126.09, 126.32, 125.00, 126.80, 126.32, 126.09, 126.80, 126.32, 126.80, 127.00];
/// let close = vec![126.82, 127.07, 125.95, 127.29, 127.10, 127.29, 127.93, 126.82, 127.60, 127.93, 127.60, 127.93, 127.60, 127.93, 128.23];
/// let volume = vec![1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0, 2100.0, 2200.0, 2300.0, 2400.0];
/// let period = 14;
/// let mfi = money_flow_index(&high, &low, &close, &volume, period);
/// assert_eq!(mfi.len(), high.len());
/// ```
pub fn money_flow_index(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    period: usize,
) -> Vec<Option<f64>> {
    let len = high.len().min(low.len()).min(close.len()).min(volume.len());
    if period == 0 || len < period + 1 {
        return vec![None; len];
    }
    let mut tp = Vec::with_capacity(len);
    for i in 0..len {
        tp.push((high[i] + low[i] + close[i]) / 3.0);
    }
    let mut raw_money_flow = Vec::with_capacity(len);
    for i in 0..len {
        raw_money_flow.push(tp[i] * volume[i]);
    }
    let mut mfi = vec![None; len];
    for i in period..len {
        let mut pos_flow = 0.0;
        let mut neg_flow = 0.0;
        for j in (i - period + 1)..=i {
            if tp[j] > tp[j - 1] {
                pos_flow += raw_money_flow[j];
            } else if tp[j] < tp[j - 1] {
                neg_flow += raw_money_flow[j];
            }
        }
        if neg_flow == 0.0 {
            mfi[i] = Some(100.0);
        } else {
            let money_flow_ratio = pos_flow / neg_flow;
            mfi[i] = Some(100.0 - 100.0 / (1.0 + money_flow_ratio));
        }
    }
    mfi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfi_basic() {
        let high = vec![
            127.01, 127.62, 126.59, 127.35, 128.17, 127.75, 128.09, 127.29, 128.47, 128.09, 127.60,
            128.21, 127.70, 128.45, 128.73,
        ];
        let low = vec![
            125.36, 126.56, 125.07, 126.50, 126.80, 126.09, 126.32, 125.00, 126.80, 126.32, 126.09,
            126.80, 126.32, 126.80, 127.00,
        ];
        let close = vec![
            126.82, 127.07, 125.95, 127.29, 127.10, 127.29, 127.93, 126.82, 127.60, 127.93, 127.60,
            127.93, 127.60, 127.93, 128.23,
        ];
        let volume = vec![
            1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0,
            2100.0, 2200.0, 2300.0, 2400.0,
        ];
        let period = 14;
        let mfi = money_flow_index(&high, &low, &close, &volume, period);
        assert_eq!(mfi.len(), high.len());
        // Only the last value should be Some
        assert!(mfi[..period].iter().all(|v| v.is_none()));
        assert!(mfi[period..].iter().all(|v| v.is_some()));
    }

    #[test]
    fn test_mfi_short_input() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![1.0, 2.0, 3.0];
        let close = vec![1.0, 2.0, 3.0];
        let volume = vec![100.0, 200.0, 300.0];
        let period = 14;
        let mfi = money_flow_index(&high, &low, &close, &volume, period);
        assert_eq!(mfi, vec![None, None, None]);
    }
}
