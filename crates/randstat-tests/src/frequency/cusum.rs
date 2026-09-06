//! NIST SP800-22 Ã‚Â§2.13 Ã¢â‚¬â€ Cumulative Sums (CUSUM) test (stub).
//!
//! Tests whether the cumulative sum of partial sequences is too large or too small
//! relative to what would be expected for a random sequence.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// CUSUM test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct CusumTest {
    pub total_bits: u64,
}

impl CusumTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for CusumTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        TestResult::NOT_IMPLEMENTED
    }
}
