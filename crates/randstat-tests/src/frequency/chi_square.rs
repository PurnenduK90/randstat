//! Chi-Square byte-uniformity test.
//!
//! `ChiSquareTest` is the streaming accumulator. The evaluation formula lives in
//! [`randstat_core::algorithms::chi_square::chi_square_test`].

use randstat_core::algorithms::chi_square::chi_square_test;
use randstat_core::bitstream::byte_freq::ByteFreqTracker;
use randstat_core::traits::{StreamTest, TestResult};

/// Chi-square byte-uniformity streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChiSquareTest {
    pub tracker: ByteFreqTracker,
}

impl ChiSquareTest {
    pub const fn new() -> Self {
        Self {
            tracker: ByteFreqTracker::new(),
        }
    }
}

impl StreamTest for ChiSquareTest {
    fn update(&mut self, chunk: &[u8]) {
        self.tracker.update(chunk);
    }

    fn reset(&mut self) {
        self.tracker.reset();
    }

    fn evaluate(&self) -> TestResult {
        chi_square_test(&self.tracker.byte_counts, self.tracker.total_bytes)
    }
}
