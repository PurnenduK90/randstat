//! Arithmetic Mean test.
//!
//! `ArithmeticMeanTest` is the streaming accumulator. The evaluation uses
//! [`randstat_core::bitstream::byte_freq::ByteFreqTracker::arithmetic_mean`]
//! directly since this is a simple linear statistic with no separate algorithm module.

use randstat_core::bitstream::byte_freq::ByteFreqTracker;
use randstat_core::traits::{StreamTest, TestResult};

/// Arithmetic mean streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct ArithmeticMeanTest {
    pub tracker: ByteFreqTracker,
}

impl ArithmeticMeanTest {
    pub const fn new() -> Self {
        Self {
            tracker: ByteFreqTracker::new(),
        }
    }
}

impl StreamTest for ArithmeticMeanTest {
    fn update(&mut self, chunk: &[u8]) {
        self.tracker.update(chunk);
    }

    fn reset(&mut self) {
        self.tracker.reset();
    }

    fn evaluate(&self) -> TestResult {
        let mean = self.tracker.arithmetic_mean();
        let diff = (mean - 127.5).abs();
        TestResult {
            statistic: mean,
            p_value: (1.0 - (diff / 5.0)).clamp(0.0, 1.0),
            passed: diff < 1.0,
        }
    }
}
