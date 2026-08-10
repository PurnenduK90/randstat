//! NIST SP800-22 §2.9 — Maurer's Universal Statistical Test (stub).
//!
//! Detects whether a sequence can be significantly compressed without loss of
//! information. A significantly compressible sequence is considered non-random.
//!
//! The test works by dividing the bit string into `Q` initialisation blocks and
//! `K` test blocks of length `L` bits each, computing the sum of log₂ distances
//! between matching `L`-bit patterns, and comparing to the expected value for
//! a truly random sequence.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Maurer's Universal Statistical Test accumulator (stub).
///
/// Default parameters: `L = 7`, `Q = 1280`, `K` determined by stream length.
#[derive(Debug, Clone, Copy, Default)]
pub struct MaurersUniversalTest {
    pub total_bits: u64,
}

impl MaurersUniversalTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for MaurersUniversalTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: implement using a 2^L lookup table (table size 128 for L=7).
        //       Compute fn = (1/K) Σ log₂(dist) and compare to expected variance.
        //       p-value = erfc(|fn - expectedValue| / (sqrt(2) * sigma)).
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
