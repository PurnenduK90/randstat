//! Dieharder: OQSO Ã¢â‚¬â€ Overlapping Quadruples Sparse Occupancy Test (stub).
//!
//! A Marsaglia occupancy test. Generates 4-letter words from a 32-letter alphabet
//! (5 bits per letter) using overlapping 20-bit windows over the bit stream.
//! The number of missing words out of 32Ã¢ÂÂ´ = 1_048_576 possible words should
//! follow a known distribution for a truly random source.
//!
//! More sensitive than DNA because overlapping windows detect short-range correlations.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// OQSO Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct OqsoTest {
    pub total_bytes: u64,
}

impl OqsoTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for OqsoTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode bits as overlapping 5-bit letters; form overlapping 4-letter
        //       (20-bit) words; maintain a 2^20 bitset; count missing words;
        //       compare to expected Poisson distribution.
        TestResult::NOT_IMPLEMENTED
    }
}
