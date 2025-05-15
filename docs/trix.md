# TRIX (Triple Exponential Average) Indicator

## Overview

TRIX is a momentum oscillator that displays the percent rate of change of a triple exponentially smoothed moving average of closing prices. It is designed to filter out insignificant price movements and highlight the prevailing trend, making it useful for identifying trend reversals and momentum.

## Calculation

1. Calculate the 1st Exponential Moving Average (EMA) of closing prices over period N.
2. Calculate the 2nd EMA of the 1st EMA (i.e., EMA of EMA).
3. Calculate the 3rd EMA of the 2nd EMA (i.e., EMA of EMA of EMA).
4. Calculate the 1-period percent rate of change of the triple EMA:

   \[
   TRIX*t = \frac{EMA3_t - EMA3*{t-1}}{EMA3\_{t-1}} \times 100
   \]
   where EMA3 is the triple-smoothed EMA.

## Usage

- **Trend Identification:**
  - TRIX above zero suggests an uptrend; below zero suggests a downtrend.
- **Signal Generation:**
  - Signal line crossovers (e.g., 9-period SMA of TRIX) can generate buy/sell signals.
- **Divergence:**
  - Divergence between TRIX and price can indicate potential reversals.

## Example

Given closing prices for 30 days, calculate the 15-period TRIX as described above. The first TRIX value is available after enough data for three EMAs and a rate of change.

## References

- [Investopedia: TRIX - Triple Exponential Average](https://www.investopedia.com/terms/t/trix.asp)
- [Wikipedia: TRIX (technical analysis)](<https://en.wikipedia.org/wiki/TRIX_(technical_analysis)>)
