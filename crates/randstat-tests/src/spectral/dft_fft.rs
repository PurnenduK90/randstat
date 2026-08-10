//! NIST SP800-22 §2.6 — Discrete Fourier Transform (DFT/FFT) spectral test (stub).
//!
//! Detects periodic features in the sequence that would indicate non-randomness
//! using the discrete Fourier transform.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// DFT/FFT spectral test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct DftTest {
    pub total_bits: u64,
}

impl DftTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for DftTest {
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
