# Average True Range (ATR) Indicator

## Overview

The Average True Range (ATR) is a technical analysis indicator that measures market volatility by decomposing the entire range of an asset price for a given period. It is widely used to assess the degree of price volatility and to set stop-loss levels.

## Calculation

The ATR is calculated using the following steps:

1. Calculate the True Range (TR) for each period:
   - TR = max(High - Low, |High - Previous Close|, |Low - Previous Close|)
2. Calculate the ATR as the n-period moving average of the True Range:
   - For the first ATR value (at period n):
     \[
     ATR*n = \frac{\sum*{i=1}^{n} TR_i}{n}
     \]
   - For subsequent ATR values:
     \[
     ATR*t = \frac{(ATR*{t-1} \times (n-1)) + TR_t}{n}
     \]

Where:

- High = current period high
- Low = current period low
- Previous Close = previous period close
- n = ATR period (commonly 14)

## Usage

- **Volatility Measurement:** Higher ATR values indicate higher volatility, lower values indicate lower volatility.
- **Stop-Loss Placement:** ATR is often used to set trailing stop-loss levels.
- **Trend Confirmation:** ATR can help confirm the strength of a trend.

## Example

If the high, low, and close prices for 15 days are given, calculate the 14-period ATR as described above. The first ATR value is calculated after 14 periods.

## References

- [Investopedia: Average True Range (ATR)](https://www.investopedia.com/terms/a/atr.asp)
- [Wikipedia: Average True Range](https://en.wikipedia.org/wiki/Average_true_range)
