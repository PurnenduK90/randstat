//! NIST SP800-22 Ã‚Â§2.14 Ã¢â‚¬â€ Random Excursions Test (stub).
//!
//! Determines whether the number of visits to a particular state (value `x`)
//! within a random walk is consistent with the distribution expected for a
//! truly random sequence.
//!
//! The random walk is constructed from the Ã‚Â±1 bit sequence (0Ã¢â€ â€™Ã¢Ë†â€™1, 1Ã¢â€ â€™+1).
//! States `x Ã¢Ë†Ë† {Ã¢Ë†â€™4, Ã¢Ë†â€™3, Ã¢Ë†â€™2, Ã¢Ë†â€™1, +1, +2, +3, +4}` are tested.
//! Each state produces a separate p-value; all must pass.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Random Excursions Test accumulator (stub).
///
/// Requires `n Ã¢â€°Â¥ 1_000_000` bits for reliable results (NIST Ã‚Â§2.14.6).
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomExcursionsTest {
    pub total_bits: u64,
}

impl RandomExcursionsTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for RandomExcursionsTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: build the cumulative sum random walk S_k; identify cycles
        //       (subsequences between returns to state 0); for each state x,
        //       count the number of cycles containing exactly k visits (k=1..5+);
        //       compute chi-square vs theoretical Markov-chain distribution.
        TestResult::NOT_IMPLEMENTED
    }
}
