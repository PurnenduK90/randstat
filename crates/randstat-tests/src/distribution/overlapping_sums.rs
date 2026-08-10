//! Dieharder: Overlapping Sums Test (stub).
//!
//! Generates overlapping sums of 100 consecutive uniform [0,1) values.
//! By the Central Limit Theorem, each sum should be approximately normally
//! distributed (μ=50, σ²=100/12). The resulting distribution of sums is
//! tested against a Kolmogorov-Smirnov normal CDF.
//!
//! Detects failure in the CLT assumption — i.e., when values are not
//! independently and uniformly distributed.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Overlapping Sums Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct OverlappingSumsTest {
    pub total_bytes: u64,
}

impl OverlappingSumsTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for OverlappingSumsTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode bytes as f64 values in [0, 1); compute overlapping
        //       windows of 100 consecutive values (slide by 1 each step);
        //       collect the sums and apply a KS test against Normal(50, 100/12).
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
