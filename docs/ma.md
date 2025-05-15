# Moving Average (MA) Indicator

## Overview

The Moving Average (MA) is one of the most widely used technical indicators in financial analysis for both stocks and cryptocurrencies. It helps smooth out price data by creating a constantly updated average price, which can help identify the direction of the trend.

## Types of Moving Averages

- **Simple Moving Average (SMA):** The arithmetic mean of a given set of prices over a specific number of periods.
- **Weighted Moving Average (WMA):** Similar to SMA, but assigns more weight to recent prices.
- **Exponential Moving Average (EMA):** A type of WMA that applies more weight to recent prices, but in an exponentially decreasing manner (covered in a separate module).

## Calculation

### Simple Moving Average (SMA)

Given a time series of prices \( P_1, P_2, ..., P_n \), the SMA over period \( N \) is:

\[
SMA*t = \frac{P*{t-N+1} + P\_{t-N+2} + ... + P_t}{N}
\]

Where:

- \( SMA_t \) is the simple moving average at time \( t \)
- \( N \) is the number of periods
- \( P_t \) is the price at time \( t \)

### Weighted Moving Average (WMA)

The WMA assigns a weight to each price, with more recent prices typically given higher weights.

\[
WMA*t = \frac{\sum*{i=1}^{N} w*i P*{t-N+i}}{\sum\_{i=1}^{N} w_i}
\]

Where \( w_i \) is the weight for each period.

## Usage

- **Trend Identification:** MAs help identify the direction of the trend (uptrend, downtrend, or sideways).
- **Support and Resistance:** MAs can act as dynamic support or resistance levels.
- **Signal Generation:** Crossovers of short-term and long-term MAs can generate buy or sell signals.

## Example

If the closing prices for 5 days are [10, 11, 12, 13, 14], the 3-day SMA for day 5 is:

\[
SMA_5 = \frac{12 + 13 + 14}{3} = 13
\]

## References

- [Investopedia: Moving Average (MA)](https://www.investopedia.com/terms/m/movingaverage.asp)
- [Wikipedia: Moving Average](https://en.wikipedia.org/wiki/Moving_average)
