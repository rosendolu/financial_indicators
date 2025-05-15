# Relative Strength Index (RSI) Indicator

## Overview

The Relative Strength Index (RSI) is a popular momentum oscillator used in technical analysis to measure the speed and change of price movements. RSI values range from 0 to 100 and are typically used to identify overbought or oversold conditions in a traded asset.

## Calculation

The RSI is calculated using the following steps:

1. Calculate the change in price between consecutive periods.
2. Separate the positive (up) and negative (down) changes.
3. Calculate the average gain and average loss over the specified period (N, commonly 14).
4. Compute the Relative Strength (RS):
   \[
   RS = \frac{\text{Average Gain}}{\text{Average Loss}}
   \]
5. Calculate the RSI:
   \[
   RSI = 100 - \frac{100}{1 + RS}
   \]

The first average gain and loss are simple averages. Subsequent values use a smoothed formula:

- Average Gain = [(previous Average Gain) × (N - 1) + current Gain] / N
- Average Loss = [(previous Average Loss) × (N - 1) + current Loss] / N

## Usage

- **Overbought/Oversold:** RSI > 70 is typically considered overbought, RSI < 30 is considered oversold.
- **Trend Confirmation:** RSI can confirm price trends or signal potential reversals.
- **Divergence:** Divergence between RSI and price can indicate a possible trend reversal.

## Example

If the closing prices for 15 days are given, calculate the 14-period RSI as described above. The first RSI value is calculated after 14 periods.

## References

- [Investopedia: Relative Strength Index (RSI)](https://www.investopedia.com/terms/r/rsi.asp)
- [Wikipedia: Relative Strength Index](https://en.wikipedia.org/wiki/Relative_strength_index)
