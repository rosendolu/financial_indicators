# DMI / ADX (Directional Movement Index / Average Directional Index)

## Overview

The Directional Movement Index (DMI) and Average Directional Index (ADX) are trend-following indicators developed by J. Welles Wilder. DMI consists of two lines, +DI and -DI, which measure the strength of upward and downward movements, while ADX quantifies the overall strength of the trend, regardless of direction.

## Calculation

### 1. Calculate True Range (TR)

For each period:

- TR = max(High - Low, |High - Previous Close|, |Low - Previous Close|)

### 2. Calculate Directional Movements

- +DM = High - Previous High (if positive and greater than Low - Previous Low, else 0)
- -DM = Previous Low - Low (if positive and greater than High - Previous High, else 0)

### 3. Smooth the values (typically using a 14-period average)

- Smoothed TR, +DM, and -DM (usually using Wilder's smoothing method)

### 4. Calculate Directional Indicators

- +DI = 100 × (Smoothed +DM / Smoothed TR)
- -DI = 100 × (Smoothed -DM / Smoothed TR)

### 5. Calculate DX and ADX

- DX = 100 × |+DI - -DI| / (+DI + -DI)
- ADX = n-period average of DX (commonly 14)

## Usage

- **+DI and -DI Crossovers:**
  - +DI crossing above -DI may signal a bullish trend.
  - -DI crossing above +DI may signal a bearish trend.
- **ADX Value:**
  - ADX above 20 or 25 typically indicates a strong trend.
  - ADX below 20 suggests a weak or non-trending market.

## Example

Given high, low, and close prices for 15 days, calculate the 14-period DMI and ADX as described above. The first ADX value is calculated after 14 periods.

## References

- [Investopedia: Average Directional Index (ADX)](https://www.investopedia.com/terms/a/adx.asp)
- [Investopedia: Directional Movement Index (DMI)](https://www.investopedia.com/terms/d/dmi.asp)
- [Wikipedia: Average Directional Movement Index](https://en.wikipedia.org/wiki/Average_directional_movement_index)
