//! NIST SP800-22 Ã‚Â§2.15 Ã¢â‚¬â€ Random Excursions Variant Test (stub).
//!
//! A generalisation of the Random Excursions test (Ã‚Â§2.14) that tests 18 states
//! (`x Ã¢Ë†Ë† {Ã¢Ë†â€™9..Ã¢Ë†â€™1, +1..+9}`) and measures the total number of times the random
//! walk visits each state during any cycle, rather than binning by visit count.
//!
//! The variant test detects sequences where the cumulative sum walk spends too
//! much (or too little) time at any given state.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Random Excursions Variant Test accumulator (stub).
///
/// Requires `n Ã¢â€°Â¥ 1_000_000` bits (NIST Ã‚Â§2.15.6). Tests 18 states.
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
        // TODO: reuse the cumulative sum walk from Ã‚Â§2.14; for each of 18 states,
        //       count total visits ÃŽÂ¾(x, J); compute p-value = erfc(|ÃŽÂ¾-J| / Ã¢Ë†Å¡(2JÃÆ’Ã‚Â²)).
        TestResult::NOT_IMPLEMENTED
    }
}
