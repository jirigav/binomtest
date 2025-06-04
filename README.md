# binomtest

binomtest is a lightweight Rust crate that provides the `binomial-test` function for hypothesis testing of binary outcomes.

## Overview

The binomial_test function calculates the p-value for the binomial test given:

- The number of observed successes,

- The total number of trials,

- The hypothesized probability of success under the null hypothesis,

- The alternative hypothesis type (two-sided, greater, or less).


## Usage

Add the crate to your Cargo.toml:

```
[dependencies]
binomial-test = "0.1.0"
```

Use the function in your Rust code:

```
use binomtest::*;

fn main() {
    let successes = 5;
    let trials = 10;
    let p_null = 0.5;
    let alt = Alternative::TwoSided;

    match binomial_test(successes, trials, p_null, alt) {
        Ok(p_value) => println!("P-value: {}", p_value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

#### Alternative Hypothesis

The crate supports three types of alternative hypotheses through the Alternative enum:

- TwoSided
- Greater
- Less

# License
binomtest is released under the MIT license. See [LICENSE](LICENSE) for more information.