use std::cmp::Ordering;

use statrs::distribution::{Binomial, Discrete, DiscreteCDF};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Alternative {
    TwoSided,
    Less,
    Greater,
}

/// Performs a binomial test for a given number of successes, trials, and hypothesized probability.
///
/// # Arguments
///
/// * `k` - Number of observed successes.
/// * `n` - Total number of trials.
/// * `p` - Hypothesized probability of success under the null hypothesis (must be between 0.0 and 1.0).
/// * `alt` - Type of alternative hypothesis (`Alternative::TwoSided`, `Alternative::Greater`, or `Alternative::Less`).
///
/// # Returns
///
/// A `Result` containing the p-value (`f64`) if successful, or a `String` with an error message if the inputs are invalid.
///
/// # Errors
///
/// Returns an error if:
/// - `p` is not in the interval [0.0, 1.0].
/// - `k > n`.
/// - `n < 1`
///
/// # Example
///
/// ```rust
/// use binomtest::*;
///
/// let result = binomial_test(5, 10, 0.5, Alternative::TwoSided);
/// assert!(result.is_ok());
/// ```
pub fn binomial_test(k: u64, n: u64, p: f64, alt: Alternative) -> Result<f64, String> {
    if n < 1 {
        return Err("Number of trials n must be > 0".to_string());
    }
    if k > n {
        return Err("Number of successes k must be <= n and > 0".to_string());
    }
    if !(0. ..=1.).contains(&p) {
        return Err("Probability p must be in [0, 1]".to_string());
    }
    let binom = Binomial::new(p, n).expect("Invalid binomial parameters");

    match alt {
        Alternative::Less => Ok(binom.cdf(k)),

        Alternative::Greater => {
            if k == 0 {
                Ok(1.0)
            } else {
                Ok(binom.sf(k - 1))
            }
        }

        Alternative::TwoSided => {
            let d = binom.pmf(k);

            match k.cmp(&((p * (n as f64)) as u64)) {
                Ordering::Equal => Ok(1.),
                Ordering::Less => {
                    let x = binary_search(
                        &|x: u64| -binom.pmf(x),
                        -d,
                        (p * (n as f64)).ceil() as u64,
                        n,
                    );

                    Ok(binom.cdf(k) + binom.sf(x))
                }

                Ordering::Greater => {
                    let x = binary_search(
                        &|x: u64| binom.pmf(x),
                        d,
                        0,
                        (p * (n as f64)).floor() as u64,
                    );

                    let cdf = if x == 0 && d < binom.pmf(x) {
                        0.
                    } else {
                        binom.cdf(x)
                    };

                    Ok(cdf + binom.sf(k - 1))
                }
            }
        }
    }
}

fn binary_search(f: &dyn Fn(u64) -> f64, key: f64, mut low: u64, mut high: u64) -> u64 {
    while low < high {
        let mid = low + (high - low) / 2;
        let midval = f(mid);

        match midval.total_cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return mid,
            Ordering::Greater => {
                if mid == 0 {
                    return 0;
                } else {
                    high = mid - 1
                }
            }
        }
    }

    if f(low) <= key {
        return low;
    }

    u64::checked_sub(low, 1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() <= b * 0.000001
    }

    #[test]
    fn test_binomial_test() {
        assert!(approx_eq(
            binomial_test(1, 1, 0.0, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(1, 1, 1.0, Alternative::TwoSided).unwrap(),
            1.0
        ));
        assert!(approx_eq(
            binomial_test(1, 1, 0.5, Alternative::TwoSided).unwrap(),
            1.0
        ));
        assert!(approx_eq(
            binomial_test(1, 1, 0.25, Alternative::TwoSided).unwrap(),
            0.25
        ));
        assert!(approx_eq(
            binomial_test(675, 8064, 0.85, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(872, 1245, 0.51, Alternative::TwoSided).unwrap(),
            2.519147904123094e-42
        ));
        assert!(approx_eq(
            binomial_test(3009, 3952, 0.87, Alternative::TwoSided).unwrap(),
            1.6048354143177452e-76
        ));
        assert!(approx_eq(
            binomial_test(1774, 6395, 0.32, Alternative::TwoSided).unwrap(),
            1.4633129278540793e-13
        ));
        assert!(approx_eq(
            binomial_test(969, 7716, 0.76, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(1225, 4231, 0.75, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(1187, 2295, 0.02, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(1993, 2228, 0.61, Alternative::TwoSided).unwrap(),
            8.219896711580438e-200
        ));
        assert!(approx_eq(
            binomial_test(4649, 5936, 0.97, Alternative::TwoSided).unwrap(),
            0.0
        ));
        assert!(approx_eq(
            binomial_test(342, 711, 0.2, Alternative::TwoSided).unwrap(),
            5.29655579272766e-63
        ));
    }
}
