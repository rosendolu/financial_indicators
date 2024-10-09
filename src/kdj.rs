//! The KDJ indicator struct.
//!
//! KDJ is a technical analysis indicator used to measure overbought and oversold levels in stock trading.
//! It consists of three components: K, D, and J values.
//!
//! # Fields
//! - `k`: The K value, which is a fast stochastic indicator.
//! - `d`: The D value, a slow stochastic indicator.
//! - `j`: The J value, which reflects the difference between K and D.
pub struct KDJ {
    pub k: f64,
    pub d: f64,
    pub j: f64,
}

impl KDJ {
    /// Computes the KDJ indicator.
    ///
    /// This function calculates the KDJ values for the given period based on the provided high, low, and close prices.
    ///
    /// # Arguments
    /// - `highs`: A slice of the highest prices over multiple periods.
    /// - `lows`: A slice of the lowest prices over multiple periods.
    /// - `closes`: A slice of the closing prices over multiple periods.
    /// - `period`: The time period to use for calculation.
    ///
    /// # Returns
    /// Returns a vector of `KDJ` structs, where each struct represents the K, D, and J values for a given period.
    ///
    /// # Panics
    /// Panics if the lengths of `highs`, `lows`, and `closes` slices are not the same.
    pub fn new(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Vec<Self> {
        // Ensure the input arrays have the same length
        if highs.len() != lows.len() || lows.len() != closes.len() {
            panic!("Highs, lows, and closes must have the same length");
        }

        let mut kdj_values = Vec::new();
        // Initialize K and D values
        let mut k: f64 = 50.0;
        let mut d: f64 = 50.0;

        // Iterate through each period to calculate KDJ values
        for i in period..highs.len() {
            // Calculate the highest high and lowest low within the period
            let high = highs[i - period..i]
                .iter()
                .copied()
                .fold(f64::MIN, f64::max);
            let low = lows[i - period..i].iter().copied().fold(f64::MAX, f64::min);

            // Calculate the RSV (Raw Stochastic Value)
            let rsv = if high != low {
                (closes[i] - low) / (high - low) * 100.0
            } else {
                50.0 // Prevent division by zero
            };

            // Update K and D values based on the formula
            k = 2.0 / 3.0 * k + 1.0 / 3.0 * rsv;
            d = 2.0 / 3.0 * d + 1.0 / 3.0 * k;

            // Calculate the J value
            let j = 3.0 * k - 2.0 * d;

            // Store the calculated K, D, and J values for this period
            kdj_values.push(KDJ { k, d, j });
        }

        kdj_values
    }
}

#[cfg(test)]
mod tests {
    use super::KDJ;

    #[test]
    fn test_kdj_calculation() {
        let highs = vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let lows = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0];
        let closes = vec![10.5, 11.0, 12.0, 13.0, 14.0, 15.0];

        let kdj_values = KDJ::new(&highs, &lows, &closes, 3);

        // Check if we have the correct number of KDJ values
        assert_eq!(kdj_values.len(), 3);

        // Validate the K, D, and J values (for illustration purposes)
        let first_kdj = &kdj_values[0];
        assert!(first_kdj.k >= 0.0 && first_kdj.k <= 100.0);
        assert!(first_kdj.d >= 0.0 && first_kdj.d <= 100.0);
        assert!(first_kdj.j >= 0.0 && first_kdj.j <= 100.0);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_mismatched_lengths() {
        let highs = vec![11.0, 12.0];
        let lows = vec![10.0, 9.0];
        let closes = vec![10.5];
        KDJ::new(&highs, &lows, &closes, 3); // This should panic
    }
}
