# Parabolic SAR (Stop and Reverse) Indicator

## Overview

The Parabolic SAR (Stop and Reverse) is a trend-following indicator developed by J. Welles Wilder. It is used to identify potential reversals in market price direction and to set trailing stop-losses. The indicator appears as a series of dots placed above or below the price chart, depending on the trend direction.

## Calculation

The Parabolic SAR is calculated as follows:

1. **Initialization:**
   - For an uptrend, the first SAR value is the lowest price of the previous period.
   - For a downtrend, the first SAR value is the highest price of the previous period.
2. **Subsequent SAR values:**
   - SAR<sub>t</sub> = SAR<sub>t-1</sub> + AF × (EP - SAR<sub>t-1</sub>)
   - Where:
     - SAR<sub>t</sub>: Current SAR value
     - SAR<sub>t-1</sub>: Previous SAR value
     - AF: Acceleration Factor (starts at 0.02, increases by 0.02 with each new EP, up to a maximum of 0.2)
     - EP: Extreme Point (highest high in uptrend, lowest low in downtrend)
3. **Trend Reversal:**
   - If price crosses the SAR, the trend reverses, and the SAR is reset to the EP. AF is reset to its initial value.

## Usage

- **Trend Identification:**
  - Dots below price indicate an uptrend; dots above price indicate a downtrend.
- **Trailing Stop-Loss:**
  - SAR can be used as a dynamic stop-loss level.
- **Reversal Signals:**
  - When price crosses the SAR, it may signal a trend reversal.

## Example

Given high and low prices for each period, calculate the Parabolic SAR as described above. The first SAR value is available after the initial trend is established.

## References

- [Investopedia: Parabolic SAR](https://www.investopedia.com/terms/p/parabolicindicator.asp)
- [Wikipedia: Parabolic SAR](https://en.wikipedia.org/wiki/Parabolic_SAR)
