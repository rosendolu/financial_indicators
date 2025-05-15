//! On-Balance Volume (OBV) Indicator
//!
//! This module provides a function to calculate the On-Balance Volume (OBV) indicator.
//!
//! # Examples
//!
//! ```
//! use financial_indicators::obv::on_balance_volume;
//!
//! let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8];
//! let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
//! let obv = on_balance_volume(&closes, &volumes);
//! assert_eq!(obv, vec![0.0, 1200.0, 100.0, 1400.0, 1400.0]);
//! ```

/// Calculates the On-Balance Volume (OBV) indicator.
///
/// Returns a vector of OBV values, starting from 0.
///
/// # Arguments
/// * `closes` - A slice of f64 closing prices.
/// * `volumes` - A slice of f64 volume values.
///
/// # Example
/// ```
/// use financial_indicators::obv::on_balance_volume;
/// let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8];
/// let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
/// let obv = on_balance_volume(&closes, &volumes);
/// assert_eq!(obv, vec![0.0, 1200.0, 100.0, 1400.0, 1400.0]);
/// ```
pub fn on_balance_volume(closes: &[f64], volumes: &[f64]) -> Vec<f64> {
    let len = closes.len().min(volumes.len());
    if len == 0 {
        return vec![];
    }
    let mut obv = Vec::with_capacity(len);
    let mut current_obv = 0.0;
    obv.push(current_obv);
    for i in 1..len {
        if closes[i] > closes[i - 1] {
            current_obv += volumes[i];
        } else if closes[i] < closes[i - 1] {
            current_obv -= volumes[i];
        }
        // If unchanged, OBV stays the same
        obv.push(current_obv);
    }
    obv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obv_basic() {
        let closes = vec![10.0, 10.5, 10.2, 10.8, 10.8];
        let volumes = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
        let obv = on_balance_volume(&closes, &volumes);
        assert_eq!(obv, vec![0.0, 1200.0, 100.0, 1400.0, 1400.0]);
    }

    #[test]
    fn test_obv_short_input() {
        let closes = vec![10.0];
        let volumes = vec![1000.0];
        let obv = on_balance_volume(&closes, &volumes);
        assert_eq!(obv, vec![0.0]);
    }

    #[test]
    fn test_obv_empty() {
        let closes: Vec<f64> = vec![];
        let volumes: Vec<f64> = vec![];
        let obv = on_balance_volume(&closes, &volumes);
        assert_eq!(obv, vec![]);
    }
}
