//! Dieharder: 2D Minimum Distance Test (stub).
//!
//! Places `n` random points in a 10000×10000 square and finds the minimum
//! distance between any two points. The square of this minimum distance
//! should follow an exponential distribution with mean λ = 0.995 / n²  × 10^8.
//!
//! A KS test on the resulting CDF across multiple samples gives the p-value.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// 2D Minimum Distance Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct MinimumDistance2DTest {
    pub total_bytes: u64,
}

impl MinimumDistance2DTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for MinimumDistance2DTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode pairs of 4-byte floats as (x, y) ∈ [0, 10000)²;
        //       compute all pairwise distances (O(n²)); find minimum;
        //       compare d²_min CDF to Exp(λ) using KS statistic.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
