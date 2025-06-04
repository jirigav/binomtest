#![no_main]
use binomtest::*;
use libfuzzer_sys::fuzz_target;
use pyo3::prelude::*;

fn alt_to_str(alt: &Alternative) -> &'static str {
    match alt {
        Alternative::TwoSided => "two-sided",
        Alternative::Less => "less",
        Alternative::Greater => "greater",
    }
}

fn p_value(k: u64, n: u64, p: f64, alt: &str) -> Result<f64, PyErr> {
    Python::with_gil(|py| {
        let scipy_stats = PyModule::import(py, "scipy.stats")?;
        scipy_stats
            .getattr("binomtest")?
            .call1((k, n, p, alt))?
            .getattr("pvalue")?.extract::<f64>()

    })
}

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub struct FuzzInput {
    pub k: u64,
    pub n: u64,
    pub p: f64,
    pub alt: Alternative,
}

fuzz_target!(|params: FuzzInput| {
    let a = binomial_test(params.k, params.n, params.p, params.alt);
    let b = p_value(params.k, params.n, params.p, alt_to_str(&params.alt));
    println!("{:?}, {:?}", a, b);
    match b {
        Ok(rb) => assert!(((a.unwrap() - rb).abs() <= 0.0000001*rb) || rb < 10e-300),
        Err(_) => ()
    }
});

