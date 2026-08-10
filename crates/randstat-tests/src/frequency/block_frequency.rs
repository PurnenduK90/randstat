//! NIST SP800-22 §2.2 — Block Frequency test (stub).
//!
//! Divides the bit stream into non-overlapping blocks of M bits and tests
//! whether the proportion of 1s in each block is approximately M/2.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Block frequency test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct BlockFrequencyTest {
    pub total_bits: u64,
}

impl BlockFrequencyTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for BlockFrequencyTest {
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
