# Bollinger Bands Indicator

## Overview

Bollinger Bands are a popular technical analysis tool that consists of a moving average (usually SMA) and two bands (upper and lower) placed a certain number of standard deviations above and below the moving average. They are used to measure market volatility and identify overbought or oversold conditions.

## Calculation

Bollinger Bands are calculated using the following steps:

1. Calculate the n-period Simple Moving Average (SMA):
   \[
   SMA*t = \frac{\sum*{i=0}^{n-1} P\_{t-i}}{n}
   \]
2. Calculate the n-period standard deviation (STD) of the price:
   \[
   STD*t = \sqrt{\frac{1}{n} \sum*{i=0}^{n-1} (P\_{t-i} - SMA_t)^2}
   \]
3. Calculate the upper and lower bands:
   \[
   Upper_t = SMA_t + k \times STD_t
   \]
   \[
   Lower_t = SMA_t - k \times STD_t
   \]
   Where k is typically 2.

## Usage

- **Volatility Measurement:** Bands widen when volatility increases and contract when volatility decreases.
- **Overbought/Oversold:** Price touching the upper band may indicate overbought, while touching the lower band may indicate oversold.
- **Trend Reversals:** Price moving outside the bands can signal a potential reversal.

## Example

If the closing prices for 20 days are given, calculate the 20-period Bollinger Bands as described above. The first bands are calculated after 20 periods.

## References

- [Investopedia: Bollinger Bands](https://www.investopedia.com/terms/b/bollingerbands.asp)
- [Wikipedia: Bollinger Bands](https://en.wikipedia.org/wiki/Bollinger_Bands)
