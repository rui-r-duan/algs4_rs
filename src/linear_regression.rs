//! Simple linear regression.
//!
//! Compute least squares solution to <em>y</em> = &beta; * <em>x</em> + &alpha;

use crate::error::InvalidArgument;
use std::fmt;

///  The methods of `LinearRegression` struct performs a simple linear regression on an set of
///  <em>n</em> data points (<em>y<sub>i</sub></em>, <em>x<sub>i</sub></em>).  That is, it fits a
///  straight line <em>y</em> = &alpha; + &beta; <em>x</em>, (where <em>y</em> is the response
///  variable, <em>x</em> is the predictor variable, &alpha; is the <em>y-intercept</em>, and &beta;
///  is the <em>slope</em>) that minimizes the sum of squared residuals of the linear regression
///  model.  It also computes associated statistics, including the coefficient of determination
///  <em>R</em><sup>2</sup> and the standard deviation of the estimates for the slope and
///  <em>y</em>-intercept.
pub struct LinearRegression {
    intercept: f64,
    slope: f64,
    r2: f64,
    svar0: f64,
    svar1: f64,
}

impl LinearRegression {
    /// Performs a linear regression on the data points `(y[i], x[i])`.
    ///
    /// # Params
    /// - `x`: the values of the predictor variable
    /// - `y`: the corresponding values of the response variable
    ///
    /// # Errors
    ///
    /// Returns `InvalidArgument` if the lengths of the two slices are not equal.
    pub fn new(x: &[f64], y: &[f64]) -> Result<Self, InvalidArgument> {
        if x.len() != y.len() {
            return Err(InvalidArgument("array length are not equal".to_string()));
        }
        let n = x.len();

        // first pass
        let (mut sumx, mut sumy) = (0.0, 0.0);
        for i in 0..n {
            sumx += x[i];
            sumy += y[i];
        }
        let xbar = sumx / n as f64;
        let ybar = sumy / n as f64;

        // second pass: compute summary statistics
        let (mut xxbar, mut yybar, mut xybar) = (0.0, 0.0, 0.0);
        for i in 0..n {
            xxbar += (x[i] - xbar) * (x[i] - xbar);
            yybar += (y[i] - ybar) * (y[i] - ybar);
            xybar += (x[i] - xbar) * (y[i] - ybar);
        }
        let slope = xybar / xxbar;
        let intercept = ybar - slope * xbar;

        // more statistical analysis
        let (mut rss, mut ssr) = (0.0, 0.0);
        for i in 0..n {
            let fit = slope * x[i] + intercept;
            rss += (fit - y[i]) * (fit - y[i]);
            ssr += (fit - ybar) * (fit - ybar);
        }

        let degrees_of_freedom = n as f64 - 2.0;
        let r2 = ssr / yybar;
        let svar = rss / degrees_of_freedom;
        let svar1 = svar / xxbar;
        let svar0 = svar / n as f64 + xbar * xbar * svar1;

        Ok(LinearRegression {
            intercept,
            slope,
            r2,
            svar0,
            svar1,
        })
    }

    /// Returns the <em>y</em>-intercept &alpha; of the best-fit line
    /// <em>y</em> = &alpha; + &beta; <em>x</em>.
    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    /// Returns the slope &beta; of the best-fit line
    /// <em>y</em> = &alpha; + &beta; <em>x</em>.
    pub fn slope(&self) -> f64 {
        self.slope
    }

    /// Returns the coefficient of determination <em>R</em><sup>2</sup>.
    pub fn r2(&self) -> f64 {
        self.r2
    }

    /// Returns the standard error of the estimate for the intercept.
    pub fn intercept_std_err(&self) -> f64 {
        self.svar0.sqrt()
    }

    /// Returns the standard error of the estimate for the slope.
    pub fn slope_std_err(&self) -> f64 {
        self.svar1.sqrt()
    }

    /// Returns the expected response `y` given the value of the predictor variable `x`.
    ///
    /// # Params
    /// - `x`: the value of the predictor variable
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

impl fmt::Display for LinearRegression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.2} n + {:.2}  (R^2 = {:.3})",
            self.slope, self.intercept, self.r2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_vectored(x: &[f64], base: f64) -> Vec<f64> {
        x.iter().map(|v| v.log(base)).collect()
    }

    #[test]
    fn test_linear_regression() {
        let y = [0.024, 0.122, 0.88, 6.707]; // seconds of ThreeSum program
        let x = [1000.0, 2000.0, 4000.0, 8000.0]; // number of input integers
        let log_y = log_vectored(&y, 2.0);
        let log_x = log_vectored(&x, 2.0);
        let lr = LinearRegression::new(&log_x, &log_y)
            .expect("vectors x and y should be of the same length");
        assert_eq!(lr.to_string(), "2.72 n + -32.69  (R^2 = 0.997)");

        let x1: f64 = 1_000_000.0;
        let log_y1 = lr.predict(x1.log(2.0));
        assert_eq!(log_y1, 21.58875082618649);
        assert!(log_y1 - (2.72 * x1 - 32.69) < 0.01);
        let y1 = log_y1.exp2();
        assert_eq!(y1, 3153999.1183853233); // 3_153_999 seconds ≈ 36.5 days
    }
}
