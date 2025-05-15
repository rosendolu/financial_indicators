//! VOL (Volume) Indicator
//!
//! This module will provide a function to return the volume series as an indicator.
//!
//! # Examples
//!
//! ```
//! // Example usage will be added after implementation
//! ```

/// Returns the volume series as an indicator.
///
/// # Arguments
/// * `volume` - A slice of f64 volume values.
///
/// # Example
/// ```
/// use financial_indicators::vol::volume_indicator;
/// let volume = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
/// let vol = volume_indicator(&volume);
/// assert_eq!(vol, volume);
/// ```
pub fn volume_indicator(volume: &[f64]) -> Vec<f64> {
    volume.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_indicator_basic() {
        let volume = vec![1000.0, 1200.0, 1100.0, 1300.0, 900.0];
        let vol = volume_indicator(&volume);
        assert_eq!(vol, volume);
    }

    #[test]
    fn test_volume_indicator_empty() {
        let volume: Vec<f64> = vec![];
        let vol = volume_indicator(&volume);
        assert_eq!(vol, vec![]);
    }
}
