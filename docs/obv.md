# On-Balance Volume (OBV) Indicator

## Overview

On-Balance Volume (OBV) is a technical analysis indicator that uses volume flow to predict changes in stock price. It measures buying and selling pressure as a cumulative indicator, adding volume on up days and subtracting it on down days.

## Calculation

The OBV is calculated as follows:

- OBV starts at 0 (or the first volume value).
- For each subsequent day:
  - If the closing price is higher than the previous close, add the day's volume to OBV.
  - If the closing price is lower than the previous close, subtract the day's volume from OBV.
  - If the closing price is unchanged, OBV remains the same.

\[
OBV*t = OBV*{t-1} + \begin{cases}
Volume*t, & \text{if } Close_t > Close*{t-1} \\
-Volume*t, & \text{if } Close_t < Close*{t-1} \\
0, & \text{if } Close*t = Close*{t-1}
\end{cases}
\]

Where:

- \( OBV_t \) is the OBV at time \( t \)
- \( Volume_t \) is the volume at time \( t \)
- \( Close_t \) is the closing price at time \( t \)

## Usage

- **Trend Confirmation:** Rising OBV confirms an uptrend, falling OBV confirms a downtrend.
- **Divergence:** Divergence between OBV and price can signal potential reversals.
- **Volume Analysis:** OBV helps identify whether volume is supporting the price trend.

## Example

If the closing prices and volumes for 5 days are given, calculate the OBV as described above.

## References

- [Investopedia: On-Balance Volume (OBV)](https://www.investopedia.com/terms/o/onbalancevolume.asp)
- [Wikipedia: On-Balance Volume](https://en.wikipedia.org/wiki/On-balance_volume)
