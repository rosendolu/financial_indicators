# Standard Deviation (STD) Indicator

## Overview

Standard Deviation (STD) is a statistical measure of volatility. In financial analysis, it is used to quantify the amount of variation or dispersion of a set of prices. A higher standard deviation indicates higher volatility, while a lower standard deviation indicates lower volatility.

## Calculation

The n-period standard deviation of a price series is calculated as follows:

\[
STD*t = \sqrt{\frac{1}{n} \sum*{i=0}^{n-1} (P\_{t-i} - SMA_t)^2}
\]

Where:

- \( STD_t \) is the standard deviation at time \( t \)
- \( P\_{t-i} \) is the price at time \( t-i \)
- \( SMA_t \) is the n-period simple moving average at time \( t \)
- \( n \) is the lookback period

## Usage

- **Volatility Measurement:** Higher STD values indicate higher volatility, lower values indicate lower volatility.
- **Risk Assessment:** STD is used to assess the risk of a security or portfolio.
- **Other Indicators:** STD is a component of other indicators, such as Bollinger Bands.

## Example

If the closing prices for 20 days are given, calculate the 20-period STD as described above. The first STD value is calculated after 20 periods.

## References

- [Investopedia: Standard Deviation](https://www.investopedia.com/terms/s/standarddeviation.asp)
- [Wikipedia: Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation)
