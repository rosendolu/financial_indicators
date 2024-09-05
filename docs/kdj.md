# KDJ Indicator

The KDJ indicator is a technical analysis tool used to predict price movements in financial markets. It is an extension of the Stochastic Oscillator and includes three lines: K, D, and J.

## Formulas

- **RSV (Raw Stochastic Value)**:
  $
  RSV = \frac{{\text{{Close}} - \text{{Low}}_{\text{{n}}}}}{{\text{{High}}_{\text{{n}}} - \text{{Low}}\_{\text{{n}}}}} \times 100
  $

- **K Line**:
  $
  K = \frac{2}{3} \times K\_{\text{{previous}}} + \frac{1}{3} \times RSV
  $

- **D Line**:
  $
  D = \frac{2}{3} \times D\_{\text{{previous}}} + \frac{1}{3} \times K
  $

- **J Line**:
  $
  J = 3 \times K - 2 \times D
  $

## Example Calculation

Given a series of OHLC data points, you can calculate the K, D, and J values as follows:

1. Determine the highest high and lowest low over the past `n` periods.
2. Calculate the RSV.
3. Update the K and D lines using their respective formulas.
4. Calculate the J line using the K and D values.

For more details, refer to the source code documentation.
