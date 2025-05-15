# VWAP (Volume Weighted Average Price) Indicator

## Overview

The Volume Weighted Average Price (VWAP) is a trading benchmark that gives the average price a security has traded at throughout the day, based on both volume and price. It is commonly used by traders to assess the current price relative to the day's average and to inform intraday trading decisions.

## Calculation

1. For each period (typically each minute or bar):
   - Calculate the Typical Price (TP):
     \[
     TP = \frac{High + Low + Close}{3}
     \]
   - Multiply TP by the period's volume to get the Period Money Flow:
     \[
     Period\ Money\ Flow = TP \times Volume
     \]
2. Calculate the cumulative sums up to each period:
   - Cumulative Money Flow = sum of Period Money Flow up to current period
   - Cumulative Volume = sum of Volume up to current period
3. Calculate VWAP for each period:
   \[
   VWAP*t = \frac{\sum*{i=1}^t (TP*i \times Volume_i)}{\sum*{i=1}^t Volume_i}
   \]

## Usage

- **Intraday Benchmark:**
  - VWAP is used to compare the current price to the average price paid throughout the day.
- **Trade Execution:**
  - Institutions use VWAP to ensure they are getting a good price relative to the day's average.
- **Support/Resistance:**
  - VWAP can act as a dynamic support or resistance level during the trading day.

## Example

Given high, low, close, and volume data for each bar in a trading day, calculate the VWAP as described above. VWAP is recalculated for each new bar.

## References

- [Investopedia: Volume Weighted Average Price (VWAP)](https://www.investopedia.com/terms/v/vwap.asp)
- [Wikipedia: Volume Weighted Average Price](https://en.wikipedia.org/wiki/Volume-weighted_average_price)
