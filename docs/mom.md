# Momentum (MOM) Indicator

## Overview

The Momentum (MOM) indicator is a simple yet effective technical analysis tool that measures the amount that a security's price has changed over a given time span. It is used to identify the speed of price movements and potential trend reversals.

## Calculation

The MOM is calculated using the following formula:

\[
MOM*t = P_t - P*{t-n}
\]

Where:

- \( MOM_t \) is the Momentum at time \( t \)
- \( P_t \) is the current price
- \( P\_{t-n} \) is the price n periods ago
- \( n \) is the lookback period (commonly 10 or 14)

## Usage

- **Trend Strength:** Positive MOM values indicate upward momentum, negative values indicate downward momentum.
- **Overbought/Oversold:** Extreme MOM values can signal overbought or oversold conditions.
- **Divergence:** Divergence between MOM and price can indicate potential trend reversals.

## Example

If the closing prices for 11 days are given, the 10-period MOM for day 11 is:
\[
MOM*{11} = P*{11} - P_1
\]

## References

- [Investopedia: Momentum (MOM)](https://www.investopedia.com/terms/m/momentum.asp)
- [Wikipedia: Momentum (technical analysis)](<https://en.wikipedia.org/wiki/Momentum_(technical_analysis)>)
