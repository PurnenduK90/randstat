//! Dieharder: Squeeze Test (stub).
//!
//! Marsaglia's Squeeze test. Starts with a large integer k = 2³¹ and divides
//! it repeatedly by successive random uniform [0,1) values until k ≤ 1.
//! The number of divisions required is recorded. This count should follow a
//! known distribution (approximately Poisson-related) for a truly random source.
//!
//! A KS test across many such trials gives the final p-value.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Squeeze Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct SqueezeTest {
    pub total_bytes: u64,
}

impl SqueezeTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for SqueezeTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode bytes as f64 in [0, 1); for each trial, start with k=2^31
        //       and count how many divisions (by successive random values) until k≤1;
        //       collect 100_000 trial counts; apply KS test vs theoretical distribution.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
