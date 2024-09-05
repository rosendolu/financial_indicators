//! The MACD (Moving Average Convergence Divergence) structure
//! representing the three key elements of the MACD indicator.
//!
//! - `macd`: The difference between the short and long-term EMAs.
//! - `signal`: The EMA of the `macd`, known as the signal line.
//! - `histogram`: The difference between the `macd` and the signal line.
pub struct MACD {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl MACD {
    /// Calculates the MACD indicator for a given set of closing prices.
    ///
    /// # Arguments
    ///
    /// - `closes`: A slice of closing prices.
    /// - `short_period`: The period for the short-term EMA (typically 12).
    /// - `long_period`: The period for the long-term EMA (typically 26).
    /// - `signal_period`: The period for the signal line EMA (typically 9).
    ///
    /// # Returns
    ///
    /// A vector of `MACD` values for each closing price starting from the
    /// point where enough data is available to calculate the long-term EMA.
    ///
    /// # Example
    ///
    /// ```
    /// use financial_indicators::macd::MACD;
    ///
    /// let closes = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    /// let macd_values = MACD::new(&closes, 12, 26, 9);
    ///
    /// for macd in macd_values {
    ///     println!("MACD: {}, Signal: {}, Histogram: {}", macd.macd, macd.signal, macd.histogram);
    /// }
    /// ```
    pub fn new(
        closes: &[f64],
        short_period: usize,
        long_period: usize,
        signal_period: usize,
    ) -> Vec<MACD> {
        let mut macd_values = Vec::new();
        if closes.is_empty() || closes.len() < long_period {
            // Not enough data to calculate MACD
            return macd_values;
        }

        let short_ema = Self::ema(closes, short_period);
        let long_ema = Self::ema(closes, long_period);

        let mut macd_raw = Vec::with_capacity(closes.len());
        for i in 0..closes.len() {
            if i >= long_period - 1 {
                let macd = short_ema[i] - long_ema[i];
                macd_raw.push(macd);
            }
        }

        let signal_ema = Self::ema(&macd_raw, signal_period);
        for i in 0..macd_raw.len() {
            let macd = macd_raw[i];
            let signal = if i >= signal_period - 1 {
                signal_ema[i]
            } else {
                0.0
            };
            let histogram = macd - signal;

            macd_values.push(MACD {
                macd,
                signal,
                histogram,
            });
        }

        macd_values
    }

    /// Internal helper function to calculate the Exponential Moving Average (EMA)
    /// for a given period.
    ///
    /// # Arguments
    ///
    /// - `prices`: A slice of price data.
    /// - `period`: The lookback period for calculating the EMA.
    ///
    /// # Returns
    ///
    /// A vector of EMA values for each price.
    pub(crate) fn ema(prices: &[f64], period: usize) -> Vec<f64> {
        let mut ema_values = Vec::with_capacity(prices.len());
        let smoothing = 2.0 / (period as f64 + 1.0);
        let mut ema = prices[0];

        for &price in prices.iter() {
            ema = (price - ema) * smoothing + ema;
            ema_values.push(ema);
        }
        ema_values
    }
}

#[cfg(test)]
mod tests {
    use super::MACD;

    #[test]
    fn test_macd_empty() {
        let closes: Vec<f64> = vec![];
        let macd_values = MACD::new(&closes, 12, 26, 9);
        assert!(macd_values.is_empty());
    }

    #[test]
    fn test_macd_single_element() {
        let closes = vec![100.0];
        let macd_values = MACD::new(&closes, 12, 26, 9);
        assert!(macd_values.is_empty()); // Not enough data to calculate
    }

    #[test]
    fn test_macd_constant_prices() {
        let closes = vec![100.0, 100.0, 100.0, 100.0, 100.0, 100.0];
        let macd_values = MACD::new(&closes, 12, 26, 9);
        for macd in macd_values {
            assert_eq!(macd.macd, 0.0);
            assert_eq!(macd.signal, 0.0);
            assert_eq!(macd.histogram, 0.0);
        }
    }
}
