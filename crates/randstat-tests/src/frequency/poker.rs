//! Poker Test accumulator (BSI AIS 31 Test T2 / DIEHARD).
//!
//! Counts frequencies of 4-bit nibbles (16 buckets) and computes chi-square uniformity.

use randstat_core::algorithms::poker::poker_test;
use randstat_core::traits::{StreamTest, TestResult};

/// Poker Test (4-bit nibble) streaming accumulator.
#[derive(Debug, Clone, Copy)]
pub struct PokerTest {
    pub nibble_counts: [u64; 16],
    pub total_nibbles: u64,
}

impl PokerTest {
    pub const ZERO: Self = Self {
        nibble_counts: [0; 16],
        total_nibbles: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }
}

impl Default for PokerTest {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamTest for PokerTest {
    fn update(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            let hi = ((byte >> 4) & 0x0F) as usize;
            let lo = (byte & 0x0F) as usize;
            self.nibble_counts[hi] += 1;
            self.nibble_counts[lo] += 1;
            self.total_nibbles += 2;
        }
    }

    fn reset(&mut self) {
        self.nibble_counts = [0; 16];
        self.total_nibbles = 0;
    }

    fn evaluate(&self) -> TestResult {
        poker_test(&self.nibble_counts, self.total_nibbles)
    }
}
