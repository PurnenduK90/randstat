//! Dieharder: Birthday Spacings Test (stub).
//!
//! Marsaglia's "Birthday Spacings" test. Selects `m` random points (integers)
//! uniformly from [0, n) and sorts them. The spacings between adjacent points
//! are computed; the number of duplicate spacings should follow a Poisson
//! distribution with λ = m³ / (4n).
//!
//! Named after the birthday paradox — the expected number of collisions in
//! spacings grows predictably for a uniform distribution.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Birthday Spacings Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct BirthdaySpacingsTest {
    pub total_bytes: u64,
}

impl BirthdaySpacingsTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for BirthdaySpacingsTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: read 4-byte integers as random points in [0, 2^24); sort them;
        //       count duplicate spacings; compare count to Poisson(λ) using
        //       chi-square or KS test across multiple samples.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
