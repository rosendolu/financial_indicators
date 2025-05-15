# Rate of Change (ROC) Indicator

## Overview

The Rate of Change (ROC) is a momentum oscillator that measures the percentage change in price between the current price and the price a certain number of periods ago. It is widely used in technical analysis to identify the strength of price trends and potential reversal points.

## Calculation

The ROC is calculated using the following formula:

\[
ROC*t = \frac{P_t - P*{t-n}}{P\_{t-n}} \times 100
\]

Where:

- \( ROC_t \) is the Rate of Change at time \( t \)
- \( P_t \) is the current price
- \( P\_{t-n} \) is the price n periods ago
- \( n \) is the lookback period (commonly 12 or 25)

## Usage

- **Momentum Identification:** Positive ROC values indicate upward momentum, negative values indicate downward momentum.
- **Overbought/Oversold:** Extreme ROC values can signal overbought or oversold conditions.
- **Divergence:** Divergence between ROC and price can indicate potential trend reversals.

## Example

If the closing prices for 13 days are given, the 12-period ROC for day 13 is:
\[
ROC*{13} = \frac{P*{13} - P_1}{P_1} \times 100
\]

## References

- [Investopedia: Rate of Change (ROC)](https://www.investopedia.com/terms/r/rateofchange.asp)
- [Wikipedia: Rate of Change](https://en.wikipedia.org/wiki/Rate_of_change)
