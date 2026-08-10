//! NIST SP800-22 §2.4 — Longest Run of Ones test (stub).
//!
//! Tests whether the longest run of 1-bits within the M-bit blocks of the
//! sequence is consistent with a random sequence.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Longest-run-of-ones test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct LongestRunTest {
    pub total_bits: u64,
}

impl LongestRunTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for LongestRunTest {
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
