//! DMI / ADX (Directional Movement Index / Average Directional Index) Indicator
//!
//! This module provides functions to calculate the +DI, -DI, and ADX indicators.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::dmi::dmi_adx;
//!
//! let high = vec![30.0, 32.0, 31.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0];
//! let low = vec![28.0, 29.0, 29.5, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0];
//! let close = vec![29.0, 31.0, 30.5, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0];
//! let period = 14;
//! let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, period);
//! assert_eq!(plus_di.len(), high.len());
//! assert_eq!(minus_di.len(), high.len());
//! assert_eq!(adx.len(), high.len());
//! ```

/// Calculates the +DI, -DI, and ADX indicators for a given period.
///
/// Returns three vectors: +DI, -DI, and ADX. Each element is `None` if there is insufficient data to compute the value,
/// or `Some(value)` for the computed indicator.
///
/// # Arguments
/// * `high` - A slice of f64 high prices.
/// * `low` - A slice of f64 low prices.
/// * `close` - A slice of f64 close prices.
/// * `period` - The number of periods to use for the calculation (commonly 14).
///
/// # Example
/// ```
/// use financial_indicators::dmi::dmi_adx;
/// let high = vec![30.0, 32.0, 31.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0];
/// let low = vec![28.0, 29.0, 29.5, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0];
/// let close = vec![29.0, 31.0, 30.5, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0];
/// let period = 14;
/// let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, period);
/// assert_eq!(plus_di.len(), high.len());
/// assert_eq!(minus_di.len(), high.len());
/// assert_eq!(adx.len(), high.len());
/// ```
pub fn dmi_adx(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>) {
    let len = high.len().min(low.len()).min(close.len());
    if period == 0 || len < period + 1 {
        return (vec![None; len], vec![None; len], vec![None; len]);
    }
    let mut tr = vec![0.0; len];
    let mut plus_dm = vec![0.0; len];
    let mut minus_dm = vec![0.0; len];
    for i in 1..len {
        let up_move = high[i] - high[i - 1];
        let down_move = low[i - 1] - low[i];
        plus_dm[i] = if up_move > down_move && up_move > 0.0 {
            up_move
        } else {
            0.0
        };
        minus_dm[i] = if down_move > up_move && down_move > 0.0 {
            down_move
        } else {
            0.0
        };
        let tr1 = high[i] - low[i];
        let tr2 = (high[i] - close[i - 1]).abs();
        let tr3 = (low[i] - close[i - 1]).abs();
        tr[i] = tr1.max(tr2).max(tr3);
    }
    // Wilder's smoothing
    let mut smoothed_tr = vec![None; len];
    let mut smoothed_plus_dm = vec![None; len];
    let mut smoothed_minus_dm = vec![None; len];
    let mut plus_di = vec![None; len];
    let mut minus_di = vec![None; len];
    let mut dx = vec![None; len];
    // Initial sums
    let tr_sum: f64 = tr[1..=period].iter().sum();
    let plus_dm_sum: f64 = plus_dm[1..=period].iter().sum();
    let minus_dm_sum: f64 = minus_dm[1..=period].iter().sum();
    smoothed_tr[period] = Some(tr_sum);
    smoothed_plus_dm[period] = Some(plus_dm_sum);
    smoothed_minus_dm[period] = Some(minus_dm_sum);
    plus_di[period] = Some(100.0 * plus_dm_sum / tr_sum);
    minus_di[period] = Some(100.0 * minus_dm_sum / tr_sum);
    dx[period] = match (plus_di[period], minus_di[period]) {
        (Some(pdi), Some(mdi)) if pdi + mdi != 0.0 => Some(100.0 * (pdi - mdi).abs() / (pdi + mdi)),
        _ => None,
    };
    // Wilder's smoothing for subsequent values
    for i in (period + 1)..len {
        smoothed_tr[i] = Some(
            smoothed_tr[i - 1].unwrap() - (smoothed_tr[i - 1].unwrap() / period as f64) + tr[i],
        );
        smoothed_plus_dm[i] = Some(
            smoothed_plus_dm[i - 1].unwrap() - (smoothed_plus_dm[i - 1].unwrap() / period as f64)
                + plus_dm[i],
        );
        smoothed_minus_dm[i] = Some(
            smoothed_minus_dm[i - 1].unwrap() - (smoothed_minus_dm[i - 1].unwrap() / period as f64)
                + minus_dm[i],
        );
        let tr_val = smoothed_tr[i].unwrap();
        let plus_dm_val = smoothed_plus_dm[i].unwrap();
        let minus_dm_val = smoothed_minus_dm[i].unwrap();
        plus_di[i] = Some(100.0 * plus_dm_val / tr_val);
        minus_di[i] = Some(100.0 * minus_dm_val / tr_val);
        dx[i] = match (plus_di[i], minus_di[i]) {
            (Some(pdi), Some(mdi)) if pdi + mdi != 0.0 => {
                Some(100.0 * (pdi - mdi).abs() / (pdi + mdi))
            }
            _ => None,
        };
    }
    // Calculate ADX
    let mut adx = vec![None; len];
    // First ADX value: average of first 'period' DX values (starting from period)
    if len > period * 2 {
        let mut adx_sum = 0.0;
        let mut count = 0;
        for i in (period + 1)..=(period * 2) {
            if let Some(val) = dx[i] {
                adx_sum += val;
                count += 1;
            }
        }
        if count == period {
            adx[period * 2] = Some(adx_sum / period as f64);
            // Wilder's smoothing for subsequent ADX values
            for i in (period * 2 + 1)..len {
                if let (Some(prev_adx), Some(dx_val)) = (adx[i - 1], dx[i]) {
                    adx[i] = Some(((prev_adx * (period as f64 - 1.0)) + dx_val) / period as f64);
                }
            }
        }
    }
    (plus_di, minus_di, adx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmi_adx_basic() {
        let high = vec![
            30.0, 32.0, 31.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0,
            44.0,
        ];
        let low = vec![
            28.0, 29.0, 29.5, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0,
            41.0,
        ];
        let close = vec![
            29.0, 31.0, 30.5, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0,
            43.0,
        ];
        let period = 14;
        let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, period);
        assert_eq!(plus_di.len(), high.len());
        assert_eq!(minus_di.len(), high.len());
        assert_eq!(adx.len(), high.len());
    }

    #[test]
    fn test_dmi_adx_short_input() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![1.0, 2.0, 3.0];
        let close = vec![1.0, 2.0, 3.0];
        let period = 14;
        let (plus_di, minus_di, adx) = dmi_adx(&high, &low, &close, period);
        assert_eq!(plus_di, vec![None, None, None]);
        assert_eq!(minus_di, vec![None, None, None]);
        assert_eq!(adx, vec![None, None, None]);
    }
}
