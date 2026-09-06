//! NIST SP800-22 Ã‚Â§2.10 Ã¢â‚¬â€ Linear Complexity (Berlekamp-Massey) test (stub).
//!
//! Tests whether the linear complexity of the sequence (length of the shortest
//! LFSR that generates it) is consistent with a random sequence.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Berlekamp-Massey linear complexity test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct BerlekampMasseyTest {
    pub total_bits: u64,
}

impl BerlekampMasseyTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for BerlekampMasseyTest {
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
