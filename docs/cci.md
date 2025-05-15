# Commodity Channel Index (CCI) Indicator

## Overview

The Commodity Channel Index (CCI) is a versatile technical indicator that measures the deviation of the price from its statistical mean. Originally developed for commodities, it is now widely used for stocks and cryptocurrencies to identify cyclical trends, overbought, and oversold conditions.

## Calculation

The CCI is calculated using the following formula:

1. Calculate the Typical Price (TP) for each period:
   \[
   TP = \frac{High + Low + Close}{3}
   \]
2. Calculate the n-period Simple Moving Average (SMA) of the Typical Price:
   \[
   SMA*{TP} = \frac{\sum*{i=1}^{n} TP_i}{n}
   \]
3. Calculate the Mean Deviation:
   \[
   MD = \frac{\sum*{i=1}^{n} |TP_i - SMA*{TP}|}{n}
   \]
4. Calculate the CCI:
   \[
   CCI = \frac{TP - SMA\_{TP}}{0.015 \times MD}
   \]

Where:

- TP = Typical Price
- SMA\_{TP} = n-period Simple Moving Average of TP
- MD = Mean Deviation
- 0.015 is a constant to ensure most CCI values fall between -100 and +100

## Usage

- **Overbought/Oversold:** CCI > 100 is considered overbought, CCI < -100 is considered oversold.
- **Trend Identification:** CCI can help identify the start and end of market trends.
- **Divergence:** Divergence between CCI and price can signal potential reversals.

## Example

If the high, low, and close prices for 20 days are given, calculate the 20-period CCI as described above. The first CCI value is calculated after 20 periods.

## References

- [Investopedia: Commodity Channel Index (CCI)](https://www.investopedia.com/terms/c/commoditychannelindex.asp)
- [Wikipedia: Commodity Channel Index](https://en.wikipedia.org/wiki/Commodity_channel_index)
