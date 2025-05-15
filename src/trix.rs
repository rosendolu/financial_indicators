//! TRIX (Triple Exponential Average) Indicator
//!
//! This module provides a function to calculate the TRIX indicator.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::trix::trix;
//!
//! let closes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
//! let period = 15;
//! let trix_values = trix(&closes, period);
//! assert_eq!(trix_values.len(), closes.len());
//! ```

/// Calculates the TRIX (Triple Exponential Average) indicator for a given period.
///
/// Returns a vector where each element is `None` if there is insufficient data to compute the TRIX,
/// or `Some(value)` for the computed TRIX.
///
/// # Arguments
/// * `closes` - A slice of f64 closing prices.
/// * `period` - The number of periods to use for the EMA (commonly 15).
///
/// # Example
/// ```
/// use financial_indicators::trix::trix;
/// let closes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
/// let period = 15;
/// let trix_values = trix(&closes, period);
/// assert_eq!(trix_values.len(), closes.len());
/// ```
pub fn trix(closes: &[f64], period: usize) -> Vec<Option<f64>> {
    if period == 0 || closes.len() < period * 3 {
        return vec![None; closes.len()];
    }
    // Helper for EMA
    fn ema(data: &[f64], period: usize) -> Vec<f64> {
        let mut ema = Vec::with_capacity(data.len());
        let alpha = 2.0 / (period as f64 + 1.0);
        let mut prev_ema = data[0];
        for &price in data.iter() {
            prev_ema = alpha * price + (1.0 - alpha) * prev_ema;
            ema.push(prev_ema);
        }
        ema
    }
    // 1st EMA
    let ema1 = ema(closes, period);
    // 2nd EMA
    let ema2 = ema(&ema1, period);
    // 3rd EMA
    let ema3 = ema(&ema2, period);
    let mut trix = vec![None; closes.len()];
    for i in 1..closes.len() {
        if i >= period * 3 - 2 {
            let prev = ema3[i - 1];
            let curr = ema3[i];
            if prev != 0.0 {
                trix[i] = Some((curr - prev) / prev * 100.0);
            } else {
                trix[i] = Some(0.0);
            }
        }
    }
    trix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trix_basic() {
        let closes: Vec<f64> = (1..=30).map(|x| x as f64).collect();
        let period = 15;
        let trix_values = trix(&closes, period);
        assert_eq!(trix_values.len(), closes.len());
        // Only the last values should be Some
        assert!(trix_values.iter().take(period * 3 - 2).all(|v| v.is_none()));
    }

    #[test]
    fn test_trix_short_input() {
        let closes = vec![1.0, 2.0, 3.0];
        let period = 15;
        let trix_values = trix(&closes, period);
        assert_eq!(trix_values, vec![None, None, None]);
    }
}
