//! Parabolic SAR (Stop and Reverse) Indicator
//!
//! This module will provide a function to calculate the Parabolic SAR indicator.
//!
//! # Examples
//!
//! ```
//! // Example usage will be added after implementation
//! ```

/// Calculates the Parabolic SAR (Stop and Reverse) indicator.
///
/// Returns a vector of SAR values for each period. The first value is always None.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `acceleration` - The acceleration factor (commonly 0.02).
/// * `max_acceleration` - The maximum acceleration factor (commonly 0.2).
///
/// # Example
/// ```
/// use financial_indicators::sar::parabolic_sar;
/// let high = vec![10.0, 10.2, 10.4, 10.3, 10.5, 10.7, 10.8];
/// let low = vec![9.7, 9.9, 10.0, 10.1, 10.2, 10.4, 10.6];
/// let sar = parabolic_sar(&high, &low, 0.02, 0.2);
/// assert_eq!(sar.len(), high.len());
/// assert!(sar[0].is_none());
/// ```
pub fn parabolic_sar(
    high: &[f64],
    low: &[f64],
    acceleration: f64,
    max_acceleration: f64,
) -> Vec<Option<f64>> {
    let len = high.len().min(low.len());
    if len < 2 {
        return vec![None; len];
    }
    let mut sar = vec![None; len];
    // Initial trend: up if close[1] > close[0], else down
    let mut uptrend = true;
    let mut af = acceleration;
    let mut ep = high[0];
    let mut sar_val = low[0];
    // Determine initial trend by comparing the first two closes
    if high[1] + low[1] < high[0] + low[0] {
        uptrend = false;
        ep = low[0];
        sar_val = high[0];
    } else {
        uptrend = true;
        ep = high[0];
        sar_val = low[0];
    }
    for i in 1..len {
        sar[i] = Some(sar_val);
        if uptrend {
            // Update EP and AF
            if high[i] > ep {
                ep = high[i];
                af = (af + acceleration).min(max_acceleration);
            }
            // Calculate next SAR
            sar_val = sar_val + af * (ep - sar_val);
            // SAR cannot be above the previous two lows
            if i >= 2 {
                sar_val = sar_val.min(low[i - 1]).min(low[i - 2]);
            } else if i >= 1 {
                sar_val = sar_val.min(low[i - 1]);
            }
            // Check for reversal
            if low[i] < sar_val {
                uptrend = false;
                sar_val = ep;
                ep = low[i];
                af = acceleration;
            }
        } else {
            // Update EP and AF
            if low[i] < ep {
                ep = low[i];
                af = (af + acceleration).min(max_acceleration);
            }
            // Calculate next SAR
            sar_val = sar_val + af * (ep - sar_val);
            // SAR cannot be below the previous two highs
            if i >= 2 {
                sar_val = sar_val.max(high[i - 1]).max(high[i - 2]);
            } else if i >= 1 {
                sar_val = sar_val.max(high[i - 1]);
            }
            // Check for reversal
            if high[i] > sar_val {
                uptrend = true;
                sar_val = ep;
                ep = high[i];
                af = acceleration;
            }
        }
    }
    sar
}
