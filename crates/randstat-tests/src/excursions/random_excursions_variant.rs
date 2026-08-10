//! NIST SP800-22 §2.15 — Random Excursions Variant Test (stub).
//!
//! A generalisation of the Random Excursions test (§2.14) that tests 18 states
//! (`x ∈ {−9..−1, +1..+9}`) and measures the total number of times the random
//! walk visits each state during any cycle, rather than binning by visit count.
//!
//! The variant test detects sequences where the cumulative sum walk spends too
//! much (or too little) time at any given state.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Random Excursions Variant Test accumulator (stub).
///
/// Requires `n ≥ 1_000_000` bits (NIST §2.15.6). Tests 18 states.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomExcursionsVariantTest {
    pub total_bits: u64,
}

impl RandomExcursionsVariantTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for RandomExcursionsVariantTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: reuse the cumulative sum walk from §2.14; for each of 18 states,
        //       count total visits ξ(x, J); compute p-value = erfc(|ξ-J| / √(2Jσ²)).
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
