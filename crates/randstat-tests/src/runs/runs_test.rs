//! NIST SP800-22 §2.3 — Runs test (stub).
//!
//! Tests whether the number of runs (uninterrupted sequences of identical bits)
//! is consistent with a random sequence.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Runs test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct RunsTest {
    pub total_bits: u64,
}

impl RunsTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for RunsTest {
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
