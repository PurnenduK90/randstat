//! NIST SP800-22 §2.5 — Binary Matrix Rank test (stub).
//!
//! Checks for linear dependence among fixed-length substrings of the sequence
//! by computing the rank of non-overlapping binary matrices.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Binary matrix rank test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct BinaryMatrixRankTest {
    pub total_bits: u64,
}

impl BinaryMatrixRankTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for BinaryMatrixRankTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
