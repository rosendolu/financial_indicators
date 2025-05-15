# MFI (Money Flow Index) Indicator

## Overview

The Money Flow Index (MFI) is a momentum indicator that uses both price and volume data to identify overbought or oversold conditions in an asset. It is similar to the Relative Strength Index (RSI) but incorporates volume, making it a volume-weighted RSI.

## Calculation

1. Calculate the Typical Price (TP) for each period:
   \[
   TP = \frac{High + Low + Close}{3}
   \]
2. Calculate the Raw Money Flow:
   \[
   Raw\ Money\ Flow = TP \times Volume
   \]
3. Determine Positive and Negative Money Flow:
   - If TP > previous TP, add Raw Money Flow to Positive Money Flow.
   - If TP < previous TP, add Raw Money Flow to Negative Money Flow.
   - If TP = previous TP, ignore.
4. Calculate the Money Flow Ratio over N periods:
   \[
   Money\ Flow\ Ratio = \frac{Sum\ of\ Positive\ Money\ Flow}{Sum\ of\ Negative\ Money\ Flow}
   \]
5. Calculate the MFI:
   \[
   MFI = 100 - \frac{100}{1 + Money\ Flow\ Ratio}
   \]

## Usage

- **Overbought/Oversold:**
  - MFI > 80 is considered overbought.
  - MFI < 20 is considered oversold.
- **Divergence:**
  - Divergence between MFI and price can signal potential reversals.
- **Trend Confirmation:**
  - MFI can confirm price trends or signal potential reversals when crossing key levels.

## Example

Given high, low, close, and volume data for 20 days, calculate the 14-period MFI as described above. The first MFI value is available after 14 periods.

## References

- [Investopedia: Money Flow Index (MFI)](https://www.investopedia.com/terms/m/mfi.asp)
- [Wikipedia: Money Flow Index](https://en.wikipedia.org/wiki/Money_flow_index)
