//! ENT-style Shannon Entropy test.
//!
//! `ShannonEntropyTest` is the streaming accumulator. The evaluation formula
//! lives in [`randstat_core::algorithms::shannon::shannon_score`].

use randstat_core::algorithms::shannon::shannon_score;
use randstat_core::bitstream::byte_freq::ByteFreqTracker;
use randstat_core::traits::{StreamTest, TestResult};

/// Shannon entropy streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct ShannonEntropyTest {
    pub tracker: ByteFreqTracker,
}

impl ShannonEntropyTest {
    pub const fn new() -> Self {
        Self {
            tracker: ByteFreqTracker::new(),
        }
    }
}

impl StreamTest for ShannonEntropyTest {
    fn update(&mut self, chunk: &[u8]) {
        self.tracker.update(chunk);
    }

    fn reset(&mut self) {
        self.tracker.reset();
    }

    fn evaluate(&self) -> TestResult {
        shannon_score(&self.tracker.byte_counts, self.tracker.total_bytes)
    }
}
