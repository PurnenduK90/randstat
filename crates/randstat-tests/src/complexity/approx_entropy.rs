//! NIST SP800-22 Ã‚Â§2.12 Ã¢â‚¬â€ Approximate Entropy test (stub).
//!
//! Compares the frequency of overlapping blocks of two consecutive lengths (m, m+1)
//! against the expected result for a random sequence.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Approximate entropy test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct ApproxEntropyTest {
    pub total_bits: u64,
}

impl ApproxEntropyTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for ApproxEntropyTest {
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
